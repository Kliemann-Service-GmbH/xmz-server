extern crate xmz_server;

use std::sync::{Arc, Mutex};
use xmz_server::{Server, Settings};
use xmz_server::sensor::RaGasCONO2Mod;


#[test]
fn sensor_get_messzelle() {
    let settings = Settings::new().unwrap();
    let mut server = Server::new(&settings);
    let sensor = RaGasCONO2Mod::new();
    assert!(server.get_sensor(0).is_none());
    server.add_sensor(Arc::new(Mutex::new(Box::new(sensor))));
    assert!(server.get_sensor(0).is_some());
}
