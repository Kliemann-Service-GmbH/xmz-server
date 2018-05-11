use ::sensor::BoxedSensor;
use api::messzelle::Messzelle;
use rocket_contrib::Json;
use rocket::State;
use std::sync::{Arc, RwLock};


#[derive(Clone, Debug, Serialize)]
pub struct Sensor {
    id: u32,
    sensor_type: ::sensor::SensorType,
    messzellen: Vec<Messzelle>,
}

impl Sensor {
    /// Liefert eine Vector der Messzellen
    ///
    pub fn get_messzellen(&self) -> Vec<Messzelle> {
        self.messzellen.clone()
    }
}

// `GET /sensors`
#[get("/")]
fn index(server: State<::server::Server>) -> Json<Vec<Sensor>> {
    let server: ::api::server::Server = server.inner().clone().into();
    Json(server.get_sensors().clone())
}

// `GET /sensor/1`
#[get("/<id>")]
fn get(id: usize, server: State<::server::Server>) -> Option<Json<Sensor>> {
    let server: ::api::server::Server = server.inner().clone().into();
    match server.get_sensor(id) {
        Some(sensor) => Some(Json(sensor)),
        None => None,
    }
}

// `GET /sensor/1/messzellen`
#[get("/<id>/messzellen")]
fn get_messzellen(id: usize, server: State<::server::Server>) -> Option<Json<Vec<Messzelle>>> {
    let server: ::api::server::Server = server.inner().clone().into();
    match server.get_sensor(id) {
        Some(sensor) => {
            let messzellen = sensor.get_messzellen();
            Some(Json(messzellen))
        },
        None => None,
    }
}

// `GET /sensor/1/messzellen`
#[get("/<sensor_id>/messzelle/<id>")]
fn get_messzelle(sensor_id: usize, id: usize, server: State<::server::Server>) -> Option<Json<Messzelle>> {
    let server: ::api::server::Server = server.inner().clone().into();
    match server.get_sensor(sensor_id) {
        Some(sensor) => {
            let messzellen = sensor.get_messzellen();
            match messzellen.get(id) {
                Some(messzelle) => Some(Json(messzelle.clone())),
                None => None,
            }
        },
        None => None,
    }
}


/// Konvertierung von den Sensor Trait Objekten `server::Sensor`
///
/// Diese Konvertierung wird indirekt vom Server, ein Modul weiter oben, aufgerufen.
impl From<Arc<RwLock<BoxedSensor>>> for Sensor {
    fn from(sensor: Arc<RwLock<BoxedSensor>>) -> Self {
        // Rekonstruiere Messzellen
        let sensor = sensor.read().unwrap();
        // Kontruiere Messzellen
        let mut messzellen: Vec<Messzelle> = vec![];
        for messzelle in sensor.get_messzellen() {
            messzellen.push(messzelle.into())
        }
        Sensor {
            id: sensor.get_id(),
            messzellen: messzellen,
            sensor_type: sensor.get_sensor_type(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::http::Status;
    use rocket::local::Client;


    #[test]
    fn index() {
        let server = ::server::Server::new();
        let client = Client::new(::api::rocket(server.into())).expect("valid rocket instance");
        let response = client.get("/sensors").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get() {
        // Server nicht mit `Server::new()` erstellt! `Server::default()` erstellt ein Server mit
        // sinnvoller Default Konfiguration.
        let server = ::server::Server::default();
        assert!(server.get_sensors().len() > 0);
        let client = Client::new(::api::rocket(server.into())).expect("valid rocket instance");
        let response = client.get("/sensor/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_messzellen() {
        let server = ::server::Server::default();
        assert!(server.get_sensors().len() > 0);
        let client = Client::new(::api::rocket(server.into())).expect("valid rocket instance");
        let response = client.get("/sensor/1/messzellen").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }


    #[test]
    fn get_messzelle() {
        use ::sensor::Sensor;
        let server = ::server::Server::default();
        assert!(server.get_sensors().len() > 0);
        let client = Client::new(::api::rocket(server.into())).expect("valid rocket instance");
        let response = client.get("/sensor/1/messzelle/0").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
