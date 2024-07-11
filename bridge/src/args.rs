use clap::Parser;
use std::fmt::{Display, Formatter};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// MQTT broker host
    #[arg(long, env = "SHA_MQTT_URL")]
    pub mqtt_url: Option<String>,

    /// MQTT broker user
    #[arg(long, env = "SHA_MQTT_USER")]
    pub mqtt_user: Option<String>,

    /// MQTT broker password
    #[arg(long, env = "SHA_MQTT_PASSWORD")]
    pub mqtt_password: Option<String>,

    /// Home assistant discovery prefix
    #[arg(long, default_value = "homeassistant", env = "SHA_DISCOVERY_PREFIX")]
    pub prefix: String,

    /// Serial device
    #[arg(long, env = "SHA_SERIAL_DEVICE")]
    pub serial: Option<String>,

    /// Program location
    #[arg(long, env = "SHAL_PROGRAM")]
    pub program: Option<String>,

    /// Determines whether we actually upload the program
    #[arg(long, default_value_t = false, env = "SHA_UPLOAD")]
    pub upload: bool,

    /// Verbose
    #[arg(long, default_value_t = false, env = "SHA_DEBUG")]
    pub debug: bool,
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(mqtt_url) = &self.mqtt_url {
            writeln!(f, "  MQTT options:")?;
            writeln!(f, "    URL: {}", mqtt_url)?;
            if let Some(mqtt_user) = &self.mqtt_user {
                writeln!(f, "    user: {}", mqtt_user)?;
            } else {
                writeln!(f, "    user: <none>")?;
            }
            writeln!(
                f,
                "    password: {}",
                if self.mqtt_password.is_some() {
                    "***"
                } else {
                    "<none>"
                }
            )?;
            writeln!(f, "    prefix: {}", self.prefix)?;
        } else {
            writeln!(f, "  MQTT: disabled")?;
        }
        if let Some(serial) = &self.serial {
            writeln!(f, "  Serial port: {}", serial)?;
        } else {
            writeln!(f, "  Serial: <disabled>")?;
        }
        if let Some(program) = &self.program {
            writeln!(f, "  Program:")?;
            writeln!(f, "    path: {program}")?;
            writeln!(
                f,
                "    upload: {}",
                if self.upload { "enabled" } else { "disabled" }
            )?;
        } else {
            writeln!(f, "  Program: <disabled>")?;
        }
        writeln!(
            f,
            "  Debug: {}",
            if self.debug { "enabled" } else { "disabled" }
        )
    }
}
