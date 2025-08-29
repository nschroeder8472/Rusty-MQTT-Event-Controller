use serde::{Deserialize, Deserializer};
use std::fs;
use std::path::PathBuf;
use rumqttc::QoS;
use serde::de::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub host: String,
    pub port: u16,
    pub script_dir: String,
    pub sub_events: Vec<SubEvent>
}

#[derive(Debug, Deserialize)]
pub struct SubEvent {
    #[serde(deserialize_with = "deserialize_qos")]
    pub qos: QoS,
    pub sub_topic: String,
    pub lua_script_file: String
}

pub fn setup_config(config_file: String) -> Config {
    let config_file = PathBuf::from(config_file);
    println!("Config File: {}", config_file.display());
    let config_str = match fs::read_to_string(config_file) {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to read config file: {:?}", e);
        }
    };

    match serde_json::from_str(&config_str) {
        Ok(config) => config,
        Err(e) => {
            panic!("Failed to parse config file: {:?}", e);
        }
    }
}

fn deserialize_qos<'de, D>(deserializer: D) -> Result<QoS, D::Error>
where D: Deserializer<'de>, {
    let value: u8 = u8::deserialize(deserializer)?;
    match value {
        0 => Ok(QoS::AtMostOnce),
        1 => Ok(QoS::AtLeastOnce),
        2 => Ok(QoS::ExactlyOnce),
        _ => Err(D::Error::custom(format!("Invalid QoS value: {}", value))),
    }
}