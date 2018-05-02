#![doc(
    html_logo_url = "https://zzeroo.github.io/share/zzeroo-logo.png",
    html_favicon_url = "https://zzeroo.github.io/share/favicon.ico",
    html_root_url = "https://gaswarnanlagen.com/"
)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

//! Server Komponente der **xMZ-Plattform**
//!
//! |||
//! |:---|:------|
//! |**master:**|[![Build Status](https://travis-ci.org/Kliemann-Service-GmbH/xmz-server.svg?branch=master)](https://travis-ci.org/Kliemann-Service-GmbH/xmz-server)&nbsp;[![Code Coverage](https://codecov.io/gh/Kliemann-Service-GmbH/xmz-server/branch/master/graph/badge.svg)](https://codecov.io/gh/Kliemann-Service-GmbH/xmz-server)|
//! |**development:**|[![Build Status](https://travis-ci.org/Kliemann-Service-GmbH/xmz-server.svg?branch=development)](https://travis-ci.org/Kliemann-Service-GmbH/xmz-server)&nbsp;[![Code Coverage](https://codecov.io/gh/Kliemann-Service-GmbH/xmz-server/branch/development/graph/badge.svg)](https://codecov.io/gh/Kliemann-Service-GmbH/xmz-server)|
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
extern crate configure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate toml;

pub mod action; // Liste von zu schaltenden Ausgängen (`output`)
mod api;
mod config; // Konfiguration via Umgebungsvariablen: https://boats.gitlab.io/blog/post/2018-01-18-configure/
mod configuration; // Datenstructur zur Erstellung des Servers aus der Konfiguration
mod error;  // Mögliche Fehler die im Serverbetrieb auftreten können
pub mod messzelle; // Einzelne Sensor Messzelle, sitzt in der Regel auf einer Sensor Platine (`sensor`)
pub mod output; // Ausgänge die vom Server Prozess geschalten werden können (z.B. LEDs, Relais, IO Module)
pub mod prelude; // Nützliche Traits und Funktionen die alle Teile dieses Projekts verwenden
pub mod schaltpunkt; // Liste von Schwellwerten (`schwellwert`) und Aktionen (`aktion`)
pub mod schwellwert; // Regel die wenn erfüllt zumeist Ausgänge schaltet
pub mod sensor; // Trait das die Eigenschaften aller vom Server unterstützten Sensoren beschreibt.
pub mod server; // Kernkomponente dieser Anwendung
pub mod server_builder; // Konstruiert eine Server Instanz aus der letzten Laufzeit Information oder einer Bootrstrpping Konfigurationsdatei
pub mod zone; // Zonen die vom Server überwacht werden

pub use config::Config;
pub use error::ServerError;
pub use messzelle::Messzelle;
pub use sensor::Sensor;
pub use server::Server;
pub use server_builder::ServerBuilder;
pub use zone::Zone;
