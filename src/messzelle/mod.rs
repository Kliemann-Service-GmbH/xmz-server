//! Trait das eine einzelne Messzelle beschreibt
//!
//! Eine einzelne `Messzelle` eines Sensors.
//! Diese sitzt in der Regel auf einer Sensor Platine (`sensor`). Jeder Sensor hat mindestens eine
//! Messzelle mit einem Wert und einem Mittelwert.

use prelude::*;
use std::fmt;
use std::time::SystemTime;

mod error;
pub mod metz_connect_analog_420;
pub mod ra_gas_co_mod;
pub mod ra_gas_no2_mod;

// Reexports
pub use self::error::MesszelleError;
pub use self::metz_connect_analog_420::MetzConnectCI4Analog420;
pub use self::ra_gas_co_mod::RaGasCOMod;
pub use self::ra_gas_no2_mod::RaGasNO2Mod;

pub type BoxedMesszelle = Box<Messzelle + Send + Sync>;
pub type MesszelleList = Vec<Arc<RwLock<BoxedMesszelle>>>;


pub const MAX_VALUES_FOR_N_MINUTES: u64 = 60;

/// Verfügbare Messzellen Typen
///
///
#[derive(Clone, Debug, Deserialize, Serialize)]
#[derive(PartialEq)]
pub enum MesszelleType {
    #[serde(rename="RA-GAS GmbH NO₂")]
    RaGasNO2Mod,
    #[serde(rename="RA-GAS GmbH CO")]
    RaGasCOMod,
    #[serde(rename="Analog 4-20mA Sensor an Metz Connect DI4")]
    MetzConnectCI4Analog420,
}

/// Basis Trait das die Eigenschaften einer Messzelle beschreibt
///
/// Jede Messzelle hat einen Direktwert (`get_value()`) sowie ein Mittelwert (`average(minutes)`).
/// Der Mittelwert Funktion kann ein Parameter übergeben werden mit dem die Dauer des zu
/// berechnendem Mittelwertes angegeben werden kann.
pub trait Messzelle: AsAny + fmt::Debug + fmt::Display {
    /// Aktueller Messzelle Wert und Timestamp der Ermittlung
    ///
    /// Jede Messzelle verfügt über ein Liste mit einem oder mehreren Paaren, Messwerten und den
    /// Timestamps die bei der Ermittlung dieses Messwertes erstellt werden.
    /// Die Implementierung der `get_value()` Funktion muss den letzten dieser Wertepaare ausgeben.
    /// Ist kein Messwert vorhanden wird `None` zurückgegeben.
    fn get_value(&self) -> Option<&(f64, SystemTime)>;

    /// Liefert alle Werte zurück
    ///
    /// Jede Messzelle verfügt über ein Liste mit einem oder mehreren Paaren, Messwerten und den
    /// Timestamps die bei der Ermittlung dieses Messwertes erstellt werden.
    /// Die Implementierung der `get_values()` Funktion muss alle dieser Wertepaare, in
    /// einem Vector zurück geben.
    fn get_values(&self) -> Vec<(f64, SystemTime)>;

    /// Liefert den Typ der Messzelle
    ///
    /// Der Typ der Messzelle kann in der Konfigurationsdatei nur als String angegben
    /// werden.
    /// Zudem ist der Typ aus den Trait Objekten nicht einfach zu ermitteln. Analog zu
    /// den Sensoren ist dieser Memeber eine Art Krückstock.
    fn get_messzelle_type(&self) -> MesszelleType;

    /// Mittelwert der letzten `min` Minuten
    ///
    /// Die Implementation dieser Funktion berechnet den Mittelwert aus den Werte.- Zeitstempel-
    /// paaren des Messzelles.
    /// Sind keine Werte vorhanden wird `None` zurück gegeben.
    ///
    /// # Parameters
    /// * `min`     - Minuten aus denen der Mittelwert berechnet werden soll
    ///
    fn average(&self, minutes: u64) -> Result<f64, MesszelleError>;

    /// Aktuellen Messzellewert ermitteln und speichern.
    ///
    /// Die Implementation dieser Funktion muss den aktuellen Messzelle Wert ermitteln (BUS Abfrage,
    /// Berechnung bie Simulationssensoren, usw.).
    ///
    /// **Gleichzeitig muss in dieser Implementation die Länge der Werteliste `values`
    /// angepasst werden. Gemäß den Vorgaben der Funktionsbeschreibung sind nur Werte
    /// der letzten Stunde (max. Stundenmittelwert) nötig.**
    fn update(&mut self);

    /// Entfernt alle Wert/Zeistempel Paare die älter als `Messzelle::max_values_for_n_minutes` sind.
    ///
    /// Diese Funktion besteht aus 2 Tests. Der erste Spezialfall tritt ein wenn nur ein
    /// Wert/Zeitstempel Paar vorhanden ist. Hier muss getestet werden ob dieses veraltete Daten
    /// enthält. Ist dem so werden alle Werte/Zeitstempel gelöscht.
    /// Der zweite Test sucht aus der Liste den Index Wert ab dem veraltet Wert/Zeitstempel
    /// Paare auftreten. Anschließend wird dieser Index Wert verwendet um den Wert/Zeitstempel
    /// Vector an dieser Stelle zu teilen. Altere Werte werden dabei verworfen.
    ///
    /// Diese Funktion wird in aller Regel in der Implementierung der `update()` Funktion
    /// aufgerufen.
    fn shrink_values(&mut self);
}
