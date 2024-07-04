use crate::controller;

#[derive(Debug, Eq, PartialEq)]
pub enum Message {
    ReloadProgram,
    ReceivedFromController(controller::message::MessageBody),
    SendToController(controller::message::MessageBody),
    Stop,
}
