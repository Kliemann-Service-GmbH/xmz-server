//! Kernkomponente dieser Anwendung
//!
use api;
use bincode;
use error::ServerError;
use prelude::*;
use sensor::{BoxedSensor, SensorsList};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;


/// Struktur der Server Komponente
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Server {
    /// Wartungsintervall in Tagen
    pub service_interval: u32,
    /// Liste der Sensoren die dieser Server verwaltet
    #[serde(skip)]
    pub sensors: SensorsList,
    pub configuration_path: Option<PathBuf>,
    pub runtime_info_path: Option<PathBuf>,
}

impl Default for Server {
    fn default() -> Self {
        let sensors: SensorsList = Arc::new(Mutex::new(vec![
            Box::new(RaGasCONO2Mod::new()),
            Box::new(MetzConnectCI4::new()),
            Box::new(TestSensor::new()),
        ]));
        Server {
            service_interval: 365,
            sensors: sensors,
            // zones: vec![],
            configuration_path: None,
            runtime_info_path: None,
        }
    }
}

impl Server {
    /// Erstellt eine neue Server Instanz
    pub fn new() -> Self {
        Server {
            service_interval: 0,
            ..Default::default()
        }
    }

    /// Aktualisiert der Reihe nach jeden Sensor
    ///
    pub fn update_sensors(&self) -> thread::JoinHandle<bool> {
        // let sensors = self.sensors.clone();
        // let guard = thread::spawn(move || loop {
        //     for sensor in sensors.clone() {
        //         if let Ok(mut sensor) = sensor.lock() {
        //             sensor.update();
        //         }
        //     }
        // });
        // guard
        thread::spawn(|| loop {})
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
    /// assert_eq!(server.get_sensors().len(), 3);
    /// ```
    pub fn get_sensors(&self) -> &SensorsList {
        &self.sensors
    }

    pub fn get_sensor(&self, num: usize) -> Option<&BoxedSensor> {
        // self.sensors.into_inner().unwrap().get(num)
        unimplemented!()
    }

    pub fn add_sensor(&mut self, sensor: &BoxedSensor) {
        // let mut sensors = self.sensors.lock().unwrap();
        // sensors.push(*sensor);
        unimplemented!()
    }

    /// Serialize Server Instanz in das Bincode format
    ///
    pub fn serialize_to_bincode(&self) -> Result<Vec<u8>, ServerError> {
        let server: runtime_info::Server = self.clone().into();

        match bincode::serialize(&server) {
            Ok(data) => {
                debug!("{:?}", &data);
                Ok(data)
            },
            Err(err) => Err(ServerError::Bincode(err)),
        }
    }

    fn store_runtime_information(&self) -> Result<(), ServerError> {
        match &self.runtime_info_path {
            Some(path) => {
                info!("Create runtime info at: {:?}", path);
                let mut buffer = File::create(path)?;

                info!("Speichere Server Instanz im bincode Format");
                let data = &self.serialize_to_bincode()?;
                buffer.write(data)?;

                Ok(())
            }
            None => Err(ServerError::RuntimePathNotSet),
        }
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
        server_update_guard
            .join()
            .expect("Fehler im Sensor Update Guard");

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
        assert_eq!(server.sensors.len(), 3);
    }
}
