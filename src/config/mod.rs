use std::path::PathBuf;

#[derive(Debug, Deserialize, Configure)]
#[serde(default)]
pub struct Config {
    pub configuration_path: PathBuf,
    pub runtime_info_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            configuration_path: PathBuf::from("/boot/xmz-server.toml"),
            runtime_info_path: PathBuf::from("/var/cache/xmz-server/status"),
        }
    }
}

impl Config {
    /// Testet ob die Datei mit den Laufzeitinformationen existiert
    ///
    ///  Diese Funktion liefert auch `false` wenn auf die Datei nicht zugegriffen werden kann,
    ///  z.B. durch fehlende Berechtigungen.
    pub fn runtime_info_available(&self) -> bool {
        self.runtime_info_path.exists()
    }

    /// Testet ob die Konfigurationsdatei existiert
    ///
    ///  Diese Funktion liefert auch `false` wenn auf die Datei nicht zugegriffen werden kann,
    ///  z.B. durch fehlende Berechtigungen.
    pub fn config_file_available(&self) -> bool {
        self.configuration_path.exists()
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn runtime_info_available() {
        let cfg = Config::default();
        assert_eq!(cfg.runtime_info_available(), false);
    }

    #[test]
    fn config_file_available() {
        let cfg = Config::default();
        assert_eq!(cfg.config_file_available(), false);
    }
}
