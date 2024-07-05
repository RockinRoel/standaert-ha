use crate::controller::message::MessageBody;

#[derive(Debug, Eq, PartialEq)]
pub enum Message {
    ReloadProgram,
    ReceivedFromController(MessageBody),
    SendToController(MessageBody),
    Stop,
}
