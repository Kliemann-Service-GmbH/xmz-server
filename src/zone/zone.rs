use messzellen::MesszellenRefList;


/// Zonen
///
/// Zonen sind die Bereiche die der Server überwacht. Jeder Zone können n `Messzellen` zugeordnet
/// werden. Diese Messzellen werden gegen
struct Zone<'a> {
    messzellen: MesszellenRefList<'a>,
}

impl<'a> Zone<'a> {
    fn new() -> Self {
        Zone {
            messzellen: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let zone = Zone::new();
        assert_eq!(zone.messzellen.len(), 0);
    }
    
    #[test]
    fn add_messzelle() {
        let mut server = Server::new();
        let sensor = RaGasCONO2Mod::new();
        server.add_sensor(Arc::new(Mutex::new(Box::new(sensor))));

        let mut zone = Zone::new();
        assert_eq!(zone.messzellen.len(), 0);

    }
}
