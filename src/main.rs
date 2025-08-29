mod structs;
mod mqtt;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::iter::Map;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::time::Duration;
use clap::Parser;
use mlua::{FromLuaMulti, Function, Lua};
use mlua::prelude::LuaError;
use rumqttc::{AsyncClient, Event, EventLoop, LastWill, MqttOptions, Packet, QoS};
use tokio::task;
use crate::mqtt::mqtt_client::{publish_message, setup_mqtt};
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

    let event_loop = setup_mqtt(&config).await;

    let mut map: HashMap<String, String>= HashMap::new();
    for sub_event in config.sub_events {
        let script_path = PathBuf::from(Path::new(&config.script_dir))
            .join(Path::new(&sub_event.lua_script_file));
        if script_path.exists() {
            map.insert(sub_event.sub_topic, script_path.to_str().unwrap().to_string());
        }
    }
    let func_map = setup_lua(map);

    start_event_loop(event_loop, func_map).await;

    println!("after the event loop")
}

fn setup_lua(map: HashMap<String, String>) -> HashMap<String, Function> {
    let lua = Lua::new();
    lua.globals().set(
        "publish_message",
        lua.create_async_function(|_, (topic, payload): (String, String)| async move {
            publish_message(&topic, &payload)
                .await.expect("Failed to publish message");
            Ok(())
        }).expect("Failed to create publish_message function in Lua"),
    ).expect("Failed to set publish_message function in Lua");
    let mut func_map = HashMap::new();
    for entry in map {
        let script_content = fs::read_to_string(entry.1.clone()).expect("Failed to read Lua script");
        let chunk = lua.load(script_content);
        let func: Function = chunk.eval().expect("Failed to load Lua script");
        func_map.insert(entry.0, func);
    };

    func_map
}

async fn start_event_loop(mut event_loop: EventLoop, map: HashMap<String, Function>) {
    loop {
        match event_loop.poll().await {
            Ok(notification) => {
                match notification {
                    Event::Incoming(Packet::Publish(publish)) => {
                        // Extract topic and payload
                        let topic = publish.topic;
                        let payload = String::from_utf8_lossy(&publish.payload).to_string();
                        let func_opt = map.get(&topic);
                        if func_opt.is_some() {
                            let result: String = func_opt.unwrap().call(payload).expect("Lua script failed to execute");
                            println!("{:?}", result);
                        }
                    }
                    _ => {}
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