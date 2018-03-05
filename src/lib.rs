#![doc(html_logo_url = "https://zzeroo.github.io/share/zzeroo-logo.png",
       html_favicon_url = "https://zzeroo.github.io/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
//! Server Komponente der **xMZ-Plattform**.
//!
//! Der Server ist die Kern Komponente. Zu seinen Aufgaben zählen zum Beispiel das Auslesen der
//! Sensoren sowie der Auswertung der Sensor-Messzellen und das Schalten der diversen Ausgänge (Outputs),
//! wie etwa Relais, LEDs und angeschlossene IO Module.
//!
//! # Struktur des Servers
//!
//! * Server
//!     * `<Sensore>`   - Ein Server kann n Sensoren verwalten
//!         * `<Messzellen>` (n Messzellen)
//!     * `<Zonen>`
//!         * `<&Messzellen`> (n Verweise auf Sensor-Messzellen)
//!         * `<Schaltpunkt>` (n Schaltpunkte)
//!             * `<Schwellwert>` (n Schwellwerte)
//!             * `<Aktion>` (n Aktionen)
//!

#[macro_use]
extern crate serde_derive;

extern crate config;
extern crate rand;
extern crate serde;


pub mod prelude;
mod error;
mod settings;
/// Ausgänge die vom Server Prozess geschalten werden können (LEDs, Relais, IO Module)
pub mod output;
/// Sensoren die vom Server unterstützt werden
pub mod sensor;
pub mod messzelle;
/// Kern der Anwendung
pub mod server;

pub use error::ServerError;
pub use server::Server;
pub use settings::Settings;
pub use self::messzelle::Messzelle;
