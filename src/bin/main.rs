#[macro_use]
extern crate configure;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate xmz_server;

use configure::Configure;
use xmz_server::prelude::*;


/// Konstruiert eine Server Instanz
///
///
fn build_server(cfg: &Config) -> Result<Server, ServerError> {
    // Wenn die konfiguriert Laufzeitinformation gefunden wurde ...
    let server = if cfg.runtime_info_available() {
        debug!(
            "Pfad Laufzeit Information: `{:?}` in Environment Variable gefunden",
            &cfg.runtime_info_path
        );
        info!("restauriere Server aus Laufzeitumgebung");
        let server = runtime_info::Server::from_runtime_info(&cfg)?;
        // Konvertiere Runtime Server in richtigen Server
        server.into()
    } else if cfg.config_file_available() {
        debug!(
            "Pfad Konfigurationsdatei: `{:?}` in Environment Variable gefunden",
            &cfg.configuration_path
        );
        info!("Erstelle Server aus Konfigurationsdatei neu");
        let server = configuration::Server::from_config_file(&cfg)?;
        // Konvertiere Configuration Server in richtigen Server
        server.into()
    } else {
        // Ansonnsten Server mit `Default::default()` Werten
        warn!("Weder Laufzeit Information noch Konfigurationsdatei gefunden, verwende Default Test Server");
        Server::new()
    };

    Ok(server)
}

fn run() -> Result<(), ServerError> {
    println!("xmz-server: {}", env!("CARGO_PKG_VERSION"));

    let cfg = Config::generate()?;
    debug!("Benutze generierte Config: {:?}", &cfg);

    let server = build_server(&cfg)?;
    debug!("Erkannter Server: {:?}", server);

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

        if let Some(cause) = e.cause() {
            println!("Cause: {}", cause);
        }
    }
}
