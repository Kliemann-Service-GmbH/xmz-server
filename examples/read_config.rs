extern crate xmz_server;
extern crate toml;
#[macro_use] extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
enum ConfigError {
    ConfigurationNotFound(std::io::Error),
    ParseError(toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::ConfigurationNotFound(ref error) => write!(f, "Konfigurationsdatein nicht gefunden: '{:?}'", error),
            ConfigError::ParseError(ref error) => write!(f, "Konnte Konfigurationsdatein nicht lesen: '{:?}'", error),
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::ConfigurationNotFound(ref error) => "Konfigurationsdatein nicht gefunden",
            ConfigError::ParseError(ref error) => "Konnte Konfigurationsdatein nicht lesen",
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::ConfigurationNotFound(error)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        ConfigError::ParseError(error)
    }
}




#[derive(Debug, Deserialize)]
struct ServerConfig {
    service_interval: i32,
}

#[derive(Debug, Deserialize)]
struct Sensor {
    messzellen: Vec<Messzelle>,
}

#[derive(Debug, Deserialize)]
struct Messzelle {

}
#[derive(Debug, Deserialize)]
struct Configuration {
    server: ServerConfig,
    sensors: Vec<Sensor>,
}


fn openconfig() -> Result<(), ConfigError> {
    let mut configfile = File::open("xmz-server.toml")?;
    let mut configstr = String::new();
    configfile.read_to_string(&mut configstr);
    let mut configuration: Configuration = toml::from_str(&configstr)?;
    println!("{:?}", configuration);

    Ok(())
}

fn main() {
    if let Err(err) = openconfig() {
        println!("Fehler: {:?}", err);
        std::process::exit(1);
    }
}
