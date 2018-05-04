//! Objekt Representation für die Web/ json Anbindung
mod messzelle;
mod sensor;
mod server;

use api;
use rocket;
use rocket::Rocket;


/// Startet die json web api
///
/// Diese statische Funktion `launch` erwartet eine Server Instanz, diese wird dann bei der
/// Übergabe an die `rocket` Funktion in eine `api::server::Server` Instanz konvertiert.
pub fn launch(server: ::server::Server) {
    rocket(server.into()).launch();
}

/// Helper, Konstruiert die Rocket Instanz
///
/// Normalerweise wird die Web/ Json Api via `apt::launch()` gestartet.
/// Die Unit Tests benötiten jedoch die `Rocket` Instanz für die Tests. Dazu kann diese Funktion
/// verwendet werden.
///
/// # Examples
///
/// ```rust
/// extern crate xmz_server;
/// use xmz_server::prelude::*;
///
/// let server = Server::new();
/// ```
fn rocket(server: api::server::Server) -> Rocket {
    rocket::ignite()
    .mount("/", routes![server::index])
    .mount("/sensors", routes![sensor::index])
    .manage(server)
}
