//! Mögliche Fehler die im Serverbetrieb auftreten können
use bincode::Error as BincodeError;
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
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerError::Output(ref err) => write!(f, "Output error: {}", err),
            ServerError::Bincode(ref err) => write!(f, "Bincode serialisation error: {}", err),
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Output(ref err) => err.description(),
            ServerError::Bincode(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ServerError::Output(ref err) => Some(err),
            ServerError::Bincode(ref err) => Some(err),
        }
    }
}


impl From<BincodeError> for ServerError {
    fn from(error: BincodeError) -> Self {
        ServerError::Bincode(error)
    }
}
