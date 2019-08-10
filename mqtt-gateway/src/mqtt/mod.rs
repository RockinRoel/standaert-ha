pub mod homeassistant;

use super::{config, Package, Service};
use log::warn;
use rumqtt::{MqttClient, MqttOptions};
use std::thread;
use std::thread::JoinHandle;

struct MqttService {
    thread: Option<JoinHandle<()>>,
}

impl MqttService {
    fn new(thread: JoinHandle<()>) -> MqttService {
        MqttService {
            thread: Some(thread),
        }
    }
}

impl Service for MqttService {
    fn handle_package(&mut self, package: &Package) { }
    fn join(&mut self) {
        if self.thread.is_some() {
            self.thread.take().unwrap().join().expect("Can't join?");
        }
    }
}

pub fn init(config: &config::Config) -> Result<Option<Box<dyn Service>>, Box<dyn std::error::Error + 'static>> {
    if !config.mqtt.homeassistant.enabled {
        warn!("No MQTT service is enabled, not setting up MQTT");
        return Ok(None);
    }

    let mqtt_opts = MqttOptions::new(
        config.mqtt.client_id.clone(),
        config.mqtt.host.clone(),
        config.mqtt.port,
    );

    let (mut mqtt, notifications) =
        MqttClient::start(mqtt_opts).expect("Could not create MQTT connection");

    if config.mqtt.homeassistant.enabled {
        homeassistant::init(&config, &mut mqtt)?;
    }

    let thread = thread::spawn(move || {
      for n in notifications {}
    });

    Ok(Some(Box::new(MqttService::new(thread))))
}
