//! Server Externe
//!
//! Dieses Modul beinhaltet die Externe Representation der Server Struktur

use rocket_contrib::Json;
use rocket::State;
use ::api::server::Server as ServerExtern;
use ::server::Server as ServerIntern;
use ::api::sensor::Sensor;


#[derive(Clone, Debug)]
#[derive(Serialize)]
pub struct Server {
    pub service_interval: u32,
    pub sensors: Vec<Sensor>,
}


impl From<ServerIntern> for Server {
    fn from(server: ServerIntern) -> Self {
        Server {
            service_interval: server.service_interval,
            sensors: vec![],
        }
    }
}


// #[get("/", rank = 2)]
// fn index(server: State<ServerExtern>) -> String {
//     format!("{:?}", server)
// }

#[get("/")]
fn index(server: State<ServerExtern>) -> Json<ServerExtern> {
    Json(server.clone())
}




#[cfg(test)]
mod test {
    use api;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn index() {
        let server = ::server::Server { service_interval: 1, sensors: vec![] };
        let client = Client::new(api::rocket(server.into())).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
