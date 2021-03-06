//! Trait das die Eigenschaften aller vom Server unterstützten Sensoren beschreibt.
//!
use prelude::*;


mod metz_connect_ci4;
mod ra_gas_co_no2_mod;
mod test_sensor;

// Reexports
pub use self::metz_connect_ci4::MetzConnectCI4;
pub use self::ra_gas_co_no2_mod::RaGasCONO2Mod;
pub use self::test_sensor::TestSensor;

pub type BoxedSensor = Box<Sensor + Send + Sync>;
pub type SensorList = Vec<Arc<RwLock<BoxedSensor>>>;


/// Verfügbare Sensor Typen
///
/// Von der 'xMZ-Plattform' unterstützte Sensor Module.
/// Ein Sensor Modul beherbergt eine oder mehrere `Messzellen` sowie ein BUS Interface über das der
/// Sensor mit der Zentrale verbunden ist (Modbus RTU/ TCP/IP).
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SensorType {
    #[serde(rename="Metz Connect CI4 Modul")]
    MetzConnectCI4,
    #[serde(rename="RA-GAS GmbH CO/ NO₂ Kombisensor mit Modbus Interface")]
    RaGasCONO2Mod,
    #[serde(rename="Test Sensor")]
    TestSensor,
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

    /// Gibt die Sensor ID wieder
    ///
    /// Die Sensor ID wird u.a. in der Konfigurationsdatei verwendet um die Messzellen den Sensoren
    /// zuzuordnen.
    ///
    fn get_id(&self) -> u32;

    /// Gibt den Sensor Type wieder
    ///
    fn get_sensor_type(&self) -> SensorType;

    /// Liefert eine Referenz auf den Vector der Messzellen
    ///
    fn get_messzellen(&self) -> Vec<Arc<RwLock<BoxedMesszelle>>>;

    /// Liefert Optional eine Messzelle (wenn vorhanden)
    ///
    /// Gibt `None` zurück wenn der Sensor keine Messzelle an Position `num` besizt.
    fn get_messzelle(&self, num: usize) -> Option<&Arc<RwLock<BoxedMesszelle>>>;
}
