pub mod homeassistant;
pub mod homie;

use super::{config, Command, CommandType, EventType, Package, Service};
use crossbeam::channel::Sender;
use crossbeam::crossbeam_channel as channel;
use crossbeam::select;
use log::warn;
use rumqtt::client::Notification;
use rumqtt::{MqttClient, MqttOptions};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub trait MqttSubService {
    fn handle_package(&mut self, package: &Package);
    fn handle_notification(&mut self, notification: &Notification);
    fn do_disconnect(&mut self) {}
}

struct MqttService {
    thread: Option<JoinHandle<()>>,
    sender: Sender<Package>,
    running: Arc<AtomicBool>,
}

impl MqttService {
    fn new(
        thread: JoinHandle<()>,
        sender: Sender<Package>,
        running: Arc<AtomicBool>,
    ) -> MqttService {
        MqttService {
            thread: Some(thread),
            sender,
            running,
        }
    }
}

impl Service for MqttService {
    fn handle_package(&mut self, package: &Package) {
        self.sender.send(package.clone()).unwrap();
    }

    fn join(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if self.thread.is_some() {
            self.thread.take().unwrap().join().expect("Can't join?");
        }
    }
}

pub fn init(
    config: &config::Config,
    sender: &Sender<Command>,
) -> Result<Option<Box<dyn Service>>, Box<dyn std::error::Error + 'static>> {
    if !config.mqtt.homeassistant.enabled && !config.mqtt.homie.enabled {
        warn!("No MQTT service is enabled, not setting up MQTT");
        return Ok(None);
    }

    let mut mqtt_opts = MqttOptions::new(
        config.mqtt.client_id.clone(),
        config.mqtt.host.clone(),
        config.mqtt.port,
    );

    if config.mqtt.homie.enabled {
        mqtt_opts = mqtt_opts.set_last_will(homie::last_will(config));
    }

    let (mut mqtt, notifications) =
        MqttClient::start(mqtt_opts).expect("Could not create MQTT connection");

    let mut sub_services: Vec<Box<dyn MqttSubService + Send + 'static>> = Vec::new();

    if config.mqtt.homeassistant.enabled {
        sub_services.push(homeassistant::init(&config, &mut mqtt, sender)?);
    }

    if config.mqtt.homie.enabled {
        sub_services.push(homie::init(&config, &mut mqtt, sender)?);
    }

    let (sender, receiver) = channel::unbounded();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let thread = thread::spawn(move || loop {
        select! {
            recv(notifications) -> n => {
                if let Ok(n) = n {
                    for service in &mut sub_services {
                        service.handle_notification(&n);
                    }
                }
            },
            recv(receiver) -> pkg => {
                if let Ok(pkg) = pkg {
                    for service in &mut sub_services {
                        service.handle_package(&pkg);
                    }
                }
            },
            default(Duration::from_millis(1000)) => {
            },
        }
        if !r.load(Ordering::SeqCst) {
            break;
        }
    });

    Ok(Some(Box::new(MqttService::new(thread, sender, running))))
}
