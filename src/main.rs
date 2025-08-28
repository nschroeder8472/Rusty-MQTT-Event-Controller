mod structs;
mod mqtt;

use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use clap::Parser;
use rumqttc::{AsyncClient, Event, LastWill, MqttOptions, Packet, QoS};
use tokio::task;
use crate::mqtt::mqtt_client::setup_mqtt;
use crate::structs::config::{setup_config, Config};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'c', long = "config", default_value = "/data/config.json")]
    config_file: String,
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Cli::parse();
    let config: Config = setup_config(args.config_file);

    let mut event_loop = setup_mqtt(&config).await;

    let mut map: HashMap<String, String>= HashMap::new();
    for sub_event in config.sub_events {
        map.insert(sub_event.sub_topic, sub_event.lua_script_file);
    }
    loop {
        match event_loop.poll().await {
            Ok(notification) => {
                // Handle different types of notifications
                match notification {
                    Event::Incoming(Packet::Publish(publish)) => {
                        // Extract topic and payload
                        let topic = publish.topic;
                        let payload = String::from_utf8_lossy(&publish.payload);
                        let message = payload.to_string();
                        if message == "3" {
                            println!("{}", map.get(&topic).unwrap());
                        }
                    }
                    Event::Incoming(event) => {
                        //println!("Received event: {:?}", event);
                    }
                    Event::Outgoing(event) => {
                        //println!("Outgoing event: {:?}", event);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error polling event loop: {:?}", e);
                // Optionally, add a delay before retrying to avoid tight loops
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}