use crate::handlers::message::Message;
use tokio::sync::broadcast::error::RecvError::{Closed, Lagged};
use tokio::sync::broadcast::Receiver;
use tokio::task::JoinHandle;
use tokio::{select, spawn};
use tokio_util::sync::CancellationToken;

struct Logger {
    rx: Receiver<Message>,
    cancellation_token: CancellationToken,
}

pub fn start(rx: Receiver<Message>, cancellation_token: CancellationToken) -> JoinHandle<()> {
    let mut logger = Logger {
        rx,
        cancellation_token,
    };
    spawn(async move { logger.run().await })
}

impl Logger {
    async fn run(&mut self) {
        loop {
            select! {
                message = self.rx.recv() => {
                    match message {
                        Ok(message) => log(&message),
                        Err(Closed) => {
                            eprintln!("Logger can't receive any more messages, since there are no more senders.");
                            break;
                        }
                        Err(Lagged(num_messages)) => {
                            eprintln!("Logger lagged behind {num_messages}!");
                        }
                    }
                }
                _ = self.cancellation_token.cancelled() => {
                    eprintln!("Logger shutting down...");
                    break
                }
            }
        }
    }
}

fn log(message: &Message) {
    eprintln!("Logging message: {message:?}");
}
