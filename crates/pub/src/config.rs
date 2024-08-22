use serde::Deserialize;

#[derive(Debug, Default)]
pub struct Config {
    pub mqtt_host: String,
    pub mqtt_port: String,
    pub mqtt_topic: String,
    pub username: String,
    pub password: String,
}

pub trait ConfigProvider {
    fn get_config(&self) -> &Config;
}

pub struct DotEnvConfigProvider(Config);

impl DotEnvConfigProvider {
    pub fn new() -> Self {
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();
        let config = Config {
            mqtt_host: env::var("MQTT_HOST").expect("Missing MQTT hostname"),
            mqtt_port: env::var("MQTT_PORT").expect("Missing MQTT port"),
            mqtt_topic: env::var("MQTT_TOPIC").expect("Missing MQTT topic"),
            username: env::var("MQTT_USERNAME").expect("Missing MQTT username"),
            password: env::var("MQTT_PASSWORD").expect("Missing MQTT password"),
        };

        DotEnvConfigProvider(config)
    }
}

impl ConfigProvider for DotEnvConfigProvider {
    fn get_config(&self) -> &Config {
        &self.0
    }
}

impl Default for DotEnvConfigProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MQTTMessage {
    measurement: String,
    timestamp: String,
    celsius: String
}
