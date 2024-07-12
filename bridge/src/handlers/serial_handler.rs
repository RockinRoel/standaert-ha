use crate::controller;
use crate::controller::command::Command;
use crate::controller::message::{MessageBody, MAX_MESSAGE_BODY_LENGTH};
use crate::handlers::message::Message;
use crate::handlers::serial_handler::HandleResult::{Break, Continue};
use futures::stream::StreamExt;
use futures::SinkExt;
use slip_codec::tokio::SlipCodec;
use slip_codec::SlipError;
use std::time::Duration;
use log::error;
use thiserror::Error;
use tokio::select;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::time::sleep;
use tokio_graceful_shutdown::SubsystemHandle;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::bytes::Bytes;
use tokio_util::codec::{Decoder, Framed};

const BAUD_RATE: u32 = 9600;

#[derive(Debug, Error)]
pub enum SerialHandlerError {
    #[error("Serial error")]
    SerialPortError(#[from] tokio_serial::Error),
}

struct SerialHandler {
    subsys: SubsystemHandle,
    framed_port: Framed<SerialStream, SlipCodec>,
    commands_buffer: Vec<Command>,
    rx: Receiver<Message>,
    tx: Sender<Message>,
}

#[derive(Eq, PartialEq)]
enum HandleResult {
    Break,
    Continue,
}

pub async fn run(
    subsys: SubsystemHandle,
    serial_port: &str,
    sender: Sender<Message>,
) -> Result<(), SerialHandlerError> {
    let serial_stream = tokio_serial::new(serial_port, BAUD_RATE).open_native_async()?;
    let framed_port = SlipCodec::new().framed(serial_stream);
    let receiver = sender.subscribe();

    let mut serial_handler = SerialHandler {
        subsys,
        framed_port,
        commands_buffer: vec![],
        rx: receiver,
        tx: sender,
    };

    serial_handler.run().await
}

impl SerialHandler {
    async fn run(&mut self) -> Result<(), SerialHandlerError> {
        loop {
            select! {
                _ = self.subsys.on_shutdown_requested() => break,
                message = self.framed_port.next() => if self.handle_serial_message(message) == Break {
                    break;
                },
                message = self.rx.recv() => if self.handle_broadcast_message(message).await == Break {
                    break;
                },
                _ = sleep(Duration::from_millis(1)) => self.send_commands().await,
            }
        }
        Ok(())
    }

    fn handle_serial_message(&mut self, message: Option<Result<Bytes, SlipError>>) -> HandleResult {
        match message {
            // TODO(Roel): is there some serial connection error that we should handle?
            Some(Ok(message)) => {
                if let Ok(message) = controller::message::Message::try_from(&message[..]) {
                    self.tx
                        .send(Message::ReceivedFromController(message.body))
                        .unwrap_or_else(|_| unreachable!());
                }
            }
            Some(Err(_)) => {
                // TODO(Roel): What to do on err?
                error!("SLIP Decoding error");
            }
            None => return Break, // TODO(Roel): what to do on none?
        }
        Continue
    }

    async fn handle_broadcast_message(
        &mut self,
        message: Result<Message, RecvError>,
    ) -> HandleResult {
        match message {
            Ok(Message::SendToController(body)) => match body {
                MessageBody::Command { mut commands } => {
                    self.commands_buffer.append(commands.as_mut());
                }
                _ => {
                    let bytes: Vec<u8> = (&controller::message::Message::new(body)).into();
                    self.framed_port
                        .send(bytes.into())
                        .await
                        .unwrap_or_else(|_| unreachable!());
                }
            },
            Ok(_) => {}
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
            self.framed_port
                .send(bytes.into())
                .await
                .unwrap_or_else(|_| unreachable!());
        }
        self.commands_buffer.clear();
    }
}
