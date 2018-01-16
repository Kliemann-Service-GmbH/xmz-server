//! Trait der generellen Sensor Funktionen
//!

pub trait Sensor {
    fn value(&self) -> f64;
    fn average(&self, min: u32) -> f64;
    fn update(&mut self);
}
