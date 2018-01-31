
#[derive(Debug)]
#[derive(Clone)]
pub struct Sensor {
    id: usize,
    value: f64,
}

impl Sensor {
    pub fn new(id: usize) -> Self {
        Sensor {
            id,
            value: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.value += 1.0;
    }
}
