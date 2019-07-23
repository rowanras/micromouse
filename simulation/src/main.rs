#![recursion_limit = "128"]
#![allow(unused)]

//mod maze2;
//mod mouse;
//mod navigate;

//mod plotters_cairo;
mod gui;
mod uart;

use std::f32;

use std::io::Read;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use std::time::Duration;
use std::time::Instant;

use image::ImageBuffer;

use micromouse_lib::mouse::Mouse;
use micromouse_lib::msgs::Msg as MouseMsg;
use micromouse_lib::CONFIG2019;

pub struct SimulatorState {
    mice: Vec<Mouse>,
    uart_buffer_len: usize,
}

impl SimulatorState {
    pub fn new() -> SimulatorState {
        SimulatorState {
            mice: vec![Mouse::new(CONFIG2019)],
            uart_buffer_len: 0,
        }
    }
}

#[derive(Debug)]
enum Msg {
    Uart(uart::UartMsg),
    Gui(gui::GuiMsg),
}

fn main() {
    let config = CONFIG2019;

    let states: Arc<Mutex<SimulatorState>> = Arc::new(Mutex::new(SimulatorState::new()));

    let (tx, rx) = mpsc::channel();

    let uart_tx = uart::start(tx.clone(), Msg::Uart);

    let state_update = Arc::clone(&states);

    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            let mut state = state_update.lock().unwrap();
            let mut next_state = state.mice.last().unwrap().clone();

            let last_time = next_state.time;

            println!("{:?}", msg);
            match msg {
                Msg::Uart(uartmsg) => match uartmsg {
                    uart::UartMsg::Mouse(mousemsg, buf_len) => {
                        next_state.update(mousemsg);
                        state.uart_buffer_len = buf_len;
                    }
                },

                Msg::Gui(guimsg) => match guimsg {
                    gui::GuiMsg::Mouse(mousemsg) => {
                        uart_tx.send(mousemsg);
                    }
                },
            }

            if last_time == next_state.time {
                *state.mice.last_mut().unwrap() = next_state
            } else {
                state.mice.push(next_state);
            }
        }
    });

    let gui_handle = gui::start(Arc::clone(&states), tx.clone(), Msg::Gui);

    gui_handle.join().unwrap();

    println!("bye");
}
