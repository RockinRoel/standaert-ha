use log::debug;
use serde_derive::Deserialize;
use std::fs;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub serial: SerialConfig,
}

#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    #[serde(default = "default_mqtt_client_id")]
    pub client_id: String,
    pub host: String,
    #[serde(default = "default_mqtt_port")]
    pub port: u16,
    pub homeassistant: HomeAssistantConfig,
}

#[derive(Debug, Deserialize)]
pub struct HomeAssistantConfig {
    #[serde(default = "default_ha_enabled")]
    pub enabled: bool,
    #[serde(default = "default_ha_prefix")]
    pub prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct SerialConfig {
    pub port: String,
    #[serde(default = "default_serial_baud_rate")]
    pub baud_rate: u32,
    #[serde(default = "default_serial_timeout", with = "serde_millis")]
    pub timeout: Duration,
}

fn default_ha_enabled() -> bool {
    false
}

fn default_ha_prefix() -> String {
    String::from("standaertha")
}

fn default_mqtt_client_id() -> String {
    String::from("standaertha-gateway")
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
