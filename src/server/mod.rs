//! Kernkomponente dieser Anwendung
//!
use api;
use bincode;
use prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;


/// Struktur der Server Komponente
#[derive(Clone, Debug)]
pub struct Server {
    // GOODTOKNOW: Die Member des Servers sind alle `pub` so das der Server in den configruation
    // und runtime_info Teilen so konstruiert werden kann `let server = Server { member: x, member: y, ...}`
    /// Wartungsintervall in Tagen
    pub service_interval: u32,
    /// Liste der Sensoren die dieser Server verwaltet
    /// `SensorList = Vec<Arc<RwLock<Box<Sensor + Send + Sync>>>>`
    pub sensors: SensorList,
    /// Liste der Ausgänge die vom Server geschalten werden können
    /// `OutputList = Vec<Arc<Box<Output + Send + Sync>>>`
    pub outputs: OutputList,
    // FIXME: eine Server Instanz kann nicht ohne diese Pfade erstellt werden, `Option` ist nicht nötig
    pub configuration_path: Option<PathBuf>,
    // FIXME: eine Server Instanz kann nicht ohne diese Pfade erstellt werden, `Option` ist nicht nötig
    pub runtime_info_path: Option<PathBuf>,
}

impl Default for Server {
    /// Default Konfiguration des Servers
    ///
    /// Die `default()` Konfiguration des Servers mit den sinnvollsten Werten.
    ///
    fn default() -> Self {
        let sensors: SensorList = vec![
            Arc::new(RwLock::new(Box::new(RaGasCONO2Mod::new()))),
            Arc::new(RwLock::new(Box::new(MetzConnectCI4::new()))),
            Arc::new(RwLock::new(Box::new(TestSensor::new()))),
        ];
        let outputs: OutputList = vec![
            Arc::new(RwLock::new(Box::new(MetzConnectMRDO4::new()))),
            Arc::new(RwLock::new(Box::new(XMZBoden100::new()))),
            Arc::new(RwLock::new(Box::new(XMZDeckel100::new()))),
        ];
        Server {
            service_interval: 365,
            sensors: sensors,
            outputs: outputs,
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
    /// let server = Server::new();
    /// assert_eq!(server.get_sensors().len(), 0);
    /// ```
    ///
    /// Alternativ kann die `default()` Funktion des Servers verwendet werden, hier werden alle
    /// Komponenten gefüllt.
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
            outputs: vec![],
            ..Default::default()
        }
    }

    /// Liefert eine Liste der Sensoren
    ///
    /// Diese funktion wird unter Anderen in der Konvertierung in das Bincode Format verwendet.
    ///
    /// # Example
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let server = Server::default();
    /// assert_eq!(server.get_sensors().len(), 3);
    /// ```
    pub fn get_sensors(&self) -> Vec<Arc<RwLock<BoxedSensor>>> {
        self.sensors.clone()
    }

    /// Liefert eine Liste der Sensoren
    ///
    /// Diese funktion wird unter Anderen in der Konvertierung in das Bincode Format verwendet.
    ///
    /// # Example
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let server = Server::default();
    /// assert_eq!(server.get_outputs().len(), 3);
    /// ```
    pub fn get_outputs(&self) -> OutputList {
        self.outputs.clone()
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
    pub fn get_sensor(&self, num: usize) -> Option<&Arc<RwLock<BoxedSensor>>> {
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
        self.sensors.push(Arc::new(RwLock::new(sensor)));
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
        let server: runtime_info::Server = self.into();
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
    pub fn update_sensors(&self) -> thread::JoinHandle<()> {
        let sensors = self.sensors.clone();
        thread::spawn(move || loop {
            for sensor in sensors.clone() {
                if let Ok(sensor) = sensor.read() {
                    sensor.update();
                }
            }
            thread::sleep(Duration::from_millis(100));
        })
    }

    /// Startet die Api (Json, Web)
    ///
    pub fn launch_api(&self) {
        api::launch(self.clone());
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


/// Konvertierung eine `&server::Server` Referenz nach `runtime_info::Server`
///
/// Konvertiert den `server::Server` in ein Format das in der Laufzeitinformation
/// gespeichert werden kann.
///
/// Diese Funktion wird in `serialize_to_bincode()` verwendet
///
/// Diese Funktion ist analog zu der Konvertierung des `server::Server`
///  nach [`configuration::Server`](../configuration/struct.Server.html)
///
impl<'r> From<&'r Server> for ::runtime_info::Server {
    fn from(server: &'r Server) -> Self {
        // Restauriere Sensoren
        let mut sensors: Vec<::runtime_info::Sensor> = vec![];
        for sensor in server.get_sensors() {
            sensors.push(sensor.into());
        }

        // Restauriere Pfade
        let configuration_path = match &server.configuration_path {
            Some(path) => path.to_string_lossy().to_string(),
            None => "not set".to_string(),
        };
        let runtime_info_path = match &server.runtime_info_path {
            Some(path) => path.to_string_lossy().to_string(),
            None => "not set".to_string(),
        };

        // Restauriere Outputs
        let mut outputs: Vec<::runtime_info::Output> = vec![];
        for output in server.get_outputs() {
            outputs.push(output.into());
        }
        ::runtime_info::Server {
            service_interval: server.service_interval,
            configuration_path: configuration_path,
            runtime_info_path: runtime_info_path,
            sensors: sensors,
            outputs: outputs,
            // zones: vec![],
        }
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
