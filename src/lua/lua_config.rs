use std::collections::HashMap;
use std::fs;
use mlua::{Function, Lua};
use crate::mqtt::mqtt_client::publish_message;

pub async fn setup_lua(map: HashMap<String, String>) -> (Lua, HashMap<String, Function>) {
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
        chunk.exec_async().await.expect("Failed to execute function");
        let func: Function = lua.globals().get("myFunction").expect("Failed to get function");
        func_map.insert(entry.0, func);
    };
    (lua, func_map)
}