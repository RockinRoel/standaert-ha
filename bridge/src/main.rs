pub mod controller;
pub mod handlers;
pub mod shal;

use std::sync::mpsc;
use crate::handlers::handler::Handler;
use crate::handlers::handler_chain::HandlerChain;
use crate::controller::command::Command;
use crate::controller::message::{Message, MessageBody};
use anyhow::Result;
use clap::Parser;
use crc::{Crc, CRC_16_XMODEM};
use futures::stream::StreamExt;
use futures::SinkExt;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use slip_codec::tokio::SlipCodec;
use std::time::Duration;
use tokio::signal;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::sleep;
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Decoder;
use tokio_util::sync::CancellationToken;
use crate::handlers::logger::Logger;
use crate::handlers::programmer::Programmer;
use crate::handlers::serial_handler::SerialHandler;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// MQTT broker host
    #[arg(long, env = "SHA_MQTT_HOST")]
    mqtt_host: Option<String>,

    /// MQTT broker user
    #[arg(long, env = "SHA_MQTT_USER")]
    mqtt_user: Option<String>,

    /// MQTT broker password
    #[arg(long, env = "SHA_MQTT_PASSWORD")]
    mqtt_password: Option<String>,

    /// Use TLS for MQTT?
    #[arg(long, default_value_t = false, env = "SHA_MQTT_USE_TLS")]
    mqtt_use_tls: bool,

    /// Home assistant discovery prefix
    #[arg(long, default_value = "homeassistant", env = "SHA_DISCOVERY_PREFIX")]
    prefix: String,

    /// Node id
    #[arg(long, default_value = "standaert_ha", env = "SHA_NODE_ID")]
    id: String,

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

    println!("Args: {:?}", args);

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

    if let Some(mqtt_host) = &args.mqtt_host {
        // TODO(Roel): add MQTT
    }

    sender.send(handlers::message::Message::ReloadProgram).expect("Could not send reload?"); // TODO(Roel): ???

    sender.send(handlers::message::Message::SendToController(MessageBody::Command {
        commands: vec![Command::Refresh],
    })).expect("Could not send refresh?"); // TODO(Roel): ???

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
                sender.send(handlers::message::Message::Stop).expect("Could not send stop?"); // TODO(Roel): ???
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
