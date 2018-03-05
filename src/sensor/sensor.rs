//! Trait der eine Sensorplatine beschreibt
//!
use std::fmt;
use std::sync::{Arc, Mutex};

use ::messzelle::BoxedMesszelle;

pub type BoxedSensor = Box<Sensor + Send + 'static>;
pub type SensorsList = Vec<Arc<Mutex<BoxedSensor>>>;


// Sensoren
pub trait Sensor: fmt::Debug {
    fn update(&self);
    fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>>;
}

// #[derive(Debug)]
// struct MetzCI4 {
//     messzellen: MesszellenList,
// }
// impl MetzCI4 {
//     fn new() -> Self {
//         Default::default()
//     }
// }
// impl Default for MetzCI4 {
//     fn default() -> Self {
//         let messzelle1 = Analog420MetzCI4::new();
//         let messzelle2 = Analog420MetzCI4::new();
//         let messzelle3 = Analog420MetzCI4::new();
//         let messzelle4 = Analog420MetzCI4::new();
//
//         MetzCI4 {
//             messzellen: vec![
//                 Arc::new(Mutex::new(Box::new(messzelle1))),
//                 Arc::new(Mutex::new(Box::new(messzelle2))),
//                 Arc::new(Mutex::new(Box::new(messzelle3))),
//                 Arc::new(Mutex::new(Box::new(messzelle4))),
//             ],
//         }
//     }
// }
// impl fmt::Display for MetzCI4 {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Metz Connect CI4 Modul")
//     }
// }
// impl Sensor for MetzCI4 {
//     // Update Sensor Platine via BUS
//     fn update(&self) {
//         println!("\nUpdate Sensor: '{}'", &self);
//         let messzellen = &self.messzellen.clone();
//         for messzelle in messzellen {
//             if let Ok(messzelle) = messzelle.lock() {
//                 messzelle.update()
//             }
//         }
//     }
//     fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>> {
//         self.messzellen.get(num)
//     }
// }
//
// #[derive(Debug)]
// struct RaGasCONO2Mod {
//     messzellen: MesszellenList,
// }
// impl RaGasCONO2Mod {
//     fn new() -> Self {
//         Default::default()
//     }
// }
// impl Default for RaGasCONO2Mod {
//     fn default() -> Self {
//         let co_messzelle = RaGasCO::new();
//         let no2_messzelle = RaGasNO2::new();
//
//         RaGasCONO2Mod {
//             messzellen: vec![
//                 Arc::new(Mutex::new(Box::new(co_messzelle))),
//                 Arc::new(Mutex::new(Box::new(no2_messzelle))),
//             ],
//         }
//     }
// }
// impl fmt::Display for RaGasCONO2Mod {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "RA-GAS CO/NO2 Kombisensor mit Modbus Interface")
//     }
// }
// impl Sensor for RaGasCONO2Mod {
//     // Update Sensor Platine via BUS
//     fn update(&self) {
//         println!("\nUpdate Sensor: '{}'", &self);
//         let messzellen = &self.messzellen.clone();
//         for messzelle in messzellen {
//             if let Ok(messzelle) = messzelle.lock() {
//                 messzelle.update()
//             }
//         }
//     }
//     fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>> {
//         self.messzellen.get(num)
//     }
// }
//
