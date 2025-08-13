use std::error::Error;
use std::time::Duration;
use rumqttc::{AsyncClient, LastWill, MqttOptions};
use tokio::task;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut mqttoptions = MqttOptions::new("rusty-scene-controller", "10.56.18.200", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    task::spawn(async move {
        req
    })
}

async fn requests(client: AsyncClient) {
    client.subscribe()
}