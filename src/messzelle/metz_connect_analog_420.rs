use messzelle::{
    MAX_VALUES_FOR_N_MINUTES,
    Messzelle,
    MesszelleError,
    MesszelleType,
};
use std::fmt;
use std::time::Duration;
use std::time::SystemTime;

// Die `pub` Public members sind nötig, da die Felder von den Konvertierungen (Configuration, RuntimInfo,
// und Api) gesetzt werden.
/// CO Messzelle eines 'RA-GAS GmbH CO/ NO2 Kombisensor mit Modbus Interface'
///
#[derive(Debug)]
pub struct MetzConnectCI4Analog420 {
    pub messzelle_type: MesszelleType,
    pub values: Vec<(f64, SystemTime)>,
}

impl MetzConnectCI4Analog420 {
    /// Erstellt eine neue Messzelle
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let messzelle = MetzConnectCI4Analog420::new();
    /// ```
    pub fn new() -> Self {
        MetzConnectCI4Analog420 {
            messzelle_type: MesszelleType::MetzConnectCI4Analog420,
            values: vec![],
        }
    }
}

impl Messzelle for MetzConnectCI4Analog420 {
    /// Aktueller Messzelle Wert und Timestamp der Ermittlung
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let messzelle = MetzConnectCI4Analog420::new();
    /// assert!(messzelle.get_value().is_none());
    /// ```
    fn get_value(&self) -> Option<&(f64, SystemTime)> {
        self.values.last()
    }

    /// Liefert alle Werte der Messzelle
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let messzelle = MetzConnectCI4Analog420::new();
    /// assert_eq!(messzelle.get_values().len(), 0);
    /// ```
    fn get_values(&self) -> Vec<(f64, SystemTime)> {
        self.values.clone()
    }

    /// Liefert den Typ der Messzelle
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let messzelle = MetzConnectCI4Analog420::new();
    /// assert_eq!(messzelle.get_messzelle_type(), MesszelleType::MetzConnectCI4Analog420);
    /// ```
    fn get_messzelle_type(&self) -> MesszelleType {
        self.messzelle_type.clone()
    }

    /// Mittelwert der letzten `min` Minuten
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// ```
    fn average(&self, minutes: u64) -> Result<f64, MesszelleError> {
        let mut values = self.values.clone();
        // Index ermitteln
        if let Some(index) = values.iter().position(|&(_value, timestamp)| {
            timestamp.elapsed().unwrap() < Duration::from_secs(minutes)
        }) {
            values = values.split_off(index);
        }

        // Spezialfall, nur noch ein Wert vorhanden. Hier muss nun geprüft werden ob dieser
        // veraltet ist.
        if values.len() == 1 {
            if let Some(&(_value, timestamp)) = values.get(0) {
                if let Ok(duration) = timestamp.elapsed() {
                    if duration > Duration::from_secs(minutes) {
                        values.clear();
                    }
                }
            }
        }

        // Anzahl der verbleibenden Wertepaare
        let len = values.len() as f64;
        // Addieren
        let sum = values
            .iter()
            .fold(0.0, |acc, &(value, _timestamp)| acc + value);
        // durch Anzahl teilen um Mittelwert zu erhalten
        let avg = sum / len;
        // Floting Point kann NaN (Not a Number) ergeben, in diesem Fall Fehler zurück
        if avg.is_nan() {
            Err(MesszelleError::NoAverage)
        } else {
            Ok(avg)
        }
    }

    /// Aktuellen Messzellewert ermitteln und speichern.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// ```
    fn update(&mut self) {
        let last_value = match self.get_value() {
            Some(&(value, _timestamp)) => value,
            None => 0.0,
        };
        self.values.push((last_value + 1.0, SystemTime::now()));
        self.shrink_values();
        debug!("|-- Update Messzelle: '{}'", &self);
    }

    /// Entfernt alle Wert/Zeistempel Paare die älter als `Messzelle::MAX_VALUES_FOR_N_MINUTES` sind.
    ///
    /// Diese Funktion besteht aus 2 Tests. Der erste Spezialfall tritt ein wenn nur ein
    /// Wert/Zeitstempel Paar vorhanden ist. Hier muss getestet werden ob dieses veraltete Daten
    /// enthält. Ist dem so werden alle Werte/Zeitstempel gelöscht.
    /// Der zweite Test sucht aus der Liste den Index Wert ab dem veraltet Wert/Zeitstempel
    /// Paare auftreten. Anschließend wird dieser Index Wert verwendet um den Wert/Zeitstempel
    /// Vector an dieser Stelle zu teilen. Altere Werte werden dabei verworfen.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// ```
    fn shrink_values(&mut self) {
        // 1. Spezialfall, ist nur ein Wert/Zeitstempel Paar vorhanden muss dieses zuerst
        // geprüft werden. Ist der Wert dieses Wert/Zeitstempel Paares veraltet dann wird das
        // Paar entfernt, der `values` Vector ist dann leer. Der folgende Test wird in diesem
        // Falle nicht ausgeführt, da kein Index mehr gefunden werden kann.
        if self.values.len() == 1 {
            if let Some(&(_value, timestamp)) = self.values.get(0) {
                if let Ok(duration) = timestamp.elapsed() {
                    if duration > Duration::from_secs(MAX_VALUES_FOR_N_MINUTES) {
                        self.values.clear();
                    }
                }
            }
        }

        // 2. Test, mind. ein aktuelles Wert/Zeitstempel Paar ist vorhanden.

        // Die [`position()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position)
        // Funktion sucht in einem Iterator nach einem Element und liefert dessen Index Wert.
        // Dieser Index wird benutzt um die Liste der Liste der Wert/Zeitstempel Paare
        // mit der Funktion [`split_off()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off)
        // zu teilen.
        if let Some(index) = self.values.iter().position(|&(_value, timestamp)| {
            timestamp.elapsed().unwrap() < Duration::from_secs(MAX_VALUES_FOR_N_MINUTES)
        }) {
            // Mit `split_off()` wird der Vector geteilt, es bleiben nur noch die
            // Wert/Zeitstempel Paare der letzten `MAX_VALUES_FOR_N_MINUTES` Minuten übrig.
            //
            // **Dieser Rest wird wieder als `values` übernommen, alle "alten" Werte werden so verworfen.**
            self.values = self.values.split_off(index);
        }
    }
}

impl fmt::Display for MetzConnectCI4Analog420 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Analog 4-20mA")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let messzelle = MetzConnectCI4Analog420::new();
        assert_eq!(messzelle.values.len(), 0);
    }

    #[test]
    fn get_value() {
        let messzelle = MetzConnectCI4Analog420::new();
        assert!(messzelle.get_value().is_none());
    }

    #[test]
    fn get_values() {
        let messzelle = MetzConnectCI4Analog420::new();
        assert_eq!(messzelle.get_values().len(), 0);
    }

    #[test]
    fn average() {
        let messzelle = MetzConnectCI4Analog420::new();
        assert!(messzelle.average(15).is_err());
    }

    #[test]
    fn update() {
        let mut messzelle = MetzConnectCI4Analog420::new();
        assert_eq!(messzelle.values.len(), 0);
        messzelle.update();
        assert_eq!(messzelle.values.len(), 1);
    }

}
