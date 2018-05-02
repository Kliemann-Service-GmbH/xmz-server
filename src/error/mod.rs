//! Mögliche Fehler die im Serverbetrieb auftreten können
use bincode::Error as BincodeError;
use configure::DeserializeError as ConfigureError;
use output::OutputError;
use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use toml::de::Error as TomlError;

/// Mögliche Server Fehler
///
#[derive(Debug)]
pub enum ServerError {
    Bincode(BincodeError),
    Configure(ConfigureError),
    CouldNotBuildFromConfig(TomlError),
    CouldNotBuildFromRuntime(BincodeError),
    IO(IOError),
    Output(OutputError),
    RuntimePathNotSet,
    ServerBuilder,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Bincode(ref err) => write!(f, "Fehler in der Bincode Serialisation: {}", err),
            ServerError::Configure(ref err) => {
                write!(f, "Konnte Konfiguration nicht Deserialisieren: {}", err)
            }
            ServerError::CouldNotBuildFromConfig(ref err) => {
                write!(f, "Konnte keine Server Instanz aus der Konfigurationsdatei erzeugen: {}", err)
            }
            ServerError::CouldNotBuildFromRuntime(ref err) => {
                write!(f, "Konnte keine Server Instanz aus der Laufzeitinformationen erzeugen: {}", err)
            }
            ServerError::IO(ref err) => write!(f, "IO Fehler: {}", err),
            ServerError::Output(ref err) => write!(f, "Output Fehler: {}", err),
            ServerError::RuntimePathNotSet => write!(f, "Pfad der Laufzeitinformation nicht gesetzt"),
            ServerError::ServerBuilder => write!(f, "Fehler im Serverbuilder"),
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Bincode(ref err) => err.description(),
            ServerError::Configure(ref err) => err.description(),
            ServerError::CouldNotBuildFromConfig(ref _err) => "Die Konfigurationsdatei konnte nicht gelesen werden. Evtl. stimmen die Dateiberechtigungen nicht, oder die Datei ist defekt.",
            ServerError::CouldNotBuildFromRuntime(ref _err) => "Die Laufzeitinformationen konnten nicht gelesen werden. Evtl. stimmen die Dateiberechtigungen nicht, oder die Datei ist defekt.",
            ServerError::IO(ref err) => err.description(),
            ServerError::Output(ref err) => err.description(),
            ServerError::RuntimePathNotSet => "Pfad der Laufzeitinformation nicht gesetzt",
            ServerError::ServerBuilder => "Server konnte nicht erstellt weden",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServerError::Bincode(ref err) => Some(err),
            ServerError::Configure(ref err) => Some(err),
            ServerError::CouldNotBuildFromConfig(ref err) => Some(err),
            ServerError::CouldNotBuildFromRuntime(ref err) => Some(err),
            ServerError::IO(ref err) => Some(err),
            ServerError::Output(ref err) => Some(err),
            ServerError::RuntimePathNotSet => None,
            ServerError::ServerBuilder => None,
        }
    }
}

impl From<BincodeError> for ServerError {
    fn from(error: BincodeError) -> Self {
        ServerError::Bincode(error)
    }
}

impl From<ConfigureError> for ServerError {
    fn from(error: ConfigureError) -> Self {
        ServerError::Configure(error)
    }
}

impl From<IOError> for ServerError {
    fn from(error: IOError) -> Self {
        ServerError::IO(error)
    }
}
