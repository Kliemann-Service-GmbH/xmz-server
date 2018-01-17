use rand::distributions::{IndependentSample, Range};
use server::sensor::Sensor;
use std::time::SystemTime;


/// Sensor an einer 4-20mA Meßschleife.
///
/// Diese Sensoren sind an einer Meßschleife angeschlossen. Werte unter 4mA signalisieren
/// Kabelbruch. 4mA Werte stehen für den minimalen Messwert des Sensors, 20mA für den max.
/// Messwert des Sensors.
pub struct Analog420 {
    values: Vec<(SystemTime, f64)>,
}

impl Analog420 {
    pub fn new() -> Self {
        Analog420 {
            values: vec![],
        }
    }
}

impl Sensor for Analog420 {
    fn value(&self) -> Option<&(SystemTime, f64)> {
        self.values.last()
    }

    fn average(&self, min: u32) -> Option<f64> {
        None
    }

    /// Aktuellen Sensorwert ermitteln und speichern.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::server::sensor::{Analog420, Sensor};
    ///
    /// let mut sensor = Analog420::new();
    /// assert_eq!(sensor.value(), 0.0);
    /// sensor.update();
    ///
    /// assert!(sensor.value() >= 0.0);
    /// assert!(sensor.value() <= 100.0);
    /// ```
    fn update(&mut self) {
        let between = Range::new(0, 100);
        let mut rng = ::rand::thread_rng();
        self.values.push((SystemTime::now(), between.ind_sample(&mut rng) as f64 / 10.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let sensor2 = Analog420::new();
    }
}
