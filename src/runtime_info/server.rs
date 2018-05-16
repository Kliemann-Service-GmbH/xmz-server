use bincode;
use prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;


#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    service_interval: u32,
    sensors: Vec<::runtime_info::sensor::Sensor>,
    outputs: Vec<::runtime_info::output::Output>,
    configuration_path: String,
    runtime_info_path: String,
}

impl Server {
    /// Stellt die Server Instanz aus den Laufzeitinformationen wieder her
    ///
    /// Die Funktion liefert ein `Result` mit einer `ServerBuilder` Instanz, oder liefert ein
    /// entsprechenden `ServerError` zurück.
    pub fn from_runtime_info(cfg: &Config) -> Result<Server, ServerError> {
        let mut file = File::open(&cfg.runtime_info_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;

        match bincode::deserialize(&s.as_bytes()) {
            Ok(server) => { Ok(server) },
            Err(err) => Err(ServerError::CouldNotBuildFromRuntime(err)),
        }
    }
}

// Diese Funktion wird von der eigentlichen Server binary `src/bin/main.rs` verwendet
/// Konvertierung des `runtime_info::Server` nach `server::Server`
///
/// Stellt den `server::Server` aus den Daten der Laufzeitinformationen wieder her.
///
impl From<Server> for ::server::Server {
    fn from(server: Server) -> Self {
        let mut sensors: SensorList = vec![];
        for s in server.sensors {
            match s.sensor_type {
                SensorType::RaGasCONO2Mod => {
                    let sensor: RaGasCONO2Mod = s.into();
                    sensors.push(Arc::new(RwLock::new(Box::new(sensor))));
                },
                SensorType::MetzConnectCI4 => {
                    let sensor: MetzConnectCI4 = s.into();
                    sensors.push(Arc::new(RwLock::new(Box::new(sensor))));
                },
                SensorType::TestSensor => {
                    let sensor: TestSensor = s.into();
                    sensors.push(Arc::new(RwLock::new(Box::new(sensor))));
                },
            }
        }
        let mut outputs: OutputList = vec![];
        ::server::Server {
            service_interval: server.service_interval,
            configuration_path: Some(PathBuf::from(server.configuration_path)),
            runtime_info_path: Some(PathBuf::from(server.runtime_info_path)),
            sensors: sensors,
            outputs: outputs,
            // zones: vec![],
        }
    }
}

// wird in `::server::Server::serialize_to_bincode()` verwendet
/// Konvertierung eine `&server::Server` Referenz nach `runtime_info::Server`
///
/// Konvertiert den `server::Server` in ein Format das in der Laufzeitinformation
/// gespeichert werden kann.
///
/// Diese Funktion wird in `serialize_to_bincode()` verwendet
///
/// Diese Funktion ist analog zu der Konvertierung des `server::Server` nach [`configuration::Server`](../configuration/struct.Server.html)
///
impl<'r> From<&'r ::server::Server> for Server {
    fn from(server: &'r ::server::Server) -> Self {
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
        let mut outputs: Vec<::runtime_info::output::Output> = vec![];
        Server {
            service_interval: server.service_interval,
            configuration_path: configuration_path,
            runtime_info_path: runtime_info_path,
            sensors: sensors,
            outputs: outputs,
            // zones: vec![],
        }
    }
}
