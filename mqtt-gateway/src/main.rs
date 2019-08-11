use crossbeam::crossbeam_channel::unbounded;
use log::{debug, error, info};
use serialport::prelude::*;
#[cfg(feature = "mqtt")]
use standaertha_mqtt_gateway::mqtt;
#[cfg(feature = "webthing")]
use standaertha_mqtt_gateway::webthing;
use standaertha_mqtt_gateway::{
    append_crc16, config, slip_encode, Command, CommandType, Package, PackageInputStream, Service,
};
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

const MAX_COMMANDS: usize = 64;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    pretty_env_logger::init();

    let config = config::read_config()?;

    let mut services: Vec<Box<dyn Service>> = vec![];

    let (sender, recv) = unbounded();

    #[cfg(feature = "mqtt")]
    {
        let mqtt = mqtt::init(&config, &sender)?;
        if mqtt.is_some() {
            services.push(mqtt.unwrap());
        }
    }

    #[cfg(feature = "webthing")]
    {
        let thing = webthing::init(&config, &sender)?;
        if thing.is_some() {
            services.push(thing.unwrap());
        }
    }

    /*
    thread::spawn(move || {
        loop {
            let mut commands = vec![];
            let notification = notifications.recv().expect("Error receiving notification");
            commands.push(notification);
            if let Ok(notification) = notifications.recv_timeout(Duration::from_millis(10)) {
                commands.push(notification);
            }
            while commands.len() < MAX_COMMANDS {
                if let Ok(notification) = notifications.try_recv() {
                    commands.push(notification);
                } else {
                    break;
                }
            }
            commands.into_iter().map(|notification| {
                if let Notification::Publish(command) = notification {
                    // TODO: handle topic name/payload
                }
            });
        }
    });
    */

    let s = SerialPortSettings {
        baud_rate: config.serial.baud_rate,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: config.serial.timeout,
    };
    debug!("Using serial port: {}", config.serial.port);
    let mut serial = serialport::open_with_settings(&config.serial.port, &s).unwrap();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        info!("Received interrupt, stopping...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let input = serial.try_clone().unwrap().bytes();

    let r = running.clone();
    let cmd_thread = thread::spawn(move || loop {
        if !r.load(Ordering::SeqCst) {
            break;
        }
        if let Ok(c) = recv.recv_timeout(Duration::from_millis(1000)) {
            let mut cmds = vec![c.raw()];
            let now = Instant::now();
            let timeout_duration = Duration::from_millis(100);
            let timeout = now + timeout_duration;
            let mut remaining = timeout_duration;
            while let Ok(c) = recv.recv_timeout(remaining) {
                cmds.push(c.raw());
                if cmds.len() >= MAX_COMMANDS {
                    break;
                }
                let now = Instant::now();
                if now > timeout {
                    break;
                }
                remaining = timeout - now;
            }
            cmds = append_crc16(cmds);
            serial.write_all(&slip_encode(&cmds)).unwrap();
        }
    });

    sender.send(Command::new(CommandType::Refresh, 0)).unwrap();

    let mut last_package = None;
    for p in PackageInputStream::new(input) {
        match p {
            Ok(p) => {
                if p.len() == 36 {
                    let pkg = Package::from_buf(&p[0..36]);
                    debug!("Package: {:?}", pkg);
                    last_package = Some(Instant::now());
                    for service in &mut services {
                        service.handle_package(&pkg);
                    }
                } else {
                    info!("Discarding package of length != 36, was {}", p.len());
                }
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::TimedOut {
                    debug!("Routine timeout on serial input");
                } else {
                    error!("Error on input stream: {:?}", e);
                }
            }
        }
        if last_package.is_some()
            && Instant::now() - last_package.unwrap() > Duration::from_secs(10)
        {
            sender.send(Command::new(CommandType::Refresh, 0)).unwrap();
        }
        if !running.load(Ordering::SeqCst) {
            break;
        }
    }

    for service in &mut services {
        service.join();
    }
    cmd_thread.join().unwrap();

    Ok(())
}
