use prelude::*;


/// 'Metz Connect MR-DO4' Schaltmodul mit Modbus Interface
///
/// Modbus Modul mit 4 digitalen Ausgängen.
/// - <https://www.metz-connect.com/de/products/1108361321>
/// - <https://www.metz-connect.com/en/products/1108361321>
///
pub struct MetzConnectMRDO4 {
    pins: usize,
    data: RwLock<usize>,
}

impl MetzConnectMRDO4 {
    /// Erzeugt eine neue Instanz des 'Metz Connect MR-DO4' Schaltmodules
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let schaltmodul = output::MetzConnectMRDO4::new();
    /// ```
    ///
    pub fn new() -> Self {
        Default::default()
    }

    /// Erzeugt eine Instanz eines 'Metz Connect MR-DO4' Schaltmodule mit beliebiger Pin Anzahl
    ///
    /// Dieser Funktion können die Anzahl der verfügbaren Pin (Relais) übergeben werden.
    /// Unter Anderem wird diese Funktion wird von den Konstruktoren der `From` Implementation in
    /// der Konfiguration, und in der Runtimeinformation verwenden.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let schaltmodul = output::MetzConnectMRDO4::new_with_pins(1);
    /// ```
    ///
    pub fn new_with_pins(pins: usize) -> Self {
        MetzConnectMRDO4 {
            pins,
            ..Default::default()
        }
    }
}

impl Default for MetzConnectMRDO4 {
    fn default() -> Self {
        MetzConnectMRDO4 {
            pins: 4,
            data: RwLock::new(0),
        }
    }
}

impl fmt::Display for MetzConnectMRDO4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Metz Connect MR-DO4")
    }
}

impl Output for MetzConnectMRDO4 {
    /// Schaltet den `num` Ausgang, ein
    ///
    /// Die Implementation muss ein Fehler zurück geben, wenn der Ausgang nicht geschalten werden konnte
    fn set(&self, num: usize) -> Result<(), OutputError> {
        Err(OutputError::CouldNotSet)
    }

    /// Liefer den aktuellen Status des `num` Ausgang, liefert ein boolean Wert
    ///
    /// Die Implementation muss ein Fehler zurück geben, wenn der Ausgang nicht gelesen werden konnte
    fn get(&self, num: usize) -> Result<bool, OutputError> {
        Err(OutputError::CouldNotGet)
    }

    /// Schaltet den `num` Ausgang, aus
    ///
    /// Die Implementation muss ein Fehler zurück geben, wenn der Ausgang nicht geschalten werden konnte
    fn clear(&self, num:usize) -> Result<(), OutputError> {
        Err(OutputError::CouldNotUnset)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let schaltmodul = MetzConnectMRDO4::new();
        assert_eq!(schaltmodul.pins, 4);
    }

    #[test]
    fn new_with_pins() {
        let schaltmodul = MetzConnectMRDO4::new_with_pins(1);
        assert_eq!(schaltmodul.pins, 1);
    }


}
