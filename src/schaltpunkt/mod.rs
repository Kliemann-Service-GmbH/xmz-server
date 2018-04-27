//! Liste von Schwellwerten (`schwellwert`) und Aktionen (`aktion`)
//!

#[derive(Debug, Default)]
pub struct Schaltpunkt {}

impl Schaltpunkt {
    /// Erstellt ein neuen Schaltpunkt
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::schaltpunkt::Schaltpunkt;
    ///
    /// let _schaltpunt = Schaltpunkt::new();
    /// ```
    pub fn new() -> Self {
        Schaltpunkt {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _schaltpunkt = Schaltpunkt::new();
    }
}
