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
    sensors: Vec<::api::sensor::Sensor>,
}

impl Server {
    /// Liefert ein Vector mit den Sensoren des Servers
    ///
    /// Diese Funktion wird in `::api::sensor::Sensor` aufgerufen.
    ///
    pub fn get_sensors(&self) -> Vec<::api::sensor::Sensor> {
        self.sensors.clone()
    }

    /// Liefert ein Sensor des Servers
    ///
    /// Diese Funktion wird in `::api::sensor::Sensor` aufgerufen.
    ///
    pub fn get_sensor(&self, id: usize) -> Option<::api::sensor::Sensor> {
        match self.sensors.clone().get(id) {
            Some(sensor) => Some(sensor.clone()),
            None => None,
        }
    }
}

/// Konvertiert den `::server::Server` in die API Version `::api::server::Server`
///
/// Diese Funktion wird in der `index()` Funktion, dieses Modules, aufgerufen.
///
impl From<server::Server> for Server {
    fn from(server: server::Server) -> Self {
        let mut sensors: Vec<::api::sensor::Sensor> = Vec::new();
        for sensor in server.get_sensors() {
            sensors.push(sensor.into());
        }

        // In den Unit Tests kann es vorkommen das die Pfade
        // `configuration_path` und `runtime_info_path`, `None` sind.
        //
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
fn index(server: State<::server::Server>) -> Json<::api::server::Server> {
    Json(server.clone().into())
}

#[get("/")]
fn index_txt(server: State<::server::Server>) -> String {
    let server: ::api::server::Server = server.clone().into();
    format!("{:#?}", server)
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn index() {
        let server = ::server::Server::new();
        let client = Client::new(::api::rocket(server.clone())).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
