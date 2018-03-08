//! Ausgänge die vom Server Prozess geschalten werden können (z.B. LEDs, Relais, IO Module)
//!
//! Module mit den Traits und Moulen die Schaltausgänge z.B. Relais/LEDs an ShiftRegister, aber
//! auch Relais an "Metz Connect"-DO4 Schaltmodulen.
//!
mod error;

pub use self::error::OutputError;
