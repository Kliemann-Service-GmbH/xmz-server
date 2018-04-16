extern crate xmz_server;

use xmz_server::prelude::*;


#[test]
fn server_create() {
    let server = Server::new();

    assert_eq!(server.service_interval, 1);
    assert_eq!(server.sensors.len(), 2);
}
//
// #[test]
// fn server_update_sensors() {
//     let server = Server::new();
//     server.update_sensors();
// }
//
// #[test]
// fn server_add_sensor() {
//     let mut server = Server::new();
//     let sensor = RaGasCONO2Mod::new();
//     assert_eq!(server.sensors.len(), 0);
//     server.add_sensor(Arc::new(Mutex::new(Box::new(sensor))));
//     assert_eq!(server.sensors.len(), 1);
// }
//
// #[test]
// fn server_add_two_sensor() {
//     let mut server = Server::new();
//     let sensor1 = RaGasCONO2Mod::new();
//     let sensor2 = MetzCI4::new();
//     assert_eq!(server.sensors.len(), 0);
//     server.add_sensor(Arc::new(Mutex::new(Box::new(sensor1))));
//     server.add_sensor(Arc::new(Mutex::new(Box::new(sensor2))));
//     assert_eq!(server.sensors.len(), 2);
// }
//
// #[test]
// fn server_get_sensor() {
//     let server = Server::new();
//     assert!(server.get_sensor(0).is_none());
// }
