#![doc(html_logo_url = "https://zzeroo.github.io/share/zzeroo-logo.png",
       html_favicon_url = "https://zzeroo.github.io/share/favicon.ico",
html_root_url = "https://gaswarnanlagen.com/")]
//! Server Komponente der **xMZ-Plattform**.
//!
//! Der Server ist die Kern Komponente. Zu seinen Aufgaben zählen zum Beispiel das Auslesen der
//! Sensoren sowie deren Auswertung und das Schalten der diversen Ausgänge, wie etwa Relais, LEDs
//! und angeschlossene IO Module.
//!

#[macro_use] extern crate serde_derive;

extern crate config;
extern crate rand;
extern crate serde;

mod error;
mod settings;
/// Ausgänge die vom Server Prozess geschalten werden können (LEDs, Relais, IO Module)
pub mod output;
/// Sensoren die vom Server unterstützt werden
pub mod sensor;
/// Kern der Anwendung
pub mod server;

pub use error::ServerError;
pub use server::Server;
pub use settings::Settings;
