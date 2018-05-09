use prelude::*;



/// Test Sensor
///
#[derive(Debug)]
pub struct TestSensor {
    /// Sensor ID
    pub id: u32,
    /// Sensor Type
    pub sensor_type: SensorType,
    /// Liste der Messzellen die vom Sensor Ausgelesen werden können.
    pub messzellen: MesszelleList,
}

impl TestSensor {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for TestSensor {
    fn default() -> Self {
        TestSensor {
            id: 0,
            sensor_type: SensorType::TestSensor,
            messzellen: vec![],
        }
    }
}

impl fmt::Display for TestSensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Test Sensor")
    }
}

/// Implementation des Sensor Traits
///
impl Sensor for TestSensor {
    fn update(&self) {
        debug!("Update Sensor: '{}'", &self);
        // let messzellen = &self.messzellen.clone();
        // for messzelle in messzellen {
        //     if let Ok(mut messzelle) = messzelle.lock() {
        //         messzelle.update()
        //     }
        // }
        ::std::thread::sleep(::std::time::Duration::from_secs(1));
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_sensor_type(&self) -> SensorType {
        self.sensor_type.clone()
    }

    fn get_messzellen(&self) -> Vec<Arc<Mutex<BoxedMesszelle>>> {
        self.messzellen.clone()
    }

    fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>> {
        let messzelle = self.messzellen.get(num);
        messzelle
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let sensor = TestSensor::new();
        assert_eq!(sensor.messzellen.len(), 0);
    }

    #[test]
    #[ignore]
    fn update() {
        assert!(false)
    }

    #[test]
    #[ignore]
    fn get_messzelle() {
        assert!(false)
    }
}
