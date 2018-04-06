use rocket::State;
use ::api::server::Server as ServerExtern;
use ::server::Server as ServerIntern;
use ::sensor::Sensor as SensorIntern;


#[derive(Clone, Debug)]
#[derive(Serialize)]
pub struct Sensor {

}


#[get("/")]
fn index(server: State<ServerExtern>) -> String {
    // let sensors = server.get_sensors().lock().unwrap();
    // Json(sensors)
    format!("{:?}", server.clone())
}



impl From<Box<SensorIntern>> for Sensor {
    fn from(sensor: Box<SensorIntern>) -> Self {
        Sensor {}
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
