extern crate xmz_server;

use xmz_server::{Server, ServerError, Settings};

fn run() -> Result<(), ServerError> {
    println!("xmz-server: {}", env!("CARGO_PKG_VERSION"));

    let settings = Settings::new()?;
    let server = Server::new(&settings);

    server.start()?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("\nError: {}", e);
    }
}
