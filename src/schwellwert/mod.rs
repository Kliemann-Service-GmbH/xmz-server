//! Regel die wenn erfüllt, Ausgänge schaltet
//!

#[derive(Debug, Default)]
pub struct Schwellwert {}

impl Schwellwert {
    /// Erstellt ein neuen Schwellwert
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::schwellwert::Schwellwert;
    ///
    /// let _schwellwert = Schwellwert::new();
    /// ```
    pub fn new() -> Self {
        Schwellwert {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _schwellwert = Schwellwert::new();
    }
}
