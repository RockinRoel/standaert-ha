use crate::controller;
use crate::controller::message::{MessageBody, MAX_MESSAGE_BODY_LENGTH};
use crate::handlers::handler::HandleResult::Continue;
use crate::handlers::handler::{HandleResult, Handler};
use crate::handlers::message::Message;
use futures::stream::StreamExt;
use futures::SinkExt;
use slip_codec::tokio::SlipCodec;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Decoder;
use tokio_util::sync::CancellationToken;

const BAUD_RATE: u32 = 9600;

pub struct SerialHandler {
    serial_sender: UnboundedSender<MessageBody>,
    cancellation_token: CancellationToken,
}

struct SerialHandlerTask {
    serial_port: String,
    sender: UnboundedSender<Message>,
    serial_receiver: UnboundedReceiver<MessageBody>,
    cancellation_token: CancellationToken,
}

impl SerialHandler {
    pub fn new(serial_port: String, sender: UnboundedSender<Message>) -> (Self, JoinHandle<()>) {
        let (serial_sender, serial_receiver) =
            tokio::sync::mpsc::unbounded_channel::<MessageBody>();
        let cancellation_token = CancellationToken::new();
        let mut task = SerialHandlerTask {
            serial_port,
            sender,
            serial_receiver,
            cancellation_token: cancellation_token.clone(),
        };
        let task = tokio::spawn(async move {
            task.run().await;
        });
        (
            Self {
                serial_sender,
                cancellation_token,
            },
            task,
        )
    }
}

impl SerialHandlerTask {
    async fn run(&mut self) {
        let serial_stream = tokio_serial::new(self.serial_port.clone(), BAUD_RATE)
            .open_native_async()
            .expect("Failed to open serial port!");
        let mut framed_port = SlipCodec::new().framed(serial_stream);
        loop {
            tokio::select! {
                message = framed_port.next() => {
                    // TODO(Roel): is there some serial connection error that we should handle?
                    match message {
                        Some(Ok(message)) => {
                            if let Ok(message) = controller::message::Message::try_from(&message[..]) {
                                self.sender.send(Message::ReceivedFromController(message.body))
                                .unwrap_or_else(|_| unreachable!());
                            }
                        },
                        Some(Err(_)) => {
                            eprintln!("SLIP Decoding error");
                        }
                        None => {
                            break;
                        }
                    }
                },
                message = self.serial_receiver.recv() => {
                    match message {
                        Some(message) => {
                            let mut messages = vec![message];
                            while let Ok(message) = self.serial_receiver.try_recv() {
                                messages.push(message);
                            }
                            let mut commands = vec![];
                            for message in messages {
                                match message {
                                    MessageBody::Command { commands: mut commands2 } => {
                                        commands.append(&mut commands2);
                                    }
                                    _ => {
                                        let bytes: Vec<u8> = (&controller::message::Message::new(message)).into();
                                        framed_port.send(bytes.into()).await
                                        .unwrap_or_else(|_| unreachable!());
                                    }
                                }
                            }
                            for commands_chunk in commands.chunks(MAX_MESSAGE_BODY_LENGTH) {
                                let message = MessageBody::Command {
                                    commands: commands_chunk.to_vec(),
                                };
                                let bytes: Vec<u8> = (&controller::message::Message::new(message)).into();
                                framed_port.send(bytes.into()).await
                                .unwrap_or_else(|_| unreachable!());
                            }
                        },
                        None => {
                            // Channel closed
                            break;
                        }
                    }
                },
                _ = self.cancellation_token.cancelled() => break,
            }
        }
    }
}

impl Handler for SerialHandler {
    fn handle(&mut self, message: &Message) -> HandleResult {
        match message {
            Message::SendToController(body) => {
                self.serial_sender
                    .send(body.clone())
                    .unwrap_or_else(|_| unreachable!());
            }
            Message::Stop => {
                self.cancellation_token.cancel();
            }
            _ => {}
        }
        Continue
    }
}
