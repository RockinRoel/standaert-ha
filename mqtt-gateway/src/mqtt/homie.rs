use super::{config, Command, CommandType, EventType, MqttSubService, Package};
use crossbeam::channel::Sender;
use log::debug;
use regex::Regex;
use rumqtt::{LastWill, MqttClient, Notification, QoS};

struct Homie {
    config: config::Config,
    client: MqttClient,
    sender: Sender<Command>,
}

static HOMIE_VERSION: &'static str = "3.0.1";

impl MqttSubService for Homie {
    fn handle_package(&mut self, package: &Package) {
        for (light_id, light_config) in &self.config.lights {
            let i = light_config.index;
            let light_state = package.state & (1 << i) != 0;
            self.client
                .publish(
                    format!(
                        "homie/{0}/{1}/power",
                        self.config.mqtt.homie.device_id, light_id
                    ),
                    QoS::AtLeastOnce,
                    true,
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
                                            "homie/{0}/{1}/pressed",
                                            self.config.mqtt.homie.device_id, button_id
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
                                            "homie/{0}/{1}/pressed",
                                            self.config.mqtt.homie.device_id, button_id
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
        debug!("Homie received notification: {:?}", notification);
        if let Notification::Publish(publish) = notification {
            let re = Regex::new(&format!(
                "homie/{0}/([^/]+)/power/set",
                self.config.mqtt.homie.device_id,
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

    fn do_disconnect(&mut self) {
        self.client
            .publish(
                format!("homie/{0}/$state", self.config.mqtt.homie.device_id,),
                QoS::AtLeastOnce,
                true,
                "disconnected",
            )
            .expect("Could not publish homie disconnected");
    }
}

pub fn init(
    config: &config::Config,
    mqtt: &mut MqttClient,
    sender: &Sender<Command>,
) -> Result<Box<dyn MqttSubService + Send>, Box<dyn std::error::Error + 'static>> {
    let homie_prefix = format!("homie/{0}", config.mqtt.homie.device_id);
    mqtt.publish(
        format!("{0}/$state", homie_prefix),
        QoS::AtLeastOnce,
        true,
        "init",
    )
    .expect("Could not publish homie state");
    mqtt.publish(
        format!("{0}/$homie", homie_prefix),
        QoS::AtLeastOnce,
        true,
        HOMIE_VERSION,
    )
    .expect("Could not publish homie version");
    mqtt.publish(
        format!("{0}/$name", homie_prefix),
        QoS::AtLeastOnce,
        true,
        config.mqtt.homie.name.as_bytes(),
    )
    .expect("Could not publish homie name");
    let mut nodes: Vec<&str> = vec![];
    for (button_id, _) in &config.buttons {
        nodes.push(button_id);
    }
    for (light_id, _) in &config.lights {
        nodes.push(light_id);
    }
    let nodes_str = nodes.join(",");
    mqtt.publish(
        format!("{0}/$nodes", homie_prefix),
        QoS::AtLeastOnce,
        true,
        nodes_str.as_bytes(),
    )
    .expect("Could not publish homie nodes");

    for (button_id, _button_config) in &config.buttons {
        let node_prefix = format!("{0}/$nodes/{1}", homie_prefix, button_id);
        mqtt.publish(
            format!("{0}/$name", node_prefix),
            QoS::AtLeastOnce,
            true,
            config.button_name(button_id).unwrap(),
        )
        .expect("Failed to publish button");
        mqtt.publish(
            format!("{0}/$type", node_prefix),
            QoS::AtLeastOnce,
            true,
            "Push button",
        )
        .expect("Could not publish button type");
        mqtt.publish(
            format!("{0}/$properties", node_prefix),
            QoS::AtLeastOnce,
            true,
            "pressed",
        )
        .expect("Could not publish button properties");
        let pressed_prefix = format!("{0}/pressed", node_prefix);
        mqtt.publish(
            format!("{0}/$name", pressed_prefix),
            QoS::AtLeastOnce,
            true,
            "Pressed",
        )
        .expect("Could not publish pressed property name");
        mqtt.publish(
            format!("{0}/$datatype", pressed_prefix),
            QoS::AtLeastOnce,
            true,
            "boolean",
        )
        .expect("Could not publish pressed property datatype");
        mqtt.publish(
            format!("{0}/$settable", pressed_prefix),
            QoS::AtLeastOnce,
            true,
            "false",
        )
        .expect("Could not publish pressed property settable = false");
        mqtt.publish(
            format!("{0}/$retained", pressed_prefix),
            QoS::AtLeastOnce,
            true,
            "false",
        )
        .expect("Could not publish pressed property retained = false");
    }

    for (light_id, _light_config) in &config.lights {
        let node_prefix = format!("{0}/$nodes/{1}", homie_prefix, light_id);
        mqtt.publish(
            format!("{0}/$name", node_prefix),
            QoS::AtLeastOnce,
            true,
            config.light_name(light_id).unwrap(),
        )
        .expect("Failed to publish light");
        mqtt.publish(
            format!("{0}/$type", node_prefix),
            QoS::AtLeastOnce,
            true,
            "Light",
        )
        .expect("Could not publish light type");
        mqtt.publish(
            format!("{0}/$properties", node_prefix),
            QoS::AtLeastOnce,
            true,
            "power",
        )
        .expect("Could not publish light properties");
        let power_prefix = format!("{0}/power", node_prefix);
        mqtt.publish(
            format!("{0}/$name", power_prefix),
            QoS::AtLeastOnce,
            true,
            "Power",
        )
        .expect("Could not publish power property name");
        mqtt.publish(
            format!("{0}/$datatype", power_prefix),
            QoS::AtLeastOnce,
            true,
            "boolean",
        )
        .expect("Could not publish power property datatype");
        mqtt.publish(
            format!("{0}/$settable", power_prefix),
            QoS::AtLeastOnce,
            true,
            "true",
        )
        .expect("Could not publish power property settable = true");
        mqtt.publish(
            format!("{0}/$retained", power_prefix),
            QoS::AtLeastOnce,
            true,
            "true",
        )
        .expect("Could not publish pressed property retained = true");
        mqtt.subscribe(format!("{0}/set", power_prefix), QoS::AtLeastOnce)
            .expect("Could not subscribe to light power property");
    }

    mqtt.publish(
        format!("{0}/$state", homie_prefix),
        QoS::AtLeastOnce,
        true,
        "ready",
    )
    .expect("Could not publish Homie ready state");

    Ok(Box::new(Homie {
        config: config.clone(),
        client: mqtt.clone(),
        sender: sender.clone(),
    }))
}

pub fn last_will(config: &config::Config) -> LastWill {
    LastWill {
        topic: format!("homie/{0}/$state", config.mqtt.homie.device_id),
        message: "lost".to_owned(),
        qos: QoS::AtLeastOnce,
        retain: true,
    }
}
