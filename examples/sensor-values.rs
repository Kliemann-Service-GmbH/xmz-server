type SensorsList = Vec<Sensor>;
type SensorValues = Vec<SensorValue>;

#[derive(Debug)]
struct SensorValue {
    value: f64,
}
impl SensorValue {
    fn update(&self) {
        println!("Update SensorValue");
    }
}
#[derive(Debug)]
struct Sensor {
    sensor_values: SensorValues,
}
impl Sensor {
    // Update Sensor Platine via BUS
    fn update(&self) {
        println!("Update Sensor");
        for sensor_value in &self.sensor_values {
            sensor_value.update()
        }
    }
}

#[derive(Debug)]
struct Server {
    sensors: SensorsList,
}
impl Server {
    fn update_sensors(&self) {
        for sensor in &self.sensors {
            sensor.update();
        }
    }
}


fn main() {

}
