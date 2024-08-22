
mod config;

use config::ConfigProvider;

use paho_mqtt as mqtt;
use std::{env, process};
use std::path::Path;
use std::fs::File;
use std::time::Duration;
use serde_json::json;
use config::MQTTMessage;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

const QOS: i32 = 1;

fn timestamp() -> u64 {
    let a = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    a
}

fn generate_random_numbers() -> i32 {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..30);
    num
}

fn main() {
    // Initialize the logger from the environment
    env_logger::init();

    // Load config
    let env_config_provider = config::DotEnvConfigProvider::new();
    let mqtt_host = env_config_provider.get_config().mqtt_host.clone();
    let mqtt_port = env_config_provider.get_config().mqtt_port.clone();
    let mqtt_topic = env_config_provider.get_config().mqtt_topic.clone();

    // Set default MQTT connstring
    let mqtt_connstring_header: String = "mqtt://".to_owned();
    let mqtt_connstring = format!("{mqtt_connstring_header}{mqtt_host}:{mqtt_port}");

    // Create a client & define connect options
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| mqtt_connstring.to_string());

    let mut client = mqtt::Client::new(host).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    
    // Use 5sec timeouts for sync calls.
    client.set_timeout(Duration::from_secs(5));

    // Connect and wait for it to complete or fail.
    // The default connection uses MQTT v3.x
    if let Err(e) = client.connect(None) {
        println!("Unable to connect: {:?}", e);
        process::exit(1);
    }

    // Load JSON 
    let path_string = String::from("pub/src/message.json");
    let json_file_path = Path::new(&path_string);
    let file = File::open(json_file_path).expect("file not found");
    let messages: Vec<MQTTMessage> = serde_json::from_reader(file).expect("error while reading");
    println!("{:?}", messages);

    // Create a message and publish it
    loop {
        // Get the current epoch
        let ts = timestamp();

        // The payload is the JSON array
        let payload = json!({"measurement": "bedroom_temperature", "timestamp": ts, "celsius": generate_random_numbers()}).to_string();
        let payload_message = payload.clone();
        // println!("{}", payload);
        let msg = mqtt::MessageBuilder::new()
            .topic(&mqtt_topic)
            .payload(payload)
            .qos(QOS)
            .finalize();
        println!("Publishing payload: {} on the {:?} topic", payload_message, &mqtt_topic);
        if let Err(e) = client.publish(msg) {
            println!("Error sending message: {:?}", e);
        } 
    }

    // // Disconnect from the broker
    // client.disconnect(None).unwrap();

}