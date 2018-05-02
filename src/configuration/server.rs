use configuration;
use config::Config;
use error::ServerError;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;
use std::sync::{Arc, Mutex};


#[derive(Debug, Deserialize)]
pub struct Server {
    pub service_interval: u32,
    pub sensors: Vec<configuration::sensor::Sensor>,
    #[serde(skip)]
    configuration_path: Option<PathBuf>,
    #[serde(skip)]
    runtime_info_path: Option<PathBuf>,

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
                builder.configuration_path = Some(cfg.configuration_path.clone());
                builder.runtime_info_path = Some(cfg.runtime_info_path.clone());
                Ok(builder)
            }
            Err(err) => Err(ServerError::CouldNotBuildFromConfig(err)),
        }
    }
}


impl From<Server> for ::server::Server {
    fn from(server: Server) -> Self {
        let sensor: ::sensor::RaGasCONO2Mod = server.sensors[0].clone().into();

        ::server::Server {
            service_interval: server.service_interval,
            sensors: vec![
                Arc::new(Mutex::new(Box::new(sensor))),
            ],
            // zones: vec![],
            configuration_path: server.configuration_path,
            runtime_info_path: server.runtime_info_path,
        }
    }
}
