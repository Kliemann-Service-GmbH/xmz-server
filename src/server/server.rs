use std::thread;
use server::sensor::Sensor;


pub struct Server {
    sensors: Vec<Box<Sensor>>,
}

impl Server {
    fn new() -> Self {
        Server {
            sensors: vec![],
        }
    }

    fn add_sensor(&mut self, sensor: Box<Sensor>) {
        self.sensors.push(sensor);
    }

    fn update_sensors(&mut self) {
        for sensor in &mut self.sensors {
            sensor.update();
        }
    }

    fn start(self) {
        thread::spawn(move || {

        });
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use server::sensor::{Analog420, CONO2};

    #[test]
    fn create_server() {
        let server = Server::new();
    }

    #[test]
    fn add_sensor() {
        let mut server = Server::new();
        let sensor1 = CONO2::new();

        server.add_sensor(Box::new(sensor1));
    }

    #[test]
    fn add_two_different_sensors() {
        let mut server = Server::new();
        let sensor1 = CONO2::new();
        let sensor2 = Analog420::new();

        server.add_sensor(Box::new(sensor1));
        server.add_sensor(Box::new(sensor2));
    }

    #[test]
    fn update_sensors() {
        let mut server = Server::new();
        let sensor1 = CONO2::new();
        server.add_sensor(Box::new(sensor1));
        assert_eq!(None, server.sensors.last().unwrap().value());
        server.update_sensors();
        //
        // assert_eq!(1.0, server.sensors.last().unwrap().value());
    }

    #[test]
    fn create_sensor_cono2() {
        let sensor1 = CONO2::new();
    }

    #[test]
    fn create_sensor_analog420() {
        let sensor2 = Analog420::new();
    }
}
