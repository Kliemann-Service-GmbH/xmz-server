use ::messzelle::{
    BoxedMesszelle,
    MesszelleType,
    MetzConnectCI4Analog420,
    RaGasCOMod,
    RaGasNO2Mod,
};
use std::time::SystemTime;
use std::sync::{Arc, Mutex, RwLock};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Messzelle {
    pub messzelle_type: MesszelleType,
    values: Vec<(f64, SystemTime)>,
}

/// Konvertierung in das Messzelle Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// ``Messzelle`` Trait Objekt.
///
impl From<Messzelle> for RaGasNO2Mod {
    fn from(messzelle: Messzelle) -> Self {
        RaGasNO2Mod {
            messzelle_type: MesszelleType::RaGasNO2Mod,
            values: messzelle.values,
        }
    }
}
/// Konvertierung in das Messzelle Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `Messzelle` Trait Objekt.
///
impl From<Messzelle> for RaGasCOMod {
    fn from(messzelle: Messzelle) -> Self {
        RaGasCOMod {
            messzelle_type: MesszelleType::RaGasCOMod,
            values: messzelle.values,
        }
    }
}
/// Konvertierung in das Messzelle Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `Messzelle` Trait Objekt.
///
impl From<Messzelle> for MetzConnectCI4Analog420 {
    fn from(messzelle: Messzelle) -> Self {
        MetzConnectCI4Analog420 {
            messzelle_type: MesszelleType::MetzConnectCI4Analog420,
            values: messzelle.values,
        }
    }
}

/// Konvertierung der Messzellen Trait Objekte des Servers `::messzelle::Messzelle`
///
impl From<Arc<RwLock<BoxedMesszelle>>> for Messzelle {
    fn from(messzelle: Arc<RwLock<BoxedMesszelle>>) -> Self {
        let messzelle = messzelle.read().unwrap();
        Messzelle {
            values: messzelle.get_values(),
            messzelle_type: messzelle.get_messzelle_type(),
        }
    }
}
