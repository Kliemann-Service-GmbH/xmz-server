use prelude::*;


/// 'xMZ-Mod-Touch-Bodenplatine v1.0.0'
///
/// 9 Relais die über Shift Register (diese sind in der xMZ-Mod-Touch-Deckelplatine v1.0.0 verbaut)
/// gesteuert werden können.
/// /// xMZ-Mod-Touch-Deckelplatine v1.0.0
///
/// 9 schaltbare Ausgänge an der eine Folie mit eingebauten LED angeschlossen ist.
///
pub struct XMZBoden100 {
    pins: usize,
    data: RwLock<usize>,
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
            pins: 9,
            data: RwLock::new(0),
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
        let bodenplaine = XMZBoden100::new();
        assert_eq!(bodenplaine.pins, 9);
    }

    #[test]
    fn new_with_pins() {
        let bodenplaine = XMZBoden100::new_with_pins(4);
        assert_eq!(bodenplaine.pins, 4);
    }


}
