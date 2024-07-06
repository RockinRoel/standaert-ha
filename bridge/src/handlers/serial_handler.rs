use crate::controller;
use crate::controller::message::{MAX_MESSAGE_BODY_LENGTH, MessageBody};
use crate::handlers::handler::HandleResult::Continue;
use crate::handlers::handler::{HandleResult, Handler};
use crate::handlers::message::Message;
use futures::stream::StreamExt;
use futures::SinkExt;
use slip_codec::tokio::SlipCodec;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Decoder;
use tokio_util::sync::CancellationToken;

const BAUD_RATE: u32 = 9600;

pub struct SerialHandler {
    task: JoinHandle<()>,
    serial_sender: UnboundedSender<MessageBody>,
    cancellation_token: CancellationToken,
}

impl SerialHandler {
    pub fn new(serial_port: String, sender: UnboundedSender<Message>) -> Self {
        let (serial_sender, mut serial_receiver) =
            tokio::sync::mpsc::unbounded_channel::<MessageBody>();
        let cancellation_token = CancellationToken::new();
        let cancellation_token_clone = cancellation_token.clone();
        let task = tokio::spawn(async move {
            let serial_stream = tokio_serial::new(serial_port, BAUD_RATE)
                .open_native_async()
                .expect("Failed to open serial port!");
            let mut framed_port = SlipCodec::new().framed(serial_stream);
            loop {
                tokio::select! {
                    message = framed_port.next() => {
                        if let Some(Ok(message)) = message {
                            if let Ok(message) = controller::message::Message::try_from(&message[..]) {
                                sender.send(Message::ReceivedFromController(message.body)).expect("Failed to send message"); // TODO(Roel): what about this error?
                            }
                        } else {
                            // TODO(Roel): ???
                        }
                    },
                    message = serial_receiver.recv() => {
                        if let Some(message) = message {
                            let mut messages = vec![message];
                            while let Ok(message) = serial_receiver.try_recv() {
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
                                        framed_port.send(bytes.into()).await.expect("Failed to send serial message?"); // TODO(Roel): what about this error?
                                    }
                                }
                            }
                            for commands_chunk in commands.chunks(MAX_MESSAGE_BODY_LENGTH) {
                                let message = MessageBody::Command {
                                    commands: commands_chunk.to_vec(),
                                };
                                let bytes: Vec<u8> = (&controller::message::Message::new(message)).into();
                                framed_port.send(bytes.into()).await.expect("Failed to send serial message?"); // TODO(Roel): what about this error?
                            }
                        } else {
                            // TODO(Roel): ???
                        }
                    },
                    _ = cancellation_token_clone.cancelled() => break,
                }
            }
        });
        Self {
            task,
            serial_sender,
            cancellation_token,
        }
    }
}

impl Handler for SerialHandler {
    fn handle(&mut self, message: &Message) -> HandleResult {
        match message {
            Message::SendToController(body) => {
                self.serial_sender.send(body.clone()).expect("Error?"); // TODO(Roel): what do I do with this error?
            }
            Message::Stop => {
                self.cancellation_token.cancel();
                while !self.task.is_finished() {
                    // Busy wait??
                }
            }
            _ => {}
        }
        Continue
    }
}
