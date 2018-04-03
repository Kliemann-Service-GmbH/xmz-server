//! Trait das eine einzelne Messzelle beschreibt
//!

// TODO:
// - `get_values()` implementieren
// - `value()` default Implementation, evtl. auch für die anderen Trait Funktionen

use messzelle::MesszelleError;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use prelude::*;

pub type BoxedMesszelle = Box<Messzelle + Send + 'static>;
pub type MesszellenList = Vec<Arc<Mutex<BoxedMesszelle>>>;
pub type MesszellenRefList<'a> = Vec<&'a Messzelle>;

/// Basis Trait das die Eigenschaften einer Messzelle beschreibt
///
/// Jede Messzelle hat einen Direktwert (`value()`) sowie ein Mittelwert (`average(minutes)`).
/// Der Mittelwert Funktion kann ein Parameter übergeben werden mit dem die Dauer des zu
/// berechnendem Mittelwertes angegeben werden kann.
pub trait Messzelle: AsAny + Debug {
    /// Aktueller Messzelle Wert und Timestamp der Ermittlung
    ///
    /// Jede Messzelle verfügt über ein Liste mit einem oder mehreren Paaren, Messwerten und den
    /// Timestamps die bei der Ermittlung dieses Messwertes erstellt werden.
    /// Die Implementierung der `value()` Funktion muss den letzten dieser Wertepaare ausgeben.
    /// Ist kein Messwert vorhanden wird `None` zurückgegeben.
    fn value(&self) -> Option<&(f64, SystemTime)>;

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
