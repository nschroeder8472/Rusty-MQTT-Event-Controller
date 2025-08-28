use std::time::Duration;
use rumqttc::{AsyncClient, EventLoop, MqttOptions};
use crate::structs::config::Config;
pub async fn setup_mqtt(config: &Config) -> EventLoop {
    let mut mqtt_options = MqttOptions::new(&config.client_id, &config.host, config.port.clone());
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    let (mqtt_client, event_loop) = AsyncClient::new(mqtt_options, 10);
    for sub_event in &config.sub_events {
        mqtt_client.subscribe(&sub_event.sub_topic, sub_event.qos).await.unwrap();
    }
    event_loop
}