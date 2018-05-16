//! Ausgänge die vom Server Prozess geschalten werden können (z.B. LEDs, Relais, IO Module)
//!
//! Die Outputs sind ganz ähnlich wie die Sensoren implementiert.
//! - Der Server besitzt n Outputs (Vector aus Tait Objekten)
//!     - die Output Trait Objekte sind in Arc Container gekapselt damit sie thread safe werden
//!     - Im Unterschied zu den Sensoren sind die Trait Objekte nicht auch noch in ein `Rwock`

use prelude::*;

mod error;
mod metz_connect_mr_do4;
mod xmz_bodenplatine_100;
mod xmz_deckelplatine_100;
// Reexports
pub use self::error::OutputError;
pub use self::metz_connect_mr_do4::MetzConnectMRDO4;
pub use self::xmz_bodenplatine_100::XMZBoden100;
pub use self::xmz_deckelplatine_100::XMZDeckel100;

pub type BoxedOutput = Box<Output + Send + Sync>;
pub type OutputList = Vec<Arc<BoxedOutput>>;


/// Verfügbare Output Typen
///
/// Von der 'xMZ-Plattform' unterstützte Output Module.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OutputType {
    #[serde(rename="xMZ-Mod-Touch-Deckelplatine v1.0.0")]
    XMZDeckel100,
    #[serde(rename="xMZ-Mod-Touch-Bodenplatine v1.0.0")]
    XMZBoden100,
    #[serde(rename="Metz Connect MR-DO4")]
    MetzConnectMRDO4,
}

/// Alle Äusgänge müssen diesen Trait implementieren
///
/// Ausgänge sind z.B. die ShiftRegister der xMZ-Mod-Touch-Bodenplatine v1.0.0 die die Relais
/// dieser Platine steuern. Oder Metz Connect MR-DO4 Schaltmodule mit je 4 schaltbaren Relais.
pub trait Output: fmt::Debug + fmt::Display {
    /// Schaltet den `num` Ausgang, ein
    ///
    /// Die Implementation muss ein Fehler zurück geben, wenn der Ausgang nicht geschalten werden konnte
    fn set(&self, num: usize) -> Result<(), OutputError>;

    /// Liefer den aktuellen Status des `num` Ausgang, liefert ein boolean Wert
    ///
    /// Die Implementation muss ein Fehler zurück geben, wenn der Ausgang nicht gelesen werden konnte
    fn get(&self, num: usize) -> Result<bool, OutputError>;

    /// Schaltet den `num` Ausgang, aus
    ///
    /// Die Implementation muss ein Fehler zurück geben, wenn der Ausgang nicht geschalten werden konnte
    fn clear(&self, num:usize) -> Result<(), OutputError>;
}
