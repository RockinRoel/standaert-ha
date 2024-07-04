use std::sync::mpsc::Sender;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use slip_codec::tokio::SlipCodec;
use futures::SinkExt;
use futures::stream::StreamExt;
use tokio::task::JoinHandle;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::codec::{Decoder, Framed};
use tokio_util::sync::CancellationToken;
use crate::controller;
use crate::controller::message::MessageBody;
use crate::handlers::handler::{Handler, HandleResult};
use crate::handlers::handler::HandleResult::Continue;
use crate::handlers::message::Message;

const BAUD_RATE: u32 = 115_200;

pub struct SerialHandler {
    task: JoinHandle<()>,
    serial_sender: UnboundedSender<MessageBody>,
    cancellation_token: CancellationToken,
}

impl SerialHandler {
    pub fn new(serial_port: String, sender: Sender<Message>) -> Self {
        let (serial_sender, mut serial_receiver) = tokio::sync::mpsc::unbounded_channel::<MessageBody>();
        let cancellation_token = CancellationToken::new();
        let cancellation_token_clone = cancellation_token.clone();
        let task = tokio::spawn(async move {
            let serial_stream = tokio_serial::new(serial_port, BAUD_RATE).open_native_async().expect("Failed to open serial port!");
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
                            let bytes: Vec<u8> = (&controller::message::Message::new(message)).into();
                            framed_port.send(bytes.into()).await.expect("Failed to send serial message?"); // TODO(Roel): what about this error?
                        } else {
                            // TODO(Roel): ???
                        }
                    },
                    _ = cancellation_token_clone.cancelled() => break,
                }
            }
            println!("After loop");
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
                // TODO(Roel): await task???
            }
            _ => {}
        }
        Continue
    }
}
