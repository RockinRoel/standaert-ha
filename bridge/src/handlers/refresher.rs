use crate::controller::command::Command::Refresh;
use crate::controller::message::MessageBody::Command;
use crate::handlers::message::Message;
use crate::handlers::message::Message::SendToController;
use std::time::Duration;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio::{select, spawn};
use tokio_util::sync::CancellationToken;

const REFRESH_INTERVAL: Duration = Duration::from_secs(10);

struct Refresher {
    tx: Sender<Message>,
    cancellation_token: CancellationToken,
}

pub fn start(tx: Sender<Message>, cancellation_token: CancellationToken) -> JoinHandle<()> {
    let refresher = Refresher {
        tx,
        cancellation_token,
    };
    spawn(async move { refresher.run().await })
}

impl Refresher {
    async fn run(&self) {
        self.send_refresh();
        loop {
            select! {
                _ = sleep(REFRESH_INTERVAL) => self.send_refresh(),
                _ = self.cancellation_token.cancelled() => break,
            }
        }
    }

    fn send_refresh(&self) {
        self.tx
            .send(SendToController(Command {
                commands: vec![Refresh],
            }))
            .unwrap_or_else(|_| unreachable!());
    }
}
