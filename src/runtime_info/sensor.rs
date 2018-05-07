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
    messzellen: Vec<::runtime_info::Messzelle>,
}

/// Konvertierung in das Sensor Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `Sensor` Trait Objekt.
///
impl From<Sensor> for RaGasCONO2Mod {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Vec<BoxedMesszelle> = vec![];
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
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `Sensor` Trait Objekt.
///
impl From<Sensor> for MetzConnectCI4 {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Vec<BoxedMesszelle> = vec![];
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
            sensor_type: SensorType::RaGasCONO2Mod,
            messzellen: messzellen,
        }
    }
}
/// Konvertierung in das Sensor Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `Sensor` Trait Objekt.
///
impl From<Sensor> for TestSensor {
    fn from(sensor: Sensor) -> Self {
        // Restauriere Messzellen
        let mut messzellen: Vec<BoxedMesszelle> = vec![];
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
        TestSensor {
            id: sensor.id,
            sensor_type: SensorType::RaGasCONO2Mod,
            messzellen: messzellen,
        }
    }
}

/// Konvertierung von den Sensor Trait Objekten `server::Sensor`
///
///
impl<'a> From<&'a BoxedSensor> for Sensor {
    fn from(sensor: &'a BoxedSensor) -> Self {
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
