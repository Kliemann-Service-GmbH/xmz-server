use error::ServerError;
use prelude::*;
use sensor::BoxedSensor;
use std::sync::{Arc, Mutex};
use std::thread;


/// Liste der Sensoren
///
/// Diese Liste ist ein `Vector` von shared (`Arc`), mutablen (`Mutex`)
/// Sensor Trait Objekten (`BoxedSensor`).
pub type SensorsList = Vec<Arc<Mutex<BoxedSensor>>>;



/// Struktur der Server Komponente
#[derive(Clone)]
pub struct Server {
    /// Wartungsintervall
    pub service_interval: u32,
    /// Liste der Sensoren die dieser Server verwaltet
    pub sensors: SensorsList,
}

impl Server {
    /// Erstellt eine neue Server Instanz
    pub fn new(settings: &Settings) -> Self {
        Server {
            service_interval: settings.server.service_interval,
            sensors: vec![],
            // zones: vec![],
        }
    }

    /// Aktualisiert der Reihe nach jeden Sensor
    ///
    pub fn update_sensors(&self) {
        let sensors = self.sensors.clone();
        let _guard = thread::spawn(move || loop {
            for sensor in sensors.clone() {
                if let Ok(mut sensor) = sensor.lock() {
                    sensor.update();
                }
            }
        });
    }

    /// Liefert eine Referenz auf die Liste der Sensoren
    ///
    /// # Example
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let settings = Settings::new().unwrap();
    /// let server = Server::new(&settings);
    /// assert_eq!(server.get_sensors().len(), 0);
    /// ```
    pub fn get_sensors(&self) -> &SensorsList {
        &self.sensors
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
