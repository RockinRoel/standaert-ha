use serialport::prelude::*;
use standaertha_mqtt_gateway::PackageInputStream;
use std::io::Read;
use std::time::Duration;

fn main() {
    let s = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1000),
    };
    let serial =
        serialport::open_with_settings("/dev/ttyUSB0", &s).expect("Could not open /dev/ttyUSB0");
    for package in PackageInputStream::new(serial.bytes()) {
        println!("{:?}", package);
    }
}
