//! Nützliche Traits und Funktionen die alle Teile dieses Projekts verwenden
//!
//! # Examples
//! ```
//! use xmz_server::prelude::*;
//! ```

use std::any::Any;

// Oft benötigte crates
pub use std::time::Duration;
pub use std::fmt;

// Reexports
pub use config::Config;
pub use configuration;
pub use error::ServerError;
pub use messzelle::{BoxedMesszelle, Messzelle, MesszelleList, MesszelleType, MetzConnectCI4Analog420, RaGasNO2Mod, RaGasCOMod};
pub use output;
pub use output::{MetzConnectMRDO4, Output, OutputError, OutputList, OutputType, XMZBoden100, XMZDeckel100};
pub use runtime_info;
pub use sensor::{BoxedSensor, MetzConnectCI4, RaGasCONO2Mod, Sensor, SensorList, SensorType, TestSensor};
pub use server::Server;
pub use std::error::Error;
pub use std::sync::{Arc, Mutex, RwLock};
pub use std::thread;
pub use zone::Zone;

/// Die `id` Funktion liefert genau den Wert zurück der auch in die Funktion gegeben wurde.
///
/// https://bluss.github.io/rust/fun/2015/10/11/stuff-the-identity-function-does/
///
/// # Examples
/// ```
/// use xmz_server::prelude::*;
///
/// assert_eq!(1, id(1));
/// ```
pub fn id<T>(x: T) -> T {
    x
}

/// Dieser Trait ist für das Upcasting nötig
///
/// * https://stackoverflow.com/questions/42056422/using-any-with-traits-in-rust
/// * https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting
/// * https://github.com/rust-lang/rust/issues/5665
pub trait AsAny: Any {
    fn as_any(&self) -> &Any;
}

/// Implementiere AsAny für alle Typen
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &Any {
        self
    }
}
