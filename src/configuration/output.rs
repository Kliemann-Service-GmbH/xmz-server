use prelude::*;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Output {
    name: String,
    pub output_type: OutputType, // public für `::configuration::Server::From<Server>`
    pins: usize,
    modbus_address: Option<usize>,
}

/// Konvertierung in das Output Trait Objekt
///
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in
/// das entsprechenden Output Trait Objekt.
///
impl From<Output> for XMZDeckel100 {
    fn from(output: Output) -> Self {
        XMZDeckel100::new()
            .init_name(output.name)
            .init_pins(output.pins)
            .build()
    }
}

/// Konvertierung in das Output Trait Objekt
///
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in
/// das entsprechenden Output Trait Objekt.
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
/// Diese impl konvertiert die toml Daten, die in der Konfigurationsdatei verwendet werden in
/// das entsprechenden Output Trait Objekt.
///
impl From<Output> for MetzConnectMRDO4 {
    fn from(output: Output) -> Self {
        MetzConnectMRDO4::new()
            .init_name(output.name)
            .init_pins(output.pins)
            .build()
    }
}
