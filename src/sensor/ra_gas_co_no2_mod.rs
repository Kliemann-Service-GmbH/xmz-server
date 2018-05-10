use prelude::*;



/// RA-GAS GmbH CO/ NO₂ Kombisensor mit Modbus Interface
///
/// Kombisensor für Kohlenmonoxid (CO) und Stickstoffdioxid (NO₂) mit Modbus Interface.
/// Diese Kombigeräte mit 2 Messzellen werden über ein Modbus RTU BUS abgefragt.
#[derive(Debug)]
pub struct RaGasCONO2Mod {
    /// Sensor ID
    pub id: u32,
    /// Sensor Type
    pub sensor_type: SensorType,
    /// Liste der Messzellen die vom Sensor Ausgelesen werden können.
    pub messzellen: MesszelleList,
}

impl RaGasCONO2Mod {
    /// Standardmäßig wird der Sensor mit CO und NO₂ Messzelle erzeugt.
    pub fn new() -> Self {
        Default::default()
    }

    /// Erzeugt einen Sensor nur mit CO Messzelle
    pub fn new_co() -> Self {
        let co_messzelle = RaGasCOMod::new();

        let messzellen: MesszelleList = vec![
            Arc::new(Mutex::new(Box::new(co_messzelle))),
        ];

        RaGasCONO2Mod {
            messzellen: messzellen,
            ..Default::default()
        }
    }

    /// Erzeugt einen Sensor nur mit NO₂ Messzelle
    pub fn new_no2() -> Self {
        let no2_messzelle = RaGasNO2Mod::new();

        let messzellen: MesszelleList = vec![
            Arc::new(Mutex::new(Box::new(no2_messzelle))),
        ];

        RaGasCONO2Mod {
            messzellen: messzellen,
            ..Default::default()
        }
    }
}

/// Standardmäßig ist ein Kombisenor mit einer NO2 und einer CO Messzelle betückt.
impl Default for RaGasCONO2Mod {
    fn default() -> Self {
        let no2_messzelle = RaGasNO2Mod::new();
        let co_messzelle = RaGasCOMod::new();

        let messzellen: MesszelleList = vec![
            Arc::new(Mutex::new(Box::new(no2_messzelle))),
            Arc::new(Mutex::new(Box::new(co_messzelle))),
        ];

        RaGasCONO2Mod {
            id: 0,
            sensor_type: SensorType::RaGasCONO2Mod,
            messzellen: messzellen,
        }
    }
}

impl fmt::Display for RaGasCONO2Mod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RA-GAS CO/NO2 Kombisensor mit Modbus Interface")
    }
}

/// Implementation des Sensor Traits
///
impl Sensor for RaGasCONO2Mod {
    // Update Sensor Platine via BUS
    fn update(&self) {
        debug!("Update Sensor: '{}'", &self);
        let messzellen = &self.messzellen.clone();
        for messzelle in messzellen {
            if let Ok(mut messzelle) = messzelle.lock() {
                messzelle.update()
            }
        }
        thread::sleep(Duration::from_secs(1));
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_sensor_type(&self) -> SensorType {
        self.sensor_type.clone()
    }

    fn get_messzellen(&self) -> Vec<Arc<Mutex<BoxedMesszelle>>> {
        self.messzellen.clone()
    }

    fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>> {
        let messzelle = self.messzellen.get(num);
        messzelle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let sensor = RaGasCONO2Mod::new();
        assert_eq!(sensor.messzellen.len(), 2);
    }

    #[test]
    fn new_co() {
        let sensor = RaGasCONO2Mod::new_co();
        assert_eq!(sensor.messzellen.len(), 1);
    }

    #[test]
    fn new_no2() {
        let sensor = RaGasCONO2Mod::new_no2();
        assert_eq!(sensor.messzellen.len(), 1);
    }

    #[test]
    #[ignore]
    fn update() {
        let sensor = RaGasCONO2Mod::new_no2();
        assert_eq!(sensor.messzellen.len(), 1);
    }

    #[test]
    #[ignore]
    fn get_messzelle() {
        let sensor = RaGasCONO2Mod::new_no2();
        assert_eq!(sensor.messzellen.len(), 1);
    }
}
