use crate::handlers::handler::{HandleResult, Handler};
use crate::handlers::message::Message;

pub struct Logger;

impl Handler for Logger {
    fn handle(&mut self, message: &Message) -> HandleResult {
        println!("Logger received message: {:?}", message);
        HandleResult::Continue
    }
}
