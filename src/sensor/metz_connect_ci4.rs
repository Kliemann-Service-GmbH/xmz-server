use messzelle::{BoxedMesszelle, MesszellenList, MetzConnectCI4Analog420};
use sensor::Sensor;
use std::fmt;
use std::sync::{Arc, Mutex};


#[derive(Debug)]
pub struct MetzConnectCI4 {
    messzellen: MesszellenList,
}

impl MetzConnectCI4 {
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

impl Sensor for MetzConnectCI4 {
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
