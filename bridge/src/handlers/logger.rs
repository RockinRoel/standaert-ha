use crate::handlers::handler::{Handler, HandleResult};
use crate::handlers::message::Message;

pub struct Logger;

impl Handler for Logger {
    fn handle(&mut self, message: &Message) -> HandleResult {
        println!("Logger received message: {:?}", message);
        HandleResult::Continue
    }
}
