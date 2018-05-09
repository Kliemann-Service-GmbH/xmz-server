//! Kernkomponente dieser Anwendung
//!
use api;
use bincode;
use prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;


/// Struktur der Server Komponente
#[derive(Debug)]
pub struct Server {
    /// Wartungsintervall in Tagen
    pub service_interval: u32,
    /// Liste der Sensoren die dieser Server verwaltet
    pub sensors: SensorList,
    pub configuration_path: Option<PathBuf>,
    pub runtime_info_path: Option<PathBuf>,
}

impl Default for Server {
    /// Default Konfiguration des Servers
    ///
    /// Die `default()` Konfiguration des Servers mit den sinnvollsten Werten.
    ///
    fn default() -> Self {
        let sensors: SensorList = vec![
            Arc::new(Mutex::new(Box::new(RaGasCONO2Mod::new()))),
            Arc::new(Mutex::new(Box::new(MetzConnectCI4::new()))),
            Arc::new(Mutex::new(Box::new(TestSensor::new()))),
        ];
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
    ///
    /// Die `new()` Funktion erstellt eine "leere" neue Server Instanz. Das heist alle Member sind
    /// Null oder leer, entsprechenden ihres Datentypes.
    /// Die `default()` Implementation hingegen liefert einen "kompletten" Server. Das bedeutet
    /// alle Member des Servers sind mit sinnvollen default Werten gefüllt. So sind zum
    /// Beispiel alle unterstützten Sensoren, Messzellen jeweils einmal verfügbar.
    ///
    /// # Example
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let server = Server::default();
    /// assert_eq!(server.get_sensors().len(), 3);
    /// ```
    ///
    pub fn new() -> Self {
        Server {
            service_interval: 0,
            sensors: vec![],
            ..Default::default()
        }
    }

    /// Startet die Api (Json, Web)
    ///
    pub fn launch_api(&self) {
        api::launch(self);
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
    pub fn get_sensors(&self) -> Vec<Arc<Mutex<BoxedSensor>>> {
        self.sensors.clone()
    }

    ///
    /// # Example
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let server = Server::default();
    /// assert_eq!(server.get_sensors().len(), 3);
    /// ```
    pub fn get_sensor(&self, num: usize) -> Option<&Arc<Mutex<BoxedSensor>>> {
        let sensor = self.sensors.get(num);
        sensor
    }

    ///
    /// # Example
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let server = Server::default();
    /// assert_eq!(server.get_sensors().len(), 3);
    /// ```
    pub fn add_sensor(&mut self, sensor: BoxedSensor) {
        self.sensors.push(Arc::new(Mutex::new(sensor)));
    }

    /// Serialize Server Instanz in das Bincode format
    ///
    /// # Example
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let server = Server::default();
    /// assert_eq!(server.get_sensors().len(), 3);
    /// ```
    ///
    pub fn serialize_to_bincode(&self) -> Result<Vec<u8>, ServerError> {
        let server: runtime_info::Server = self.clone().into();
        debug!("{:?}", &server);

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

    /// Aktualisiert der Reihe nach jeden Sensor
    ///
    /// # Example
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let server = Server::default();
    /// assert_eq!(server.get_sensors().len(), 3);
    /// ```
    ///
    pub fn update_sensors(&self) -> thread::JoinHandle<bool> {
        let sensors = self.sensors.clone();
        thread::spawn(move || loop {
            for sensor in sensors.clone() {
                if let Ok(sensor) = sensor.lock() {
                    sensor.update();
                }
            }
            thread::sleep(Duration::from_millis(100));
        })
    }

    /// Started alle Komponenten des Servers
    ///
    /// Viele Teile des Servers werden in eigenen Threads gestarted.
    pub fn start(&self) -> Result<(), ServerError> {
        // Laufzeit Informationen speichern
        self.store_runtime_information()?;

        // Sensor Update Thread starten
        self.update_sensors();

        // JSON Api (rocket) starten
        self.launch_api();

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
