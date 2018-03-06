use std::thread;
use sensor::{BoxedSensor, Sensor};
use settings::Settings;
use error::ServerError;
use std::sync::{Arc, Mutex};
use server::SensorsList;

/// Struktur der Server Komponente
pub struct Server {
    /// Liste der Sensoren die dieser Server verwaltet
    pub sensors: SensorsList,
}

impl Server {
    /// Erstellt eine neue Server Instanz
    pub fn new(settings: &Settings) -> Self {
        Server {
            sensors: vec![],
            // zones: vec![],
        }
    }

    /// Aktualisiert der Reihe nach jeden Sensor
    ///
    pub fn update_sensors(&self) {
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


    pub fn get_sensor(&self, num: usize) -> Option<&Arc<Mutex<BoxedSensor>>> {
        self.sensors.get(num)
    }

    pub fn add_sensor(&mut self, sensor: Arc<Mutex<BoxedSensor>>) {
        self.sensors.push(sensor);
    }

    pub fn start(&self) -> Result<(), ServerError> {
        self.update_sensors();

        Ok(())
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let settings = Settings::new();
        let server = Server::new(&settings.unwrap());
        assert_eq!(server.sensors.len(), 0);
    }
}
