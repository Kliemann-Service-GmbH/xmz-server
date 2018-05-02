extern crate xmz_server;

use xmz_server::prelude::*;


#[test]
fn sensor_get_messzelle() {
    let sensor = RaGasCONO2Mod::new();
    assert!(sensor.get_messzelle(0).is_some());
}
