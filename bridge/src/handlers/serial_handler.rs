use crate::controller;
use crate::controller::message::{MessageBody, MAX_MESSAGE_BODY_LENGTH};
use crate::handlers::message::Message;
use futures::stream::StreamExt;
use futures::SinkExt;
use slip_codec::tokio::SlipCodec;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio::{select, spawn};
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Decoder;
use tokio_util::sync::CancellationToken;

const BAUD_RATE: u32 = 9600;

#[derive(Debug, Error)]
pub enum SerialHandlerError {
    #[error("Serial error")]
    SerialPortError(#[from] tokio_serial::Error),
}

pub async fn start(
    serial_port: String,
    sender: Sender<Message>,
    cancellation_token: CancellationToken,
) -> Result<JoinHandle<()>, SerialHandlerError> {
    let serial_stream = tokio_serial::new(serial_port.clone(), BAUD_RATE).open_native_async()?;
    let mut framed_port = SlipCodec::new().framed(serial_stream);
    let mut receiver = sender.subscribe();
    Ok(spawn(async move {
        let mut commands = vec![];
        loop {
            select! {
                message = framed_port.next() => {
                    // TODO(Roel): is there some serial connection error that we should handle?
                    match message {
                        Some(Ok(message)) => {
                            if let Ok(message) = controller::message::Message::try_from(&message[..]) {
                                sender.send(Message::ReceivedFromController(message.body))
                                .unwrap_or_else(|_| unreachable!());
                            }
                        }
                        Some(Err(_)) => {
                            // TODO(Roel): What to do on err?
                            eprintln!("SLIP Decoding error");
                        }
                        None => break, // TODO(Roel): what to do on none?
                    }
                }
                message = receiver.recv() => {
                    match message {
                        Ok(Message::SendToController(body)) => {
                            match body {
                                MessageBody::Command { commands: mut commands2 } => {
                                    commands.append(commands2.as_mut());
                                }
                                _ => {
                                    let bytes: Vec<u8> = (&controller::message::Message::new(body)).into();
                                    framed_port.send(bytes.into()).await
                                    .unwrap_or_else(|_| unreachable!());
                                }
                            }
                        }
                        Ok(_) => {},
                        Err(_) => break, // TODO(Roel): what to do on err?
                    }
                }
                _ = sleep(Duration::from_millis(1)) => {
                    if commands.is_empty() {
                        continue;
                    }
                    for commands_chunk in commands.chunks(MAX_MESSAGE_BODY_LENGTH) {
                        let message = MessageBody::Command {
                            commands: commands_chunk.to_vec(),
                        };
                        let bytes: Vec<u8> = (&controller::message::Message::new(message)).into();
                        framed_port.send(bytes.into()).await
                        .unwrap_or_else(|_| unreachable!());
                    }
                    commands.clear();
                }
                _ = cancellation_token.cancelled() => break,
            }
        }
    }))
}
