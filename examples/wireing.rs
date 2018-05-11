use std::fmt;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;


trait Sensor: fmt::Debug { fn update(&mut self); }

#[derive(Debug)]
struct CO { value: i32, }
impl CO { fn new() -> Self { CO { value: 0 } } }
impl Sensor for CO { fn update(&mut self) { self.value += 10; } }
#[derive(Debug)]
struct NOx { value: i32, }
impl NOx { fn new() -> Self { NOx { value: 0 } } }
impl Sensor for NOx { fn update(&mut self) { self.value += 1; } }

struct Api {}
impl Api {
    fn launch(server: Server) {
        println!("start api");
        let server = server.clone();
        thread::spawn(move || loop {
            for sensor in server.sensors.clone() {
                if let Ok(sensor) = sensor.read() {
                    println!("{:?}", sensor);
                }
                thread::sleep(Duration::from_millis(2));
            }
        });
    }
}

#[derive(Clone)]
struct Server {
    sensors: Vec<Arc<RwLock<Box<Sensor + Send + Sync>>>>,
}
impl Server {
    fn new() -> Self {
        Server {
             sensors: vec![
                Arc::new(RwLock::new(Box::new(CO::new()))),
                Arc::new(RwLock::new(Box::new(NOx::new()))),
             ],
        }
    }
    fn update(&self) {
        println!("Update Server");
        let sensors = self.sensors.clone(); // clone the vector for the thread
        thread::spawn(move || loop {
            for sensor in sensors.clone() {
                if let Ok(mut sensor) = sensor.write() {
                    sensor.update();
                }
            }
        });
    }
    // Die Api
    fn start_api(&self) {
        Api::launch(self.clone());
    }
}


fn main() {
    let server = Server::new();
    server.update(); // thread 1
    server.start_api(); // thread 2
    // take some time, in real world both threads, from above, should run forever
    thread::sleep(Duration::from_millis(10));
}
