use crate::controller::command::Command::Refresh;
use crate::controller::message::MessageBody::Command;
use crate::handlers::message::Message;
use crate::handlers::message::Message::SendToController;
use std::time::Duration;
use tokio::select;
use tokio::sync::broadcast::Sender;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

const REFRESH_INTERVAL: Duration = Duration::from_secs(10);

struct Refresher {
    cancellation_token: CancellationToken,
    tx: Sender<Message>,
}

pub async fn run(cancellation_token: CancellationToken, tx: Sender<Message>) -> Result<(), anyhow::Error> {
    let refresher = Refresher { cancellation_token, tx };
    refresher.run().await;
    Ok(())
}

impl Refresher {
    async fn run(&self) {
        self.send_refresh();
        loop {
            select! {
                _ = self.cancellation_token.cancelled() => break,
                _ = sleep(REFRESH_INTERVAL) => self.send_refresh(),
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
