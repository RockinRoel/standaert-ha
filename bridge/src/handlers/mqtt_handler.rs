use crate::controller::command::Command;
use crate::controller::event::Event;
use crate::controller::message::MessageBody;
use crate::handlers::message::Message;
use crate::handlers::message::Message::ReceivedFromController;
use if_chain::if_chain;
use rumqttc::{AsyncClient, EventLoop, Incoming, MqttOptions, OptionError, QoS};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::{join, select};
use tokio_util::sync::CancellationToken;

struct MqttHandlerClientTask {
    client: AsyncClient,
    rx: Receiver<Message>,
    options: MqttOptions,
    prefix: String,
    cancellation_token: CancellationToken,
}

struct MqttHandlerEventLoopTask {
    event_loop: EventLoop,
    tx: Sender<Message>,
    options: MqttOptions,
    prefix: String,
    cancellation_token: CancellationToken,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum MqttHandlerError {
    #[error("MQTT option error")]
    OptionError(#[from] OptionError),
}

pub async fn start(
    url: String,
    credentials: Option<(String, String)>,
    prefix: String,
    tx: Sender<Message>,
    cancellation_token: CancellationToken,
) -> Result<JoinHandle<()>, MqttHandlerError> {
    let mut options = MqttOptions::parse_url(url)?;
    if let Some(credentials) = credentials {
        options.set_credentials(credentials.0, credentials.1);
    }
    let (client, event_loop) = AsyncClient::new(options.clone(), 10);
    let mut client_task = MqttHandlerClientTask {
        client: client.clone(),
        rx: tx.subscribe(),
        options: options.clone(),
        prefix: prefix.clone(),
        cancellation_token: cancellation_token.clone(),
    };
    let mut event_loop_task = MqttHandlerEventLoopTask {
        event_loop,
        tx,
        options: options.clone(),
        prefix: prefix.clone(),
        cancellation_token,
    };
    // TODO(Roel): announce and subscribe here!!!
    let client_task = tokio::spawn(async move {
        client_task.run().await;
    });
    let event_loop_task = tokio::spawn(async move {
        event_loop_task.run().await;
    });
    Ok(tokio::spawn(async move {
        let (_, _) = join!(client_task, event_loop_task);
    }))
}

impl MqttHandlerClientTask {
    async fn run(&mut self) {
        self.announce().await;
        self.subscribe().await;
        loop {
            select! {
                message = self.rx.recv() => {
                    match message {
                        Ok(ReceivedFromController(body)) => {
                            self.handle_message_from_controller(&body).await;
                        }
                        Ok(_) => {}
                        Err(_) => break,
                    }
                },
                _ = self.cancellation_token.cancelled() => break,
            }
        }
    }

    async fn handle_message_from_controller(&mut self, body: &MessageBody) {
        if let MessageBody::Update { outputs, events } = body {
            for i in 0..32 {
                let state_topic = format!(
                    "{}/light/{}/{}/status",
                    self.prefix,
                    self.options.client_id(),
                    i
                );
                self.client
                    .publish(
                        state_topic,
                        QoS::AtLeastOnce,
                        true,
                        if (outputs & (1 << i)) == 0 {
                            "OFF"
                        } else {
                            "ON"
                        },
                    )
                    .await
                    .unwrap(); // TODO(Roel): unwrap?
            }
            for event in events {
                let (i, state) = match event {
                    Event::RisingEdge(i) => (i, "OFF"),
                    Event::FallingEdge(i) => (i, "ON"),
                };
                let state_topic = format!(
                    "{}/binary_sensor/{}/{}/pressed",
                    self.prefix,
                    self.options.client_id(),
                    i
                );
                self.client
                    .publish(state_topic, QoS::AtLeastOnce, true, state)
                    .await
                    .unwrap(); // TODO(Roel): unwrap?
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
                                    self.tx.send(Message::SendToController(
                                        MessageBody::Command {
                                            commands: vec![command],
                                        }
                                    )).unwrap_or_else(|_| unreachable!());
                                }
                            )
                        },
                        Err(err) => {
                            // TODO(Roel): handle?
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
