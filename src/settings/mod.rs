use config::{Config, ConfigError, File};

// Type Alias für besser lesbare Konfigurationsparameter
type Days = u32;

/// Serverkonfigurations Parameter
#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Wartungsintervall in Tagen
    service_interval: Days,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("/boot/xmz.hjson").required(false))?;
        s.merge(File::with_name("xmz.hjson").required(false))?;

        s.try_into()
    }
}
