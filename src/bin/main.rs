extern crate xmz_server;

use xmz_server::Settings;

fn main() {
    println!("xmz-server: {}", env!("CARGO_PKG_VERSION"));

    let settings = Settings::new();

    println!("{:?}", settings);
}
