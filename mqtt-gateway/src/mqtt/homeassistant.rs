use super::MqttSubService;
use super::{config, Command, CommandType, EventType, Package};

use crossbeam::channel::Sender;
use log::debug;
use regex::Regex;
use rumqtt::{MqttClient, Notification, QoS};
use serde_json::json;

struct HomeAssistant {
    sender: Sender<Command>,
    client: MqttClient,
    config: config::Config,
}

impl MqttSubService for HomeAssistant {
    fn handle_package(&mut self, package: &Package) {
        for (light_id, light_config) in &self.config.lights {
            let i = light_config.index;
            let light_state = package.state & (1 << (i as u32)) != 0;
            self.client
                .publish(
                    format!(
                        "{0}/switch/{1}",
                        self.config.mqtt.homeassistant.prefix, light_id
                    ),
                    QoS::AtMostOnce,
                    false,
                    if light_state { "true" } else { "false" },
                )
                .expect("Could not publish light state");
        }
        for event in &package.events {
            if event.valid() {
                for (button_id, button_config) in &self.config.buttons {
                    if button_config.index == event.button() {
                        match event.event_type() {
                            EventType::PressStart => {
                                self.client
                                    .publish(
                                        format!(
                                            "{0}/binary_sensor/{1}",
                                            self.config.mqtt.homeassistant.prefix, button_id
                                        ),
                                        QoS::AtMostOnce,
                                        false,
                                        "true",
                                    )
                                    .expect("Could not publish button state");
                            }
                            EventType::PressEnd => {
                                self.client
                                    .publish(
                                        format!(
                                            "{0}/binary_sensor/{1}",
                                            self.config.mqtt.homeassistant.prefix, button_id
                                        ),
                                        QoS::AtMostOnce,
                                        false,
                                        "false",
                                    )
                                    .expect("Could not publish button state");
                            }
                        }
                        break;
                    }
                }
            }
        }
    }

    fn handle_notification(&mut self, notification: &Notification) {
        debug!("HomeAssistant received notification: {:?}", notification);
        if let Notification::Publish(publish) = notification {
            let re = Regex::new(&format!(
                "{0}/switch/([^/]+)/set",
                self.config.mqtt.homeassistant.prefix
            ))
            .unwrap();
            if let Some(cap) = re.captures(&publish.topic_name) {
                let light_id = cap.get(1).unwrap().as_str();
                let payload: &Vec<u8> = &publish.payload;
                let light_state = match &payload[..] {
                    p if p == b"true" => Some(true),
                    p if p == b"false" => Some(false),
                    _ => None,
                };
                if let Some(light_state) = light_state {
                    let command = if light_state {
                        CommandType::On
                    } else {
                        CommandType::Off
                    };
                    for (light_id2, light_config) in &self.config.lights {
                        if light_id == light_id2 {
                            self.sender
                                .send(Command::new(command, light_config.index))
                                .expect("Could not send command");
                            break;
                        }
                    }
                }
            }
        }
    }
}

pub fn init(
    config: &config::Config,
    mqtt: &mut MqttClient,
    sender: &Sender<Command>,
) -> Result<Box<dyn MqttSubService + Send>, Box<dyn std::error::Error + 'static>> {
    for (button_id, button_config) in &config.buttons {
        let config_json = json!({
            "name": if button_config.name.is_empty() { button_id } else { &button_config.name },
            "state_topic": format!("{0}/binary_sensor/{1}", config.mqtt.homeassistant.prefix, button_id),
            "payload_on": "true",
            "payload_off": "false",
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
        /*
        mqtt.publish(
            format!(
                "{0}/binary_sensor/{1}",
                config.mqtt.homeassistant.prefix, button_id
            ),
            QoS::AtMostOnce,
            false,
            "false",
        )
        .expect("Error publishing button state");
        */
    }

    for (light_id, light_config) in &config.lights {
        let config_json = json!({
            "name": if light_config.name.is_empty() { light_id } else { &light_config.name },
            "command_topic": format!("{0}/switch/{1}/set", config.mqtt.homeassistant.prefix, light_id),
            "state_topic": format!("{0}/switch/{1}", config.mqtt.homeassistant.prefix, light_id),
            "payload_on": "true",
            "payload_off": "false",
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
        /*
        mqtt.publish(
            format!("{0}/switch/{1}", config.mqtt.homeassistant.prefix, light_id),
            QoS::AtLeastOnce,
            false,
            "false",
        )
        .expect("Error publishing switch state");
        */
        mqtt.subscribe(
            format!(
                "{0}/switch/{1}/set",
                config.mqtt.homeassistant.prefix, light_id
            ),
            QoS::AtLeastOnce,
        )
        .expect("Error subscribing to switch set command");
    }

    Ok(Box::new(HomeAssistant {
        sender: sender.clone(),
        client: mqtt.clone(),
        config: config.clone(),
    }))
}
