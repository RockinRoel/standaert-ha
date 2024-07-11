use crate::controller;
use crate::controller::message::{MessageBody, MAX_MESSAGE_BODY_LENGTH};
use crate::handlers::message::Message;
use futures::stream::StreamExt;
use futures::SinkExt;
use slip_codec::tokio::SlipCodec;
use std::time::Duration;
use slip_codec::SlipError;
use thiserror::Error;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio::{select, spawn};
use tokio::sync::broadcast::error::RecvError;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::bytes::Bytes;
use tokio_util::codec::{Decoder, Framed};
use tokio_util::sync::CancellationToken;
use crate::controller::command::Command;
use crate::handlers::serial_handler::HandleResult::{Break, Continue};

const BAUD_RATE: u32 = 9600;

#[derive(Debug, Error)]
pub enum SerialHandlerError {
    #[error("Serial error")]
    SerialPortError(#[from] tokio_serial::Error),
}

struct SerialHandler {
    framed_port: Framed<SerialStream, SlipCodec>,
    commands_buffer: Vec<Command>,
    rx: Receiver<Message>,
    tx: Sender<Message>,
    cancellation_token: CancellationToken,
}

#[derive(Eq, PartialEq)]
enum HandleResult {
    Break,
    Continue,
}

pub async fn start(
    serial_port: String,
    sender: Sender<Message>,
    cancellation_token: CancellationToken,
) -> Result<JoinHandle<()>, SerialHandlerError> {
    let serial_stream = tokio_serial::new(serial_port.clone(), BAUD_RATE).open_native_async()?;
    let framed_port = SlipCodec::new().framed(serial_stream);
    let receiver = sender.subscribe();

    let mut serial_handler = SerialHandler {
        framed_port,
        commands_buffer: vec![],
        rx: receiver,
        tx: sender,
        cancellation_token,
    };

    Ok(spawn(async move { serial_handler.run().await }))
}

impl SerialHandler {
    async fn run(&mut self) {
        loop {
            select! {
                message = self.framed_port.next() => if self.handle_serial_message(message) == Break {
                    break;
                },
                message = self.rx.recv() => if self.handle_broadcast_message(message).await == Break {
                    break;
                },
                _ = sleep(Duration::from_millis(1)) => self.send_commands().await,
                _ = self.cancellation_token.cancelled() => break,
            }
        }
    }

    fn handle_serial_message(&mut self, message: Option<Result<Bytes, SlipError>>) -> HandleResult {
        match message {
            // TODO(Roel): is there some serial connection error that we should handle?
            Some(Ok(message)) => {
                if let Ok(message) = controller::message::Message::try_from(&message[..]) {
                    self.tx.send(Message::ReceivedFromController(message.body))
                        .unwrap_or_else(|_| unreachable!());
                }
            }
            Some(Err(_)) => {
                // TODO(Roel): What to do on err?
                eprintln!("SLIP Decoding error");
            }
            None => return Break, // TODO(Roel): what to do on none?
        }
        Continue
    }

    async fn handle_broadcast_message(&mut self, message: Result<Message, RecvError>) -> HandleResult {
        match message {
            Ok(Message::SendToController(body)) => {
                match body {
                    MessageBody::Command { mut commands } => {
                        self.commands_buffer.append(commands.as_mut());
                    }
                    _ => {
                        let bytes: Vec<u8> = (&controller::message::Message::new(body)).into();
                        self.framed_port.send(bytes.into()).await
                            .unwrap_or_else(|_| unreachable!());
                    }
                }
            }
            Ok(_) => {},
            Err(_) => return Break, // TODO(Roel): what to do on err?
        }
        Continue
    }
    
    async fn send_commands(&mut self) {
        if self.commands_buffer.is_empty() {
            return;
        }
        for commands_chunk in self.commands_buffer.chunks(MAX_MESSAGE_BODY_LENGTH) {
            let message = MessageBody::Command {
                commands: commands_chunk.to_vec(),
            };
            let bytes: Vec<u8> = (&controller::message::Message::new(message)).into();
            self.framed_port.send(bytes.into()).await
                .unwrap_or_else(|_| unreachable!());
        }
        self.commands_buffer.clear();
    }
}
