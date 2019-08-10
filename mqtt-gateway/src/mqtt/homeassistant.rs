use super::config;

use rumqtt::{MqttClient, QoS};
use serde_json::json;

pub fn init(
    config: &config::Config,
    mqtt: &mut MqttClient,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    for (button_id, button_config) in &config.buttons {
        let config_json = json!({
            "name": if button_config.name.is_empty() { button_id } else { &button_config.name },
            "state_topic": format!("{0}/binary_sensor/{1}", config.mqtt.homeassistant.prefix, button_id),
        });
        mqtt.publish(
            format!(
                "{0}/binary_sensor/{1}/config",
                config.mqtt.homeassistant.prefix, button_id
            ),
            QoS::AtLeastOnce,
            true,
            config_json.to_string(),
        )
        .expect("Error publishing button config");
        mqtt.publish(
            format!(
                "{0}/binary_sensor/{1}",
                config.mqtt.homeassistant.prefix, button_id
            ),
            QoS::AtMostOnce,
            false,
            "OFF",
        )
        .expect("Error publishing button state");
    }

    for (light_id, light_config) in &config.lights {
        let config_json = json!({
            "name": if light_config.name.is_empty() { light_id } else { &light_config.name },
            "command_topic": format!("{0}/switch/{1}/set", config.mqtt.homeassistant.prefix, light_id),
            "state_topic": format!("{0}/switch/{1}", config.mqtt.homeassistant.prefix, light_id),
        });
        mqtt.publish(
            format!(
                "{0}/switch/{1}/config",
                config.mqtt.homeassistant.prefix, light_id
            ),
            QoS::AtLeastOnce,
            true,
            config_json.to_string(),
        )
        .expect("Error publishing switch config");
        mqtt.publish(
            format!("{0}/switch/{1}", config.mqtt.homeassistant.prefix, light_id),
            QoS::AtLeastOnce,
            false,
            "OFF",
        )
        .expect("Error publishing switch state");
    }

    Ok(())
}
