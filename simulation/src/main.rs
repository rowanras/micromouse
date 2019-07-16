#![recursion_limit = "128"]
#![allow(unused)]

//mod maze2;
//mod mouse;
//mod navigate;

//mod plotters_cairo;
mod gui;
mod uart;

use std::f64;

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

//use maze2::Edge;
//use maze2::Maze;

/*
use navigate::Navigate;
use navigate::CountingDeadEndNavigate;
use navigate::CountingNavigate;
use navigate::DeadEndNavigate;
use navigate::FloodFillDeadEndNavigate;
use navigate::FloodFillNavigate;
use navigate::FloodFillSquareDeadEndNavigate;
use navigate::FloodFillSquareNavigate;
use navigate::LeftWall;
use navigate::RandomNavigate;
use navigate::TwelvePartitionNavigate;

use mouse::Direction;
use mouse::Mouse;
*/

pub const MM_PER_PIXEL: f64 = 10.0;

/*
pub trait Visualize {
    fn color(&self) -> [f32; 4];
    fn text(&self) -> String;
}

impl Visualize for () {
    fn color(&self) -> [f32; 4] {
        [0.0; 4]
    }
    fn text(&self) -> String {
        "".to_owned()
    }
}

impl Visualize for u8 {
    fn color(&self) -> [f32; 4] {
        [1.0, 0.0, 0.0, *self as f32 / 16.0]
    }
    fn text(&self) -> String {
        self.to_string()
    }
}

impl Visualize for bool {
    fn color(&self) -> [f32; 4] {
        [1.0, 0.0, 0.0, if *self { 1.0 } else { 0.0 }]
    }
    fn text(&self) -> String {
        self.to_string()
    }
}

fn edge_to_opacity(edge: Edge) -> f32 {
    match edge {
        Edge::Closed => 1.0,
        Edge::Unknown => 0.5,
        Edge::Open => 0.0,
    }
}

fn draw_maze<C: Visualize + Copy>(
    maze: &Maze<C>,
) -> Vec<((String, f64, f64), Vec<([f32; 4], [f64; 4])>)> {
    let mut drawings = Vec::new();
    for cell_x in 0..16 {
        let x = cell_x as f64 * CELL_SIZE;
        for cell_y in 0..16 {
            let y = cell_y as f64 * CELL_SIZE;

            let (cell, north_edge, south_edge, east_edge, west_edge) =
                maze.get(cell_x, (maze2::HEIGHT - 1) - cell_y);

            drawings.push((
                (cell.text(), x + WALL_SIZE, y + CELL_SIZE - WALL_SIZE),
                vec![
                    (cell.color(), [x, y, CELL_SIZE, CELL_SIZE]),
                    (
                        [0.0, 0.0, 0.0, edge_to_opacity(south_edge)],
                        [x, y + CELL_SIZE - WALL_SIZE, CELL_SIZE, WALL_SIZE],
                    ),
                    (
                        [0.0, 0.0, 0.0, edge_to_opacity(east_edge)],
                        [x + CELL_SIZE - WALL_SIZE, y, WALL_SIZE, CELL_SIZE],
                    ),
                    (
                        [0.0, 0.0, 0.0, edge_to_opacity(north_edge)],
                        [x, y, CELL_SIZE, WALL_SIZE],
                    ),
                    (
                        [0.0, 0.0, 0.0, edge_to_opacity(west_edge)],
                        [x, y, WALL_SIZE, CELL_SIZE],
                    ),
                ],
            ));
        }
    }

    drawings
}
*/

#[derive(Debug, Clone)]
pub struct MouseState {
    time: f64,
    left: Option<f64>,
    right: Option<f64>,
    left_power: Option<f64>,
    right_power: Option<f64>,
    x: Option<f64>,
    y: Option<f64>,
    dir: Option<f64>,
}

#[derive(Debug)]
enum Msg {
    Uart(u32, i32, i32),
    Gui(gui::GuiMsg),
}

fn main() {
    let config = CONFIG2019;

    let states: Arc<Mutex<Vec<MouseState>>> = Arc::new(Mutex::new(Vec::new()));

    let (tx, rx) = mpsc::channel();

    let uart_tx = uart::start(tx.clone(), Msg::Uart);

    let state_update = Arc::clone(&states);

    thread::spawn(move || {
        let mut last_msg_instant = Instant::now();
        while let Ok(msg) = rx.recv() {
            let elapsed_time = last_msg_instant.elapsed();
            let mut state = state_update.lock().unwrap();
            let mut next_state = if let Some(last_state) = state.last()
            {
                last_state.clone()
            } else {
                MouseState {
                    time: 0.0,
                    left: None,
                    right: None,
                    left_power: None,
                    right_power: None,
                    x: None,
                    y: None,
                    dir: None,
                }
            };

            let last_time = next_state.time;

            match msg {
                Msg::Uart(time, left, right) => {
                    let time = time as f64 / 1000.0;
                    let left = config.mouse.ticks_to_mm(left as f64);
                    let right = config.mouse.ticks_to_mm(right as f64);


                    let (dx, dy, ddir) =
                        if let (Some(last_left), Some(last_right)) =
                            (next_state.left, next_state.right)
                        {
                            let delta_left =
                                left - next_state.left.unwrap_or(0.0);
                            let delta_right =
                                right - next_state.right.unwrap_or(0.0);

                            let delta_linear = (delta_left + delta_right) / 2.0;
                            let delta_angular = config
                                .mouse
                                .mm_to_rads((delta_left - delta_right) / 2.0);

                            let mid_dir = next_state.dir.unwrap_or(0.0)
                                + delta_angular / 2.0;

                            (
                                delta_linear * f64::cos(mid_dir),
                                delta_linear * f64::sin(mid_dir),
                                delta_angular,
                            )
                        } else {
                            (0.0, 0.0, 0.0)
                        };

                    next_state.time = time;
                    next_state.left = Some(left);
                    next_state.right = Some(right);
                    next_state.x = Some(next_state.x.unwrap_or(0.0) + dx);
                    next_state.y = Some(next_state.y.unwrap_or(0.0) + dy);
                    next_state.dir = Some(next_state.dir.unwrap_or(0.0) + ddir);

                }

                Msg::Gui(guimsg) => {
                    println!("GuiMsg: {:?}", guimsg);

                    match guimsg {
                        gui::GuiMsg::LeftMotorPower(p) => next_state.left_power = Some(p),
                        gui::GuiMsg::RightMotorPower(p) => next_state.right_power = Some(p),
                    }

                    uart_tx.send(guimsg);
                }
            }

            if last_time == next_state.time {
                next_state.time += elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 / 10e9;
            }

            state.push(next_state);
            last_msg_instant += elapsed_time;
        }
    });

    let gui_handle = gui::start(Arc::clone(&states), tx.clone(), Msg::Gui);

    gui_handle.join().unwrap();

    println!("bye");
}
