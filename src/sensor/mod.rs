use std::sync::{Arc, Mutex};

mod metz_connect_ci4;
mod ra_gas_co_no2_mod;
mod sensor;

pub use self::metz_connect_ci4::MetzConnectCI4;
pub use self::ra_gas_co_no2_mod::RaGasCONO2Mod;
pub use self::sensor::Sensor;

pub type BoxedSensor = Box<Sensor + Send + 'static>;
pub type SensorsList = Vec<Arc<Mutex<BoxedSensor>>>;
