extern crate xmz_server;

use xmz_server::prelude::*;


fn main() {
    let server = Server::new();

    println!("Server Sensoren: {}", server.get_sensors().len());
}
