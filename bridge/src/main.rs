pub mod controller;
pub mod handlers;
pub mod shal;

use crate::controller::command::Command;
use crate::controller::message::{Message, MessageBody};
use crate::handlers::handler::Handler;
use crate::handlers::handler_chain::HandlerChain;
use crate::handlers::logger::Logger;
use crate::handlers::mqtt_handler::MqttHandler;
use crate::handlers::programmer::Programmer;
use crate::handlers::serial_handler::SerialHandler;
use anyhow::Result;
use clap::Parser;
use futures::stream::StreamExt;
use futures::SinkExt;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use slip_codec::tokio::SlipCodec;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use tokio::signal;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::sleep;
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Decoder;
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

const BAUD_RATE: u32 = 115200;

async fn do_controller_comms(
    cancellation_token: CancellationToken,
    serial_port: &str,
    mut receiver_to_controller: UnboundedReceiver<Message>,
    sender_to_bridge: Sender<Message>,
) -> Result<()> {
    let serial_port = tokio_serial::new(serial_port, BAUD_RATE).open_native_async()?;
    let mut framed_port = SlipCodec::new().framed(serial_port);
    // let crc = Crc::<u16>::new(&CRC_16_XMODEM);

    loop {
        tokio::select! {
            message = framed_port.next() => {
                if let Some(Ok(message)) = message {
                    if let Ok(message) = (&message[..]).try_into() {
                        sender_to_bridge.send(message)?;
                    } else {
                        // Error
                    }
                } else {
                    // ???
                }
            },
            message = receiver_to_controller.recv() => {
                if let Some(message) = message {
                    let bytes: Vec<u8> = (&message).into();
                    println!("Sending: {:?}", bytes);
                    framed_port.send(bytes.into()).await.expect("Error?");
                } else {
                    // ???
                }
            },
            _ = cancellation_token.cancelled() => break,
        }
    }

    Ok(())
}

async fn do_mqtt_comms(
    cancellation_token: CancellationToken,
    mqtt_options: MqttOptions,
    prefix: &str,
    mut receiver_to_bridge: Receiver<Message>,
    sender_to_controller: UnboundedSender<Message>,
) -> Result<()> {
    let (mut client, mut event_loop) = AsyncClient::new(mqtt_options, 10);
    client
        .subscribe(format!("{}/#", prefix), QoS::AtMostOnce)
        .await?;

    loop {
        tokio::select! {
            event = event_loop.poll() => {
                println!("Received = {:?}", event);
            },
            _ = cancellation_token.cancelled() => break,
        }
    }

    Ok(())
}

async fn blinker(
    cancellation_token: CancellationToken,
    sender_to_controller: UnboundedSender<Message>,
) -> Result<()> {
    loop {
        tokio::select! {
            _ = sleep(Duration::from_millis(1000)) => {
                sender_to_controller.send(Message::new(MessageBody::Command {
                    commands: vec![Command::Toggle(12)],
                }))?;
            },
            _ = cancellation_token.cancelled() => break,
        }
    }

    Ok(())
}

async fn do_logging(
    cancellation_token: CancellationToken,
    mut receiver_to_bridge: Receiver<Message>,
) -> Result<()> {
    loop {
        tokio::select! {
            message = receiver_to_bridge.recv() => {
                println!("Message = {:?}", message);
            },
            _ = cancellation_token.cancelled() => break,
        }
    }

    Ok(())
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

    if let Some(serial_port) = &args.serial {
        chain.add_handler(SerialHandler::new(serial_port.clone(), sender.clone()));
    }

    if let Some(program) = &args.program {
        chain.add_handler(Programmer::new(program.clone(), sender.clone()));
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
    }

    sender
        .send(handlers::message::Message::ReloadProgram)
        .unwrap_or_else(|_| unreachable!());

    sender
        .send(handlers::message::Message::SendToController(
            MessageBody::Command {
                commands: vec![Command::Refresh],
            },
        ))
        .unwrap_or_else(|_| unreachable!());

    let mut hup_stream = signal(SignalKind::hangup())?;

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
            _ = hup_stream.recv() => {
                sender.send(handlers::message::Message::ReloadProgram)
                .unwrap_or_else(|_| unreachable!());
            },
            _ = signal::ctrl_c() => {
                sender.send(handlers::message::Message::Stop)
                .unwrap_or_else(|_| unreachable!());
            }
        }
    }

    // let mqtt_port = if args.mqtt_use_tls { 8883 } else { 1883 };
    // let mut mqtt_options = MqttOptions::new(args.id, args.mqtt_host, mqtt_port);
    // if let (Some(username), Some(password)) = (args.mqtt_user, args.mqtt_password) {
    //     mqtt_options.set_credentials(username, password);
    // }
    // let (mut client, mut event_loop) = AsyncClient::new(mqtt_options, 10);
    // client
    //     .subscribe(format!("{}/#", args.prefix), QoS::AtMostOnce)
    //     .await?;

    // let (sender_to_controller, receiver_to_controller) = mpsc::unbounded_channel::<Message>();
    // let (sender_to_bridge, receiver_to_bridge) = broadcast::channel::<Message>(16);
    //
    // let cancellation_token = CancellationToken::new();
    //
    // let mut tasks = vec![];
    //
    // let token = cancellation_token.clone();
    // tasks.push(tokio::spawn(async move {
    //     do_controller_comms(
    //         token,
    //         &args.serial,
    //         receiver_to_controller,
    //         sender_to_bridge,
    //     )
    //     .await
    // }));
    // /*
    // let token = cancellation_token.clone();
    // let receiver = receiver_to_bridge.resubscribe();
    // tasks.push(tokio::spawn(async move {
    //     do_mqtt_comms(token, mqtt_options, &args.prefix, receiver, sender_to_controller).await
    // }));
    //  */
    // let token = cancellation_token.clone();
    // tasks.push(tokio::spawn(async move {
    //     do_logging(token, receiver_to_bridge).await
    // }));

    // let token = cancellation_token.clone();
    // let sender = sender_to_controller.clone();
    // tasks.push(tokio::spawn(async move {
    //     blinker(token, sender).await
    // }));

    // tokio::select! {
    //     _ = do_controller_comms(&args.serial, receiver_to_controller, sender_to_bridge) => {},
    //     _ = signal::ctrl_c() => {
    //         println!("Ctrl-C pressed, shutting down...");
    //     },
    //     _ = recv_mqtt(client, event_loop) => {},
    // }
    // recv_messages(&args.serial).await?;

    // sender_to_controller.send(Message::new(MessageBody::Command {
    //     commands: vec![Command::Refresh],
    // }))?;
    //
    // if let Some(program) = program {
    //     let header = program[0..8].try_into()?;
    //     let code = &program[8..];
    //     sender_to_controller.send(Message::new(MessageBody::ProgramStart { header }))?;
    //     let mut pos = 0;
    //     while pos < code.len() {
    //         if code.len() - pos > Message::max_message_body_length() {
    //             sender_to_controller.send(Message::new(MessageBody::ProgramData {
    //                 code: code[pos..pos + Message::max_message_body_length()].into(),
    //             }))?;
    //             pos += Message::max_message_body_length();
    //         } else {
    //             sender_to_controller.send(Message::new(MessageBody::ProgramEnd {
    //                 code: code[pos..].into(),
    //             }))?;
    //             pos += code[pos..].len();
    //         }
    //     }
    // }
    //
    // signal::ctrl_c().await?;
    // println!("Shutting down...");
    //
    // cancellation_token.cancel();
    // for task in tasks {
    //     task.await??;
    // }
    //
    // Ok(())
}
