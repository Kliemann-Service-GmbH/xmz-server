use std::thread;
use sensor::Sensor;
use settings::Settings;
use error::ServerError;

/// Struktur der Server Komponente
pub struct Server {
    sensors: Vec<Box<Sensor>>,
}

impl Server {
    pub fn new(_config: &Settings) -> Self {
        Server { sensors: vec![] }
    }

    pub fn start(self) -> Result<(), ServerError> {
        let guard = thread::spawn(move || {
            loop {
                // self.update_sensors();

                print!(".");
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
                ::std::thread::sleep(::std::time::Duration::from_millis(1000));
            }
        }).join();

        drop(guard);

        Ok(())
    }
}
