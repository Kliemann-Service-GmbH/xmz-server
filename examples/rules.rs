use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &Any;
}

/// Implementiere AsAny für alle Typen
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &Any {
        self
    }
}
#[derive(Debug)]
enum SensorType {
    A,
    B,
}
trait Sensor: AsAny {
    fn value(&self) -> f64;
    fn average(&self, minutes: i32) -> f64;
}
#[derive(Debug)]
struct A { value: f64 }
impl Sensor for A {
    fn value(&self) -> f64 { self.value }
    fn average(&self, minutes: i32) -> f64 { self.value }
}
#[derive(Debug)]
struct B { value: f64 }
impl Sensor for B {
    fn value(&self) -> f64 { self.value }
    fn average(&self, minutes: i32) -> f64 { self.value }
}

// Schwellwerte
use std::fmt;


type FnType = fn(&(Sensor + Send + 'static)) -> f64;
struct Schwellwert {
    sensor_type: SensorType,
    threshold: f64,
    fun: FnType,
}
impl fmt::Debug for Schwellwert {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Schwellwert {{ SensorType: {:?}, Schwellwert: {:.02} }}", self.sensor_type, self.threshold)
   }
}
impl Schwellwert {
    fn new(sensor_type: SensorType, threshold: f64, fun: FnType) -> Self {
        Schwellwert { sensor_type, threshold, fun, }
    }

    fn check(&self, thing: &(Sensor + Send + 'static)) -> bool {
        match self.sensor_type {
            SensorType::A => {
                if let Some(ref A) = thing.as_any().downcast_ref::<A>() {
                    (self.fun)(thing) >= self.threshold
                } else {
                    false
                }
            }
            SensorType::B => {
                if let Some(ref B) = thing.as_any().downcast_ref::<B>() {
                    (self.fun)(thing) >= self.threshold
                } else {
                    false
                }
            }
        }
    }
}

fn main() {
    let a = A { value: 10.0 };
    let b = B { value: 10.0 };
    let rule1 = Schwellwert::new(SensorType::A, 1.0, Sensor::value);
    let rule2 = Schwellwert::new(SensorType::B, 1.0, Sensor::value);

    println!("| Schwellwert | Sensor | Auswertung |");
    println!("| ------------- | ------------- | --------------- |");
    println!("| {:?} | {:?} | {:?} |", rule1, &a, rule1.check(&a));
    println!("| {:?} | {:?} | {:?} |", rule1, &b, rule1.check(&b));
    println!("| {:?} | {:?} | {:?} |", rule2, &a, rule2.check(&a));
    println!("| {:?} | {:?} | {:?} |", rule2, &b, rule2.check(&b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let a = A { value: 10.0 };
        let b = B { value: 10.0 };
        let rule1 = Schwellwert::new(SensorType::A, 1.0, Sensor::value);
        let rule2 = Schwellwert::new(SensorType::B, 10.0, Sensor::value);

        rule1.check(&a);
        rule1.check(&b);

        rule2.check(&a);
        rule2.check(&b);
    }
}
