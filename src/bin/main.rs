extern crate xmz_server;

use xmz_server::{Server, ServerError};

fn run() -> Result<(), ServerError> {
    println!("xmz-server: {}", env!("CARGO_PKG_VERSION"));
    let server = Server::new();

    server.start()?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("\nError: {}", e);
    }
}
