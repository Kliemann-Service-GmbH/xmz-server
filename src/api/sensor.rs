use rocket::State;
use rocket_contrib::Json;


#[derive(Clone, Debug, Serialize)]
pub struct Sensor {
    id: u32,
    sensor_type: ::sensor::SensorType,
    messzellen: Vec<::api::messzelle::Messzelle>,
}

impl Sensor {
    /// Liefert eine Vector der Messzellen
    ///
    pub fn get_messzellen(&self) -> &Vec<::api::messzelle::Messzelle> {
        &self.messzellen
    }
}

#[get("/")]
fn index(server: State<::api::server::Server>) -> Json<Vec<Sensor>> {
    Json(server.clone().get_sensors().clone())
}


/// Konvertierung von den Sensor Trait Objekten `server::Sensor`
///
/// Diese Konvertierung wird indirekt vom Server, ein Modul weiter oben, aufgerufen.
impl<'a> From<&'a Box<::sensor::Sensor + Send>> for Sensor {
    fn from(sensor: &'a Box<::sensor::Sensor + Send>) -> Self {
        // Kontruiere Messzellen
        let mut messzellen: Vec<::api::messzelle::Messzelle> = vec![];
        // for messzelle in sensor.get_messzellen() {
        //     if let Ok(messzelle) = messzelle.lock() {
        //         messzellen.push((&*messzelle).into())
        //     }
        // }
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
    fn get_messzellen() {
        let sensor = Sensor {
            id: 0,
            sensor_type: ::sensor::SensorType::TestSensor,
            messzellen: Vec::new(),
        };
        assert_eq!(sensor.get_messzellen().len(), 0);
    }
}
