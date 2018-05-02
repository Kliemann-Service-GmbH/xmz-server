use bincode;
use config::Config;
use configuration;
use error::ServerError;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "server")]
pub struct Server {
    pub service_interval: u32,
    pub sensors: Vec<configuration::Sensor>,
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


impl From<Server> for ::server::Server {
    fn from(server: Server) -> Self {
        ::server::Server::new()
    }
}
