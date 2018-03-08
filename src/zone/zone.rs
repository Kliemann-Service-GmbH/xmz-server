use prelude::*;
use schaltpunkt::Schaltpunkt;
use messzelle::{Messzelle, RaGasNO2Mod, RaGasCOMod};


/// Zonen, die Bereiche die der Server überwacht
///
/// Jeder Zone können n `Schaltpunkte` zugeordnet
/// werden. Diese Messzellen werden gegen
pub struct Zone {
    messzellen: Vec<Box<Messzelle>>,
    schaltpunkte: Vec<Schaltpunkt>,
}

impl Zone {
    pub fn new() -> Self {
        Zone {
            messzellen: vec![],
            schaltpunkte: vec![],
        }
    }

    /// Fügt eine Referenz auf eine Messzelle hinzu
    ///
    fn add_messzelle(&mut self, messzelle: Box<Messzelle>) {
        self.messzellen.push(messzelle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let zone = Zone::new();
        assert_eq!(zone.messzellen.len(), 0);
        assert_eq!(zone.schaltpunkte.len(), 0);
    }

    #[test]
    #[ignore]
    fn add_messzelle() {
        let messzelle = RaGasNO2Mod::new();
        let mut zone = Zone::new();
        zone.add_messzelle(Box::new(messzelle));
        assert_eq!(zone.messzellen.len(), 1);
    }


    #[test]
    #[ignore]
    fn add_more_messzellen() {
        let messzelle1 = RaGasNO2Mod::new();
        let messzelle2 = RaGasCOMod::new();
        let mut zone = Zone::new();
        zone.add_messzelle(Box::new(messzelle1));
        zone.add_messzelle(Box::new(messzelle2));
        assert_eq!(zone.messzellen.len(), 2);
    }
}
