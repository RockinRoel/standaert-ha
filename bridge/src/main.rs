pub mod controller;
pub mod handlers;
pub mod shal;

use crate::handlers::message::Message;
use crate::handlers::{logger, mqtt_handler, programmer, refresher, serial_handler};
use anyhow::Result;
use clap::Parser;
use futures::future::join_all;
use std::fmt::{Display, Formatter};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinHandle;
use tokio::{select, signal};
use tokio_util::sync::CancellationToken;

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

    let (sender, _receiver) = broadcast::channel(100);
    let cancellation_token = CancellationToken::new();

    let mut tasks = vec![];

    let result = spawn_tasks(&mut tasks, &args, &sender, cancellation_token.clone()).await;

    if let Ok(()) = result {
        select! {
            _ = cancellation_token.cancelled() => {}
            _ = signal::ctrl_c() => cancellation_token.cancel(),
        }
    } else {
        // Error occurred, cleanly terminate all started tasks
        cancellation_token.cancel();
    }

    // TODO(Roel): tasks may exit with error?
    join_all(tasks).await;

    result
}

async fn spawn_tasks(
    tasks: &mut Vec<JoinHandle<()>>,
    args: &Args,
    sender: &Sender<Message>,
    cancellation_token: CancellationToken,
) -> Result<()> {
    if args.debug {
        tasks.push(logger::start(
            sender.subscribe(),
            cancellation_token.clone(),
        ));
    }

    if let Some(mqtt_url) = &args.mqtt_url {
        let mut credentials = None;
        if let (Some(user), Some(password)) = (&args.mqtt_user, &args.mqtt_password) {
            credentials = Some((user.clone(), password.clone()));
        }
        let task = mqtt_handler::start(
            mqtt_url.clone(),
            credentials,
            args.prefix.clone(),
            sender.clone(),
            cancellation_token.clone(),
        )
        .await?;
        tasks.push(task);

        // TODO(Roel): we should wait until we're done with init?
    }

    if let Some(serial_port) = &args.serial {
        let task = serial_handler::start(
            serial_port.clone(),
            sender.clone(),
            cancellation_token.clone(),
        )
        .await?;
        tasks.push(task);
    }

    if let Some(program) = &args.program {
        let task =
            programmer::start(program.clone(), sender.clone(), cancellation_token.clone()).await?;
        tasks.push(task);
    }

    tasks.push(refresher::start(sender.clone(), cancellation_token.clone()));

    Ok(())
}
