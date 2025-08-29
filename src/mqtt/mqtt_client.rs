use std::error::Error;
use std::time::Duration;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use tokio::sync::OnceCell;
use crate::structs::config::Config;

static MQTT_CLIENT: OnceCell<AsyncClient> = OnceCell::const_new();

pub async fn setup_mqtt(config: &Config) -> EventLoop {
    let mut mqtt_options = MqttOptions::new(&config.client_id, &config.host, config.port.clone());
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    let (mqtt_client, event_loop) = AsyncClient::new(mqtt_options, 10);
    for sub_event in &config.sub_events {
        mqtt_client.subscribe(&sub_event.sub_topic, sub_event.qos).await.unwrap();
    }
    MQTT_CLIENT.set(mqtt_client).unwrap();
    event_loop
}

pub async fn publish_message(topic: &str, payload: &str) -> Result<(), Box<dyn Error>> {
    let client = MQTT_CLIENT
        .get()
        .ok_or("MQTT client not initialized")?;

    client
        .publish(topic, QoS::ExactlyOnce, false, payload.to_string())
        .await?;

    println!("Published message to {}: {}", topic, payload);
    Ok(())
}