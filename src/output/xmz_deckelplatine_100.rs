use prelude::*;


/// 'xMZ-Mod-Touch-Deckelplatine v1.0.0'
///
/// 20 schaltbare Ausgänge an der eine Folie mit eingebauten LED angeschlossen ist.
///
pub struct XMZDeckel100 {
    pins: usize,
    data: RwLock<usize>,
    oe_pin: usize,
    ds_pin: usize,
    clock_pin: usize,
    latch_pin: usize,
}

impl XMZDeckel100 {
    /// Erzeugt eine neue Instanz einer 'xMZ-Mod-Touch-Deckelplatine v1.0.0'
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    ///
    /// let deckelplaine = output::XMZDeckel100::new();
    /// ```
    ///
    pub fn new() -> Self {
        Default::default()
    }

    /// Erzeugt eine Instanz einer 'xMZ-Mod-Touch-Deckelplatine v1.0.0' mit beliebiger Pin Anzahl
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
    /// let deckelplaine = output::XMZDeckel100::new_with_pins(24);
    /// ```
    ///
    pub fn new_with_pins(pins: usize) -> Self {
        XMZDeckel100 {
            pins,
            ..Default::default()
        }
    }
}

impl Default for XMZDeckel100 {
    fn default() -> Self {
        XMZDeckel100 {
            pins: 20,
            data: RwLock::new(0),
            oe_pin: 276,
            ds_pin: 38,
            clock_pin: 44,
            latch_pin: 40,
        }
    }
}

impl fmt::Display for XMZDeckel100 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "xMZ-Mod-Touch-Deckelplatine v1.0.0")
    }
}

impl Output for XMZDeckel100 {
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
        let deckelplaine = XMZDeckel100::new();
        assert_eq!(deckelplaine.pins, 20);
    }

    #[test]
    fn new_with_pins() {
        let deckelplaine = XMZDeckel100::new_with_pins(24);
        assert_eq!(deckelplaine.pins, 24);
    }


}
