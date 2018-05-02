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


/// Verfügbare Sensor Typen
///
/// Von der 'xMZ-Plattform' unterstützte Sensor Module.
/// Ein Sensor Modul beherbergt eine oder mehrere `Messzellen` sowie ein BUS Interface über das der
/// Sensor mit der Zentrale verbunden ist (Modbus RTU/ TCP/IP).
///
#[derive(Debug, Deserialize)]
pub enum SensorType {
    #[serde(rename="Metz Connect CI4 Modul")]
    MetzConnectCI4,
    #[serde(rename="RA-GAS GmbH CO/ NO₂ Kombisensor mit Modbus Interface")]
    RaGasCONO2Mod,
}


/// Trait das die Eigenschaften aller Sensoren beschreibt.
///
/// Jeder Sensor kann meherer Messzellen besitzen. So verfügt beispielsweise der
/// "CO/NO2 Kombisensor mit Modbus Interface" der Firma RA-GAS GmbH über 2 Messzellen, je eine
/// für CO (Kohlenmonoxid) und NO2 (Stickstoffdioxid).
pub trait Sensor: fmt::Debug + fmt::Display {
    /// In der Update Funktion werden die Sensoren ausgelesen
    ///
    /// In dieser Funktion sollten auch die Werte (`values`) der Messzellen aktualisiert werden.
    fn update(&self);

    /// Liefert eine Referenz auf den Vector der Messzellen
    ///
    fn get_messzellen(&self) -> &Vec<Arc<Mutex<BoxedMesszelle>>>;

    /// Liefert Optional eine Messzelle (wenn vorhanden)
    ///
    /// Gibt `None` zurück wenn der Sensor keine Messzelle an Position `num` besizt.
    fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>>;
}
