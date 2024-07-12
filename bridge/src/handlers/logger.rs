use crate::handlers::message::Message;
use anyhow::Result;
use tokio::select;
use tokio::sync::broadcast::error::RecvError::{Closed, Lagged};
use tokio::sync::broadcast::Receiver;
use tokio_graceful_shutdown::SubsystemHandle;

struct Logger {
    subsys: SubsystemHandle,
    rx: Receiver<Message>,
}

pub async fn run(subsys: SubsystemHandle, rx: Receiver<Message>) -> Result<()> {
    let mut logger = Logger { subsys, rx };
    logger.run().await;
    Ok(())
}

impl Logger {
    async fn run(&mut self) {
        loop {
            select! {
                _ = self.subsys.on_shutdown_requested() => {
                    eprintln!("Logger shutting down...");
                    break
                }
                message = self.rx.recv() => {
                    match message {
                        Ok(message) => eprintln!("Logging message: {message:?}"),
                        Err(Closed) => {
                            eprintln!("Logger can't receive any more messages, since there are no more senders.");
                            break;
                        }
                        Err(Lagged(num_messages)) => {
                            eprintln!("Logger lagged behind {num_messages}!");
                        }
                    }
                }
            }
        }
    }
}
