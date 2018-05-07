use ::sensor::{
    BoxedSensor,
    MetzConnectCI4,
    RaGasCONO2Mod,
    SensorType,
    TestSensor,
};
use config::Config;
use error::ServerError;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;


/// Server Representation zum Speichern/ Wiederherstellen einer Konfigurationsdatei
///
#[derive(Debug, Deserialize)]
pub struct Server {
    pub service_interval: u32,
    pub sensors: Vec<::configuration::sensor::Sensor>,
    #[serde(skip)]
    pub configuration_path: Option<PathBuf>,
    #[serde(skip)]
    pub runtime_info_path: Option<PathBuf>,
}

impl Server {
    /// Bildet eine Server Instanz aus der Konfigurationsdatei
    ///
    /// Die Funktion liefert ein `Result` mit einer `ServerBuilder` Instanz, oder liefert ein
    /// `ServerError`.
    pub fn from_config_file(cfg: &Config) -> Result<Server, ServerError> {
        let mut file = File::open(&cfg.configuration_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;

        match toml::from_str::<Server>(&s) {
            Ok(mut builder) => {
                // Die Pfade zur Konfigurationsdatei und Laufzeitinformation stammen aus dem `Config` Modul.
                // Sie stehen nicht in der Konfigurationsdatei selbst.
                builder.configuration_path = Some(cfg.configuration_path.clone());
                builder.runtime_info_path = Some(cfg.runtime_info_path.clone());
                Ok(builder)
            }
            Err(err) => Err(ServerError::CouldNotBuildFromConfig(err)),
        }
    }
}

/// Konvertierung des `configuration::Server` nach `server::Server`
///
/// Stellt den `server::Server` aus den Daten der Konfigurationsdatei wieder her.
///
impl From<Server> for ::server::Server {
    fn from(server: Server) -> Self {
        // Restauriere Sensoren
        let mut sensors: Vec<BoxedSensor> = vec![];
        for s in server.sensors {
            match s.sensor_type {
                SensorType::RaGasCONO2Mod => {
                    let sensor: RaGasCONO2Mod = s.into();
                    sensors.push(Box::new(sensor));
                },
                SensorType::MetzConnectCI4 => {
                    let sensor: MetzConnectCI4 = s.into();
                    sensors.push(Box::new(sensor));
                },
                SensorType::TestSensor => {
                    let sensor: TestSensor = s.into();
                    sensors.push(Box::new(sensor));
                },
            }
        }
        ::server::Server {
            service_interval: server.service_interval,
            sensors:  sensors,
            // zones: vec![],
            configuration_path: server.configuration_path,
            runtime_info_path: server.runtime_info_path,
        }
    }
}
