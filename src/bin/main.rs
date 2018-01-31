extern crate xmz_server;
use xmz_server::{Server, Sensor, Zone};

fn main() {
    let mut server = Server::new();
    let sensor1 = Sensor::new(1);
    let sensor2 = Sensor::new(2);
    let sensor3 = Sensor::new(3);

    let zone1 = Zone::new();
    let zone2 = Zone::new();

    server.add_sensor(sensor1);
    server.add_sensor(sensor2);
    server.add_sensor(sensor3);
    server.add_zone(zone1);
    server.add_zone(zone2);

    server.link_sensor_zone(0, 0);

    server.run();
}
