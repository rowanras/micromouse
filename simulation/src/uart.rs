use std::usize;

use std::time::Duration;
use std::time::Instant;

use std::io::Read;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use serialport::ClearBuffer;
use serialport::DataBits;
use serialport::FlowControl;
use serialport::Parity;
use serialport::SerialPort;
use serialport::SerialPortSettings;
use serialport::StopBits;

use crate::gui;
use micromouse_lib::msgs::Msg as MouseMsg;
use micromouse_lib::msgs::MsgId as MouseMsgId;
use micromouse_lib::msgs::ReadExact;
use micromouse_lib::msgs::WriteExact;
use micromouse_lib::msgs::ParseError;

pub struct Uart {
    serialport: Box<dyn SerialPort>,
    buf: Vec<u8>,
}

impl Uart {
    pub fn new(port: &str, settings: SerialPortSettings) -> Result<Uart, serialport::Error> {
        let mut serialport = serialport::open_with_settings(port, &settings)?;

        println!("Opened port");

        thread::sleep(Duration::from_secs(10));

        loop {
            println!("Disabling logging");
            serialport.write_all(&[MouseMsgId::Logged as u8, 0x00])?;
            serialport.flush();

            println!("Clearing buffer");
            serialport.clear(ClearBuffer::All);

            println!("Reading in leftover bytes");
            let mut leftovers = [0; 16];

            match serialport.read(&mut leftovers) {
                Ok(n) if n < leftovers.len() => {
                    println!("Bytes cleared!");
                    break;
                }

                Err(e) => {
                    println!("Bytes cleared!");
                    break;
                }

                _ => {
                    println!("Bytes not clear, trying again");
                }
            }

            println!("Bytes are still coming, so logging was not disabled. Trying again...");
        }

        Ok(Uart {
            serialport,
            buf: Vec::new(),
        })
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    fn read_to_buffer(&mut self) {
        let mut buf = [0; 5];
        if let Ok(n) = self.serialport.read(&mut buf) {
            self.buf.extend_from_slice(&buf[0..n]);
        }
    }

    fn buffer_len(&self) -> usize {
        self.buf.len()
    }
}

#[derive(Debug)]
pub enum UartError {
    NotEnoughBytes,
    Io(std::io::Error),
    SerialPort(serialport::Error),
}

impl From<std::io::Error> for UartError {
    fn from(err: std::io::Error) -> UartError {
        UartError::Io(err)
    }
}

impl From<serialport::Error> for UartError {
    fn from(err: serialport::Error) -> UartError {
        UartError::SerialPort(err)
    }
}

impl ReadExact for Uart {
    type Error = UartError;

    fn peek(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.read_to_buffer();

        if self.buf.len() >= buf.len() {
            buf.clone_from_slice(&self.buf[0..buf.len()]);
            Ok(())
        } else {
            Err(UartError::NotEnoughBytes)
        }
    }

    fn take(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.read_to_buffer();

        if self.buf.len() >= buf.len() {
            let bytes: Vec<u8> = self.buf.drain(0..buf.len()).collect();
            buf.clone_from_slice(&bytes);
            Ok(())
        } else {
            Err(UartError::NotEnoughBytes)
        }
    }
}

impl WriteExact for Uart {
    type Error = UartError;

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.serialport.write_all(buf)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum UartMsg {
    Mouse(MouseMsg, usize),
}

pub fn start<Msg: 'static + Send>(tx: Sender<Msg>, msg: fn(UartMsg) -> Msg) -> Sender<MouseMsg> {
    let (uart_tx, rx): (Sender<MouseMsg>, Receiver<MouseMsg>) = mpsc::channel();

    thread::spawn(move || {
        let serial_settings = SerialPortSettings {
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(1),
        };

        if let Ok(mut port) = Uart::new("/dev/ttyUSB0", serial_settings) {
            let mut last_msg_time = Instant::now();
            loop {
                let now = Instant::now();
                let mousemsg = MouseMsg::parse_bytes(&mut port);

                if mousemsg.is_err() && now.duration_since(last_msg_time) >= Duration::from_secs(1) {
                    port.clear();
                    last_msg_time = now;
                }

                match mousemsg {
                    Ok(mousemsg) => {
                        tx.send(msg(UartMsg::Mouse(mousemsg, port.buffer_len())));
                        last_msg_time = now;
                    },

                    Err(ParseError::ReadExact(UartError::NotEnoughBytes)) => { },

                    Err(e) => {
                        println!("Uart Error! {:?}", e);
                    }
                }

                if let Ok(mousemsg) = rx.try_recv() {
                    mousemsg.generate_bytes(&mut port);
                }
            }
        } else {
            println!("Could not open serial port");
        }
    });

    uart_tx
}
