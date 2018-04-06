//! Trait das die Eigenschaften aller vom Server unterstützten Sensoren beschreibt.
//! 
use messzelle::BoxedMesszelle;
use std::fmt;
use std::sync::{Arc, Mutex};

mod metz_connect_ci4;
mod ra_gas_co_no2_mod;

pub use self::metz_connect_ci4::MetzConnectCI4;
pub use self::ra_gas_co_no2_mod::RaGasCONO2Mod;

pub type BoxedSensor = Box<Sensor + Send + 'static>;
pub type SensorsList = Vec<Arc<Mutex<BoxedSensor>>>;

/// Trait das die Eigenschaften aller Sensoren beschreibt.
///
/// Jeder Sensor kann meherer Messzellen besitzen. So verfügt beispielsweise der
/// "CO/NO2 Kombisensor mit Modbus Interface" der Firma RA-GAS GmbH über 2 Messzellen, je eine
/// für CO (Kohlenmonoxid) und NO2 (Stickstoffdioxid).
pub trait Sensor: fmt::Debug {
    /// In der Update Funktion werden die Sensoren ausgelesen
    ///
    /// In dieser Funktion sollten auch die Werte (`values`) der Messzellen aktualisiert werden.
    fn update(&self);

    /// Liefert Optional eine Messzelle (wenn vorhanden)
    ///
    /// Gibt `None` zurück wenn der Sensor keine Messzelle an Position `num` besizt.
    fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>>;
}
