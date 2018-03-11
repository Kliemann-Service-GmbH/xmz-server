//! Nützliche Traits und Funktionen die alle Teile dieses Projekts verwenden
//!
//! # Examples
//! ```
//! use xmz_server::prelude::*;
//! ```

use std::any::Any;
// Reexports
pub use server::Server;
pub use settings::Settings;
pub use std::sync::{Arc, Mutex};
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
pub fn id<T>(x: T) -> T { x }


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
