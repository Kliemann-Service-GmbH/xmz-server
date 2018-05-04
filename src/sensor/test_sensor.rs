use messzelle::{BoxedMesszelle, MesszellenList};
use sensor::{Sensor, SensorType};
use std::fmt;
use std::sync::{Arc, Mutex};


/// Test Sensor
///
#[derive(Debug)]
pub struct TestSensor {
    /// Sensor Type
    pub sensor_type: SensorType,
    /// Liste der Messzellen die vom Sensor Ausgelesen werden können.
    pub messzellen: MesszellenList,
}

impl TestSensor {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for TestSensor {
    fn default() -> Self {

        TestSensor {
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

impl Sensor for TestSensor {
    fn update(&self) {
        debug!("Update Sensor: '{}'", &self);
        let messzellen = &self.messzellen.clone();
        for messzelle in messzellen {
            if let Ok(mut messzelle) = messzelle.lock() {
                messzelle.update()
            }
        }
        ::std::thread::sleep(::std::time::Duration::from_secs(1));
    }

    fn get_sensor_type(&self) -> SensorType {
        self.sensor_type.clone()
    }

    fn get_messzellen(&self) -> &Vec<Arc<Mutex<BoxedMesszelle>>> {
        &self.messzellen
    }

    fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>> {
        self.messzellen.get(num)
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
