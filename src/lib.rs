#![doc(html_logo_url = "https://zzeroo.github.io/share/zzeroo-logo.png",
       html_favicon_url = "https://zzeroo.github.io/share/favicon.ico",
html_root_url = "https://gaswarnanlagen.com/")]
//! Server Komponente der **xMZ-Plattform**.
//!
//! Der Server ist die Kern Komponente. Zu seinen Aufgaben zählen zum Beispiel das Auslesen der
//! Sensoren sowie deren Auswertung und das Schalten der diversen Ausgänge, wie Relais und IO Module.
//!

#[macro_use] extern crate serde_derive;

extern crate config;
extern crate rand;
extern crate serde;

mod error;
mod settings;
pub mod output;
pub mod sensor;
pub mod server;

pub use error::ServerError;
pub use server::Server;
pub use settings::Settings;
