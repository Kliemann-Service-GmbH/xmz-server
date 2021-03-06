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
//! Der Server ist die Kern Komponente der ['xMZ-Plattform'](https://github.com/Kliemann-Service-GmbH/xMZ-Plattform).
//! Zu seinen Aufgaben zählen zum Beispiel das Auslesen der Sensoren sowie der Auswertung der
//! Sensor-Messzellen und das Schalten der diversen Ausgänge (Outputs), wie etwa Relais, LEDs und
//! angeschlossene IO Module.
//!
//! * **Dokumentation:** [https://kliemann-service-gmbh.github.io/xmz-server](https://kliemann-service-gmbh.github.io/xmz-server)
//! * **Quellcode:** [https://github.com/Kliemann-Service-GmbH/xmz-server](https://github.com/Kliemann-Service-GmbH/xmz-server)
//!
//! # Struktur des Servers
//!
//! * Server
//!     * `<Sensor>`        - Ein `Server` kann n Sensoren verwalten
//!         * `<Messzelle>` - ein `Sensor` besizt n `Messzelle`e
//!     * `<Zonen>`
//!         * `<&Messzellen`> - eine `Zone` besitzt n Verweise auf `Messzelle`n
//!         * `<Schaltpunkt>` - eine `Zone` besitzt n `Schaltpunkt`e
//!             * `<Schwellwert>` - ein `Schaltpunkt` besitzt n `Schwellwert`e
//!             * `<Aktion>`  - ein `Schaltpunkt` besitzt n `Aktion`en
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

mod api;
mod config; // Konfiguration via Umgebungsvariablen: https://boats.gitlab.io/blog/post/2018-01-18-configure/
mod error;  // Mögliche Fehler die im Serverbetrieb auftreten können
pub mod action; // Liste von zu schaltenden Ausgängen (`output`)
pub mod configuration; // Datenstruktur zur Erstellung des Servers aus der Konfigurationsdatei `xmz-server.toml`
pub mod messzelle; // Einzelne Sensor Messzelle, sitzt in der Regel auf einer Sensor Platine (`sensor`)
pub mod output; // Ausgänge die vom Server Prozess geschalten werden können (z.B. LEDs, Relais, IO Module)
pub mod prelude; // Nützliche Traits und Funktionen die alle Teile dieses Projekts verwenden
pub mod runtime_info; // Datentrukturen zur Erstellen der Server Instanz aus den Laufzeitinformationen
pub mod schaltpunkt; // Liste von Schwellwerten (`schwellwert`) und Aktionen (`aktion`)
pub mod schwellwert; // Regel die wenn erfüllt zumeist Ausgänge schaltet
pub mod sensor; // Trait das die Eigenschaften aller vom Server unterstützten Sensoren beschreibt.
pub mod server; // Kernkomponente dieser Anwendung
pub mod zone; // Zonen die vom Server überwacht werden

pub use config::Config;
pub use error::ServerError;
pub use messzelle::Messzelle;
pub use sensor::Sensor;
pub use server::Server;
pub use zone::Zone;
