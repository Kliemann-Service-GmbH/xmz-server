/// Alle Ausgänge implementieren das `Output` Trait.
/// Die eigentlichen Geräte Stukturen haben den `data` Speicher gemeinsam.
/// `data` ist eine in ein `RwLock` gekapselte `usize` Variable.
/// Das `Output` Trait erlaubt das Ändern des gemeinsamen Speichers (data).
/// In den konkreten Implementationen der Output Funktionen (z.B. set()) werden
/// dann die Funktionen der konreten Hardware aufgerufen.
/// Wie shift_out für ShiftRegister ...
/// Je nach Gerät werden dann die ShiftRegister oder Modbus Details modeliert.
/// Der Server hält die Output Trait Objekte in einem Arc Container, so das diese
/// thread safe sind.

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;


trait Output {
    fn set(&self, num: usize) -> Result<(), String>;
    // fn get(&self, num: usize) -> Result<bool, String>;
    // fn clear(&self, num:usize) -> Result<(), String>
}
// xMZ-Mod-Touch-Bodenplatine v1.0.0
struct XMZBoden100 {
    data: RwLock<usize>,
}
impl XMZBoden100 {
    fn new() -> Self {
        XMZBoden100 {
            data: RwLock::new(0),
        }
    }
    fn shift_out(&self) -> Result<(), String> {
        if let Ok(data) = self.data.read() {
            println!("latch out: {:0b}", *data);
        }

        Ok(())
    }
}
impl Output for XMZBoden100 {
    // TODO: alten data Wert speichern, wenn shift_out failed Wert wieder herstellen
    fn set(&self, num: usize) -> Result<(), String> {
        println!("xMZ-Mod-Touch-Bodenplatine set: {}", &num);
        if let Ok(mut data) = self.data.write() {
            *data |= 1 << (num - 1);
        }

        self.shift_out()?;

        Ok(())
    }
}
// MetzConnect MR-DO4
struct MRDO4 {
    data: RwLock<usize>,
}
impl MRDO4 {
    fn new() -> Self {
        MRDO4 {
            data: RwLock::new(0),
        }
    }
}
impl Output for MRDO4 {
    fn set(&self, num: usize) -> Result<(), String> {
        println!("Metz Connect MR-DO4 set: {}", &num);
        if let Ok(mut data) = self.data.write() {
            *data |= 1 << (num - 1);
        }
        // Relais 1 - 4 => Modul 1 (Modbus Adresse 34)
        // Relais 5 - 8 => Modul 2 (Modbus Adresse 35)
        // Relais 9 - 12 => Modul 3 (Modbus Adresse 36)

        Ok(())
    }
}
#[derive(Clone)]
struct Server {
    leds: Arc<Box<Output + Send + Sync>>,
    relais: Arc<Box<Output + Send + Sync>>,
}
impl Server {
    fn new() -> Self {
        Server {
            leds: Arc::new(Box::new(XMZBoden100::new())),
            relais: Arc::new(Box::new(MRDO4::new())),
        }
    }

    fn run(&self) -> Result<(), String> {
        let mut i = 1;
        let server = self.clone();
        thread::spawn(move || loop {
            server.leds.set(i).unwrap();
            server.relais.set(i).unwrap();

            i += 1;
            thread::sleep(Duration::from_millis(1));
        });

        Ok(())
    }
}


fn main() -> Result<(), String> {
    let server = Server::new();

    server.run()?;

    thread::sleep(Duration::from_millis(10));
    Ok(())
}
