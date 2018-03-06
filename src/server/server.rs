use std::thread;
use sensor::Sensor;
use settings::Settings;
use error::ServerError;
use std::sync::{Arc, Mutex};


type SensorsList = Vec<Arc<Mutex<Box<Sensor + Send + 'static>>>>;

/// Struktur der Server Komponente
pub struct Server {
    pub sensors: SensorsList,
}

impl Server {
    pub fn new(_config: &Settings) -> Self {
        Server {
            sensors: vec![],
            // zones: vec![],
        }
    }

    fn update_sensors(&self) {
        let sensors = self.sensors.clone();
        thread::spawn(move || {
            loop {
                for sensor in sensors.clone() {
                    if let Ok(mut sensor) = sensor.lock() {
                        sensor.update();
                    }
                }
            }
        });
    }

    pub fn start(&self) -> Result<(), ServerError> {
        self.update_sensors();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
