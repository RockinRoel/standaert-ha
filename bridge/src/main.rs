mod args;
pub mod controller;
pub mod handlers;
pub mod shal;

use crate::args::Args;
use crate::handlers::message::Message;
use crate::handlers::mqtt_handler::{MqttHandler, MqttHandlerConfig};
use crate::handlers::serial_handler::SerialHandler;
use crate::handlers::{logger, programmer, refresher};
use crate::shal::bytecode::Program;
use anyhow::Result;
use clap::Parser;
use if_chain::if_chain;
use log::Level::Trace;
use log::{info, log_enabled};
use std::collections::VecDeque;
use std::panic;
use tokio::select;
use tokio::signal::ctrl_c;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

const BROADCAST_CHANNEL_CAPACITY: usize = 100;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    info!("Starting SHA bridge with arguments:\n{}", args);

    let mut program = None;
    if let Some(program_path) = &args.program {
        program = Some(programmer::compile(program_path).await?);
    }

    let (sender, _receiver) = broadcast::channel(BROADCAST_CHANNEL_CAPACITY);

    let mut join_set: JoinSet<Result<()>> = JoinSet::new();
    let cancellation_token = CancellationToken::new();

    {
        let cancellation_token = cancellation_token.clone();
        join_set.spawn(async move {
            select! {
                _ = cancellation_token.cancelled() => Ok(()),
                result = ctrl_c() => {
                    info!("Ctrl-C pressed, shutting down...");
                    cancellation_token.cancel();
                    result.map_err(Into::into)
                }
            }
        });
    }

    let result = spawn_tasks(&mut join_set, &cancellation_token, args, program, sender).await;

    let mut errors = VecDeque::new();
    if let Err(e) = result {
        errors.push_back(e);
    }

    while let Some(r) = join_set.join_next().await {
        match r {
            // Task completed successfully
            Ok(Ok(())) => {}
            // Task completed with error, cancel all other tasks
            Ok(Err(e)) => {
                errors.push_back(e);
                cancellation_token.cancel();
            }
            // Join error
            Err(join_error) => {
                // Propagate panic
                if let Ok(reason) = join_error.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }

    // TODO: what about all of the other errors?
    if let Some(e) = errors.pop_front() {
        Err(e)
    } else {
        Ok(())
    }
}

async fn spawn_tasks(
    join_set: &mut JoinSet<Result<()>>,
    cancellation_token: &CancellationToken,
    args: Args,
    program: Option<Program>,
    sender: Sender<Message>,
) -> Result<()> {
    let drop_guard = cancellation_token.clone().drop_guard();

    if log_enabled!(Trace) {
        let rx = sender.subscribe();
        let cancellation_token = cancellation_token.clone();
        join_set.spawn(async move { logger::run(cancellation_token, rx).await });
    }

    if let Some(mqtt_url) = &args.mqtt_url {
        let mut credentials = None;
        if let (Some(user), Some(password)) = (&args.mqtt_user, &args.mqtt_password) {
            credentials = Some((user.clone(), password.clone()));
        }
        let config = MqttHandlerConfig::new(
            args.prefix.clone(),
            program.clone(),
            mqtt_url.clone(),
            credentials,
        )?;
        let sender = sender.clone();
        let handler = MqttHandler::new(cancellation_token.clone(), config, sender).await?;
        join_set.spawn(async move { handler.run().await.map_err(Into::into) });
    }

    if let Some(serial_port) = &args.serial {
        let cancellation_token = cancellation_token.clone();
        let sender = sender.clone();
        let handler = SerialHandler::new(cancellation_token, serial_port, sender).await?;
        join_set.spawn(async move { handler.run().await.map_err(Into::into) });
    }

    if_chain!(
        if let Some(program) = program;
        if args.upload;
        then {
            let cancellation_token = cancellation_token.clone();
            let sender = sender.clone();
            join_set.spawn(async move {
                programmer::run(cancellation_token, program, sender).await
            });
        }
    );

    {
        let cancellation_token = cancellation_token.clone();
        join_set.spawn(async move { refresher::run(cancellation_token, sender).await });
    }

    drop_guard.disarm();
    
    Ok(())
}
