use prelude::*;

// // TODO: Prüfe ob das nicht eine `enum` sein kann
// pub enum Output {
//     // MetzConnectMRDO4 ("Metz Connect MR-DO4".to_string(), OutputType::MetzConnectMRDO4, 3)
//     MetzConnectMRDO4 (String, OutputType, usize),
//     XMZBoden100 (String, OutputType, usize),
//     XMZDeckel100 (String, OutputType, usize),
// }
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Output {
    name: String,
    // `pub` das der Typ in `impl From<Server> for ::server::Server` ausgelesen werden kann. Siehe `runtime_info/server.rs`
    pub output_type: OutputType,
    pins: usize,
    modbus_address: Option<usize>,
}

// Die Default Implementation wird weiter unten in diesem Modul bei der Serialization der Trait
// Objekte benötigt
impl Default for Output {
    fn default() -> Self {
        Output {
            name: String::new(),
            output_type: OutputType::XMZDeckel100, // irgend ein Typ, der Typ ist an dieser Stelle im Code garantiert gesetzt
            pins: 0,
            modbus_address: None,
        }
    }
}

/// Konvertierung in das Output Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `::output::Output` Trait Objekt.
///
impl From<Output> for MetzConnectMRDO4 {
    fn from(output: Output) -> Self {
        MetzConnectMRDO4::new()
            .init_name(output.name)
            .init_pins(output.pins)
            .build()
    }
}

/// Konvertierung in das Output Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `::output::Output` Trait Objekt.
///
impl From<Output> for XMZBoden100 {
    fn from(output: Output) -> Self {
        XMZBoden100::new()
            .init_name(output.name)
            .init_pins(output.pins)
            .build()
    }
}

/// Konvertierung in das Output Trait Objekt
///
/// Diese impl konvertiert die bincode Daten, der Laufzeitinformationen, in das entsprechenden
/// `::output::Output` Trait Objekt.
///
impl From<Output> for XMZDeckel100 {
    fn from(output: Output) -> Self {
        XMZDeckel100::new()
            .init_name(output.name)
            .init_pins(output.pins)
            .build()
    }
}

// Diese Funktion wird im `server::Server` in der Runtime `From` Implementation
//  `impl<'r> From<&'r Server> for ::runtime_info::Server` aufgerufen
/// Konvertierung von den Output Trait Objekten in das `runtime_info::Output` Objekt.
///
impl From<Arc<RwLock<BoxedOutput>>> for Output {
    fn from(output: Arc<RwLock<BoxedOutput>>) -> Self {
        let output = output.read().unwrap();
        match output.get_output_type() {
            OutputType::MetzConnectMRDO4 => {
                Output {
                    name: output.get_name(),
                    output_type: OutputType::MetzConnectMRDO4,
                    pins: output.get_pins(),
                    ..Default::default()
                }
            },
            OutputType::XMZBoden100 => {
                Output {
                    name: output.get_name(),
                    output_type: OutputType::XMZBoden100,
                    pins: output.get_pins(),
                    ..Default::default()
                }
            },
            OutputType::XMZDeckel100 => {
                Output {
                    name: output.get_name(),
                    output_type: OutputType::XMZDeckel100,
                    pins: output.get_pins(),
                    ..Default::default()
                }
            },
        }
    }
}
