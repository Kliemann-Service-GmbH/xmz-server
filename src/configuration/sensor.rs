use sensor::SensorType;
use configuration;


#[derive(Debug, Deserialize)]
pub struct Sensor {
    id: u32,
    sensor_type: SensorType,
    messzellen: Vec<configuration::Messzelle>,
}
