use std::time::SystemTime;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Messzelle {
    pub messzelle_type: ::messzelle::MesszelleType,
    values: Vec<(f64, SystemTime)>,
}

/// Konvertierung in das Messzelle Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// ``Messzelle`` Trait Objekt.
///
impl From<Messzelle> for ::messzelle::RaGasNO2Mod {
    fn from(messzelle: Messzelle) -> Self {
        ::messzelle::RaGasNO2Mod {
            messzelle_type: ::messzelle::MesszelleType::RaGasNO2Mod,
            values: messzelle.values,
        }
    }
}
/// Konvertierung in das Messzelle Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `Messzelle` Trait Objekt.
///
impl From<Messzelle> for ::messzelle::RaGasCOMod {
    fn from(messzelle: Messzelle) -> Self {
        ::messzelle::RaGasCOMod {
            messzelle_type: ::messzelle::MesszelleType::RaGasCOMod,
            values: messzelle.values,
        }
    }
}
/// Konvertierung in das Messzelle Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `Messzelle` Trait Objekt.
///
impl From<Messzelle> for ::messzelle::MetzConnectCI4Analog420 {
    fn from(messzelle: Messzelle) -> Self {
        ::messzelle::MetzConnectCI4Analog420 {
            messzelle_type: ::messzelle::MesszelleType::MetzConnectCI4Analog420,
            values: messzelle.values,
        }
    }
}

/// Konvertierung der Messzellen Trait Objekte des Servers `::messzelle::Messzelle`
///
impl<'a> From<&'a Box<::messzelle::Messzelle + Send>> for Messzelle {
    fn from(messzelle: &'a Box<::messzelle::Messzelle + Send>) -> Self {
        Messzelle {
            values: messzelle.get_values(),
            messzelle_type: messzelle.get_messzelle_type(),
        }
    }
}
