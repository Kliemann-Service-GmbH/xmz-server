//! Kernkomponente dieser Anwendung
//!
use api;
use bincode::serialize;
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    /// Wartungsintervall in Tagen
    pub service_interval: u32,
    /// Liste der Sensoren die dieser Server verwaltet
    #[serde(skip)]
    pub sensors: SensorsList,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            service_interval: 365,
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
        Server {
            service_interval: 0,
            sensors: Vec::new(),
        }
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
    /// let server = Server::default();
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

    /// Serialize in das Bincode format
    pub fn serialize_to_bincode(&self) -> Result<Vec<u8>, ServerError> {
        match serialize(&self) {
            Ok(data) => Ok(data),
            Err(err) => Err(ServerError::Bincode(err)),
        }
    }

    fn store_runtime_information(&self) -> Result<(), ServerError> {
        println!("Bincode: {:?}", &self.serialize_to_bincode());
        Ok(())
    }

    pub fn start(&self) -> Result<(), ServerError> {
        // Laufzeit Informationen speichern
        self.store_runtime_information()?;

        // Sensor Update Thread starten
        let server_update_guard = self.update_sensors();

        // JSON Api (rocket) starten
        self.launch_api();

        // Der Sensor Update Thread wird gejoint, somit läuft der Server solange dieser Thread
        // läuft.
        server_update_guard.join().expect("Fehler im Sensor Update Guard");

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let server = Server::new();
        assert_eq!(server.service_interval, 0);
        assert_eq!(server.sensors.len(), 0);
    }

    #[test]
    fn default() {
        let server = Server::default();
        assert_eq!(server.service_interval, 365);
        assert_eq!(server.sensors.len(), 2);
    }
}
