use ::api::messzelle::Messzelle as MesszelleExtern;
use ::api::sensor::Sensor as SensorExtern;
use ::api::server::Server as ServerExtern;
use ::sensor::Sensor as SensorIntern;
use ::server::Server as ServerIntern;
use rocket_contrib::Json;
use rocket::State;


#[derive(Clone, Debug)]
#[derive(Serialize)]
pub struct Sensor {
    sensor_type: String,
    messzellen: Vec<MesszelleExtern>,
}
impl Sensor {
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
    use api;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn index() {
        let server = ::server::Server { service_interval: 1, sensors: vec![] };
        let client = Client::new(api::rocket(server.into())).expect("valid rocket instance");
        let response = client.get("/sensors").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
