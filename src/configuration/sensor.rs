
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sensor {
    id: u32,
    pub sensor_type: ::sensor::SensorType,
    messzellen: Vec<::configuration::Messzelle>,
}


/// Konvertierung in das Sensor Trait Objekt
///
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in das
/// entsprechenden Sensor Trait Objekt.
///
impl From<Sensor> for ::sensor::RaGasCONO2Mod {
    fn from(sensor: Sensor) -> Self {
        // Kontruiere Messzellen
        println!("implementiere mich!");
        ::sensor::RaGasCONO2Mod {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::RaGasCONO2Mod,
            messzellen: Vec::new(),
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
        // Kontruiere Messzellen
        println!("implementiere mich!");
        ::sensor::MetzConnectCI4 {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::MetzConnectCI4,
            messzellen: Vec::new(),
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
        // Kontruiere Messzellen
        println!("implementiere mich!");
        ::sensor::TestSensor {
            id: sensor.id,
            sensor_type: ::sensor::SensorType::TestSensor,
            messzellen: Vec::new(),
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
        // Kontruiere Messzellen
        let mut messzellen: Vec<::configuration::Messzelle> = vec![];
        for messzelle in sensor.get_messzellen() {
            if let Ok(messzelle) = messzelle.lock() {
                messzellen.push((&*messzelle).into())
            }
        }
        Sensor {
            id: sensor.get_id(),
            messzellen: messzellen,
            sensor_type: sensor.get_sensor_type(),
        }
    }
}
