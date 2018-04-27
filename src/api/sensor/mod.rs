use ::api::messzelle::Messzelle as MesszelleExtern;
use ::api::server::Server as ServerExtern;
use ::sensor::Sensor as SensorIntern;
use rocket_contrib::Json;
use rocket::State;


#[derive(Clone, Debug)]
#[derive(Serialize)]
pub struct Sensor {
    sensor_type: String,
    messzellen: Vec<MesszelleExtern>,
}
impl Sensor {
    #[allow(dead_code)]
    pub fn get_messzellen(&self) -> &Vec<MesszelleExtern> {
        &self.messzellen
    }
}

#[get("/")]
fn index(server: State<ServerExtern>) -> Json<Vec<Sensor>> {
    Json(server.clone().get_sensors().clone())
}

impl<'a> From<&'a Box<SensorIntern + Send>> for Sensor {
    fn from(sensor: &'a Box<SensorIntern + Send>) -> Self {
        // Kontruiere Messzellen
        let mut messzellen: Vec<MesszelleExtern> = vec![];
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
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn index() {
        let server = ::server::Server::new();
        let client = Client::new(api::rocket(server.into())).expect("valid rocket instance");
        let response = client.get("/sensors").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_messzellen() {
        let sensor = Sensor { sensor_type: "Test Sensor".to_string(), messzellen: Vec::new(), };
        assert_eq!(sensor.get_messzellen().len(), 0);
    }
}
