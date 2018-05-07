use ::messzelle::{
    BoxedMesszelle,
    MesszelleType,
    MetzConnectCI4Analog420,
    RaGasCOMod,
    RaGasNO2Mod,
};
use ::sensor::{
    BoxedSensor,
    MetzConnectCI4,
    RaGasCONO2Mod,
    SensorType,
    TestSensor,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sensor {
    id: u32,
    pub sensor_type: SensorType,
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
impl From<Sensor> for RaGasCONO2Mod {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Vec<BoxedMesszelle>= vec![];
        for m in sensor.messzellen {
            match m.messzelle_type {
                MesszelleType::RaGasNO2Mod => {
                    let messzelle: RaGasNO2Mod = m.into();
                    messzellen.push(Box::new(messzelle));
                },
                MesszelleType::RaGasCOMod => {
                    let messzelle: RaGasCOMod = m.into();
                    messzellen.push(Box::new(messzelle));
                },
                MesszelleType::MetzConnectCI4Analog420 => {
                    let messzelle: MetzConnectCI4Analog420 = m.into();
                    messzellen.push(Box::new(messzelle));
                },
            }
        }
        RaGasCONO2Mod {
            id: sensor.id,
            sensor_type: SensorType::RaGasCONO2Mod,
            messzellen: messzellen,
        }
    }
}
/// Konvertierung in das Sensor Trait Objekt
///
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in das
/// entsprechenden Sensor Trait Objekt.
///
impl From<Sensor> for MetzConnectCI4 {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Vec<BoxedMesszelle>= vec![];
        for m in sensor.messzellen {
            match m.messzelle_type {
                MesszelleType::RaGasNO2Mod => {
                    let messzelle: RaGasNO2Mod = m.into();
                    messzellen.push(Box::new(messzelle));
                },
                MesszelleType::RaGasCOMod => {
                    let messzelle: RaGasCOMod = m.into();
                    messzellen.push(Box::new(messzelle));
                },
                MesszelleType::MetzConnectCI4Analog420 => {
                    let messzelle: MetzConnectCI4Analog420 = m.into();
                    messzellen.push(Box::new(messzelle));
                },
            }
        }
        MetzConnectCI4 {
            id: sensor.id,
            sensor_type: SensorType::MetzConnectCI4,
            messzellen: messzellen,
        }
    }
}
/// Konvertierung in das Sensor Trait Objekt
///
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in das
/// entsprechenden Sensor Trait Objekt.
///
impl From<Sensor> for TestSensor {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Vec<BoxedMesszelle>= vec![];
        for m in sensor.messzellen {
            match m.messzelle_type {
                MesszelleType::RaGasNO2Mod => {
                    let messzelle: RaGasNO2Mod = m.into();
                    messzellen.push(Box::new(messzelle));
                },
                MesszelleType::RaGasCOMod => {
                    let messzelle: RaGasCOMod = m.clone().into();
                    messzellen.push(Box::new(messzelle));
                },
                MesszelleType::MetzConnectCI4Analog420 => {
                    let messzelle: MetzConnectCI4Analog420 = m.into();
                    messzellen.push(Box::new(messzelle));
                },
            }
        }
        TestSensor {
            id: sensor.id,
            sensor_type: SensorType::TestSensor,
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
impl<'a> From<&'a BoxedSensor> for Sensor {
    fn from(sensor: &'a BoxedSensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Vec<::configuration::Messzelle> = vec![];
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
