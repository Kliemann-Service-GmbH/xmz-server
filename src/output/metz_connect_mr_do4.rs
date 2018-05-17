use prelude::*;


/// 'Metz Connect MR-DO4' Schaltmodul mit Modbus Interface
///
/// Modbus Modul mit 4 digitalen Ausgängen.
/// - <https://www.metz-connect.com/de/products/1108361321>
/// - <https://www.metz-connect.com/en/products/1108361321>
///
#[derive(Debug)]
#[derive(Clone)] // Clone damit die Datenstruktur in `server.get_outputs()` gecloned werden kann
pub struct MetzConnectMRDO4 {
    name: String,
    output_type: OutputType,
    pins: usize,
    data: usize,
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

    /// Überschreibt den Namen
    ///
    /// Diese Funktion ist Teil des Builder Patterns mit dem der Output gebildet werden kann.
    /// Siehe dazu <https://abronan.com/rust-trait-objects-box-and-rc/>
    ///
    /// Wichtig ist das, wenn diese Funktion verwendet werden soll, im Anschluss, die Funktion
    /// `build()` verwendet wird. Siehe folgendes Beispiel:
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let schaltmodul = MetzConnectMRDO4::new()
    ///         .init_name("Relais".to_string())
    ///         .build();
    /// ```
    pub fn init_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Überschreibt die Anzahl der Pins
    ///
    /// Diese Funktion ist Teil des Builder Patterns mit dem der Output gebildet werden kann.
    /// Siehe dazu <https://abronan.com/rust-trait-objects-box-and-rc/>
    ///
    /// Wichtig ist das, wenn diese Funktion verwendet werden soll, im Anschluss, die Funktion
    /// `build()` verwendet wird. Siehe folgendes Beispiel:
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let schaltmodul = MetzConnectMRDO4::new()
    ///         .init_pins(1)
    ///         .build();
    /// ```
    pub fn init_pins(&mut self, pins: usize) -> &mut Self {
        self.pins = pins;
        self
    }

    /// Finale Funktion des Builder Patterns
    ///
    /// Accumuliert alle init_ Funktionen
    pub fn build(&self) -> Self {
        MetzConnectMRDO4 {
            name: self.name.clone(),
            pins: self.pins.clone(),
            ..Default::default()
        }
    }


    // FIXME: Kann sicher weg, wenn das Builder Pattern mit den `init_` Funktionen funktioniert
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
            name: "Metz Connect MR-DO4".to_string(),
            output_type: OutputType::MetzConnectMRDO4,
            pins: 4,
            data: 0,
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
    fn set(&mut self, num: usize) -> Result<(), OutputError> {
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
    fn unset(&mut self, num:usize) -> Result<(), OutputError> {
        Err(OutputError::CouldNotUnset)
    }

    /// Liefert den Typen des Ausgangs
    ///
    fn get_output_type(&self) -> OutputType {
        self.output_type.clone()
    }

    /// Liefert den Name des Ausgangs
    ///
    /// Diese Getter Funktion wird bei der Konvertierung von/zu den Laufzeitinformationen benötigt.
    /// Siehe `runtime_info/output.rs`
    fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Liefert die Anzahl der Pinks des Ausgangs
    ///
    /// Diese Getter Funktion wird bei der Konvertierung von/zu den Laufzeitinformationen benötigt.
    /// Siehe `runtime_info/output.rs`
    fn get_pins(&self) -> usize {
        self.pins
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let schaltmodul = MetzConnectMRDO4::new();
        assert_eq!(schaltmodul.name, "Metz Connect MR-DO4".to_string());
        assert_eq!(schaltmodul.pins, 4);
    }

    #[test]
    fn init_name() {
        let schaltmodul = MetzConnectMRDO4::new()
            .init_name("Relais".to_string())
            .build();

        assert_eq!(schaltmodul.name, "Relais".to_string());
    }

    #[test]
    fn init_pins() {
        let schaltmodul = MetzConnectMRDO4::new()
            .init_pins(1)
            .build();

        assert_eq!(schaltmodul.pins, 1);
    }

    #[test]
    fn init_name_and_pins() {
        let schaltmodul = MetzConnectMRDO4::new()
            .init_name("Relais".to_string())
            .init_pins(1)
            .build();

        assert_eq!(schaltmodul.name, "Relais".to_string());
        assert_eq!(schaltmodul.pins, 1);
    }


    #[test]
    fn new_with_pins() {
        let schaltmodul = MetzConnectMRDO4::new_with_pins(1);
        assert_eq!(schaltmodul.pins, 1);
    }


}
