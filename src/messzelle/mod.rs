//! Messzelle auf einer Sensorplatine
//!
//! Jeder Sensor hat mindestens eine Messzelle mit einem Wert und einem Mittelwert.
mod error;
mod messzelle;
mod metz_connect_analog_420;
mod ra_gas_co_mod;
mod ra_gas_no2_mod;

pub use self::error::MesszelleError;
pub use self::messzelle::{BoxedMesszelle, Messzelle, MesszellenList};
pub use self::metz_connect_analog_420::MetzConnectCI4Analog420;
pub use self::ra_gas_co_mod::RaGasCO;
pub use self::ra_gas_no2_mod::RaGasNO2;
