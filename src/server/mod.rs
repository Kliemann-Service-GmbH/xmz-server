//! Kernkomponente dieser Anwendung
//!
use api;
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

impl Default for Server {
    fn default() -> Self {
        Server {
            service_interval: 1,
            sensors: vec![
                Arc::new(Mutex::new(Box::new(RaGasCONO2Mod::new()))),
                Arc::new(Mutex::new(Box::new(MetzConnectCI4::new()))),
            ],
            // zones: vec![],
        }
    }
}

impl Server {
    /// Erstellt eine neue Server Instanz
    pub fn new() -> Self {
        Default::default()
    }

    /// Aktualisiert der Reihe nach jeden Sensor
    ///
    pub fn update_sensors(&self) -> thread::JoinHandle<bool> {
        let sensors = self.sensors.clone();
        let guard = thread::spawn(move || loop {
            for sensor in sensors.clone() {
                if let Ok(mut sensor) = sensor.lock() {
                    sensor.update();
                }
            }
        });

        guard
    }

    /// Startet die Api (Json, Web)
    ///
    pub fn launch_api(&self) {
        api::launch(self.clone());
    }

    /// Liefert eine Referenz auf die Liste der Sensoren
    ///
    /// # Example
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let server = Server::new();
    /// assert_eq!(server.service_interval, 1);
    /// assert_eq!(server.get_sensors().len(), 2);
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
        let server_update_guard = self.update_sensors();

        self.launch_api();

        server_update_guard.join().expect("Fehler im Sensor Update Guard");

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let server = Server::new();
        assert_eq!(server.service_interval, 1);
        assert_eq!(server.sensors.len(), 2);
    }
}
