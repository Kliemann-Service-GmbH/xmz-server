use std::time::SystemTime;


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Messzelle {
    messzelle_type: ::messzelle::MesszelleType,
    value: Option<(f64, SystemTime)>,
    max_values_for_n_minutes: u64, // in Sekunden
}

// Diese Default Implementation sorgt dafür das Felder die in der Konfigurationsdatei
// fehlen, mit diesen Default Werten ersetzt werden.
impl Default for Messzelle {
    fn default() -> Self {
        Messzelle {
            messzelle_type: ::messzelle::MesszelleType::MetzConnectCI4Analog420,
            value: None,
            max_values_for_n_minutes: 60,
        }
    }
}


/// Konvertierung der Messzellen Trait Objekte des Servers `::messzelle::Messzelle`
///
impl<'a> From<&'a Box<::messzelle::Messzelle + Send>> for Messzelle {
    fn from(messzelle: &'a Box<::messzelle::Messzelle + Send>) -> Self {
        // Wert aus der Referenz auspacken
        let value = match messzelle.value() {
            Some(ref x) => Some(**x),
            None => None,
        };

        Messzelle {
            value: value,
            max_values_for_n_minutes: 60,
            messzelle_type: messzelle.get_messzelle_type(),
        }
    }
}
