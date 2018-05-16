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
    outputs: Vec<::api::output::Output>,
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

    /// Liefert ein Vector mit den Outputs des Servers
    ///
    /// Diese Funktion wird in `::api::output::Output` aufgerufen.
    ///
    pub fn get_outputs(&self) -> Vec<::api::output::Output> {
        self.outputs.clone()
    }

    /// Liefert den Output mit der `id` des Servers
    ///
    /// Diese Funktion wird in `::api::output::Output` aufgerufen.
    ///
    pub fn get_output(&self, id: usize) -> Option<::api::output::Output> {
        match self.outputs.clone().get(id) {
            Some(output) => Some(output.clone()),
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

        // **Nicht einfach `.unwrap()`'en!!**
        // In den Unit Tests kann es vorkommen das die Pfade
        // `configuration_path` und `runtime_info_path`, `None` sind.
        let configuration_path = match &server.configuration_path {
            Some(ref path) => path.clone(),
            None => PathBuf::from(""),
        };
        let runtime_info_path = match &server.runtime_info_path {
            Some(ref path) => path.clone(),
            None => PathBuf::from(""),
        };

        let mut outputs: Vec<::api::output::Output> = Vec::new();
        for output in server.get_outputs() {
            outputs.push(output.into());
        }

        Server {
            service_interval: server.service_interval,
            sensors: sensors,
            configuration_path: configuration_path.to_string_lossy().to_string(),
            runtime_info_path: runtime_info_path.to_string_lossy().to_string(),
            outputs: outputs,
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
    fn index_txt() {
        let server = ::server::Server::new();
        let client = Client::new(::api::rocket(server.clone())).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn index_json() {
        let server = ::server::Server::new();
        let client = Client::new(::api::rocket(server.clone())).expect("valid rocket instance");
        let response = client.get("/api").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
