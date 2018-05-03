use sensor::SensorType;
use configuration;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sensor {
    id: u32,
    pub sensor_type: SensorType,
    messzellen: Vec<configuration::Messzelle>,
}

impl From<Sensor> for ::sensor::RaGasCONO2Mod {
    fn from(sensor: Sensor) -> Self {
        ::sensor::RaGasCONO2Mod {
            sensor_type: ::sensor::SensorType::RaGasCONO2Mod,
            messzellen: Vec::new(),
        }
    }
}

impl From<Sensor> for ::sensor::MetzConnectCI4 {
    fn from(sensor: Sensor) -> Self {
        ::sensor::MetzConnectCI4 {
            sensor_type: ::sensor::SensorType::MetzConnectCI4,
            messzellen: Vec::new(),
        }
    }
}

impl From<Sensor> for ::sensor::TestSensor {
    fn from(sensor: Sensor) -> Self {
        ::sensor::TestSensor {
            sensor_type: ::sensor::SensorType::TestSensor,
            messzellen: Vec::new(),
        }
    }
}
