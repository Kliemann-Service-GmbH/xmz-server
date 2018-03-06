use config::ConfigError;
use output::OutputError;
use std::error::Error;
use std::fmt;

/// Mögliche Server Fehler
///
#[derive(Debug)]
pub enum ServerError {
    /// Fehler in der Konfiguration
    Config(ConfigError),
    /// Fehler beim Schalten eines Ausgangs
    Output(OutputError),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Config(ref err) => write!(f, "Configuration error: {}", err),
            ServerError::Output(ref err) => write!(f, "Output error: {}", err),
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Config(ref err) => err.description(),
            ServerError::Output(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServerError::Config(ref err) => Some(err),
            ServerError::Output(ref err) => Some(err),
        }
    }
}

impl From<ConfigError> for ServerError {
    fn from(err: ConfigError) -> ServerError {
        ServerError::Config(err)
    }
}
