#![recursion_limit = "128"]

extern crate piston_window;

//mod maze2;
//mod mouse;
//mod navigate;

use std::f64;

use std::io::Read;

use std::time::Duration;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use piston_window::clear;
use piston_window::image as draw_image;
use piston_window::rectangle;
use piston_window::PistonWindow;
use piston_window::RenderEvent;
use piston_window::Texture;
use piston_window::TextureSettings;
use piston_window::Transformed;
use piston_window::WindowSettings;

use image::ImageBuffer;

use plotters::chart::ChartBuilder;
use plotters::drawing::draw_piston_window;
use plotters::drawing::BitMapBackend;
use plotters::drawing::IntoDrawingArea;
use plotters::series::LineSeries;
use plotters::style;
use plotters::style::Color;
use plotters::style::IntoFont;
use plotters::style::Palette;

use serialport::DataBits;
use serialport::FlowControl;
use serialport::Parity;
use serialport::SerialPortSettings;
use serialport::StopBits;

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

#[derive(Debug)]
struct MouseState {
    time: f64,
    left: f64,
    right: f64,
    x: f64,
    y: f64,
    dir: f64,
}

fn main() {
    let config = CONFIG2019;

    let states: Arc<Mutex<Vec<MouseState>>> = Arc::new(Mutex::new(Vec::new()));

    let state_uart = Arc::clone(&states);
    thread::spawn(move || {
        let serial_settings = SerialPortSettings {
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_secs(1),
        };

        let mut port =
            serialport::open_with_settings("/dev/ttyUSB0", &serial_settings)
                .expect("Could not open port");

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

            let time = (((buffer[3] as u32) << 16)
                | ((buffer[2] as u32) << 8)
                | (buffer[1] as u32)) as f64
                / 1000.0;
            let left = config.mouse.ticks_to_mm(
                (((buffer[5] as i32) << 8) | (buffer[4] as i32)) as f64,
            );
            let right = config.mouse.ticks_to_mm(
                (((buffer[7] as i32) << 8) | (buffer[6] as i32)) as f64,
            );

            let mut s = state_uart.lock().unwrap();

            let (x, y, dir) = if let Some(last_state) = s.last() {
                let delta_left = left - last_state.left;
                let delta_right = right - last_state.right;

                let delta_linear = (delta_left + delta_right) / 2.0;
                let delta_angular =
                    config.mouse.mm_to_rads((delta_left - delta_right) / 2.0);

                let mid_dir = last_state.dir + delta_angular / 2.0;

                (
                    last_state.x + delta_linear * f64::cos(mid_dir),
                    last_state.y + delta_linear * f64::sin(mid_dir),
                    last_state.dir + delta_angular,
                )
            } else {
                (90.0, 90.0, f64::consts::PI / 2.0)
            };

            let new_state = MouseState {
                time,
                left,
                right,
                x,
                y,
                dir,
            };

            s.push(new_state);
        }
    });

    let states_maze = Arc::clone(&states);

    let mut window: PistonWindow = WindowSettings::new("Mouse", [800, 300])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut texture_context = window.create_texture_context();

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                clear([1.0; 4], graphics);

                let states = states_maze.lock().unwrap();

                if let Some(state) = dbg!(states.last()) {
                    let transform = context
                        .transform
                        .trans(state.x / MM_PER_PIXEL, state.y / MM_PER_PIXEL)
                        .rot_rad(state.dir);

                    rectangle(
                        [0.0, 1.0, 0.0, 1.0],
                        [
                            -(config.mouse.length - config.mouse.front_offset)
                                / MM_PER_PIXEL,
                            -config.mouse.width / MM_PER_PIXEL / 2.0,
                            config.mouse.length / MM_PER_PIXEL,
                            config.mouse.width / MM_PER_PIXEL,
                        ],
                        transform,
                        graphics,
                    );
                }

                let mut graph_image_buf = Vec::new();

                {
                    let root = BitMapBackend::with_buffer(
                        &mut graph_image_buf,
                        (400, 300),
                    )
                    .into_drawing_area();

                    root.fill(&style::White).unwrap();

                    let time = if let Some(state) = states.last() {
                        state.time
                    } else {
                        0.0
                    };

                    let mut cc = ChartBuilder::on(&root)
                        .margin(10)
                        .caption("Left Encoder", ("Arial", 20).into_font())
                        .x_label_area_size(40)
                        .y_label_area_size(50)
                        .build_ranged(time - 60.0..time, 0.0..50.0)
                        .unwrap();

                    cc.configure_mesh()
                        .x_label_formatter(&|x| format!("{}", x))
                        .y_label_formatter(&|y| format!("{}", y))
                        .x_labels(15)
                        .y_labels(10)
                        .x_desc("seconds")
                        .y_desc("mm")
                        .axis_desc_style(("Arial", 15).into_font())
                        .disable_x_mesh()
                        .disable_y_mesh()
                        .draw()
                        .unwrap();

                    cc.draw_series(LineSeries::new(
                        states.iter().map(|s| (s.time, s.left)),
                        &style::Palette99::pick(0),
                    ))
                    .unwrap();
                }

                // Add the alpha channel
                let graph_image_buf: Vec<u8> = graph_image_buf
                    .chunks_exact(3)
                    .flat_map(|rgb| {
                        vec![rgb[0], rgb[1], rgb[2], 255].into_iter()
                    })
                    .collect();

                let graph_texture = Texture::from_image(
                    &mut texture_context,
                    &ImageBuffer::from_vec(400, 300, graph_image_buf.clone())
                        .expect("Buffer not big enough!"),
                    &TextureSettings::new(),
                )
                .unwrap();

                let transform = context.transform.trans(400.0, 0.0);

                draw_image(&graph_texture, transform, graphics);
            });
        }
    }
}
