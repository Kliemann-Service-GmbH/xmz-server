use std::thread;


trait Sensor {
    fn value(&self) -> f32;
    fn average(&self, min: u32) -> f32;
    fn update(&mut self);
}


struct CONO2 {
    values: Vec<f32>,
}
impl CONO2 {
    fn new() -> Self {
        CONO2 {
            values: vec![],
        }
    }
}

struct Analog420 {
    values: Vec<f32>,
}
impl Analog420 {
    fn new() -> Self {
        Analog420 {
            values: vec![],
        }
    }
}

impl Sensor for CONO2 {
    fn value(&self) -> f32 {
        match self.values.last() {
            None => 0.0,
            Some(value) => *value,
        }
    }

    fn average(&self, min: u32) -> f32 {
        0.0
    }

    fn update(&mut self) {
        self.values.push(1.0);
    }
}
impl Sensor for Analog420 {
    fn value(&self) -> f32 {
        match self.values.last() {
            None => 0.0,
            Some(value) => *value,
        }
    }

    fn average(&self, min: u32) -> f32 {
        0.0
    }

    fn update(&mut self) {
        self.values.push(1.0);
    }
}


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
        assert_eq!(0.0, server.sensors.last().unwrap().value());
        server.update_sensors();

        assert_eq!(1.0, server.sensors.last().unwrap().value());
    }

    #[test]
    fn create_cono2() {
        let sensor1 = CONO2::new();
    }
}
