use messzelle::{BoxedMesszelle, MesszellenList, RaGasCO, RaGasNO2};
use sensor::Sensor;
use std::fmt;
use std::sync::{Arc, Mutex};


/// RA-GAS GmbH CO/ NO₂ Kombisensor mit Modbus Interface
///
/// Kombisensor für Kohlenmonoxid (CO) und Stickstoffdioxid (NO₂) mit Modbus Interface.
/// Diese Kombigeräte mit 2 Messzellen werden über ein Modbus RTU BUS abgefragt.
#[derive(Debug)]
pub struct RaGasCONO2Mod {
    messzellen: MesszellenList,
}

impl RaGasCONO2Mod {
    /// Standardmäßig wird der Sensor mit CO und NO₂ Messzelle erzeugt.
    pub fn new() -> Self {
        Default::default()
    }

    /// Erzeugt einen Sensor nur mit CO Messzelle
    pub fn new_co() -> Self {
        let co_messzelle = RaGasCO::new();

        RaGasCONO2Mod {
            messzellen: vec![
                Arc::new(Mutex::new(Box::new(co_messzelle))),
            ],
        }
    }

    /// Erzeugt einen Sensor nur mit NO₂ Messzelle
    pub fn new_no2() -> Self {
        let no2_messzelle = RaGasNO2::new();

        RaGasCONO2Mod {
            messzellen: vec![
                Arc::new(Mutex::new(Box::new(no2_messzelle))),
            ],
        }
    }
}

impl Default for RaGasCONO2Mod {
    fn default() -> Self {
        let co_messzelle = RaGasCO::new();
        let no2_messzelle = RaGasNO2::new();

        RaGasCONO2Mod {
            messzellen: vec![
                Arc::new(Mutex::new(Box::new(co_messzelle))),
                Arc::new(Mutex::new(Box::new(no2_messzelle))),
            ],
        }
    }
}

impl fmt::Display for RaGasCONO2Mod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RA-GAS CO/NO2 Kombisensor mit Modbus Interface")
    }
}

impl Sensor for RaGasCONO2Mod {
    // Update Sensor Platine via BUS
    fn update(&self) {
        println!("\nUpdate Sensor: '{}'", &self);
        let messzellen = &self.messzellen.clone();
        for messzelle in messzellen {
            if let Ok(mut messzelle) = messzelle.lock() {
                messzelle.update()
            }
        }
    }
    fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>> {
        self.messzellen.get(num)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let sensor = RaGasCONO2Mod::new();
        assert_eq!(sensor.messzellen.len(), 2);
    }

    #[test]
    fn create_co() {
        let sensor = RaGasCONO2Mod::new_co();
        assert_eq!(sensor.messzellen.len(), 1);
    }

    #[test]
    fn create_no2() {
        let sensor = RaGasCONO2Mod::new_no2();
        assert_eq!(sensor.messzellen.len(), 1);
    }
}
