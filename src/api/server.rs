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
    // FIXME: `&` entfernen
    /// Liefert ein Vector mit den Sensoren des Servers
    ///
    /// Diese Funktion wird in ::api::sensor::Sensor aufgerufen.
    pub fn get_sensors(&self) -> &Vec<::api::sensor::Sensor> {
        &self.sensors
    }
}

/// Konvertierung des `server::Server` nach `configuration::Server`
///
/// Konvertiert den `server::Server` in ein Format das in der Laufzeitinformation
/// gespeichert werden kann.
///
/// Diese Funktion ist analog zu der Konvertierung des `server::Server` nach [`runtime_info::Server`](../runtime_info/struct.Server.html)
///
impl<'s> From<&'s server::Server> for Server {
    fn from(server: &'s server::Server) -> Self {
        // Restauriere Sensoren
        let mut sensors: Vec<::api::sensor::Sensor> = Vec::new();
        for sensor in server.get_sensors() {
            sensors.push(sensor.into());
        }

        let configuration_path = match &server.configuration_path {
            Some(path) => path.to_string_lossy().to_string(),
            None => "not set".to_string(),
        };
        let runtime_info_path = match &server.runtime_info_path {
            Some(path) => path.to_string_lossy().to_string(),
            None => "not set".to_string(),
        };
        Server {
            service_interval: server.service_interval,
            configuration_path: configuration_path,
            runtime_info_path: runtime_info_path,
            sensors: sensors,
        }
    }
}

/// Alternative Konvertierung
///
/// TODO: Checken ob das besser ist
impl From<server::Server> for Server {
    fn from(server: server::Server) -> Self {
        let mut sensors: Vec<::api::sensor::Sensor> = Vec::new();
        for sensor in server.get_sensors() {
            sensors.push(sensor.into());
        }

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
