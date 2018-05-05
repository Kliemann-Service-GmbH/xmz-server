//! Server Externe
//!
//! Dieses Modul beinhaltet die Externe Representation der Server Struktur
use rocket_contrib::Json;
use rocket::State;
use server;
use std::path::PathBuf;


#[derive(Clone, Debug, Serialize)]
pub struct Server {
    pub service_interval: u32,
    // FIXME: Evtl. sollte dieser Pfad nicht öffentlich sein
    configuration_path: String,
    // FIXME: Evtl. sollte dieser Pfad nicht öffentlich sein
    runtime_info_path: String,
    pub sensors: Vec<::api::sensor::Sensor>,
}

impl Server {
    /// Liefert ein Vector mit den Sensoren des Servers
    ///
    /// Diese Funktion wird in ::api::sensor::Sensor aufgerufen.
    pub fn get_sensors(&self) -> &Vec<::api::sensor::Sensor> {
        &self.sensors
    }
}

impl<'s> From<&'s server::Server> for Server {
    fn from(server: &'s server::Server) -> Self {
        Server {
            service_interval: 1337,
            configuration_path: "Config".to_string(),
            runtime_info_path: "Runtime".to_string(),
            sensors: vec![],
        }
    }
}

impl From<server::Server> for Server {
    fn from(server: server::Server) -> Self {
        let mut sensors: Vec<::api::sensor::Sensor> = Vec::new();
        // for sensor in server.get_sensors() {
        //     if let Ok(sensor) = sensor.lock() {
        //         sensors.push((&*sensor).into());
        //     }
        // }

        // In den Unit Tests kann es vorkommen das die Pfade `configuration_path` und `runtime_info_path`
        // `None` sind.
        let configuration_path = match server.configuration_path {
            Some(path) => path,
            None => PathBuf::from(""),
        };
        let runtime_info_path = match server.runtime_info_path {
            Some(path) => path,
            None => PathBuf::from(""),
        };

        Server {
            service_interval: server.service_interval,
            sensors: sensors,
            configuration_path: configuration_path.to_string_lossy().to_string(),
            runtime_info_path: runtime_info_path.to_string_lossy().to_string(),
        }
    }
}

#[get("/")]
fn index(server: State<::api::server::Server>) -> Json<::api::server::Server> {
    Json(server.clone())
}

#[cfg(test)]
mod test {
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
