use api;
use rocket::State;
use rocket_contrib::Json;
use sensor;


#[derive(Clone, Debug, Serialize)]
pub struct Sensor {
    sensor_type: String,
    messzellen: Vec<api::messzelle::Messzelle>,
}
impl Sensor {
    #[allow(dead_code)]
    pub fn get_messzellen(&self) -> &Vec<api::messzelle::Messzelle> {
        &self.messzellen
    }
}

#[get("/")]
fn index(server: State<api::server::Server>) -> Json<Vec<Sensor>> {
    Json(server.clone().get_sensors().clone())
}

impl<'a> From<&'a Box<sensor::Sensor + Send>> for Sensor {
    fn from(sensor: &'a Box<sensor::Sensor + Send>) -> Self {
        // Kontruiere Messzellen
        let mut messzellen: Vec<api::messzelle::Messzelle> = vec![];
        for messzelle in sensor.get_messzellen() {
            if let Ok(messzelle) = messzelle.lock() {
                messzellen.push((&*messzelle).into())
            }
        }
        // Sensor Typ auslesen und setzen
        let sensor_type = format!("{}", sensor);

        Sensor {
            messzellen: messzellen,
            sensor_type: sensor_type,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use api;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn index() {
        let server = ::server::Server::new();
        let client = Client::new(api::rocket(server.into())).expect("valid rocket instance");
        let response = client.get("/sensors").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_messzellen() {
        let sensor = Sensor {
            sensor_type: "Test Sensor".to_string(),
            messzellen: Vec::new(),
        };
        assert_eq!(sensor.get_messzellen().len(), 0);
    }
}
