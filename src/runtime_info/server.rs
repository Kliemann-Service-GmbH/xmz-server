use bincode;
use prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;


// Alle Member sind public, so das der Server aus `::server::Server` gebildet werden kann
#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub service_interval: u32,
    pub sensors: Vec<::runtime_info::sensor::Sensor>,
    pub outputs: Vec<::runtime_info::output::Output>,
    pub configuration_path: String,
    pub runtime_info_path: String,
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
        for o in server.outputs {
            match o.output_type {
                OutputType::MetzConnectMRDO4 => {
                    let output: MetzConnectMRDO4 = o.into();
                    outputs.push(Arc::new(RwLock::new(Box::new(output))));
                },
                OutputType::XMZBoden100 => {
                    let output: XMZBoden100 = o.into();
                    outputs.push(Arc::new(RwLock::new(Box::new(output))));
                },
                OutputType::XMZDeckel100 => {
                    let output: XMZDeckel100 = o.into();
                    outputs.push(Arc::new(RwLock::new(Box::new(output))));
                },
            }
        }
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
