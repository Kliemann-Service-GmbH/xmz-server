
#[macro_use] extern crate configure;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate xmz_server;

use configure::Configure;
use xmz_server::prelude::*;


fn build_server(cfg: &Config) -> Result<Server, ServerError> {
    let server = if ServerBuilder::runtime_info_available(&cfg) {
        ServerBuilder::from_runtime_info(&cfg)?
    } else if ServerBuilder::config_file_available(&cfg) {
        ServerBuilder::from_config_file(&cfg)?
    } else {
        // Ansonnsten Server mit `Default::default()` Werten
        Server::new()
    };

    Ok(server)
}

fn run() -> Result<(), ServerError> {
    println!("xmz-server: {}", env!("CARGO_PKG_VERSION"));

    let cfg = Config::generate()?;
    println!("Benutze Config: {:?}", &cfg);

    let server = build_server(&cfg)?;

    server.start()?;

    Ok(())
}

fn main() {
    env_logger::init();
    // Configure trait initalisation: https://boats.gitlab.io/blog/post/2018-01-18-configure/
    use_default_config!();

    if let Err(e) = run() {
        println!("\nError: {}", e);
        println!("Additional Information: {}", e.description());
    }
}
