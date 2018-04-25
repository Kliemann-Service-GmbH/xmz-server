//! Mögliche Fehler die im Serverbetrieb auftreten können
use bincode::Error as BincodeError;
use configure::DeserializeError as ConfigureError;
use output::OutputError;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;


/// Mögliche Server Fehler
///
#[derive(Debug)]
pub enum ServerError {
    /// Fehler beim Schalten eines Ausgangs
    Output(OutputError),
    Bincode(BincodeError),
    Configure(ConfigureError),
    CouldNotBuildFromConfig,
    CouldNotBuildFromRuntime,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Output(ref err) => write!(f, "Output error: {}", err),
            ServerError::Bincode(ref err) => write!(f, "Bincode serialisation error: {}", err),
            ServerError::Configure(ref err) => write!(f, "Could not deserialize configuration: {}", err),
            ServerError::CouldNotBuildFromConfig => write!(f, "Could not build server from config file"),
            ServerError::CouldNotBuildFromRuntime => write!(f, "Could not build server from runtime information"),
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Output(ref err) => err.description(),
            ServerError::Bincode(ref err) => err.description(),
            ServerError::Configure(ref err) => err.description(),
            ServerError::CouldNotBuildFromConfig => "Maybe the configuration file is not present, corrupt or not readable. Please check file access rights.",
            ServerError::CouldNotBuildFromRuntime => "Maybe the runtime information file is not present, corrupt or not readable. Please check file access rights.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServerError::Output(ref err) => Some(err),
            ServerError::Bincode(ref err) => Some(err),
            ServerError::Configure(ref err) => Some(err),
            ServerError::CouldNotBuildFromConfig => None,
            ServerError::CouldNotBuildFromRuntime => None,
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
