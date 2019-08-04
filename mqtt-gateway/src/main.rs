use log::debug;
use rumqtt::MqttOptions;
use serialport::prelude::*;
use standaertha_mqtt_gateway::{config, mqtt, Package, PackageInputStream};
use std::io::Read;

const MAX_COMMANDS: usize = 64;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    env_logger::init();

    let config = config::read_config()?;

    mqtt::init(&config.mqtt)?;

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
    let serial = serialport::open_with_settings(&config.serial.port, &s)?;
    for p in PackageInputStream::new(serial.bytes())
        .filter_map(|p| p.ok())
        .filter(|p| p.len() == 36)
        .map(|p| Package::from_buf(&p[0..36]))
    {
        debug!("{:?}", p);
    }

    Ok(())
}
