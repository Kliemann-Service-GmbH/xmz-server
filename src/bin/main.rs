
#[macro_use] extern crate configure;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate xmz_server;

use configure::Configure;
use xmz_server::prelude::*;


fn run() -> Result<(), ServerError> {
    println!("xmz-server: {}", env!("CARGO_PKG_VERSION"));

    let cfg = Config::generate()?;
    println!("Benutze Config: {:?}", cfg);

    let server = Server::new();

    server.start()?;

    Ok(())
}

fn main() {
    env_logger::init();
    // Configure trait initalisation: https://boats.gitlab.io/blog/post/2018-01-18-configure/
    use_default_config!();

    if let Err(e) = run() {
        println!("\nError: {}", e);
    }
}
