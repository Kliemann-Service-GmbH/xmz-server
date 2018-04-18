
#[macro_use] extern crate log;
extern crate env_logger;
extern crate xmz_server;

use xmz_server::{Server, ServerError};


fn run() -> Result<(), ServerError> {
    println!("xmz-server: {}", env!("CARGO_PKG_VERSION"));
    let server = Server::new();

    server.start()?;

    Ok(())
}

fn main() {
    env_logger::init();
    
    if let Err(e) = run() {
        println!("\nError: {}", e);
    }
}
