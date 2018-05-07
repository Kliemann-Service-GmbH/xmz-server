extern crate xmz_server;

use xmz_server::prelude::*;

fn main() {
    let sensor = RaGasCONO2Mod::new();

    println!("{:?}", sensor.get_messzelle(0));
}
