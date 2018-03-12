use config::{Config, ConfigError, File};

// Type Alias für besser lesbare Konfigurationsparameter
type Days = u32;

#[derive(Debug, Deserialize)]
pub struct Server {
    /// Wartungsintervall in Tagen
    pub service_interval: Days,
    pub api_url: String,
}

/// Serverkonfigurations Parameter
///
/// Die Settings werden mit dem `config` crate gebildet.
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("/boot/xmz.toml").required(false))?;
        s.merge(File::with_name("xmz.toml").required(false))?;

        s.try_into()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let settings = Settings::new();
        assert!(settings.is_ok());
    }

    #[test]
    fn server() {
        let settings = Settings::new().unwrap();
        assert_eq!(settings.server.api_url, "0.0.0.0:3000");
    }
}
