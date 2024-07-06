pub mod controller;
pub mod handlers;
pub mod shal;

use crate::controller::command::Command;
use crate::controller::message::MessageBody;
use crate::handlers::handler::Handler;
use crate::handlers::handler_chain::HandlerChain;
use crate::handlers::logger::Logger;
use crate::handlers::mqtt_handler::MqttHandler;
use crate::handlers::programmer::Programmer;
use crate::handlers::serial_handler::SerialHandler;
use anyhow::Result;
use clap::Parser;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use tokio::signal;
use tokio::time::sleep;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// MQTT broker host
    #[arg(long, env = "SHA_MQTT_URL")]
    mqtt_url: Option<String>,

    /// MQTT broker user
    #[arg(long, env = "SHA_MQTT_USER")]
    mqtt_user: Option<String>,

    /// MQTT broker password
    #[arg(long, env = "SHA_MQTT_PASSWORD")]
    mqtt_password: Option<String>,

    /// Home assistant discovery prefix
    #[arg(long, default_value = "homeassistant", env = "SHA_DISCOVERY_PREFIX")]
    prefix: String,

    /// Serial device
    #[arg(long, env = "SHA_SERIAL_DEVICE")]
    serial: Option<String>,

    /// Program location
    #[arg(long, env = "SHAL_PROGRAM")]
    program: Option<String>,

    /// Verbose
    #[arg(long, default_value_t = false, env = "SHA_DEBUG")]
    debug: bool,
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(mqtt_url) = &self.mqtt_url {
            writeln!(f, "  MQTT options:")?;
            writeln!(f, "    URL: {}", mqtt_url)?;
            if let Some(mqtt_user) = &self.mqtt_user {
                writeln!(f, "    user: {}", mqtt_user)?;
            } else {
                writeln!(f, "    user: <none>")?;
            }
            writeln!(
                f,
                "    password: {}",
                if self.mqtt_password.is_some() {
                    "***"
                } else {
                    "<none>"
                }
            )?;
            writeln!(f, "    prefix: {}", self.prefix)?;
        } else {
            writeln!(f, "  MQTT: disabled")?;
        }
        if let Some(serial) = &self.serial {
            writeln!(f, "  Serial port: {}", serial)?;
        } else {
            writeln!(f, "  Serial: <disabled>")?;
        }
        if let Some(program) = &self.program {
            writeln!(f, "  Program path: {}", program)?;
        } else {
            writeln!(f, "  Program: <disabled>")?;
        }
        writeln!(
            f,
            "  Debug: {}",
            if self.debug { "enabled" } else { "disabled" }
        )
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Starting SHA bridge with arguments:\n{}", args);

    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

    let mut chain = HandlerChain::new();

    if args.debug {
        chain.add_handler(Logger);
    }

    if let Some(mqtt_url) = &args.mqtt_url {
        let mut credentials = None;
        if let (Some(user), Some(password)) = (&args.mqtt_user, &args.mqtt_password) {
            credentials = Some((user.clone(), password.clone()));
        }
        let handler = MqttHandler::new(
            mqtt_url.clone(),
            credentials,
            args.prefix,
            sender.clone(),
        )?;
        chain.add_handler(handler);

        sleep(Duration::from_secs(1)).await;
    }

    if let Some(serial_port) = &args.serial {
        chain.add_handler(SerialHandler::new(serial_port.clone(), sender.clone()));

        sleep(Duration::from_secs(1)).await;
    }

    if let Some(program) = &args.program {
        chain.add_handler(Programmer::new(program.clone(), sender.clone()));

        sleep(Duration::from_secs(1)).await;
    }

    sender
        .send(handlers::message::Message::ReloadProgram)
        .unwrap_or_else(|_| unreachable!());

    sleep(Duration::from_secs(1)).await;

    sender
        .send(handlers::message::Message::SendToController(
            MessageBody::Command {
                commands: vec![Command::Refresh],
            },
        ))
        .unwrap_or_else(|_| unreachable!());

    sleep(Duration::from_secs(1)).await;

    loop {
        tokio::select! {
            message = receiver.recv() => {
                if let Some(message) = message {
                    chain.handle(&message);
                    if message == handlers::message::Message::Stop {
                        return Ok(())
                    }
                } else {
                    // TODO(Roel): ???
                }
            },
            _ = signal::ctrl_c() => {
                sender.send(handlers::message::Message::Stop)
                .unwrap_or_else(|_| unreachable!());
            }
        }
    }
}
