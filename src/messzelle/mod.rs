//! Messzelle auf einer Sensorplatine
//!
//! Jeder Sensor hat mindestens eine Messzelle mit einem Wert und einem Mittelwert.
mod error;
mod messzelle;
mod ragas_co_mod;
mod ragas_no2_mod;

pub use self::error::MesszelleError;
pub use self::messzelle::{BoxedMesszelle, Messzelle};
pub use self::ragas_co_mod::RaGasCO;
pub use self::ragas_no2_mod::RaGasNO2;
