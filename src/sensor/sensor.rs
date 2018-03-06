//! Trait der eine Sensorplatine beschreibt
//!
use std::fmt;
use std::sync::{Arc, Mutex};

use ::messzelle::BoxedMesszelle;


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
