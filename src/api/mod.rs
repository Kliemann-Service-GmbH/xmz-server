mod messzelle;
mod sensor;
mod server;

use api;
use rocket;
use rocket::Rocket;


/// Konstruiert die Rocket Instanz
///
/// Diese Funktion wird auch in den Unti Tests der Api Untermodule aufgerufen
fn rocket(server: api::server::Server) -> Rocket {
    rocket::ignite()
        .mount("/", routes![server::index])
        .mount("/sensors", routes![sensor::index])
        .manage(server)
}

/// Startet die json web api
///
/// Diese statische Funktion `launch` erwartet eine Server Instanz, diese wird dann bei der
/// Übergabe an die `rocket` Funktion in eine `api::server::Server` Instanz konvertiert.
pub fn launch(server: ::server::Server) {
    rocket(server.into()).launch();
}
