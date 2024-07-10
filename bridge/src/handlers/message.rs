use crate::controller::message::MessageBody;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    ReceivedFromController(MessageBody),
    SendToController(MessageBody),
}
