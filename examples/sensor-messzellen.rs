#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;
use std::sync::{Arc, Mutex};

    type BoxedMesszelle = Box<Messzelle + Send + 'static>;
    type MesszellenList = Vec<Arc<Mutex<BoxedMesszelle>>>;
    type BoxedSensor = Box<Sensor + Send + 'static>;
    type SensorsList = Vec<Arc<Mutex<BoxedSensor>>>;
    type MesszellenRefList<'a> = Vec<&'a Messzelle>;


    // Messzellen
    trait Messzelle: fmt::Debug {
        fn value(&self) -> f64;
        fn average(&self, minutes: u32) -> f64;
        fn update(&self);
    }

    #[derive(Debug)]
    struct RaGasCO {
        value: f64,
    }
    impl RaGasCO {
        fn new() -> Self {
            RaGasCO {
                value: 0.0,
            }
        }
    }
    impl fmt::Display for RaGasCO {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "CO")
        }
    }
    impl Messzelle for RaGasCO {
        fn value(&self) -> f64 {
            0.0
        }

        fn average(&self, minutes: u32) -> f64 {
            0.0
        }

        fn update(&self) {
            println!("|-- Update Messzelle: '{}'", &self);
        }
    }
    #[derive(Debug)]
    struct RaGasNO2 {
        value: f64,
    }
    impl RaGasNO2 {
        fn new() -> Self {
            RaGasNO2 {
                value: 0.0,
            }
        }
    }
    impl fmt::Display for RaGasNO2 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "NO2")
        }
    }
    impl Messzelle for RaGasNO2 {
        fn value(&self) -> f64 {
            0.0
        }

        fn average(&self, minutes: u32) -> f64 {
            0.0
        }

        fn update(&self) {
            println!("|-- Update Messzelle: '{}'", &self);
        }
    }
#[derive(Debug)]
struct Analog420MetzCI4 {
    value: f64,
}
impl Analog420MetzCI4 {
    fn new() -> Self {
        Analog420MetzCI4 {
            value: 0.0,
        }
    }
}
impl fmt::Display for Analog420MetzCI4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Analog 4-20mA an Metz Connect CI4 Modul")
    }
}
impl Messzelle for Analog420MetzCI4 {
    fn value(&self) -> f64 {
        0.0
    }

    fn average(&self, minutes: u32) -> f64 {
        0.0
    }

    fn update(&self) {
        println!("|-- Update Messzelle: '{}'", &self);
    }
}

    // Sensoren
    trait Sensor: fmt::Debug {
        fn update(&self);
        fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>>;
    }
    #[derive(Debug)]
    struct MetzCI4 {
        messzellen: MesszellenList,
    }
    impl MetzCI4 {
        fn new() -> Self {
            Default::default()
        }
    }
    impl Default for MetzCI4 {
        fn default() -> Self {
            let messzelle1 = Analog420MetzCI4::new();
            let messzelle2 = Analog420MetzCI4::new();
            let messzelle3 = Analog420MetzCI4::new();
            let messzelle4 = Analog420MetzCI4::new();

            MetzCI4 {
                messzellen: vec![
                    Arc::new(Mutex::new(Box::new(messzelle1))),
                    Arc::new(Mutex::new(Box::new(messzelle2))),
                    Arc::new(Mutex::new(Box::new(messzelle3))),
                    Arc::new(Mutex::new(Box::new(messzelle4))),
                ],
            }
        }
    }
    impl fmt::Display for MetzCI4 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Metz Connect CI4 Modul")
        }
    }
    impl Sensor for MetzCI4 {
        // Update Sensor Platine via BUS
        fn update(&self) {
            println!("\nUpdate Sensor: '{}'", &self);
            let messzellen = &self.messzellen.clone();
            for messzelle in messzellen {
                if let Ok(messzelle) = messzelle.lock() {
                    messzelle.update()
                }
            }
        }
        fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>> {
            self.messzellen.get(num)
        }
    }

    #[derive(Debug)]
    struct RaGasCONO2Mod {
        messzellen: MesszellenList,
    }
    impl RaGasCONO2Mod {
        fn new() -> Self {
            Default::default()
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
                if let Ok(messzelle) = messzelle.lock() {
                    messzelle.update()
                }
            }
        }
        fn get_messzelle(&self, num: usize) -> Option<&Arc<Mutex<BoxedMesszelle>>> {
            self.messzellen.get(num)
        }
    }

// Zone
struct Zone<'a> {
    messzellen: MesszellenRefList<'a>,
}
impl<'a> Zone<'a> {
    fn new() -> Self {
        Zone {
            messzellen: vec![],
        }
    }
}

// Server
#[derive(Debug)]
struct Server {
    sensors: SensorsList,
}
impl Server {
    fn new() -> Self {
        Server {
            sensors: vec![],
        }
    }
    fn get_sensor(&self, num: usize) -> Option<&Arc<Mutex<BoxedSensor>>> {
        self.sensors.get(num)
    }
    fn add_sensor(&mut self, sensor: Arc<Mutex<BoxedSensor>>) {
        self.sensors.push(sensor);
    }
    fn update_sensors(&self) {
        let sensors = &self.sensors.clone();
        for sensor in sensors {
            if let Ok(sensor) = sensor.lock() {
                sensor.update();
            }
        }
    }
}


fn main() {
    let mut server = Server::new();
    let zone = Zone::new();
    let sensor1 = RaGasCONO2Mod::new();
    let sensor2 = MetzCI4::new();

    server.add_sensor(Arc::new(Mutex::new(Box::new(sensor1))));
    server.add_sensor(Arc::new(Mutex::new(Box::new(sensor2))));

    server.update_sensors();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensor_get_messzelle() {
        let mut server = Server::new();
        let sensor = RaGasCONO2Mod::new();
        assert!(server.get_sensor(0).is_none());
        server.add_sensor(Arc::new(Mutex::new(Box::new(sensor))));
        assert!(server.get_sensor(0).is_some());
    }

    #[test]
    fn ragas_co_create() {
        let messzelle = RaGasCO::new();
        assert_eq!(messzelle.value(), 0.0);
    }
    #[test]
    fn ragas_co_value() {
        let messzelle = RaGasCO::new();
        assert_eq!(messzelle.value(), 0.0);
    }
    #[test]
    fn ragas_co_average() {
        let messzelle = RaGasCO::new();
        assert_eq!(messzelle.average(15), 0.0);
    }
    #[test]
    fn ragas_no2_create() {
        let messzelle = RaGasNO2::new();
        assert_eq!(messzelle.value(), 0.0);
    }
    #[test]
    fn ragas_no2_value() {
        let messzelle = RaGasNO2::new();
        assert_eq!(messzelle.value(), 0.0);
    }
    #[test]
    fn ragas_no2_average() {
        let messzelle = RaGasNO2::new();
        assert_eq!(messzelle.average(15), 0.0);
    }
    #[test]
    fn metz_ci4_create() {
        let messzelle = Analog420MetzCI4::new();
        assert_eq!(messzelle.value(), 0.0);
    }
    #[test]
    fn metz_ci4_value() {
        let messzelle = Analog420MetzCI4::new();
        assert_eq!(messzelle.value(), 0.0);
    }
    #[test]
    fn metz_ci4_average() {
        let messzelle = Analog420MetzCI4::new();
        assert_eq!(messzelle.average(15), 0.0);
    }

    #[test]
    fn zone_create() {
        let zone = Zone::new();
        assert_eq!(zone.messzellen.len(), 0);
    }
    #[test]
    fn zone_add_messzelle() {
        let mut server = Server::new();
        let sensor = RaGasCONO2Mod::new();
        server.add_sensor(Arc::new(Mutex::new(Box::new(sensor))));

        let mut zone = Zone::new();
        assert_eq!(zone.messzellen.len(), 0);

    }
}
