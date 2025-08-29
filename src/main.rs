mod structs;
mod mqtt;
mod lua;

use crate::lua::lua_config::setup_lua;
use crate::mqtt::mqtt_client::setup_mqtt;
use crate::structs::config::{setup_config, Config};
use clap::Parser;
use mlua::{Function, Lua};
use rumqttc::{Event, EventLoop, Packet};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'c', long = "config", default_value = "/data/config.json")]
    config_file: String,
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Cli::parse();
    let config: Config = setup_config(args.config_file);

    let event_loop = setup_mqtt(&config).await;

    let mut map: HashMap<String, String>= HashMap::new();
    for sub_event in config.sub_events {
        let script_path = PathBuf::from(Path::new(&config.script_dir))
            .join(Path::new(&sub_event.lua_script_file));
        if script_path.exists() {
            map.insert(sub_event.sub_topic, script_path.to_str().unwrap().to_string());
        }
    }
    let (lua, func_map) = setup_lua(map).await;

    start_event_loop(event_loop, lua, func_map).await;

    println!("after the event loop")
}

async fn start_event_loop(mut event_loop: EventLoop, lua: Lua, map: HashMap<String, Function>) {
    loop {
        match event_loop.poll().await {
            Ok(notification) => {
                match notification {
                    Event::Incoming(Packet::Publish(publish)) => {
                        let topic = publish.topic;
                        let payload = String::from_utf8_lossy(&publish.payload).to_string();
                        let func_opt = map.get(&topic);
                        if func_opt.is_some() {
                            match func_opt.unwrap().call_async(payload).await.expect("Lua script failed to execute") {
                                true => println!("Successfully ran script for topic {}", topic),
                                false => ()
                            }
                        }

                    }
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("Error polling event loop: {:?}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}