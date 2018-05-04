//! Server Externe
//!
//! Dieses Modul beinhaltet die Externe Representation der Server Struktur
use rocket_contrib::Json;
use rocket::State;
use server;


#[derive(Clone, Debug, Serialize)]
pub struct Server {
    pub service_interval: u32,
    pub sensors: Vec<::api::sensor::Sensor>,
    configuration_path: String,
    runtime_info_path: String,
}

impl Server {
    pub fn get_sensors(&self) -> &Vec<::api::sensor::Sensor> {
        &self.sensors
    }
}

impl From<server::Server> for Server {
    fn from(server: server::Server) -> Self {
        let mut sensors: Vec<::api::sensor::Sensor> = Vec::new();
        for sensor in server.get_sensors() {
            if let Ok(sensor) = sensor.lock() {
                sensors.push((&*sensor).into());
            }
        }

        Server {
            service_interval: server.service_interval,
            sensors: sensors,
            configuration_path: server.configuration_path.unwrap().to_string_lossy().to_string(),
            runtime_info_path: server.runtime_info_path.unwrap().to_string_lossy().to_string(),
        }
    }
}

#[get("/")]
fn index(server: State<::api::server::Server>) -> Json<::api::server::Server> {
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
        let client = Client::new(::api::rocket(server.into())).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
