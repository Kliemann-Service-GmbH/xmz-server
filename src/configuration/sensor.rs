use sensor::SensorType;



#[derive(Debug, Deserialize)]
pub struct Sensor {
    id: u32,
    sensor_type: SensorType,
}
