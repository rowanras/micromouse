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

use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::series::LineSeries;
use plotters::style;
use plotters::style::Color;
use plotters::style::IntoFont;
use plotters::style::Palette;

use micromouse_lib::CONFIG2019;
use micromouse_lib::msgs::Msg as MouseMsg;

pub struct SimulatorState {
    mouse_states: Vec<MouseState>,
    uart_buffer_len: usize,
}

impl SimulatorState {
    pub fn new() -> SimulatorState {
        SimulatorState {
            mouse_states: vec![MouseState::default()],
            uart_buffer_len: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MouseState {
    time: f32,
    battery: Option<f32>,
    left_pos: Option<f32>,
    right_pos: Option<f32>,
    linear_pos: Option<f32>,
    angular_pos: Option<f32>,
    linear_set: Option<f32>,
    angular_set: Option<f32>,
    linear_power: Option<f32>,
    angular_power: Option<f32>,
    left_power: Option<f32>,
    right_power: Option<f32>,
}

impl MouseState {
    fn update(&mut self, msg: MouseMsg) {
        match msg {
            // Core
            MouseMsg::Time(t) => self.time = t,
            MouseMsg::Logged(m) => {},
            MouseMsg::Provided(m) => {},

            // Raw in/out
            MouseMsg::LeftPos(p) => self.left_pos = Some(p),
            MouseMsg::RightPos(p) => self.right_pos = Some(p),
            MouseMsg::LeftPower(p) => self.left_power = Some(p),
            MouseMsg::RightPower(p) => self.right_power = Some(p),
            MouseMsg::Battery(v) => self.battery = Some(v),

            // Calculated
            MouseMsg::LinearPos(p) => self.linear_pos = Some(p),
            MouseMsg::AngularPos(p) => self.angular_pos = Some(p),
            MouseMsg::LinearSet(s) => self.linear_set = Some(s),
            MouseMsg::AngularSet(s) => self.angular_pos = Some(s),
            MouseMsg::AddLinear(v, d) => {},
            MouseMsg::AddAngular(v, d) => {},

            // Config
            MouseMsg::LinearP(p) => {},
            MouseMsg::LinearI(i) => {},
            MouseMsg::LinearD(d) => {},
            MouseMsg::LinearAcc(a) => {},
            MouseMsg::AngularP(p) => {},
            MouseMsg::AngularI(i) => {},
            MouseMsg::AngularD(d) => {},
            MouseMsg::AngularAcc(a) => {},
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
        let mut last_msg_instant = Instant::now();
        while let Ok(msg) = rx.recv() {
            let elapsed_time = last_msg_instant.elapsed();
            let mut state = state_update.lock().unwrap();
            let mut next_state = if let Some(last_state) = state.mouse_states.last() {
                last_state.clone()
            } else {
                MouseState::default()
            };

            let last_time = next_state.time;

            println!("{:?}", msg);
            match msg {
                Msg::Uart(uartmsg) => {
                    match uartmsg {
                        uart::UartMsg::Mouse(mousemsg, buf_len) => {
                            next_state.update(mousemsg);
                            state.uart_buffer_len = buf_len;
                        }
                    }
                },

                Msg::Gui(guimsg) => {
                    match guimsg {
                        gui::GuiMsg::Mouse(mousemsg) => {
                            uart_tx.send(mousemsg);
                        }
                    }
                }
            }

            if last_time == next_state.time {
                //next_state.time += elapsed_time.as_secs() as f32 + elapsed_time.subsec_nanos() as f32 / 10e9;
            }

            state.mouse_states.push(next_state);
            last_msg_instant += elapsed_time;
        }
    });

    let gui_handle = gui::start(Arc::clone(&states), tx.clone(), Msg::Gui);

    gui_handle.join().unwrap();

    println!("bye");
}
