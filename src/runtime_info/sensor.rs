

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sensor {
    id: u32,
    pub sensor_type: ::sensor::SensorType,
    messzellen: Vec<::runtime_info::Messzelle>,
}

impl From<Sensor> for ::sensor::RaGasCONO2Mod {
    fn from(sensor: Sensor) -> Self {
        ::sensor::RaGasCONO2Mod {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::RaGasCONO2Mod,
            messzellen: Vec::new(),
        }
    }
}

impl From<Sensor> for ::sensor::MetzConnectCI4 {
    fn from(sensor: Sensor) -> Self {
        ::sensor::MetzConnectCI4 {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::MetzConnectCI4,
            messzellen: Vec::new(),
        }
    }
}

impl From<Sensor> for ::sensor::TestSensor {
    fn from(sensor: Sensor) -> Self {
        ::sensor::TestSensor {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::TestSensor,
            messzellen: Vec::new(),
        }
    }
}

/// Konvertierung von den Sensor Trait Objekten `server::Sensor`
///
///
impl<'a> From<&'a Box<::sensor::Sensor + Send>> for Sensor {
    fn from(sensor: &'a Box<::sensor::Sensor + Send>) -> Self {
        // Kontruiere Messzellen
        let mut messzellen: Vec<::runtime_info::Messzelle> = vec![];
        for messzelle in sensor.get_messzellen() {
            if let Ok(messzelle) = messzelle.lock() {
                messzellen.push((&*messzelle).into())
            }
        }
        Sensor {
            id: sensor.get_id(),
            messzellen: messzellen,
            sensor_type: sensor.get_sensor_type(),
        }
    }
}
