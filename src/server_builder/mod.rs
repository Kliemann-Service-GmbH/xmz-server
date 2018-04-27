/// Erzeugt eine neue Server Instanz aus Laufzeitinformation oder Konfigurationsdatein
///
/// Beim allerersten Start des Servers wird die Server Instanz aus einer initialen
/// Konfigurationsdatein erstellt.
/// Konnte aus dieser ersten Konfiguration eine funktionale Server Intanz gestartet werden,
/// wird eine Datei mit den Laufzeitinformationen (der aktuelle Zustand des Servers) erzeugt.
/// Alle weiteren Starts des Servers verwenden diese Laufzeitinformationen.
/// Dies gewährleistet das Daten wie die Laufzeit "persistent" gespeichert werden können
/// (d.h. das diese nicht nach einem Neustart verloren gehen).
///
/// Siehe: [Dokumentation der 'xMZ-Plattform'](https://kliemann-service-gmbh.github.io/xmz-doc/
use bincode;
use error::ServerError;
use prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;

#[derive(Debug, Deserialize)]
#[serde(rename = "server")]
struct ServerFromConf {
    service_interval: u32,
}

#[derive(Debug, Deserialize)]
pub struct ServerBuilder {
    configuration_path: Option<PathBuf>,
    runtime_info_path: Option<PathBuf>,
    server: ServerFromConf,
}

impl ServerBuilder {
    pub fn generate(self) -> Server {
        self.into()
    }
}

impl From<ServerBuilder> for Server {
    fn from(builder: ServerBuilder) -> Server {
        Server {
            service_interval: builder.server.service_interval,
            sensors: Vec::new(),
            configuration_path: builder.configuration_path,
            runtime_info_path: builder.runtime_info_path,
        }
    }
}

impl ServerBuilder {
    /// Testet ob die Datei mit den Laufzeitinformationen existiert
    ///
    ///  Diese Funktion liefert auch `false` wenn auf die Datei nicht zugegriffen werden kann,
    ///  z.B. durch fehlende Berechtigungen.
    pub fn runtime_info_available(cfg: &Config) -> bool {
        cfg.runtime_info_path.exists()
    }

    /// Testet ob die Konfigurationsdatei existiert
    ///
    ///  Diese Funktion liefert auch `false` wenn auf die Datei nicht zugegriffen werden kann,
    ///  z.B. durch fehlende Berechtigungen.
    pub fn config_file_available(cfg: &Config) -> bool {
        cfg.configuration_path.exists()
    }

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
            Err(err) => Err(ServerError::CouldNotBuildFromRuntime),
        }
    }

    /// Bildet eine Server Instanz aus der Konfigurationsdatei
    ///
    /// Die Funktion liefert ein `Result` mit einer `ServerBuilder` Instanz, oder liefert ein
    /// `ServerError`.
    pub fn from_config_file(cfg: &Config) -> Result<ServerBuilder, ServerError> {
        let mut file = File::open(&cfg.configuration_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;

        match toml::from_str::<ServerBuilder>(&s) {
            Ok(mut builder) => {
                builder.configuration_path = Some(cfg.configuration_path.clone());
                builder.runtime_info_path = Some(cfg.runtime_info_path.clone());
                Ok(builder)
            }
            Err(err) => Err(ServerError::CouldNotBuildFromConfig(err)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn runtime_info_available() {
        let cfg = Config::default();
        assert_eq!(ServerBuilder::runtime_info_available(&cfg), false);
    }

    #[test]
    fn config_file_available() {
        let cfg = Config::default();
        assert_eq!(ServerBuilder::config_file_available(&cfg), false);
    }

    #[test]
    fn no_config_no_runtime_is_error() {
        use std::path::PathBuf;

        let mut cfg = Config::default();
        cfg.configuration_path = PathBuf::from("/path/to/not/existend/configuration");
        cfg.runtime_info_path = PathBuf::from("/path/to/not/existend/runtime_info");
        assert!(ServerBuilder::from_config_file(&cfg).is_err());
    }
}
