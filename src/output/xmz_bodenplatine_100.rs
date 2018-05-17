use prelude::*;


/// 'xMZ-Mod-Touch-Bodenplatine v1.0.0'
///
/// 9 Relais die über Shift Register (diese sind in der xMZ-Mod-Touch-Deckelplatine v1.0.0 verbaut)
/// gesteuert werden können.
///
#[derive(Debug)]
#[derive(Clone)] // Clone damit die Datenstruktur in `server.get_outputs()` gecloned werden kann
pub struct XMZBoden100 {
    name: String,
    output_type: OutputType,
    pins: usize,
    data: usize,
    oe_pin: usize,
    ds_pin: usize,
    clock_pin: usize,
    latch_pin: usize,
}

impl XMZBoden100 {
    /// Erzeugt eine neue Instanz einer 'xMZ-Mod-Touch-Bodenplatine v1.0.0'
    ///
    /// Die `new()` Funktion stellt die Default Representation einer Deckelplaine her.
    /// Für Alternative Konstruktoren siehe die `new_with_?()` Funktionen
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let bodenplaine = output::XMZBoden100::new();
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
    /// let schaltmodul = XMZBoden100::new()
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
    /// let schaltmodul = XMZBoden100::new()
    ///         .init_pins(1)
    ///         .build();
    /// ```
    pub fn init_pins(&mut self, pins: usize) -> &mut Self {
        self.pins = pins;
        self
    }

    /// Überschreibt den OE Pin des Shift Registers
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
    /// let schaltmodul = XMZBoden100::new()
    ///         .init_oe_pin(1)
    ///         .build();
    /// ```
    pub fn init_oe_pin(&mut self, oe_pin: usize) -> &mut Self {
        self.oe_pin = oe_pin;
        self
    }

    /// Überschreibt den DS Pin des Shift Registers
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
    /// let schaltmodul = XMZBoden100::new()
    ///         .init_ds_pin(1)
    ///         .build();
    /// ```
    pub fn init_ds_pin(&mut self, ds_pin: usize) -> &mut Self {
        self.ds_pin = ds_pin;
        self
    }

    /// Überschreibt den CLOCK Pin des Shift Registers
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
    /// let schaltmodul = XMZBoden100::new()
    ///         .init_clock_pin(1)
    ///         .build();
    /// ```
    pub fn init_clock_pin(&mut self, clock_pin: usize) -> &mut Self {
        self.clock_pin = clock_pin;
        self
    }

    /// Überschreibt den LATCH Pin des Shift Registers
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
    /// let schaltmodul = XMZBoden100::new()
    ///         .init_latch_pin(1)
    ///         .build();
    /// ```
    pub fn init_latch_pin(&mut self, latch_pin: usize) -> &mut Self {
        self.latch_pin = latch_pin;
        self
    }

    /// Finale Funktion des Builder Patterns
    ///
    /// Accumuliert alle init_ Funktionen
    pub fn build(&self) -> Self {
        XMZBoden100 {
            name: self.name.clone(),
            pins: self.pins.clone(),
            oe_pin: self.oe_pin.clone(),
            ds_pin: self.ds_pin.clone(),
            clock_pin: self.clock_pin.clone(),
            latch_pin: self.latch_pin.clone(),
            ..Default::default()
        }
    }


    // FIXME: Kann sicher weg, wenn das Builder Pattern mit den `init_` Funktionen funktioniert
    /// Erzeugt eine Instanz einer 'xMZ-Mod-Touch-Bodenplatine v1.0.0' mit beliebiger Pin Anzahl
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
    /// let bodenplaine = output::XMZBoden100::new_with_pins(4);
    /// ```
    ///
    pub fn new_with_pins(pins: usize) -> Self {
        XMZBoden100 {
            pins,
            ..Default::default()
        }
    }
}

impl Default for XMZBoden100 {
    fn default() -> Self {
        XMZBoden100 {
            name: "xMZ-Mod-Touch-Bodenplatine v1.0.0".to_string(),
            output_type: OutputType::XMZBoden100,
            pins: 9,
            data: 0,
            oe_pin: 277,
            ds_pin: 45,
            clock_pin: 39,
            latch_pin: 37,
        }
    }
}

impl fmt::Display for XMZBoden100 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "xMZ-Mod-Touch-Bodenplatine v1.0.0")
    }
}

impl Output for XMZBoden100 {
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
        let bodenplaine = XMZBoden100::new();
        assert_eq!(bodenplaine.pins, 9);
    }

    #[test]
    fn init_name() {
        let schaltmodul = XMZBoden100::new()
            .init_name("Relais".to_string())
            .build();

        assert_eq!(schaltmodul.name, "Relais".to_string());
    }

    #[test]
    fn init_pins() {
        let schaltmodul = XMZBoden100::new()
            .init_pins(1)
            .build();

        assert_eq!(schaltmodul.pins, 1);
    }

    #[test]
    fn init_oe_pin() {
        let schaltmodul = XMZBoden100::new()
            .init_oe_pin(1)
            .build();

        assert_eq!(schaltmodul.oe_pin, 1);
    }

    #[test]
    fn init_ds_pin() {
        let schaltmodul = XMZBoden100::new()
            .init_ds_pin(1)
            .build();

        assert_eq!(schaltmodul.ds_pin, 1);
    }

    #[test]
    fn init_clock_pin() {
        let schaltmodul = XMZBoden100::new()
            .init_clock_pin(1)
            .build();

        assert_eq!(schaltmodul.clock_pin, 1);
    }

    #[test]
    fn init_latch_pin() {
        let schaltmodul = XMZBoden100::new()
            .init_latch_pin(1)
            .build();

        assert_eq!(schaltmodul.latch_pin, 1);
    }

    #[test]
    fn combined_init_() {
        let schaltmodul = XMZBoden100::new()
            .init_name("Relais".to_string())
            .init_pins(1)
            .init_oe_pin(1)
            .init_ds_pin(1)
            .init_clock_pin(1)
            .init_latch_pin(1)
            .build();

        assert_eq!(schaltmodul.name, "Relais".to_string());
        assert_eq!(schaltmodul.pins, 1);
        assert_eq!(schaltmodul.oe_pin, 1);
        assert_eq!(schaltmodul.ds_pin, 1);
        assert_eq!(schaltmodul.clock_pin, 1);
        assert_eq!(schaltmodul.latch_pin, 1);
    }

    #[test]
    fn new_with_pins() {
        let bodenplaine = XMZBoden100::new_with_pins(4);
        assert_eq!(bodenplaine.pins, 4);
    }
}