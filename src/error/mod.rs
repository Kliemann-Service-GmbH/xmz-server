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
    CouldNotBuildFromRuntime,
    IO(IOError),
    Output(OutputError),
    RuntimePathNotSet,
    ServerBuilder,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Bincode(ref err) => write!(f, "Bincode serialisation error: {}", err),
            ServerError::Configure(ref err) => write!(f, "Could not deserialize configuration: {}", err),
            ServerError::CouldNotBuildFromConfig(ref err) => write!(f, "Could not build server from config file: {}", err),
            ServerError::CouldNotBuildFromRuntime => write!(f, "Could not build server from runtime information"),
            ServerError::IO(ref err) => write!(f, "IO Error: {}", err),
            ServerError::Output(ref err) => write!(f, "Output error: {}", err),
            ServerError::RuntimePathNotSet => write!(f, "Runtime path not set"),
            ServerError::ServerBuilder => write!(f, "Server build error"),
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Bincode(ref err) => err.description(),
            ServerError::Configure(ref err) => err.description(),
            ServerError::CouldNotBuildFromConfig(ref _err) => "Maybe the configuration file is not present, corrupt or not readable. Please check file access rights.",
            ServerError::CouldNotBuildFromRuntime => "Maybe the runtime information file is not present, corrupt or not readable. Please check file access rights.",
            ServerError::IO(ref err) => err.description(),
            ServerError::Output(ref err) => err.description(),
            ServerError::RuntimePathNotSet => "The runtime path is not set.",
            ServerError::ServerBuilder => "Server could not build",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServerError::Bincode(ref err) => Some(err),
            ServerError::Configure(ref err) => Some(err),
            ServerError::CouldNotBuildFromConfig(ref err) => Some(err),
            ServerError::CouldNotBuildFromRuntime => None,
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
