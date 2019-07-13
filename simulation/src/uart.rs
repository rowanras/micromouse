use std::time::Duration;

use std::sync::mpsc::Sender;
use std::thread;

use serialport::DataBits;
use serialport::FlowControl;
use serialport::Parity;
use serialport::SerialPortSettings;
use serialport::StopBits;

pub fn start<Msg: 'static + Send>(
    tx: Sender<Msg>,
    msg: fn(u32, i32, i32) -> Msg,
) {
    thread::spawn(move || {
        let serial_settings = SerialPortSettings {
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_secs(1),
        };

        if let Ok(mut port) =
            serialport::open_with_settings("/dev/ttyUSB0", &serial_settings)
        {
            port.write_all(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
                .expect("Could not write bytes");

            let mut leftover_bytes = Vec::new();
            port.read_to_end(&mut leftover_bytes);

            port.write_all(&[0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00])
                .expect("Could not write bytes");

            let mut buffer = [0; 8];

            loop {
                port.read_exact(&mut buffer)
                    .expect("Could not read from port");

                let time = ((buffer[3] as u32) << 16)
                    | ((buffer[2] as u32) << 8)
                    | (buffer[1] as u32);
                let left = ((buffer[5] as i32) << 8) | (buffer[4] as i32);
                let right = ((buffer[7] as i32) << 8) | (buffer[6] as i32);

                tx.send(msg(time, left, right));
            }
        } else {
            println!("Could not open serial port");
        }
    });
}
