use crate::controller::command::Command;
use crate::controller::event::Event;
use crate::controller::message::MessageBody;
use crate::handlers::message::Message;
use crate::handlers::message::Message::ReceivedFromController;
use crate::shal::ast::PinID;
use crate::shal::bytecode::Program;
use if_chain::if_chain;
use rumqttc::{
    AsyncClient, ClientError, ConnectionError, EventLoop, Incoming, MqttOptions, OptionError, QoS,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::panic;
use log::warn;
use thiserror::Error;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinSet;
use tokio::select;
use tokio::sync::broadcast::error::RecvError;
use tokio_util::sync::CancellationToken;

const BUF_SIZE: usize = 100;

#[derive(Clone)]
pub struct MqttHandlerConfig {
    prefix: String,
    program: Option<Program>,
    options: MqttOptions,
}

pub struct MqttHandler {
    cancellation_token: CancellationToken,
    config: MqttHandlerConfig,
    client: AsyncClient,
    rx: Receiver<Message>,
    join_set: JoinSet<Result<(), MqttHandlerError>>,
    event_loop_cancellation_token: CancellationToken,
}

struct MqttEventLoop {
    cancellation_token: CancellationToken,
    config: MqttHandlerConfig,
    event_loop: EventLoop,
    tx: Sender<Message>,
}

#[derive(Debug, Error)]
pub enum MqttHandlerError {
    #[error("MQTT option error")]
    OptionError(#[from] OptionError),
    #[error("MQTT client error")]
    ClientError(#[from] ClientError),
    #[error("MQTT connection error")]
    ConnectionError(#[from] ConnectionError),
}

impl MqttHandler {
    pub async fn new(
        cancellation_token: CancellationToken,
        config: MqttHandlerConfig,
        tx: Sender<Message>,
    ) -> Result<Self, MqttHandlerError> {
        let rx = tx.subscribe();
        let (client, event_loop) = AsyncClient::new(config.options.clone(), BUF_SIZE);
        let event_loop_cancellation_token = CancellationToken::new();
        let mut event_loop = MqttEventLoop {
            cancellation_token: event_loop_cancellation_token.clone(),
            config: config.clone(),
            event_loop,
            tx,
        };
        let mut join_set = JoinSet::new();
        join_set.spawn(async move { event_loop.run().await });
        let mut mqtt_handler = MqttHandler {
            cancellation_token,
            config,
            client,
            rx,
            join_set,
            event_loop_cancellation_token,
        };
        let result = mqtt_handler.announce().await;
        if let Err(e) = result {
            mqtt_handler.event_loop_cancellation_token.cancel();
            let _ = mqtt_handler.join_set.join_next().await;
            return Err(e.into());
        }
        let result = mqtt_handler.subscribe().await;
        if let Err(e) = result {
            mqtt_handler.event_loop_cancellation_token.cancel();
            let _ = mqtt_handler.join_set.join_next().await;
            return Err(e.into());
        }
        Ok(mqtt_handler)
    }

    async fn announce(&mut self) -> Result<(), ClientError> {
        for i in 0..32 {
            let prefix = format!(
                "{}/binary_sensor/{}/{}",
                self.config.prefix,
                self.config.options.client_id(),
                i
            );
            let discovery_topic = format!("{}/config", prefix);
            let state_topic = format!("{}/pressed", prefix);
            let spec = BinarySensorSpec {
                unique_id: self.config.unique_input_id(i.try_into().unwrap()),
                name: self.config.input_name(i.try_into().unwrap()),
                icon: "mdi:light-switch-off".to_string(),
                state_topic: state_topic.clone(),
            };
            self.client
                .publish(
                    discovery_topic,
                    QoS::AtLeastOnce,
                    false,
                    serde_json::to_string(&spec).unwrap(),
                )
                .await?;
            self.client
                .publish(state_topic, QoS::AtLeastOnce, false, "OFF")
                .await?;
        }
        // Announce lights
        for i in 0..32 {
            let prefix = format!(
                "{}/light/{}/{}",
                self.config.prefix,
                self.config.options.client_id(),
                i
            );
            let discovery_topic = format!("{}/config", prefix);
            let state_topic = format!("{}/status", prefix);
            let command_topic = format!("{}/switch", prefix);
            let spec = LightSpec {
                unique_id: self.config.unique_output_id(i.try_into().unwrap()),
                name: self.config.output_name(i.try_into().unwrap()),
                state_topic: state_topic.clone(),
                command_topic: command_topic.clone(),
            };
            self.client
                .publish(
                    discovery_topic,
                    QoS::AtLeastOnce,
                    false,
                    serde_json::to_string(&spec).unwrap(),
                )
                .await?;
            self.client
                .publish(state_topic, QoS::AtLeastOnce, false, "OFF")
                .await?;
        }
        Ok(())
    }

    async fn subscribe(&mut self) -> Result<(), ClientError> {
        let topic = format!(
            "{}/light/{}/+/switch",
            self.config.prefix,
            self.config.options.client_id()
        );
        self.client.subscribe(topic, QoS::AtLeastOnce).await
    }

    pub async fn run(mut self) -> Result<(), MqttHandlerError> {
        let mut error: Option<MqttHandlerError> = None;
        loop {
            select! {
                _ = self.cancellation_token.cancelled() => break,
                join_result = self.join_set.join_next() => match join_result {
                    Some(Ok(Ok(()))) => break,
                    Some(Ok(Err(e))) => {
                        error = Some(e);
                        break;
                    }
                    Some(Err(e)) => if let Ok(panic) = e.try_into_panic() {
                        panic::resume_unwind(panic)
                    } else {
                        break;
                    },
                    None => break,
                },
                message = self.rx.recv() => {
                    match message {
                        Ok(ReceivedFromController(body)) => {
                            if let Err(e) = self.handle_message_from_controller(&body).await {
                                error = Some(e.into());
                                break;
                            }
                        }
                        Ok(_) => {}
                        Err(RecvError::Closed) => break,
                        Err(RecvError::Lagged(n)) => {
                            warn!("MQTT handler lagged behind {n} messages!");
                        }
                    }
                },
            }
        }
        self.event_loop_cancellation_token.cancel();
        match self.join_set.join_next().await {
            Some(Ok(result)) => error.map_or(result, Err),
            Some(Err(e)) => if let Ok(panic) = e.try_into_panic() {
                panic::resume_unwind(panic)
            } else {
                error.map_or(Ok(()), Err)
            }
            None => error.map_or(Ok(()), Err)
        }
    }

    async fn handle_message_from_controller(
        &mut self,
        body: &MessageBody,
    ) -> Result<(), ClientError> {
        if let MessageBody::Update { outputs, events } = body {
            for i in 0..32 {
                let state_topic = format!(
                    "{}/light/{}/{}/status",
                    self.config.prefix,
                    self.config.options.client_id(),
                    i
                );
                self.client
                    .publish(
                        state_topic,
                        QoS::AtLeastOnce,
                        false,
                        if (outputs & (1 << i)) == 0 {
                            "OFF"
                        } else {
                            "ON"
                        },
                    )
                    .await?;
            }
            for event in events {
                let (i, state) = match event {
                    Event::RisingEdge(i) => (i, "OFF"),
                    Event::FallingEdge(i) => (i, "ON"),
                };
                let state_topic = format!(
                    "{}/binary_sensor/{}/{}/pressed",
                    self.config.prefix,
                    self.config.options.client_id(),
                    i
                );
                self.client
                    .publish(state_topic, QoS::AtLeastOnce, false, state)
                    .await?;
            }
        }
        Ok(())
    }
}

impl MqttHandlerConfig {
    pub fn new(
        prefix: String,
        program: Option<Program>,
        url: String,
        credentials: Option<(String, String)>,
    ) -> Result<Self, MqttHandlerError> {
        let mut options = MqttOptions::parse_url(url)?;
        if let Some(credentials) = credentials {
            options.set_credentials(credentials.0, credentials.1);
        }
        Ok(MqttHandlerConfig {
            prefix,
            program,
            options,
        })
    }

    fn input_name(&self, pin: PinID) -> String {
        if_chain!(
            if let Some(program) = &self.program;
            if let Some(declaration) = program.declarations.inputs.values().find(|&declaration| declaration.pin == pin);
            if let Some(name) = &declaration.name;
            then {
                name.clone()
            } else {
                format!("Input {pin}")
            }
        )
    }

    fn input_id(&self, pin: PinID) -> String {
        if_chain!(
            if let Some(program) = &self.program;
            if let Some((id, _)) = program.declarations.inputs.iter().find(|(_, declaration)| declaration.pin == pin);
            then {
                id.clone().into()
            } else {
                pin.to_string()
            }
        )
    }

    fn unique_input_id(&self, pin: PinID) -> String {
        let client_id = &self.options.client_id();
        let input_id = self.input_id(pin);
        format!("{client_id}_input_{input_id}")
    }

    fn output_name(&self, pin: PinID) -> String {
        if_chain!(
            if let Some(program) = &self.program;
            if let Some(declaration) = program.declarations.outputs.values().find(|&declaration| declaration.pin == pin);
            if let Some(name) = &declaration.name;
            then {
                name.clone()
            } else {
                format!("Output {pin}")
            }
        )
    }

    fn output_id(&self, pin: PinID) -> String {
        if_chain!(
            if let Some(program) = &self.program;
            if let Some((id, _)) = program.declarations.outputs.iter().find(|(_, declaration)| declaration.pin == pin);
            then {
                id.clone().into()
            } else {
                pin.to_string()
            }
        )
    }

    fn unique_output_id(&self, pin: PinID) -> String {
        let client_id = &self.options.client_id();
        let output_id = self.output_id(pin);
        format!("{client_id}_output_{output_id}")
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

impl MqttEventLoop {
    async fn run(&mut self) -> Result<(), MqttHandlerError> {
        loop {
            select! {
                _ = self.cancellation_token.cancelled() => return Ok(()),
                notification = self.event_loop.poll() => match notification {
                    Ok(event) => {
                        let prefix = format!("{}/light/{}/", self.config.prefix, self.config.options.client_id());
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
                        self.cancellation_token.cancel();
                        return Err(err.into());
                    }
                },
            }
        }
    }
}
