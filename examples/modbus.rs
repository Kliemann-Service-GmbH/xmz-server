extern crate libmodbus_rs;

use libmodbus_rs::{
    Modbus,
    ModbusRTU,
    ModbusClient,
    RequestToSendMode,
};
use libmodbus_rs::Error as ModbusError;


struct Output<'o> {
    device: &'o str,
    baud: i32,
    data_bit: i32,
    parity: char,
    stop_bit: i32,
    ctx: Option<Modbus>,
}
impl<'o> Output<'o> {
    fn new() -> Self {
        Output {
            // device: "/dev/ttyUSB0",
            device: "/dev/ttyS1",   // xMZ-Plattform
            baud: 9600,
            data_bit: 8,
            parity: 'N',
            stop_bit: 1,
            ctx: None,
        }
    }
    fn init_ctx(&mut self) {
        let ctx = Modbus::new_rtu(self.device, self.baud, self.parity, self.data_bit, self.stop_bit).unwrap();
        self.ctx = Some(ctx);
    }
}


fn main() {
    let mut outputs: Vec<Output> = (0..11).map(|_i| {
        Output::new()
    }).collect();

    for mut output in &mut outputs {
        output.init_ctx();
    }

    for output in &mut outputs {
        if let Some(ref mut ctx) = output.ctx {
            let mut buffer = vec![0u8; 10];

            ctx.set_slave(36).expect("Could not set slave id");
            // ctx.set_debug(true).expect("Could not set debug mode");
            // ctx.rtu_set_rts(RequestToSendMode::RtuRtsNone).expect("");
            // ctx.rtu_set_rts(RequestToSendMode::RtuRtsUp).expect("");
            ctx.rtu_set_rts(RequestToSendMode::RtuRtsDown).expect("");   // xMZ-Plattform benötigt RequestToSendMode Down
            ctx.connect().expect("Could not connect");

            ctx.write_bit(0, true).expect("Could not write bit");
            ctx.read_bits(0, 1, &mut buffer).expect("Could not read");
            std::thread::sleep(std::time::Duration::from_millis(30));
            ctx.write_bit(0, false).expect("Could not write bit");
        }
    }
}
