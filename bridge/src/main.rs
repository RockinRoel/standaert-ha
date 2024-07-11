mod args;
pub mod controller;
pub mod handlers;
pub mod shal;

use crate::args::Args;
use crate::handlers::message::Message;
use crate::handlers::{logger, mqtt_handler, programmer, refresher, serial_handler};
use crate::shal::bytecode::Program;
use anyhow::Result;
use clap::Parser;
use futures::future::join_all;
use if_chain::if_chain;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio::task::JoinHandle;
use tokio::{select, signal};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Starting SHA bridge with arguments:\n{}", args);

    let mut program = None;
    if let Some(program_path) = &args.program {
        program = Some(programmer::compile(program_path).await?);
    }

    let (sender, _receiver) = broadcast::channel(100);
    let cancellation_token = CancellationToken::new();

    let mut tasks = vec![];

    let result = spawn_tasks(
        &mut tasks,
        &args,
        program,
        &sender,
        cancellation_token.clone(),
    )
    .await;

    if let Ok(()) = result {
        select! {
            _ = cancellation_token.cancelled() => {}
            _ = signal::ctrl_c() => cancellation_token.cancel(),
        }
    } else {
        // Error occurred, cleanly terminate all started tasks
        cancellation_token.cancel();
    }

    // TODO(Roel): tasks may exit with error?
    join_all(tasks).await;

    result
}

async fn spawn_tasks(
    tasks: &mut Vec<JoinHandle<()>>,
    args: &Args,
    program: Option<Program>,
    sender: &Sender<Message>,
    cancellation_token: CancellationToken,
) -> Result<()> {
    if args.debug {
        tasks.push(logger::start(
            sender.subscribe(),
            cancellation_token.clone(),
        ));
    }

    if let Some(mqtt_url) = &args.mqtt_url {
        let mut credentials = None;
        if let (Some(user), Some(password)) = (&args.mqtt_user, &args.mqtt_password) {
            credentials = Some((user.clone(), password.clone()));
        }
        let task = mqtt_handler::start(
            mqtt_url.clone(),
            credentials,
            args.prefix.clone(),
            program.clone(),
            sender.clone(),
            cancellation_token.clone(),
        )
        .await?;
        tasks.push(task);

        // TODO(Roel): we should wait until we're done with init?
    }

    if let Some(serial_port) = &args.serial {
        let task = serial_handler::start(
            serial_port.clone(),
            sender.clone(),
            cancellation_token.clone(),
        )
        .await?;
        tasks.push(task);
    }

    if_chain!(
        if let Some(program) = program;
        if args.upload;
        then {
            tasks.push(programmer::start(program, sender.clone(), cancellation_token.clone()));
        }
    );

    tasks.push(refresher::start(sender.clone(), cancellation_token.clone()));

    Ok(())
}
