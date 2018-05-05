use std::sync::{Arc, Mutex};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sensor {
    id: u32,
    pub sensor_type: ::sensor::SensorType,
    messzellen: Vec<::configuration::Messzelle>,
}

// TODO: Werden in der Konfiguration Messzellen in Sensoren eingetragen die nicht unterstützt sind,
// dann muss das ein Fehler werfen.
// Zum Beispiel: `::messzelle::MesszelleType::MetzConnectCI4Analog420` in `::sensor::SensorType::RaGasCONO2Mod`
//    => "Fehler der RA-GAS CO/NO2 Sensor unterstützt keine Analogen Sensoren. Bitte prüfen Sie die Konfigurationsdatei!"
/// Konvertierung in das Sensor Trait Objekt
///
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in das
/// entsprechenden Sensor Trait Objekt.
///
impl From<Sensor> for ::sensor::RaGasCONO2Mod {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Arc<Mutex<Vec<Box<::messzelle::Messzelle + Send + 'static>>>> = Arc::new(Mutex::new(vec![]));
        // let mut messzellen: Vec<Arc<Mutex<Box<::messzelle::Messzelle + Send + 'static>>>> = vec![];
        // for m in sensor.messzellen {
        //     match m.messzelle_type {
        //         ::messzelle::MesszelleType::RaGasNO2Mod => {
        //             let messzelle: ::messzelle::RaGasNO2Mod = m.into();
        //             messzellen.push(Arc::new(Mutex::new(Box::new(messzelle))));
        //         },
        //         ::messzelle::MesszelleType::RaGasCOMod => {
        //             let messzelle: ::messzelle::RaGasCOMod = m.clone().into();
        //             messzellen.push(Arc::new(Mutex::new(Box::new(messzelle))));
        //         },
        //         ::messzelle::MesszelleType::MetzConnectCI4Analog420 => {
        //             let messzelle: ::messzelle::MetzConnectCI4Analog420 = m.into();
        //             messzellen.push(Arc::new(Mutex::new(Box::new(messzelle))));
        //         },
        //     }
        // }
        ::sensor::RaGasCONO2Mod {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::RaGasCONO2Mod,
            messzellen: messzellen,
        }
    }
}
/// Konvertierung in das Sensor Trait Objekt
///
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in das
/// entsprechenden Sensor Trait Objekt.
///
impl From<Sensor> for ::sensor::MetzConnectCI4 {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Arc<Mutex<Vec<Box<::messzelle::Messzelle + Send + 'static>>>> = Arc::new(Mutex::new(vec![]));
        // let mut messzellen: Vec<Arc<Mutex<Box<::messzelle::Messzelle + Send + 'static>>>> = vec![];
        // for m in sensor.messzellen {
        //     match m.messzelle_type {
        //         ::messzelle::MesszelleType::RaGasNO2Mod => {
        //             let messzelle: ::messzelle::RaGasNO2Mod = m.into();
        //             messzellen.push(Arc::new(Mutex::new(Box::new(messzelle))));
        //         },
        //         ::messzelle::MesszelleType::RaGasCOMod => {
        //             let messzelle: ::messzelle::RaGasCOMod = m.clone().into();
        //             messzellen.push(Arc::new(Mutex::new(Box::new(messzelle))));
        //         },
        //         ::messzelle::MesszelleType::MetzConnectCI4Analog420 => {
        //             let messzelle: ::messzelle::MetzConnectCI4Analog420 = m.into();
        //             messzellen.push(Arc::new(Mutex::new(Box::new(messzelle))));
        //         },
        //     }
        // }
        ::sensor::MetzConnectCI4 {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::MetzConnectCI4,
            messzellen: messzellen,
        }
    }
}
/// Konvertierung in das Sensor Trait Objekt
///
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in das
/// entsprechenden Sensor Trait Objekt.
///
impl From<Sensor> for ::sensor::TestSensor {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Arc<Mutex<Vec<Box<::messzelle::Messzelle + Send + 'static>>>> = Arc::new(Mutex::new(vec![]));
        // let mut messzellen: Vec<Arc<Mutex<Box<::messzelle::Messzelle + Send + 'static>>>> = vec![];
        // for m in sensor.messzellen {
        //     match m.messzelle_type {
        //         ::messzelle::MesszelleType::RaGasNO2Mod => {
        //             let messzelle: ::messzelle::RaGasNO2Mod = m.into();
        //             messzellen.push(Arc::new(Mutex::new(Box::new(messzelle))));
        //         },
        //         ::messzelle::MesszelleType::RaGasCOMod => {
        //             let messzelle: ::messzelle::RaGasCOMod = m.clone().into();
        //             messzellen.push(Arc::new(Mutex::new(Box::new(messzelle))));
        //         },
        //         ::messzelle::MesszelleType::MetzConnectCI4Analog420 => {
        //             let messzelle: ::messzelle::MetzConnectCI4Analog420 = m.into();
        //             messzellen.push(Arc::new(Mutex::new(Box::new(messzelle))));
        //         },
        //     }
        // }
        ::sensor::TestSensor {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::TestSensor,
            messzellen: messzellen,
        }
    }
}

/// Konvertierung von den Sensor Trait Objekt `server::Sensor` in das toml Sensor Format
///
/// Konvertierung von einem Sensor Trait Objekt nach einem Sensor Objekt welches in das toml
/// Datenformat konvertiert werden kann.
///
/// Hauptsächlich werden die generic Komponenten in konkrete Typen gewandelt.
///
impl<'a> From<&'a Box<::sensor::Sensor + Send>> for Sensor {
    fn from(sensor: &'a Box<::sensor::Sensor + Send>) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Vec<::configuration::Messzelle> = vec![];
        // for messzelle in sensor.get_messzellen() {
        //     if let Ok(messzelle) = messzelle.lock() {
        //         messzellen.push((&*messzelle).into())
        //     }
        // }
        Sensor {
            id: sensor.get_id(),
            messzellen: messzellen,
            sensor_type: sensor.get_sensor_type(),
        }
    }
}
