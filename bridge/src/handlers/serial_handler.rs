use crate::controller;
use crate::controller::command::Command;
use crate::controller::message::{MessageBody, MAX_MESSAGE_BODY_LENGTH};
use crate::handlers::message::Message;
use futures::stream::StreamExt;
use futures::SinkExt;
use log::{error, warn};
use slip_codec::tokio::SlipCodec;
use slip_codec::SlipError;
use std::time::Duration;
use thiserror::Error;
use tokio::select;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::time::sleep;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::bytes::Bytes;
use tokio_util::codec::{Decoder, Framed};
use tokio_util::sync::CancellationToken;
use crate::handlers::serial_handler::SerialHandlerError::NoMoreMessages;

const BAUD_RATE: u32 = 9600;

#[derive(Debug, Error)]
pub enum SerialHandlerError {
    #[error("Serial error")]
    SerialPortError(#[from] tokio_serial::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("No more serial messages")]
    NoMoreMessages,
}

pub struct SerialHandler {
    cancellation_token: CancellationToken,
    framed_port: Framed<SerialStream, SlipCodec>,
    commands_buffer: Vec<Command>,
    rx: Receiver<Message>,
    tx: Sender<Message>,
}

impl SerialHandler {
    pub async fn new(
        cancellation_token: CancellationToken,
        serial_port: &str,
        sender: Sender<Message>,
    ) -> Result<Self, SerialHandlerError> {
        let serial_stream = tokio_serial::new(serial_port, BAUD_RATE).open_native_async()?;
        let framed_port = SlipCodec::new().framed(serial_stream);
        let receiver = sender.subscribe();
        Ok(Self {
            cancellation_token,
            framed_port,
            commands_buffer: vec![],
            rx: receiver,
            tx: sender,
        })
    }

    pub async fn run(mut self) -> Result<(), SerialHandlerError> {
        loop {
            select! {
                _ = self.cancellation_token.cancelled() => break,
                message = self.framed_port.next() => self.handle_serial_message(message)?,
                message = self.rx.recv() => match message {
                   Ok(message) => self.handle_broadcast_message(message).await?,
                   Err(RecvError::Lagged(n)) => warn!("Serial handler skipped {n} messages!"),
                   Err(RecvError::Closed) => break,
                },
                _ = sleep(Duration::from_millis(1)) => self.send_commands().await,
            }
        }
        Ok(())
    }

    fn handle_serial_message(&mut self, message: Option<Result<Bytes, SlipError>>) -> Result<(), SerialHandlerError> {
        match message {
            Some(Ok(message)) => {
                match controller::message::Message::try_from(&message[..]) {
                    Ok(message) => {
                        self.tx
                            .send(Message::ReceivedFromController(message.body))
                            .unwrap_or_else(|_| unreachable!());
                    }
                    Err(e) => error!("Failed to decode serial message: {e}"),
                }
                Ok(())
            }
            Some(Err(e)) => Err(std::io::Error::from(e))?,
            None => Err(NoMoreMessages),
        }
    }

    async fn handle_broadcast_message(
        &mut self,
        message: Message,
    ) -> Result<(), SerialHandlerError> {
        if let Message::SendToController(body) = message {
            match body {
                MessageBody::Command { mut commands } => {
                    self.commands_buffer.append(commands.as_mut());
                }
                _ => {
                    let bytes: Vec<u8> = (&controller::message::Message::new(body)).into();
                    self.framed_port.send(bytes.into()).await.map_err(std::io::Error::from)?;
                }
            }
        }
        Ok(())
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
