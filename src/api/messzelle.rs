use std::time::SystemTime;
use messzelle::{BoxedMesszelle};
use std::sync::{Arc, Mutex};


#[derive(Clone, Debug, Serialize)]
pub struct Messzelle {
    messzelle_type: ::messzelle::MesszelleType,
    value: Option<(f64, SystemTime)>,
}

// #[get("/")]
// fn index(server: State<::api::server::Server>) -> Json<Vec<Sensor>> {
//     Json(server.clone().get_sensor(1).get_messzellen().clone())
// }


/// Konvertierung der Messzellen Trait Objekte des Servers `::messzelle::Messzelle`
///
impl From<Arc<Mutex<BoxedMesszelle>>> for Messzelle {
    fn from(messzelle: Arc<Mutex<BoxedMesszelle>>) -> Self {
        // Wert aus der Referenz auspacken
        let messzelle = messzelle.lock().unwrap();
        let value = match messzelle.get_value() {
            Some(ref x) => Some(**x),
            None => None,
        };
        println!("{:?}", messzelle);
        Messzelle {
            value: value,
            messzelle_type: messzelle.get_messzelle_type(),
        }
    }
}
