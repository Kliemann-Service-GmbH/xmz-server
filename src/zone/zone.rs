use std::sync::{Arc, Mutex};
use sensor::Sensor;


#[derive(Debug)]
#[derive(Clone)]
pub struct Zone {
    pub sensors: Vec<Arc<Mutex<Sensor>>>,
}

impl Zone {
    pub fn new() -> Self {
        Zone {
            sensors: vec![],
        }
    }

    pub fn add_sensor(&mut self, sensor: Arc<Mutex<Sensor>>) {
        self.sensors.push(sensor);
    }

    pub fn check(&self) {
        // for sensor in &self.sensors {
        //     if sensor.value >= 100_000 as f64 {
        //         println!("Alarm");
        //     }
        // }
    }
}
