use bincode;
use config::Config;
use error::ServerError;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "server")]
pub struct Server {
    pub service_interval: u32,
    pub sensors: Vec<::runtime_info::sensor::Sensor>,
    #[serde(skip)]
    pub configuration_path: Option<PathBuf>,
    #[serde(skip)]
    pub runtime_info_path: Option<PathBuf>,
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
            Ok(server) => Ok(server),
            Err(err) => Err(ServerError::CouldNotBuildFromRuntime(err)),
        }
    }
}


/// Konvertierung des `runtime_info::Server` nach `server::Server`
///
/// Stellt den `server::Server` aus den Daten der Laufzeitinformationen wieder her.
impl From<Server> for ::server::Server {
    fn from(server: Server) -> Self {
        let mut sensors: Vec<Arc<Mutex<Box<::sensor::Sensor + Send + 'static>>>> = vec![];
        for s in server.sensors {
            match s.sensor_type {
                ::sensor::SensorType::RaGasCONO2Mod => {
                    let sensor: ::sensor::RaGasCONO2Mod = s.clone().into();
                    sensors.push(Arc::new(Mutex::new(Box::new(sensor))));
                },
                ::sensor::SensorType::MetzConnectCI4 => {
                    let sensor: ::sensor::MetzConnectCI4 = s.into();
                    sensors.push(Arc::new(Mutex::new(Box::new(sensor))));
                },
                ::sensor::SensorType::TestSensor => {
                    let sensor: ::sensor::TestSensor = s.into();
                    sensors.push(Arc::new(Mutex::new(Box::new(sensor))));
                },
            }
        }

        ::server::Server {
            service_interval: server.service_interval,
            sensors: sensors,
            // zones: vec![],
            configuration_path: server.configuration_path,
            runtime_info_path: server.runtime_info_path,
        }
    }
}
