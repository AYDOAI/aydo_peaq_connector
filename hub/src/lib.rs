use node::calls::{call::Call, extrinsic::Extrinsic};
use node::errors::NodeError;
use node::extrinsics::{AddAttribute, ExtrinsicCall};
use rand::{distributions::Alphanumeric, Rng};
use rumqttc::{Client, Connection, ConnectionError, Event, Incoming, MqttOptions, Publish, QoS};
use sp_core::crypto::AccountId32;
use sp_core::Pair;
use sp_keyring::AccountKeyring;
use std::time::Duration;

pub mod consts {
    pub const CLIENT_ID: &str = "aydo-peaq-client-id";
    pub const BROKER_IP: &str = "127.0.0.1";
    pub const BROKER_PORT: u16 = 1883;
    pub const TARGET: Option<&str> = None;
    pub const TOPIC: &str = "aydo/#";
}

pub async fn run(url: &str) -> Result<(), ConnectionError> {
    let (client, mut connection) = make_connection();
    client.subscribe(consts::TOPIC, QoS::AtMostOnce).unwrap();

    let mut iter = connection.iter().enumerate();
    while let Some((index, event)) = iter.next() {
        if let Some(publish) = process_event(event?).await {
            let extrinsic = compose_tx().await;
            let tx = send_tx(url, extrinsic).await;
            let _ = print_results(index, publish, tx);
        }
    }
    Ok(())
}

fn make_connection() -> (Client, Connection) {
    let mut mqtt = MqttOptions::new(consts::CLIENT_ID, consts::BROKER_IP, consts::BROKER_PORT);
    mqtt.set_keep_alive(Duration::from_secs(4));

    Client::new(mqtt, 10)
}

fn print_results(index: usize, publish: Publish, tx: Result<String, NodeError>) {
    println!(
        "Index: {} Target: {} Value: {}",
        index,
        publish.topic,
        String::from_utf8(publish.payload.to_vec()).unwrap()
    );
    println!("↳  TX: {:?}", tx);
}

async fn process_event(event: Event) -> Option<Publish> {
    match event {
        Event::Incoming(in_event) => process_in_event(in_event).await,
        _ => None,
    }
}

async fn process_in_event(in_event: rumqttc::Incoming) -> Option<Publish> {
    match in_event {
        Incoming::Publish(publish) => process_publish(publish).await,
        _ => None,
    }
}

async fn process_publish(publish: Publish) -> Option<Publish> {
    match consts::TARGET {
        Some(target) => process_target(publish, target).await,
        None => Some(publish),
    }
}

async fn process_target(publish: Publish, target: &str) -> Option<Publish> {
    match publish.topic.eq(target) {
        true => Some(publish),
        false => None,
    }
}

fn get_rand_chars() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

fn get_name() -> Vec<u8> {
    format!("did:aydo:{}-{}", consts::CLIENT_ID, get_rand_chars())
        .as_bytes()
        .to_vec()
}

async fn compose_tx() -> node::extrinsics::Extrinsic<AddAttribute> {
    let alice = AccountKeyring::Alice.pair();
    let did_account = AccountId32::from(alice.public());
    let name = get_name();
    let value = Vec::new();

    AddAttribute::new(alice, (did_account, name, value, None))
}

async fn send_tx(
    url: &str,
    extrinsic: node::extrinsics::Extrinsic<AddAttribute>,
) -> Result<String, NodeError> {
    let extrinsic_hash = extrinsic.build(url).await?;
    Ok(Extrinsic.get(url, Some(vec![extrinsic_hash])).await?)
}
