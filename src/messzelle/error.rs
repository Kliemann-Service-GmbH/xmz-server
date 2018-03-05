use std::error::Error;
use std::fmt;

/// Mögliche Fehler einer Messzelle
///
#[derive(Debug)]
pub enum MesszelleError {
    /// Mittelwert konnte nicht berechnet werden
    NoAverage,
}

impl fmt::Display for MesszelleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MesszelleError::NoAverage => write!(f, "Mittelwert konnte nicht berechnet werden."),
        }
    }
}

impl Error for MesszelleError {
    fn description(&self) -> &str {
        match *self {
            MesszelleError::NoAverage => "Mittelwert konnte nicht berechnet werden.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            MesszelleError::NoAverage => None,
        }
    }
}
