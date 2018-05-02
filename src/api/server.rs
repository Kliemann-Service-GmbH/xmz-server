//! Server Externe
//!
//! Dieses Modul beinhaltet die Externe Representation der Server Struktur

use api::sensor::Sensor as SensorExtern;
use api::server::Server as ServerExtern;
use rocket::State;
use rocket_contrib::Json;
use server::Server as ServerIntern;

#[derive(Clone, Debug, Serialize)]
pub struct Server {
    pub service_interval: u32,
    pub sensors: Vec<SensorExtern>,
}

impl Server {
    pub fn get_sensors(&self) -> &Vec<SensorExtern> {
        &self.sensors
    }
}

impl From<ServerIntern> for Server {
    fn from(server: ServerIntern) -> Self {
        let mut sensors: Vec<SensorExtern> = Vec::new();
        for sensor in server.get_sensors() {
            if let Ok(sensor) = sensor.lock() {
                sensors.push((&*sensor).into());
            }
        }

        Server {
            service_interval: server.service_interval,
            sensors: sensors,
        }
    }
}

#[get("/")]
fn index(server: State<ServerExtern>) -> Json<ServerExtern> {
    Json(server.clone())
}

#[cfg(test)]
mod test {
    use api;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn index() {
        let server = ::server::Server::new();
        let client = Client::new(api::rocket(server.into())).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
