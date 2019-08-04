pub mod homeassistant;

use super::config;
use log::warn;
use rumqtt::{MqttClient, MqttOptions};

pub fn init(config: &config::MqttConfig) -> Result<(), Box<dyn std::error::Error + 'static>> {
    if !config.homeassistant.enabled {
        warn!("No MQTT service is enabled, not setting up MQTT");
        return Ok(())
    }

    let mqtt_opts = MqttOptions::new(config.client_id.clone(), config.host.clone(), config.port);

    let (mut mqtt, notifications) =
        MqttClient::start(mqtt_opts).expect("Could not create MQTT connection");

    if config.homeassistant.enabled {
        homeassistant::init(&config.homeassistant, &mut mqtt)?;
    }

    for n in notifications {}

    Ok(())
}
