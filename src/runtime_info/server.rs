use configuration;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "server")]
pub struct Server {
    pub service_interval: u32,
    pub sensors: Vec<configuration::sensor::Sensor>,
}
