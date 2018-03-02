//! Trait der generellen Sensor Funktionen
//!
use std::any::Any;
use std::fmt::Debug;
use std::io::Error;
use std::time::SystemTime;


/// Dieser Trait ist für das Upcasting der SensorTypen nötig
pub trait AsAny: Any {
    fn as_any(&self) -> &Any;
}

/// Implementiere AsAny für alle (Sensor)Typen
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &Any {
        self
    }
}

/// Basis Trait das die Eigenschaften der Sensoren beschreibt
///
/// Jeder Sensor hat einen Direktwert (`value()`) sowie ein Mittelwert (`average(minutes)`).
/// Der Mittelwert Funktion kann ein Parameter übergeben werden mit dem die Dauer des zu
/// berechnendem Mittelwertes angegeben werden kann.
pub trait Sensor: AsAny + Debug {
    /// Aktueller Sensor Wert und Timestamp der Ermittlung
    ///
    /// Jeder Sensor verfügt über ein Liste mit einem oder mehreren Paaren, Messwerten und den
    /// Timestamps die bei der Ermittlung dieses Messwertes erstellt werden.
    /// Die Implementierung der `value()` Funktion muss den letzten dieser Wertepaare ausgeben.
    /// Ist kein Messwert vorhanden wird `None` zurückgegeben.
    fn value(&self) -> Option<&(f64, SystemTime)>;

    /// Mittelwert der letzten `min` Minuten
    ///
    /// Die Implementation dieser Funktion berechnet den Mittelwert aus den Werte.- Zeitstempel-
    /// paaren des Sensors.
    /// Sind keine Werte vorhanden wird `None` zurück gegeben.
    ///
    /// # Parameters
    /// * `min`     - Minuten aus denen der Mittelwert berechnet werden soll
    ///
    fn average(&self, minutes: u64) -> Result<f64, Error>;

    /// Aktuellen Sensorwert ermitteln und speichern.
    ///
    /// Die Implementation dieser Funktion muss den aktuellen Sensor Wert ermitteln (BUS Abfrage,
    /// Berechnung bie Simulationssensoren, usw.).
    ///
    /// **Gleichzeitig muss in dieser Implementation die Länge der Werteliste `values`
    /// angepasst werden. Gemäß den Vorgaben der Funktionsbeschreibung sind nur Werte
    /// der letzten Stunde (max. Stundenmittelwert) nötig.**
    fn update(&mut self);

    /// Entfernt alle Wert/Zeistempel Paare die älter als `Sensor::max_values_for_n_minutes` sind.
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
