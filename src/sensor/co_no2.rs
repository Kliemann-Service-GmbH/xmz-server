use sensor::Sensor;
use std::time::SystemTime;

/// RA-GAS GmbH CO/ NO₂ Kombisensor mit Modbus Interface
///
/// Kombisensor für Kohlenmonoxid (CO) und Stickstoffdioxid (NO₂) mit Modbus Interface.
/// Diese Kombigeräte mit 2 Messzellen werden über ein Modbus RTU BUS abgefragt.
pub struct CONO2 {
    values: Vec<(SystemTime, f64)>,
}

impl CONO2 {
    pub fn new() -> Self {
        CONO2 {
            values: vec![],
        }
    }
}

impl Sensor for CONO2 {
    fn value(&self) -> Option<&(SystemTime, f64)> {
        self.values.last()
    }

    fn average(&self, min: u32) -> Option<f64> {
        None
    }

    fn update(&mut self) {
        self.values.push((SystemTime::now(), 1.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let sensor2 = CONO2::new();
    }
}
