use sensor::BoxedSensor;
use std::sync::{Arc, Mutex};

mod server;

pub use self::server::Server;

/// Liste der Sensoren
///
/// Diese Liste ist ein `Vector` von shared (`Arc`), mutablen (`Mutex`)
/// Sensor Trait Objekten (`BoxedSensor`).
pub type SensorsList = Vec<Arc<Mutex<BoxedSensor>>>;
