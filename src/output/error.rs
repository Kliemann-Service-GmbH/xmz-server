use std::error::Error;
use std::fmt;

/// Output Error
///
/// Fehler die beim Schalten der Ausgänge auftreten können.
#[derive(Debug)]
pub enum OutputError {
    // Ausgang konnte nicht geschalten werden
    CouldNotSet,
    // Ausgang konnte nicht abgefragt werden
    CouldNotGet,
    // Ausgang konnte nicht ausgeschalten werden
    CouldNotUnset,
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OutputError::CouldNotSet => write!(f, "Could not set Output"),
            OutputError::CouldNotGet => write!(f, "Could not get Output"),
            OutputError::CouldNotUnset => write!(f, "Could not unset Output"),
        }
    }
}

impl Error for OutputError {
    fn description(&self) -> &str {
        match *self {
            OutputError::CouldNotSet => "Could not set Output",
            OutputError::CouldNotGet => "Could not get Output",
            OutputError::CouldNotUnset => "Could not unset Output",
        }
    }
    
    fn cause(&self) -> Option<&Error> {
        match *self {
            OutputError::CouldNotSet => None,
            OutputError::CouldNotGet => None,
            OutputError::CouldNotUnset => None,
        }
    }
}
