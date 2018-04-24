/// Erzeugt eine neue Server Instanz aus Laufzeit Information oder Konfigurationsdatein
///
/// Beim allerersten Start des Servers wird die Server Instanz aus einer initialen
/// Konfigurationsdatein erstellt.
/// Konnte aus dieser ersten Konfiguration eine funktionale Server Intanz gestartet werden,
/// wird eine Datei mit den Laufzeit Informationen (der aktuelle Zustand des Servers) erzeugt.
/// Alle weiteren Starts des Servers verwenden diese Laufzeit Informationen.
/// Dies gewährleistet das Daten wie die Laufzeit "persistent" gespeichert werden können
/// (d.h. das diese nicht nach einem Neustart verloren gehen).
///
/// Siehe: [Dokumentation der 'xMZ-Plattform'](https://kliemann-service-gmbh.github.io/xmz-doc/


pub struct ServerBuilder;


impl ServerBuilder {
    pub fn runtime_info_available() -> bool {
        false
    }

    pub fn config_file_available() -> bool {
        false
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn runtime_info_available() {
        assert_eq!(ServerBuilder::runtime_info_available(), false);
    }

    #[test]
    fn config_file_available() {
        assert_eq!(ServerBuilder::config_file_available(), false);
    }

    #[test]
    fn no_config_no_runtim_is_error() {
        assert!(false)
    }
}
