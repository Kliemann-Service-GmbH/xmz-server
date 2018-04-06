extern crate xmz_server;

use xmz_server::prelude::*;

fn main() {
    let settings = Settings::new().unwrap();
    let server = Server::new(&settings);

    println!("Server Sensoren: {}", server.get_sensors().len());
}
