//! Mögliche Fehler die im Serverbetrieb auftreten können
use bincode::Error as BincodeError;
use configure::DeserializeError as ConfigureError;
use output::OutputError;
use std::error::Error;
use std::fmt;

/// Mögliche Server Fehler
///
#[derive(Debug)]
pub enum ServerError {
    /// Fehler beim Schalten eines Ausgangs
    Output(OutputError),
    Bincode(BincodeError),
    Configure(ConfigureError),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Output(ref err) => write!(f, "Output error: {}", err),
            ServerError::Bincode(ref err) => write!(f, "Bincode serialisation error: {}", err),
            ServerError::Configure(ref err) => write!(f, "Could not deserialize configuration: {}", err),
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Output(ref err) => err.description(),
            ServerError::Bincode(ref err) => err.description(),
            ServerError::Configure(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServerError::Output(ref err) => Some(err),
            ServerError::Bincode(ref err) => Some(err),
            ServerError::Configure(ref err) => Some(err),
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
