use crate::handlers::message::Message;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HandleResult {
    Continue,
    Stop,
}

pub trait Handler {
    fn handle(&mut self, message: &Message) -> HandleResult;
}
