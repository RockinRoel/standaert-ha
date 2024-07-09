use crate::handlers::handler::{HandleResult, Handler};
use crate::handlers::message::Message;

#[derive(Default)]
pub struct HandlerChain {
    chain: Vec<Box<dyn Handler>>,
}

impl HandlerChain {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO(Roel): can we put anything less than a 'static lifetime on this?
    pub fn add_handler<H: Handler + 'static>(&mut self, handler: H) {
        self.chain.push(Box::new(handler));
    }
}

impl Handler for HandlerChain {
    fn handle(&mut self, message: &Message) -> HandleResult {
        for handler in &mut self.chain {
            let result = handler.handle(message);
            if result == HandleResult::Stop {
                return HandleResult::Stop;
            }
        }
        HandleResult::Continue
    }
}
