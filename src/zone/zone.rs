use prelude::*;
use schaltpunkt::Schaltpunkt;


/// Zonen, die Bereiche die der Server überwacht
///
/// Jeder Zone können n `Schaltpunkte` zugeordnet
/// werden. Diese Messzellen werden gegen
pub struct Zone {
    schaltpunkte: Vec<Schaltpunkt>,
}

impl Zone {
    pub fn new() -> Self {
        Zone {
            schaltpunkte: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let zone = Zone::new();
        assert_eq!(zone.schaltpunkte.len(), 0);
    }

    #[test]
    #[ignore]
    fn add_messzelle() {
        let settings = Settings::new();
        let _server = Server::new(&settings.unwrap());
    }
}
