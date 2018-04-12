extern crate xmz_server;

use std::sync::{Arc, Mutex};
use xmz_server::prelude::*;
use xmz_server::sensor::RaGasCONO2Mod;


#[test]
fn sensor_get_messzelle() {
    let mut server = Server::new();
    let sensor = RaGasCONO2Mod::new();
    assert!(server.get_sensor(0).is_none());

    server.add_sensor(Arc::new(Mutex::new(Box::new(sensor))));
    assert!(server.get_sensor(0).is_some());
}
