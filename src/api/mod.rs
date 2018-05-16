//! Web/ Json Anbindung
//!
//! Dieses Modul ist ähnlich wie die Module `configuration` und `runtime_info` aufgebaut.
//! Auch hier wird der `server::Server` in eine andere Struktur gewandelt (via Rust `From` Trait)
//!

// runtime_info und api Module sind sehr ähnlich

mod messzelle;
mod output;
mod sensor;
mod server;

use rocket;
use rocket::Rocket;


/// Startet die json web api
///
/// Diese statische Funktion `launch` erwartet eine Server Instanz, diese wird dann bei der
/// Übergabe an die `rocket` Funktion in eine `api::server::Server` Instanz konvertiert.
pub fn launch(server: ::server::Server) {
    rocket(server.clone()).launch();
}

/// Helper, Konstruiert die Rocket Instanz
///
/// Die rocket Routing Informationen werden in dieser Funktion konfiguriert.
///
/// # Tipp
///
/// Normalerweise wird die Web/ Json Api via der public `apt::launch()` Funktion gestartet.
/// Die Unit Tests benötiten jedoch die `Rocket` Instanz für die Tests. In Unit Tests kann diese
/// Funktion direkt verwendet werden.
///
fn rocket(server: ::server::Server) -> Rocket {
    rocket::ignite()
    .mount("/", routes![server::index_txt])
    .mount("/api/", routes![server::index])
    .mount("/api/sensors", routes![sensor::index])
    .mount("/api/sensor", routes![sensor::get, sensor::get_messzellen, sensor::get_messzelle])
    // .mount("/sensor/<id>", routes![sensor::index])
    .manage(server.clone())
}
