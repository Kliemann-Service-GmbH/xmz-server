use std::time::SystemTime;


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Messzelle {
    pub messzelle_type: ::messzelle::MesszelleType,
    values: Vec<(f64, SystemTime)>,
    max_values_for_n_minutes: u64, // in Sekunden
}

// Diese Default Implementation sorgt dafür das Felder die in der Konfigurationsdatei
// fehlen, mit diesen Default Werten ersetzt werden.
//
impl Default for Messzelle {
    fn default() -> Self {
        Messzelle {
            messzelle_type: ::messzelle::MesszelleType::MetzConnectCI4Analog420,
            values: vec![],
            max_values_for_n_minutes: 60,
        }
    }
}

/// Konvertierung in das Messzelle Trait Objekt
///
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in das
/// entsprechenden Messzelle Trait Objekt.
///
impl From<Messzelle> for ::messzelle::RaGasNO2Mod {
    fn from(messzelle: Messzelle) -> Self {
        ::messzelle::RaGasNO2Mod {
            messzelle_type: ::messzelle::MesszelleType::RaGasNO2Mod,
            // values: messzelle.get_values(),
            values: messzelle.values,
            max_values_for_n_minutes: messzelle.max_values_for_n_minutes,
        }
    }
}
/// Konvertierung in das Messzelle Trait Objekt
///
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in das
/// entsprechenden Messzelle Trait Objekt.
///
impl From<Messzelle> for ::messzelle::RaGasCOMod {
    fn from(messzelle: Messzelle) -> Self {
        ::messzelle::RaGasCOMod {
            messzelle_type: ::messzelle::MesszelleType::RaGasCOMod,
            // values: messzelle.get_values(),
            values: messzelle.values,
            max_values_for_n_minutes: messzelle.max_values_for_n_minutes,
        }
    }
}
/// Konvertierung in das Messzelle Trait Objekt
///
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in das
/// entsprechenden Messzelle Trait Objekt.
///
impl From<Messzelle> for ::messzelle::MetzConnectCI4Analog420 {
    fn from(messzelle: Messzelle) -> Self {
        ::messzelle::MetzConnectCI4Analog420 {
            messzelle_type: ::messzelle::MesszelleType::MetzConnectCI4Analog420,
            // values: messzelle.get_values(),
            values: messzelle.values,
            max_values_for_n_minutes: messzelle.max_values_for_n_minutes,
        }
    }
}

/// Konvertierung der Messzellen Trait Objekte des Servers `::messzelle::Messzelle`
///
impl<'a> From<&'a Box<::messzelle::Messzelle + Send>> for Messzelle {
    fn from(messzelle: &'a Box<::messzelle::Messzelle + Send>) -> Self {
        Messzelle {
            values: messzelle.get_values(),
            max_values_for_n_minutes: 60, // FIXME: should be a const in `::messzelle`
            messzelle_type: messzelle.get_messzelle_type(),
        }
    }
}
