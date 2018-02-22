//! Trait der generellen Sensor Funktionen
//!
use std::time::SystemTime;


pub trait Sensor {
    /// Aktueller Sensor Wert und Timestamp der Ermittlung
    ///
    /// Jeder Sensor verfügt über ein Liste mit einem oder mehreren Paaren von Messwerten und den
    /// Timestamps die bei der Ermittlung des Wertes erstellt werden.
    /// Die Implementierung dieser Funktion muss den letzten dieser Wertepaare ausgeben.
    /// Ist kein Wert vorhanden wird `None` zurückgegeben.
    fn value(&self) -> Option<&(SystemTime, f64)>;

    /// Mittelwert der letzten `min` Minuten
    ///
    /// Die Implementation dieser Funktion berechnet den Mittelwert aus den `values` Werten
    /// des Sensors.
    /// Sind keine Werte vorhanden wird `None` zurück gegeben.
    fn average(&self, min: u32) -> Option<f64>;

    /// Aktuellen Sensorwert ermitteln und speichern.
    ///
    /// Die Implementation dieser Funktion muss den aktuellen Sensor Wert ermitteln (BUS Abfrage,
    /// Berechnung bie Simmulationssensoren, usw.).
    ///
    /// **Gleichzeitig muss in dieser Implementation die Länge der Werteliste `values`
    /// angepasst werden. Gemäß den Vorgaben der Funktionsbeschreibung sind nur Werte
    /// der letzten halben Stunde (Halbstundenmittelwert) nötig.**
    fn update(&mut self);
}
