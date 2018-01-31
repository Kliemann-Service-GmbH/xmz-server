use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use sensor::Sensor;
use zone::Zone;
use server::error::ServerError;

#[derive(Debug)]
pub struct Server {
    sensors: Vec<Arc<Mutex<Sensor>>>,
    zones: Vec<Arc<Mutex<Zone>>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            sensors: vec![],
            zones: vec![],
        }
    }
    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(Arc::new(Mutex::new(sensor)));
    }
    pub fn add_zone(&mut self, zone: Zone) {
        self.zones.push(Arc::new(Mutex::new(zone)));
    }

    pub fn run(&self) {
        println!("Start sensor update thread ...");
        self.update_sensors();
        println!("Start check zones thread ...");
        self.check_zones();

        // Pause
        thread::sleep(Duration::from_millis(2000));
    }

    fn update_sensors(&self) -> Result<(), ServerError> {
        let sensors = self.sensors.clone();
        thread::spawn(move|| {
            loop {
                for sensor in sensors.clone() {
                    if let Ok(mut sensor) = sensor.lock() {
                        // println!("Update Sensor ({}): {} ppm", sensor.id, sensor.value);
                        sensor.update();
                    }
                }
            }
        });

        Ok(())
    }

    fn check_zones(&self) -> Result<(), ServerError> {
        let zones = self.zones.clone();
        thread::spawn(move || {
            loop {
                for zone in zones.clone() {
                    if let Ok(zone) = zone.lock() {
                        zone.check();
                    }
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        Ok(())
    }

    pub fn link_sensor_zone(&self, sensor_num: usize, zone_num: usize) -> Result<(), ServerError> {
        let sensor = self.sensors.get(sensor_num).unwrap();
        let zone = self.zones.get(zone_num).unwrap();

        if let Ok(mut zone) = zone.lock() {
            zone.sensors.push(sensor.clone());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_create() {
        let server = Server::new();

        assert_eq!(server.sensors.len(), 0);
        assert_eq!(server.zones.len(), 0);
    }

    #[test]
    fn server_add_sensor() {
        let mut server = Server::new();
        let sensor1 = Sensor::new(1);
        assert_eq!(server.sensors.len(), 0);

        server.add_sensor(sensor1);
        assert_eq!(server.sensors.len(), 1);
    }

    #[test]
    fn server_add_zone() {
        let mut server = Server::new();
        let zone1 = Zone::new();
        assert_eq!(server.zones.len(), 0);

        server.add_zone(zone1);
        assert_eq!(server.zones.len(), 1);
    }

    #[test]
    fn server_link_zone() {
        let mut server = Server::new();
        let sensor1 = Sensor::new(1);
        let sensor2 = Sensor::new(2);
        let zone1 = Zone::new();
        server.add_sensor(sensor1);
        server.add_sensor(sensor2);
        server.add_zone(zone1);

        server.link_sensor_zone(0, 0);
        //
        // server.link_sensor_zone(1, 0);
        // server.link_sensor_zone(0, 1);
    }

    #[test]
    fn server_update_sensors() {
        // let server = Server::new();
        //
        // assert!(server.update_sensors().is_ok());
    }
}
