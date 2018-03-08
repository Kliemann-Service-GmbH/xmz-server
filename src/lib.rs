#![doc(html_logo_url = "https://zzeroo.github.io/share/zzeroo-logo.png",
       html_favicon_url = "https://zzeroo.github.io/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
//! Server Komponente der **xMZ-Plattform**
//!
//! |&nbsp;[![Build Status](https://travis-ci.org/Kliemann-Service-GmbH/xmz-server.svg?branch=master)](https://travis-ci.org/Kliemann-Service-GmbH/xmz-server)&nbsp;<sub>**master**</sub>
//! |&nbsp;[![Build Status](https://travis-ci.org/Kliemann-Service-GmbH/xmz-server.svg?branch=development)](https://travis-ci.org/Kliemann-Service-GmbH/xmz-server)&nbsp;<sub>**development**</sub>
//!
//! Der Server ist die Kern Komponente. Zu seinen Aufgaben zählen zum Beispiel das Auslesen der
//! Sensoren sowie der Auswertung der Sensor-Messzellen und das Schalten der diversen Ausgänge (Outputs),
//! wie etwa Relais, LEDs und angeschlossene IO Module.
//!
//! * **Dokumentation:** [https://kliemann-service-gmbh.github.io/xmz-server](https://kliemann-service-gmbh.github.io/xmz-server)
//! * **Quellcode:** [https://github.com/Kliemann-Service-GmbH/xmz-server](https://github.com/Kliemann-Service-GmbH/xmz-server)
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


mod error;              // Mögliche Fehler die im Serverbetrieb auftreten können
mod settings;           // Einstellungen die beim Serverstart ausgewertet werden, Wrapper um Config crate
pub mod action;         // Liste von zu schaltenden Ausgängen (`output`)
pub mod messzelle;      // Einzelne Sensor Messzelle, sitzt in der Regel auf einer Sensor Platine (`sensor`)
pub mod output;         // Ausgänge die vom Server Prozess geschalten werden können (z.B. LEDs, Relais, IO Module)
pub mod prelude;        // Nützliche Traits und Funktionen die alle Teile dieses Projekts verwenden
pub mod schaltpunkt;    // Liste von Schwellwerten (`schwellwert`) und Aktionen (`aktion`)
pub mod schwellwert;    // Regel die wenn erfüllt zumeist Ausgänge schaltet
pub mod sensor;         // Sensoren die vom Server unterstützt werden
pub mod server;         // Kern der Anwendung
pub mod zone;           // Zonen die vom Server überwacht werden

pub use error::ServerError;
pub use messzelle::Messzelle;
pub use server::Server;
pub use settings::Settings;
pub use sensor::Sensor;
pub use zone::Zone;
