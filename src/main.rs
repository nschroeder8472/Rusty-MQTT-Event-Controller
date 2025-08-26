use std::error::Error;
use std::time::Duration;
use rumqttc::{AsyncClient, Event, LastWill, MqttOptions, Packet, QoS};
use tokio::task;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut mqtt_options = MqttOptions::new("rusty-scene-controller", "10.56.18.200", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    client.subscribe("zwave/Deck/BackyardLight/37/0/currentValue", QoS::AtLeastOnce).await.unwrap();

    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                // Handle different types of notifications
                match notification {
                    Event::Incoming(Packet::Publish(publish)) => {
                        // Extract topic and payload
                        let topic = publish.topic;
                        let payload = String::from_utf8_lossy(&publish.payload);
                        println!("Received message on topic '{}'", topic);
                    }
                    Event::Incoming(event) => {
                        println!("Received event: {:?}", event);
                    }
                    Event::Outgoing(event) => {
                        println!("Outgoing event: {:?}", event);
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