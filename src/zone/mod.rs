//! Zonen die vom Server überwacht werden
//!
use messzelle::Messzelle;
use prelude::*;
use schaltpunkt::Schaltpunkt;

/// Zonen, die Bereiche die der Server überwacht
///
/// Jeder Zone können n `Schaltpunkte` zugeordnet
/// werden. Diese Messzellen werden gegen
#[derive(Default)]
pub struct Zone {
    pub messzellen: Vec<Arc<Mutex<Box<Messzelle>>>>,
    pub schaltpunkte: Vec<Schaltpunkt>,
}

impl Zone {
    pub fn new() -> Self {
        Zone {
            messzellen: vec![],
            schaltpunkte: vec![],
        }
    }

    /// Fügt eine eine Messzelle hinzu
    ///
    /// Die Messzellen des Servers sind in `Arc`, `Mutex` und `Box` gekapselt.
    /// Dadurch sind die Messzellen in den unterschiedlichen Server Threads `Server::update_sensors()`
    /// und `Server::check_zones()` verfügbar (geteilt, veränderlich).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use xmz_server::prelude::*;
    /// use xmz_server::Zone;
    /// use xmz_server::messzelle::{Messzelle, RaGasNO2Mod};
    /// let messzelle1 = Arc::new(Mutex::new(Box::new( RaGasNO2Mod::new() ) as Box<Messzelle>));
    /// let mut zone = Zone::new();
    ///
    /// zone.add_messzelle(messzelle1);
    /// assert_eq!(zone.messzellen.len(), 1);
    /// ```
    pub fn add_messzelle(&mut self, messzelle: Arc<Mutex<Box<Messzelle>>>) {
        self.messzellen.push(messzelle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use messzelle::{Messzelle, RaGasCOMod, RaGasNO2Mod};

    #[test]
    fn new() {
        let zone = Zone::new();
        assert_eq!(zone.messzellen.len(), 0);
        assert_eq!(zone.schaltpunkte.len(), 0);
    }

    #[test]
    fn add_messzelle() {
        let messzelle1 = Arc::new(Mutex::new(Box::new(RaGasNO2Mod::new()) as Box<Messzelle>));
        let mut zone = Zone::new();

        zone.add_messzelle(messzelle1);
        assert_eq!(zone.messzellen.len(), 1);
    }

    #[test]
    fn add_more_messzellen() {
        let messzelle1 = Arc::new(Mutex::new(Box::new(RaGasNO2Mod::new()) as Box<Messzelle>));
        let messzelle2 = Arc::new(Mutex::new(Box::new(RaGasCOMod::new()) as Box<Messzelle>));
        let mut zone = Zone::new();

        zone.add_messzelle(messzelle1);
        zone.add_messzelle(messzelle2);
        assert_eq!(zone.messzellen.len(), 2);
    }
}
