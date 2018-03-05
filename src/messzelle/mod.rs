//! Messzelle auf einer Sensorplatine
//!
//! Jeder Sensor hat mindestens eine Messzelle mit einem Wert und einem Mittelwert.
mod error;
mod messzelle;

pub use self::error::MesszelleError;
pub use self::messzelle::{BoxedMesszelle, Messzelle};
