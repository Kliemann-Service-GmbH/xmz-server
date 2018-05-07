use std::time::SystemTime;
use messzelle::BoxedMesszelle;


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
impl<'a> From<&'a BoxedMesszelle> for Messzelle {
    fn from(messzelle: &'a BoxedMesszelle) -> Self {
        // Wert aus der Referenz auspacken
        // let value = match messzelle.get_value() {
        //     Some(ref x) => Some(**x),
        //     None => None,
        // };
        println!("{:?}", messzelle);
        Messzelle {
            value: None, //value,
            messzelle_type: messzelle.get_messzelle_type(),
        }
    }
}
