use log::info;
use tokio::select;
use tokio::signal::ctrl_c;
use tokio_util::sync::CancellationToken;

pub async fn run(cancellation_token: CancellationToken) -> Result<(), anyhow::Error> {
    select! {
        _ = cancellation_token.cancelled() => Ok(()),
        result = ctrl_c() => {
            info!("Ctrl-C pressed, shutting down...");
            cancellation_token.cancel();
            result.map_err(Into::into)
        }
    }
}