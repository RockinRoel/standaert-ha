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
use if_chain::if_chain;
use std::time::Duration;
use log::info;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle, Toplevel};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let args = Args::parse();

    info!("Starting SHA bridge with arguments:\n{}", args);

    let mut program = None;
    if let Some(program_path) = &args.program {
        program = Some(programmer::compile(program_path).await?);
    }

    let (sender, _receiver) = broadcast::channel(100);

    Toplevel::new(move |s| spawn_subsystems(s, args, program, sender))
        .catch_signals()
        .handle_shutdown_requests(Duration::from_secs(1))
        .await
        .map_err(Into::into)
}

async fn spawn_subsystems(
    handle: SubsystemHandle,
    args: Args,
    program: Option<Program>,
    sender: Sender<Message>,
) {
    if args.debug {
        let rx = sender.subscribe();
        handle.start(SubsystemBuilder::new("Debug logger", |subsys| {
            logger::run(subsys, rx)
        }));
    }

    if let Some(mqtt_url) = &args.mqtt_url {
        let mut credentials = None;
        if let (Some(user), Some(password)) = (&args.mqtt_user, &args.mqtt_password) {
            credentials = Some((user.clone(), password.clone()));
        }
        let mqtt_url = mqtt_url.clone();
        let args_prefix = args.prefix.clone();
        let program = program.clone();
        let sender = sender.clone();
        handle.start(SubsystemBuilder::new("MQTT", |subsys| {
            mqtt_handler::run(subsys, mqtt_url, credentials, args_prefix, program, sender)
        }));
    }

    if let Some(serial_port) = &args.serial {
        let serial_port = serial_port.clone();
        let sender = sender.clone();
        handle.start(SubsystemBuilder::new("Serial", |subsys| async move {
            serial_handler::run(subsys, &serial_port, sender).await
        }));
    }

    if_chain!(
        if let Some(program) = program;
        if args.upload;
        then {
            let sender = sender.clone();
            handle.start(
                SubsystemBuilder::new(
                    "Programmer",
                    |subsys| programmer::run(subsys, program, sender)
                )
            );
        }
    );

    handle.start(SubsystemBuilder::new("Refresher", |subsys| {
        refresher::run(subsys, sender)
    }));
}
