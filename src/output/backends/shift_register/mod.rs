mod error;

pub use self::error::ShiftRegisterError;
use sysfs_gpio::{Direction, Pin};


#[derive(Clone, Debug)]
pub struct ShiftRegister {
    oe_pin: u64,
    ds_pin: u64,
    clock_pin: u64,
    latch_pin: u64,
}

impl ShiftRegister {
    /// Erzeugt ein neuen Shift Register
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn new(oe_pin: u64, ds_pin: u64, clock_pin: u64, latch_pin: u64,) -> Self {
        ShiftRegister {
            oe_pin,
            ds_pin,
            clock_pin,
            latch_pin,
        }
    }

    /// Schiebt die übergebenen Daten in die Schiebe Register
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn shift_out(&self, data: usize) -> Result<(), ShiftRegisterError> {
        // Wenn export_pins erfolgreich ist werden die Daten eingeclocked, ansonsten passiert nix
        self.export_pins()?;
        self.set_pin_direction_output()?;

        // Daten einclocken
        for i in (0..64).rev() {
            match (data >> i) & 1 {
                1 => { Pin::new(self.ds_pin).set_value(1)? },
                _ => { Pin::new(self.ds_pin).set_value(0)? },
            }
            self.clock_in()?;
        }
        self.latch_out()?;

        Ok(())
    }


    /// Exportiert die Pins in das sysfs des Linux Kernels
    ///
    fn export_pins(&self) -> Result<(), ShiftRegisterError> {
        Pin::new(self.oe_pin).export()?;
        Pin::new(self.ds_pin).export()?;
        Pin::new(self.clock_pin).export()?;
        Pin::new(self.latch_pin).export()?;

        Ok(())
    }

    /// Schaltet die Pins in den OUTPUT Pin Modus
    ///
    fn set_pin_direction_output(&self) -> Result<(), ShiftRegisterError> {
        Pin::new(self.oe_pin).set_direction(Direction::Out)?;
        Pin::new(self.oe_pin).set_value(0)?; // !OE pin low == Shift register enabled.
        Pin::new(self.ds_pin).set_direction(Direction::Out)?;
        Pin::new(self.ds_pin).set_value(0)?;
        Pin::new(self.clock_pin).set_direction(Direction::Out)?;
        Pin::new(self.clock_pin).set_value(0)?;
        Pin::new(self.latch_pin).set_direction(Direction::Out)?;
        Pin::new(self.latch_pin).set_value(0)?;

        Ok(())
    }

    /// Toggelt den Latch Pin pin high->low,
    ///
    fn latch_out(&self) -> Result<(), ShiftRegisterError> {
        Pin::new(self.latch_pin).set_value(1)?;
        Pin::new(self.latch_pin).set_value(0)?;

        Ok(())
    }

    /// Toogelt den Clock Pin high->low
    ///
    fn clock_in(&self) -> Result<(), ShiftRegisterError> {
        Pin::new(self.clock_pin).set_value(1)?;
        Pin::new(self.clock_pin).set_value(0)?;

        Ok(())
    }
}
