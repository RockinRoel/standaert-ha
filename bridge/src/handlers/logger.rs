use crate::handlers::message::Message;
use anyhow::Result;
use log::{error, info, trace};
use tokio::select;
use tokio::sync::broadcast::error::RecvError::{Closed, Lagged};
use tokio::sync::broadcast::Receiver;
use tokio_util::sync::CancellationToken;

struct Logger {
    cancellation_token: CancellationToken,
    rx: Receiver<Message>,
}

pub async fn run(cancellation_token: CancellationToken, rx: Receiver<Message>) -> Result<()> {
    let mut logger = Logger {
        cancellation_token,
        rx,
    };
    logger.run().await;
    Ok(())
}

impl Logger {
    async fn run(&mut self) {
        info!("Starting logger...");
        loop {
            select! {
                _ = self.cancellation_token.cancelled() => {
                    info!("Logger shutting down...");
                    break
                }
                message = self.rx.recv() => {
                    match message {
                        Ok(message) => trace!("Logging message: {message:?}"),
                        Err(Closed) => {
                            info!("Logger can't receive any more messages, since there are no more senders.");
                            break;
                        }
                        Err(Lagged(num_messages)) => {
                            error!("Logger lagged behind {num_messages}!");
                        }
                    }
                }
            }
        }
    }
}
