use super::config;

use rumqtt::{MqttClient, QoS};

pub fn init(
    config: &config::HomeAssistantConfig,
    mqtt: &mut MqttClient,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    for i in 1..=32 {
        mqtt.publish(
            format!("{0}/binary_sensor/btn{1}/config", config.prefix, i),
            QoS::AtLeastOnce,
            true,
            format!(
                r#"{{
                    "name": "Button {1}",
                    "state_topic": "{0}/binary_sensor/btn{1}"
                }}"#,
                config.prefix, i
            ),
        )
        .expect("Error publishing button config");
        mqtt.publish(
            format!("{0}/binary_sensor/btn{1}", config.prefix, i),
            QoS::AtMostOnce,
            false,
            "OFF",
        )
        .expect("Error publishing button state");
    }

    for i in 1..=32 {
        mqtt.publish(
            format!("{0}/switch/sw{1}/config", config.prefix, i),
            QoS::AtLeastOnce,
            true,
            format!(
                r#"{{
                    "name": "Switch {1}",
                    "command_topic": "{0}/switch/sw{1}/set",
                    "state_topic": "{0}/switch/sw{1}"
                }}"#,
                config.prefix, i
            ),
        )
        .expect("Error publishing switch config");
        mqtt.publish(
            format!("{0}/switch/sw{1}", config.prefix, i),
            QoS::AtLeastOnce,
            false,
            "OFF",
        )
        .expect("Error publishing switch state");
    }

    Ok(())
}
