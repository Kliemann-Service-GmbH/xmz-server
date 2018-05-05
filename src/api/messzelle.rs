use std::time::SystemTime;

#[derive(Clone, Debug, Serialize)]
pub struct Messzelle {
    messzelle_type: ::messzelle::MesszelleType,
    value: Option<(f64, SystemTime)>,
    max_values_for_n_minutes: u64, // in Sekunden
}

// #[get("/")]
// fn index(server: State<::api::server::Server>) -> Json<Vec<Sensor>> {
//     Json(server.clone().get_sensor(1).get_messzellen().clone())
// }


/// Konvertierung der Messzellen Trait Objekte des Servers `::messzelle::Messzelle`
///
impl<'a> From<&'a Box<::messzelle::Messzelle + Send>> for Messzelle {
    fn from(messzelle: &'a Box<::messzelle::Messzelle + Send>) -> Self {
        // Wert aus der Referenz auspacken
        let value = match messzelle.get_value() {
            Some(ref x) => Some(**x),
            None => None,
        };
        Messzelle {
            value: value,
            max_values_for_n_minutes: 0,
            messzelle_type: messzelle.get_messzelle_type(),
        }
    }
}
