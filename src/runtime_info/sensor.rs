

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sensor {
    id: u32,
    pub sensor_type: ::sensor::SensorType,
    messzellen: Vec<::runtime_info::Messzelle>,
}

/// Konvertierung in das Sensor Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `Sensor` Trait Objekt.
///
impl From<Sensor> for ::sensor::RaGasCONO2Mod {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Vec<Box<::messzelle::Messzelle + Send + 'static>> = vec![];
        for m in sensor.messzellen {
            match m.messzelle_type {
                ::messzelle::MesszelleType::RaGasNO2Mod => {
                    let messzelle: ::messzelle::RaGasNO2Mod = m.into();
                    messzellen.push(Box::new(messzelle));
                },
                ::messzelle::MesszelleType::RaGasCOMod => {
                    let messzelle: ::messzelle::RaGasCOMod = m.into();
                    messzellen.push(Box::new(messzelle));
                },
                ::messzelle::MesszelleType::MetzConnectCI4Analog420 => {
                    let messzelle: ::messzelle::MetzConnectCI4Analog420 = m.into();
                    messzellen.push(Box::new(messzelle));
                },
            }
        }
        ::sensor::RaGasCONO2Mod {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::RaGasCONO2Mod,
            messzellen: messzellen,
        }
    }
}
/// Konvertierung in das Sensor Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `Sensor` Trait Objekt.
///
impl From<Sensor> for ::sensor::MetzConnectCI4 {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Vec<Box<::messzelle::Messzelle + Send + 'static>> = vec![];
        for m in sensor.messzellen {
            match m.messzelle_type {
                ::messzelle::MesszelleType::RaGasNO2Mod => {
                    let messzelle: ::messzelle::RaGasNO2Mod = m.into();
                    messzellen.push(Box::new(messzelle));
                },
                ::messzelle::MesszelleType::RaGasCOMod => {
                    let messzelle: ::messzelle::RaGasCOMod = m.into();
                    messzellen.push(Box::new(messzelle));
                },
                ::messzelle::MesszelleType::MetzConnectCI4Analog420 => {
                    let messzelle: ::messzelle::MetzConnectCI4Analog420 = m.into();
                    messzellen.push(Box::new(messzelle));
                },
            }
        }
        ::sensor::MetzConnectCI4 {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::RaGasCONO2Mod,
            messzellen: messzellen,
        }
    }
}
/// Konvertierung in das Sensor Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `Sensor` Trait Objekt.
///
impl From<Sensor> for ::sensor::TestSensor {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Vec<Box<::messzelle::Messzelle + Send + 'static>> = vec![];
        for m in sensor.messzellen {
            match m.messzelle_type {
                ::messzelle::MesszelleType::RaGasNO2Mod => {
                    let messzelle: ::messzelle::RaGasNO2Mod = m.into();
                    messzellen.push(Box::new(messzelle));
                },
                ::messzelle::MesszelleType::RaGasCOMod => {
                    let messzelle: ::messzelle::RaGasCOMod = m.into();
                    messzellen.push(Box::new(messzelle));
                },
                ::messzelle::MesszelleType::MetzConnectCI4Analog420 => {
                    let messzelle: ::messzelle::MetzConnectCI4Analog420 = m.into();
                    messzellen.push(Box::new(messzelle));
                },
            }
        }
        ::sensor::TestSensor {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::RaGasCONO2Mod,
            messzellen: messzellen,
        }
    }
}

/// Konvertierung von den Sensor Trait Objekten `server::Sensor`
///
///
impl<'a> From<&'a Box<::sensor::Sensor + Send>> for Sensor {
    fn from(sensor: &'a Box<::sensor::Sensor + Send>) -> Self {
        // Kontruiere Messzellen
        let mut messzellen: Vec<::runtime_info::Messzelle> = vec![];
        for messzelle in sensor.get_messzellen() {
            messzellen.push(messzelle.into())
        }
        Sensor {
            id: sensor.get_id(),
            messzellen: messzellen,
            sensor_type: sensor.get_sensor_type(),
        }
    }
}
