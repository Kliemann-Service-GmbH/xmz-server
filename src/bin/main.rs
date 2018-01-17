extern crate xmz_server;

use xmz_server::Error;
use xmz_server::Settings;


fn run() -> Result<(), Error> {
    println!("xmz-server: {}", env!("CARGO_PKG_VERSION"));

    let settings = Settings::new();
    println!("{:?}", settings);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
    }
}
