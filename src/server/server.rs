use std::thread;
use sensor::BoxedSensor;
use error::ServerError;
use server::SensorsList;
use prelude::*;


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
        let _guard = thread::spawn(move || {
            loop {
                for sensor in sensors.clone() {
                    if let Ok(mut sensor) = sensor.lock() {
                        sensor.update();
                    }
                }
            }
        }).join();
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
