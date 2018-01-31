use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    NoSensor,
    NoZone,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NoSensor => write!(f, "No sensor found"),
            NoZone => write!(f, "No zone found"),
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match self {
            NoSensor => "No sensor found",
            NoZone => "No zone found",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            NoSensor => None,
            NoZone => None,
        }
    }
}
