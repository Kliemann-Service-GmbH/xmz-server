use config::{Config, ConfigError, File};

use std::fmt;
use std::fmt::Display;


#[derive(Debug, Deserialize)]
pub struct Settings {
    service_interval: u32,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("/boot/xmz.hjson").required(false))?;

        s.merge(File::with_name("xmz.hjson").required(false))?;

        s.try_into()
    }
}
