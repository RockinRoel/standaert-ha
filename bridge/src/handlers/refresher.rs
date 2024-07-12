use crate::controller::command::Command::Refresh;
use crate::controller::message::MessageBody::Command;
use crate::handlers::message::Message;
use crate::handlers::message::Message::SendToController;
use std::time::Duration;
use tokio::select;
use tokio::sync::broadcast::Sender;
use tokio::time::sleep;
use tokio_graceful_shutdown::SubsystemHandle;

const REFRESH_INTERVAL: Duration = Duration::from_secs(10);

struct Refresher {
    subsys: SubsystemHandle,
    tx: Sender<Message>,
}

pub async fn run(subsys: SubsystemHandle, tx: Sender<Message>) -> Result<(), anyhow::Error> {
    let refresher = Refresher { subsys, tx };
    refresher.run().await;
    Ok(())
}

impl Refresher {
    async fn run(&self) {
        self.send_refresh();
        loop {
            select! {
                _ = self.subsys.on_shutdown_requested() => break,
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
