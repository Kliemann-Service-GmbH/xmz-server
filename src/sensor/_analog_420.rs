use sensor::Sensor;
use std::io::{Error, ErrorKind};
use std::time::Duration;
use std::time::SystemTime;


/// RA-GAS GmbH CO/ NO₂ Kombisensor mit Modbus Interface
///
/// Kombisensor für Kohlenmonoxid (CO) und Stickstoffdioxid (NO₂) mit Modbus Interface.
/// Diese Kombigeräte mit 2 Messzellen werden über ein Modbus RTU BUS abgefragt.
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct Analog420 {
    values: Vec<(f64, SystemTime)>,
    max_values_for_n_minutes: u64, // in Sekunden
}

impl Analog420 {
    pub fn new() -> Self {
        Analog420 {
            values: vec![],
            // max_values_for_n_minutes: 5 * 60 * 60,    // Normale Sensoren arbeiten mit Minuten Werten
            max_values_for_n_minutes: 5,    // Simulator Sensoren arbeiten mit Sekunden Werten
        }
    }
}

impl Sensor for Analog420 {
    /// Liefert Optional das letzte Wert/Zeitstempel Paar.
    ///
    /// `None` wenn kein Wert/Zeitstempel Paar vorhanden ist.
    fn value(&self) -> Option<&(f64, SystemTime)> {
        self.values.last()
    }

    /// Entfernt alle Wert/Zeistempel Paare die älter als `Sensor::max_values_for_n_minutes` sind.
    ///
    /// Diese Funktion besteht aus 2 Tests. Der erste Spezialfall tritt ein wenn nur ein
    /// Wert/Zeitstempel Paar vorhanden ist. Hier muss getestet werden ob dieses veraltete Daten
    /// enthält. Ist dem so werden alle Werte/Zeitstempel gelöscht.
    /// Der zweite Test sucht aus der Liste den Index Wert ab dem veraltet Wert/Zeitstempel
    /// Paare auftreten. Anschließend wird dieser Index Wert verwendet um den Wert/Zeitstempel
    /// Vector an dieser Stelle zu teilen. Altere Werte werden dabei verworfen.
    fn shrink_values(&mut self) {
        // 1. Spezialfall, ist nur ein Wert/Zeitstempel Paar vorhanden muss dieses zuerst
        // geprüft werden. Ist der Wert dieses Wert/Zeitstempel Paares veraltet dann wird das
        // Paar entfernt, der `values` Vector ist dann leer. Der folgende Test wird in diesem
        // Falle nicht ausgeführt, da kein Index mehr gefunden werden kann.
        if self.values.len() == 1 {
            if let Some( &(_value, timestamp) ) = self.values.get(0) {
                if let Ok(duration) = timestamp.elapsed() {
                    if duration > Duration::from_secs(self.max_values_for_n_minutes) {
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
        if let Some(index) = self.values.iter()
                                        .position(|&(_value, timestamp)| {
                                            timestamp.elapsed().unwrap() < Duration::from_secs(self.max_values_for_n_minutes)
                                        }) {
            // Mit `split_off()` wird der Vector geteilt, es bleiben nur noch die
            // Wert/Zeitstempel Paare der letzten `max_values_for_n_minutes` Minuten übrig.
            //
            // **Dieser Rest wird wieder als `values` übernommen, alle "alten" Werte werden so verworfen.**
            self.values = self.values.split_off(index);
        }
    }

    /// Die Update Funktion beinhaltet die Logic zum Update dieses Sensors.
    ///
    /// Gleichzeitig beinhaltet die Update Funktion einen Aufruf zur `shrink_values` Funktion
    /// die "veraltete" Werte aus dem `values` Vector entfernt (siehe deren Dokumentation).
    ///
    /// # Examples
    ///
    /// ```
    /// # use xmz_server::sensor::{Analog420, Sensor};
    /// let mut sensor = Analog420::new();
    /// sensor.update();
    /// assert!(sensor.average(30).is_ok());
    /// ```
    fn update(&mut self) {
        let last_value = match self.value() {
            Some( &(value, _timestamp) ) => value,
            None => 0.0,
        };
        self.values.push((last_value + 1.0, SystemTime::now()));
        self.shrink_values();
    }

    /// Mittelwert Berechnung
    ///
    /// Ein Mittelwert steht erst unmittelbar nach einem `update` des Sensors zur Verfügung.
    ///
    /// # Parameters
    /// * `minutes`     - Anzahl der Minuten über die der Mittelwert berechnet werden soll
    ///                 (z.B. 30) für Halbstunden Mittelwerte
    ///
    /// # Examples
    ///
    /// ```
    /// # use xmz_server::sensor::{Analog420, Sensor};
    /// let mut sensor = Analog420::new();
    /// // Kein Update -> kein Mittelwert
    /// assert!(sensor.average(30).is_err());
    ///
    /// sensor.update();
    /// assert!(sensor.average(30).is_ok());
    /// ```
    fn average(&self, minutes: u64) -> Result<f64, Error> {
        let mut values = self.values.clone();
        // Index ermitteln
        if let Some(index) = values.iter()
            .position(|&(_value, timestamp)| {
                timestamp.elapsed().unwrap() < Duration::from_secs(minutes)
            }) {
            values = values.split_off(index);
            println!("index: {:?}", index);
        }

        // Spezialfall, nur noch ein Wert vorhanden. Hier muss nun geprüft werden ob dieser
        // veraltet ist.
        if values.len() == 1 {
            if let Some( &(_value, timestamp) ) = values.get(0) {
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
        let sum = values.iter().fold(0.0, |acc, &(value, _timestamp)| acc + value);
        // durch Anzahl teilen um Mittelwert zu erhalten
        let avg = sum / len;
        // Floting Point kann NaN (Not a Number) ergeben, in diesem Fall Fehler zurück
        if avg.is_nan() {
            Err(Error::new(ErrorKind::Other, "Mittelwert konnte nicht berechnet werden (NaN)"))
        } else {
            Ok(avg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn create() {
        let sensor = Analog420::new();
        assert_eq!(sensor.values.len(), 0);
    }

    #[test]
    fn value() {
        let mut sensor = Analog420::new();
        for _ in 0..10 {
            sensor.update();
        }
        let last_value = sensor.value().unwrap().0;
        assert_eq!(last_value, 10.0);
    }

    #[test]
    fn update() {
        let mut sensor = Analog420::new();
        assert_eq!(sensor.value(), None);
        sensor.update();
        assert!(sensor.value().is_some());
    }

    #[test]
    fn average() {
        let sensor = Analog420::new();
        assert!(sensor.average(1).is_err());
    }

    #[test]
    fn average_after_update() {
        let mut sensor = Analog420::new();
        sensor.update();
        assert!(sensor.average(1).is_ok());
    }

    #[test]
    fn average_after_some_time() {
        let mut sensor = Analog420::new();
        sensor.update();
        thread::sleep(Duration::from_secs(2));
        assert!(sensor.average(1).is_err());
    }

    #[test]
    fn shrink_values() {
        let mut sensor = Analog420 {
            values: vec![],
            max_values_for_n_minutes: 1,
        };
        sensor.update();
        assert!(sensor.value().is_some());
        thread::sleep(Duration::from_secs(1));
        sensor.shrink_values();
        assert_eq!(sensor.value(), None);
    }
}
