use crate::controller::command::Command;
use crate::controller::event::Event;
use crate::controller::message::MessageBody;
use crate::handlers::handler::{HandleResult, Handler};
use crate::handlers::message::Message;
use if_chain::if_chain;
use rumqttc::{AsyncClient, EventLoop, Incoming, MqttOptions, OptionError, QoS};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;
use tokio::select;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub struct MqttHandler {
    client_task: JoinHandle<()>,
    event_loop_task: JoinHandle<()>,
    mqtt_sender: UnboundedSender<MessageBody>,
    cancellation_token: CancellationToken,
}

struct MqttHandlerClientTask {
    client: AsyncClient,
    mqtt_receiver: UnboundedReceiver<MessageBody>,
    cancellation_token: CancellationToken,
    options: MqttOptions,
    prefix: String,
}

struct MqttHandlerEventLoopTask {
    event_loop: EventLoop,
    cancellation_token: CancellationToken,
    sender: UnboundedSender<Message>,
    options: MqttOptions,
    prefix: String,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum MqttHandlerError {
    #[error("MQTT option error")]
    OptionError(#[from] OptionError),
}

impl MqttHandler {
    pub fn new(
        url: String,
        credentials: Option<(String, String)>,
        prefix: String,
        sender: UnboundedSender<Message>,
    ) -> Result<Self, MqttHandlerError> {
        let mut options = MqttOptions::parse_url(url)?;
        if let Some(credentials) = credentials {
            options.set_credentials(credentials.0, credentials.1);
        }
        let cancellation_token = CancellationToken::new();
        let (mqtt_sender, mqtt_receiver) = tokio::sync::mpsc::unbounded_channel::<MessageBody>();
        let (client, event_loop) = AsyncClient::new(options.clone(), 10);
        let mut client_task = MqttHandlerClientTask {
            client: client.clone(),
            mqtt_receiver,
            cancellation_token: cancellation_token.clone(),
            options: options.clone(),
            prefix: prefix.clone(),
        };
        let mut event_loop_task = MqttHandlerEventLoopTask {
            event_loop,
            cancellation_token: cancellation_token.clone(),
            sender,
            options: options.clone(),
            prefix: prefix.clone(),
        };
        let client_task = tokio::spawn(async move {
            client_task.run().await;
        });
        let event_loop_task = tokio::spawn(async move {
            event_loop_task.run().await;
        });
        Ok(Self {
            client_task,
            event_loop_task,
            mqtt_sender,
            cancellation_token,
        })
    }
}

impl MqttHandlerClientTask {
    async fn run(&mut self) {
        self.announce().await;
        self.subscribe().await;
        loop {
            select! {
                message = self.mqtt_receiver.recv() => {
                    if let Some(MessageBody::Update { outputs, events}) = message {
                        for i in 0..32 {
                            let state_topic = format!("{}/light/{}/{}/status", self.prefix, self.options.client_id(), i);
                            self.client.publish(
                                state_topic,
                                QoS::AtLeastOnce,
                                true,
                                if (outputs & (1 << i)) == 0 { "OFF" } else { "ON" },
                            ).await.unwrap();
                        }
                        for event in &events {
                            let (i, state) = match event {
                                Event::RisingEdge(i) => (i, "OFF"),
                                Event::FallingEdge(i) => (i, "ON"),
                            };
                            let state_topic = format!("{}/binary_sensor/{}/{}/pressed", self.prefix, self.options.client_id(), i);
                            self.client.publish(
                                state_topic,
                                QoS::AtLeastOnce,
                                true,
                                state
                            ).await.unwrap();
                        }
                    }
                },
                _ = self.cancellation_token.cancelled() => break,
            }
        }
    }

    async fn announce(&mut self) {
        for i in 0..32 {
            let prefix = format!(
                "{}/binary_sensor/{}/{}",
                self.prefix,
                self.options.client_id(),
                i
            );
            let discovery_topic = format!("{}/config", prefix);
            let state_topic = format!("{}/pressed", prefix);
            let spec = BinarySensorSpec {
                unique_id: format!("{}_input_{}", self.options.client_id(), i),
                name: format!("Standaert Home Automation button #{}", i),
                icon: "mdi:light-switch-off".to_string(),
                state_topic: state_topic.clone(),
            };
            self.client
                .publish(
                    discovery_topic,
                    QoS::AtLeastOnce,
                    true,
                    serde_json::to_string(&spec).unwrap(),
                )
                .await
                .unwrap();
            self.client
                .publish(state_topic, QoS::AtLeastOnce, true, "OFF")
                .await
                .unwrap();
        }
        // Announce lights
        for i in 0..32 {
            let prefix = format!("{}/light/{}/{}", self.prefix, self.options.client_id(), i);
            let discovery_topic = format!("{}/config", prefix);
            let state_topic = format!("{}/status", prefix);
            let command_topic = format!("{}/switch", prefix);
            let spec = LightSpec {
                unique_id: format!("{}_output_{}", self.options.client_id(), i),
                name: format!("Standaert Home Automation light #{}", i),
                state_topic: state_topic.clone(),
                command_topic: command_topic.clone(),
            };
            self.client
                .publish(
                    discovery_topic,
                    QoS::AtLeastOnce,
                    true,
                    serde_json::to_string(&spec).unwrap(),
                )
                .await
                .unwrap();
            self.client
                .publish(state_topic, QoS::AtLeastOnce, true, "OFF")
                .await
                .unwrap();
        }
    }

    async fn subscribe(&mut self) {
        let topic = format!(
            "{}/light/{}/+/switch",
            self.prefix,
            self.options.client_id()
        );
        self.client
            .subscribe(topic, QoS::AtLeastOnce)
            .await
            .unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BinarySensorSpec {
    unique_id: String,
    name: String,
    icon: String,
    state_topic: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LightSpec {
    unique_id: String,
    name: String,
    command_topic: String,
    state_topic: String,
}

impl MqttHandlerEventLoopTask {
    async fn run(&mut self) {
        loop {
            select! {
                notification = self.event_loop.poll() => {
                    match notification {
                        Ok(event) => {
                            let prefix = format!("{}/light/{}/", self.prefix, self.options.client_id());
                            if_chain!(
                                if let rumqttc::Event::Incoming(incoming) = event;
                                if let Incoming::Publish(publish) = incoming;
                                if let Some(suffix) = publish.topic.strip_prefix(&prefix);
                                if let Some(id) = suffix.strip_suffix("/switch");
                                if let Ok(id) = id.parse::<u8>();
                                if id < 32;
                                if let Ok(payload) = String::from_utf8(publish.payload.to_vec());
                                then {
                                    let command = match &payload[..] {
                                        "ON" => Command::On(id),
                                        "OFF" => Command::Off(id),
                                        _ => continue,
                                    };
                                    self.sender.send(Message::SendToController(
                                        MessageBody::Command {
                                            commands: vec![command],
                                        }
                                    )).unwrap_or_else(|_| unreachable!());
                                }
                            )
                        },
                        Err(err) => {
                            eprintln!("Connection error: {}", err);
                            break;
                        }
                    }
                },
                _ = self.cancellation_token.cancelled() => break,
            }
        }
    }
}

impl Handler for MqttHandler {
    fn handle(&mut self, message: &Message) -> HandleResult {
        match message {
            Message::ReceivedFromController(message_body) => {
                self.mqtt_sender
                    .send(message_body.clone())
                    .unwrap_or_else(|_| unreachable!());
            }
            Message::Stop => {
                self.cancellation_token.cancel();
                while !self.client_task.is_finished() {
                    // Busy wait??
                }
                while !self.event_loop_task.is_finished() {
                    // Busy wait??
                }
            }
            _ => {}
        }
        HandleResult::Continue
    }
}
