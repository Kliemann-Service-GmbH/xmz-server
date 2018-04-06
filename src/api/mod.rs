mod server;
mod sensor;

use rocket;
use rocket::Rocket;
use rocket::http::RawStr;
use rocket::State;
use ::api::server::Server as ServerExtern;
use ::server::Server as ServerIntern;





/// Konstruiert die Rocket Instanz
///
/// Diese Funktion wird auch in den Unti Tests der Api Untermodule aufgerufen
fn rocket(server: ServerExtern) -> Rocket {
    rocket::ignite()
        .mount("/", routes![server::index])
        .mount("/sensors", routes![sensor::index])
        .manage(server)
}


/// Startet die json web api
///
pub fn launch(server: ServerIntern) {
    rocket(server.into()).launch();
}
