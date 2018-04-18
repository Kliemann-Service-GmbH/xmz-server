use ::api::server::Server as ServerExtern;
use ::sensor::Sensor as SensorIntern;
use ::messzelle::Messzelle as MesszelleIntern;
use ::server::Server as ServerIntern;
use rocket_contrib::Json;
use rocket::State;
use std::time::SystemTime;


#[derive(Clone, Debug)]
#[derive(Serialize)]
pub struct Messzelle {
    value: Option<(f64, SystemTime)>,
    max_values_for_n_minutes: u64, // in Sekunden
}

// #[get("/")]
// fn index(server: State<ServerExtern>) -> Json<Messzelle> {
//     Json()
// }

impl<'a> From<&'a Box<MesszelleIntern + Send>> for Messzelle {
    fn from(messzelle: &'a Box<MesszelleIntern + Send>) -> Self {
        let value = match messzelle.value() {
            Some(ref x) => Some(**x),
            None => None,
        };
        Messzelle {
            value: value,
            max_values_for_n_minutes: 0,
        }
    }
}
