use std::time::SystemTime;

#[derive(Clone, Debug, Serialize)]
pub struct Messzelle {
    messzelle_type: String,
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
        let value = match messzelle.value() {
            Some(ref x) => Some(**x),
            None => None,
        };
        // Type finden
        let messzelle_type = format!("{}", messzelle);

        Messzelle {
            value: value,
            max_values_for_n_minutes: 0,
            messzelle_type: messzelle_type,
        }
    }
}
