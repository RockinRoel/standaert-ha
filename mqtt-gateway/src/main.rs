use rumqtt::{MqttClient, MqttOptions, Notification, QoS};
use serialport::prelude::*;
use standaertha_mqtt_gateway::{Package, PackageInputStream};
use std::io::Read;
use std::thread;
use std::time::Duration;

const MAX_COMMANDS: usize = 64;

fn main() {
    let opts = MqttOptions::new("standaertha-gateway", "192.168.0.11", 1883);
    let (mut mqtt, notifications) =
        MqttClient::start(opts).expect("Could not create MQTT connection");

    for i in 1..=32 {
        mqtt.publish(
            format!("homeassistant/binary_sensor/btn{0}/config", i),
            QoS::ExactlyOnce,
            true,
            format!(
                r#"{{
                  "name": "Button {0}",
                  "state_topic": "homeassistant/binary_sensor/btn{0}"
                }}"#,
                i
            ),
        )
        .expect("Error publishing button config");
        mqtt.publish(
            format!("homeassistant/binary_sensor/btn{0}", i),
            QoS::AtMostOnce,
            false,
            "OFF",
        )
        .expect("Error publishing button state");
    }

    for i in 1..=32 {
        mqtt.publish(
            format!("homeassistant/switch/sw{0}/config", i),
            QoS::ExactlyOnce,
            true,
            format!(
                r#"{{
              "name": "Switch {0}",
              "command_topic": "homeassistant/switch/sw{0}/set",
              "state_topic": "homeassistant/switch/sw{0}"
            }}"#,
                i
            ),
        )
        .expect("Error publishing switch config");
        mqtt.publish(
            format!("homeassistant/switch/sw{0}", i),
            QoS::AtMostOnce,
            false,
            if i % 2 == 0 { "OFF" } else { "ON" },
        )
        .expect("Error publishing switch state");
    }

    for n in notifications {}

    return;

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

    let s = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1000),
    };
    let serial =
        serialport::open_with_settings("/dev/ttyUSB0", &s).expect("Could not open /dev/ttyUSB0");
    for p in PackageInputStream::new(serial.bytes())
        .filter_map(|p| p.ok())
        .filter(|p| p.len() == 36)
        .map(|p| Package::from_buf(&p[0..36]))
    {}
}
