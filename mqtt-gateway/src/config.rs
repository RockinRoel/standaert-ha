use log::debug;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::time::Duration;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub webthing: WebThingConfig,
    pub serial: SerialConfig,
    #[serde(default = "default_lights_config")]
    pub lights: HashMap<String, LightConfig>,
    #[serde(default = "default_buttons_config")]
    pub buttons: HashMap<String, ButtonConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MqttConfig {
    #[serde(default = "default_mqtt_client_id")]
    pub client_id: String,
    pub host: String,
    #[serde(default = "default_mqtt_port")]
    pub port: u16,
    pub homeassistant: HomeAssistantConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HomeAssistantConfig {
    #[serde(default = "default_ha_enabled")]
    pub enabled: bool,
    #[serde(default = "default_ha_prefix")]
    pub prefix: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SerialConfig {
    pub port: String,
    #[serde(default = "default_serial_baud_rate")]
    pub baud_rate: u32,
    #[serde(default = "default_serial_timeout", with = "serde_millis")]
    pub timeout: Duration,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebThingConfig {
    #[serde(default = "default_webthing_enabled")]
    pub enabled: bool,
    pub base_uri: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LightConfig {
    pub index: u8,
    #[serde(default = "default_light_name")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ButtonConfig {
    pub index: u8,
    #[serde(default = "default_button_name")]
    pub name: String,
}

fn default_ha_enabled() -> bool {
    false
}

fn default_ha_prefix() -> String {
    "homeassistant".to_owned()
}

fn default_mqtt_client_id() -> String {
    "standaertha-gateway".to_owned()
}

fn default_mqtt_port() -> u16 {
    1883
}

fn default_serial_baud_rate() -> u32 {
    9600
}

fn default_serial_timeout() -> Duration {
    Duration::from_secs(1)
}

fn default_webthing_enabled() -> bool {
    false
}

fn default_lights_config() -> HashMap<String, LightConfig> {
    HashMap::new()
}

fn default_buttons_config() -> HashMap<String, ButtonConfig> {
    HashMap::new()
}

fn default_light_name() -> String {
    "".to_owned()
}

fn default_button_name() -> String {
    "".to_owned()
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error + 'static>> {
    let clap_matches = clap::App::new("StandaertHA MQTT gateway")
        .version("0.1.0")
        .author("Roel Standaert <roel@abittechnical.com>")
        .about("Bridge between the StandaertHA I/O board and MQTT (HomeAssistant)")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets the path to the config file")
                .takes_value(true),
        )
        .get_matches();

    let config_path = clap_matches.value_of("config").unwrap_or("config.toml");
    debug!("Using config file: {}", config_path);

    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;

    debug!("Configuration:\n{:?}", config);

    Ok(config)
}
