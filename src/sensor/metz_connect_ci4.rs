use messzelle::{BoxedMesszelle, MesszellenList, MetzConnectCI4Analog420};
use sensor::{Sensor, SensorType};
use std::fmt;
use std::sync::{Arc, Mutex};


// FIXME: pub's checken
/// Metz Connect CI4 Modul
///
/// Das Metz Connect CI4 Modul für die Hutschiene verfügt über ein Modbus Interface. An das Modul
/// können 4 analog Sensoren angeschlossen werden. Für die 4-20mA Messtechnik wird die Messzelle
/// `MetzConnectCI4Analog420` verwendet.
///
#[derive(Debug)]
pub struct MetzConnectCI4 {
    /// Sensor ID
    pub id: u32,
    /// Sensor Type
    pub sensor_type: SensorType,
    /// Liste der Messzellen die vom Sensor Ausgelesen werden können.
    pub messzellen: MesszellenList,
}

impl MetzConnectCI4 {
    /// Erzeut einen neuen Sensor mit view 4-20mA Messzellen.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for MetzConnectCI4 {
    fn default() -> Self {
        let messzelle1 = MetzConnectCI4Analog420::new();
        let messzelle2 = MetzConnectCI4Analog420::new();
        let messzelle3 = MetzConnectCI4Analog420::new();
        let messzelle4 = MetzConnectCI4Analog420::new();

        MetzConnectCI4 {
            id: 0,
            sensor_type: SensorType::MetzConnectCI4,
            messzellen: vec![
                Arc::new(Mutex::new(Box::new(messzelle1))),
                Arc::new(Mutex::new(Box::new(messzelle2))),
                Arc::new(Mutex::new(Box::new(messzelle3))),
                Arc::new(Mutex::new(Box::new(messzelle4))),
            ],
        }
    }
}

impl fmt::Display for MetzConnectCI4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Metz Connect CI4 Modul")
    }
}

/// Implementation des Sensor Traits
///
impl Sensor for MetzConnectCI4 {
    // Update Sensor Platine via BUS
    fn update(&self) {
        debug!("Update Sensor: '{}'", &self);
        let messzellen = &self.messzellen.clone();
        for messzelle in messzellen {
            if let Ok(mut messzelle) = messzelle.lock() {
                messzelle.update()
            }
        }
        ::std::thread::sleep(::std::time::Duration::from_secs(1));
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_sensor_type(&self) -> SensorType {
        self.sensor_type.clone()
    }

    fn get_messzellen(&self) -> &Vec<Arc<Mutex<BoxedMesszelle>>> {
        &self.messzellen
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
        let sensor = MetzConnectCI4::new();
        assert_eq!(sensor.messzellen.len(), 4);
    }

    // TODO: Siehe RaGasCONO2Mod für `new_420_with_len(len: usize)` Funktion

    #[test]
    #[ignore]
    fn update() {
        let sensor = MetzConnectCI4::new();
        assert_eq!(sensor.messzellen.len(), 4);
    }

    #[test]
    #[ignore]
    fn get_messzelle() {
        let sensor = MetzConnectCI4::new();
        assert_eq!(sensor.messzellen.len(), 4);
    }
}
