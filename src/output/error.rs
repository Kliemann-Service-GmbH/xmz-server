use output::backends::ShiftRegisterError;
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
    ShiftRegister(ShiftRegisterError),
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OutputError::CouldNotSet => write!(f, "Konnte den Ausgang nicht schreiben"),
            OutputError::CouldNotGet => write!(f, "Konnte den Ausgang nicht nicht lesen"),
            OutputError::CouldNotUnset => write!(f, "Konnte den Ausgang nicht löschen"),
            OutputError::ShiftRegister(ref err) => write!(f, "Fehler beim Schreiben des ShiftRegisters: {}", err)
        }
    }
}

impl Error for OutputError {
    fn description(&self) -> &str {
        match *self {
            OutputError::CouldNotSet => "Konnte den Ausgang nicht schreiben",
            OutputError::CouldNotGet => "Konnte den Ausgang nicht nicht lesen",
            OutputError::CouldNotUnset => "Konnte den Ausgang nicht löschen",
            OutputError::ShiftRegister(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            OutputError::CouldNotSet => None,
            OutputError::CouldNotGet => None,
            OutputError::CouldNotUnset => None,
            OutputError::ShiftRegister(ref err) => Some(err),
        }
    }
}


impl From<ShiftRegisterError> for OutputError {
    fn from(error: ShiftRegisterError) -> Self {
        OutputError::ShiftRegister(error)
    }
}
