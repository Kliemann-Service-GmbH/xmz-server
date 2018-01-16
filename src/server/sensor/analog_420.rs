use server::sensor::Sensor;
use std::time::SystemTime;

pub struct Analog420 {
    values: Vec<(SystemTime, f64)>,
}

impl Analog420 {
    pub fn new() -> Self {
        Analog420 {
            values: vec![],
        }
    }
}

impl Sensor for Analog420 {
    fn value(&self) -> f64 {
        match self.values.last() {
            None => 0.0,
            Some(&(time, value)) => value,
        }
    }

    fn average(&self, min: u32) -> f64 {
        0.0
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
        let sensor2 = Analog420::new();
    }
}
