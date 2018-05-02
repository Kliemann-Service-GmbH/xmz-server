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
}
