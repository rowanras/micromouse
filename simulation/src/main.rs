extern crate piston_window;

mod maze2;
mod mouse;
mod navigate;

use std::fs::File;
use std::io::Read;

use piston_window::*;

use maze2::Edge;
use maze2::Maze;

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

pub const CELL_SIZE: f64 = 20.0;
pub const WALL_SIZE: f64 = 1.0;

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

fn main() {
    //let maze = Maze::new((), Edge::Open);

    let mut args = std::env::args().skip(1);

    let maze_path = args.next().unwrap_or("mouse_maze_tool/mazefiles/binary/APEC2016.MAZ".to_owned());

    println!("Loading from: {}", maze_path);

    let mut file =
        File::open(maze_path)
            .unwrap();
    let mut bytes = [0; 256];
    file.read_exact(&mut bytes).unwrap();
    let maze = Maze::from_file(0, bytes);

    let nav_string = args.next().unwrap_or("TwelvePartitionNavigate".to_owned());

    let nav: Box<dyn Navigate<Cell = u8>> = match nav_string.as_ref() {
        "CountingNavigate" => Box::new(CountingNavigate::new()),
        "CountingDeadEndNavigate" => Box::new(CountingDeadEndNavigate::new()),
        "FloodFillNavigate" => Box::new(FloodFillNavigate::new()),
        "FloodFillSquareNavigate" => Box::new(FloodFillSquareNavigate::new()),
        "FloodFillDeadEndNavigate" => Box::new(FloodFillDeadEndNavigate::new()),
        "FloodFillSquareDeadEndNavigate" => Box::new(FloodFillSquareDeadEndNavigate::new()),
        "TwelvePartitionNavigate" => Box::new(TwelvePartitionNavigate::new()),
        _ => Box::new(TwelvePartitionNavigate::new()),
    };

    //let nav= LeftWall::new();
    //let nav = DeadEndNavigate::new();
    //let nav = RandomNavigate::new([0; 16]);
    //let nav = CountingNavigate::new();
    //let nav = CountingDeadEndNavigate::new();
    //let nav = FloodFillNavigate::new();
    //let nav = FloodFillSquareDeadEndNavigate::new();
    //let nav = TwelvePartitionNavigate::new();
    let mut mouse = Mouse::new(nav, maze);

    let start_time = std::time::Instant::now();

    /*
    loop {
        let (x, y, d) = mouse.maze_location();

        let runtime = std::time::Instant::now().duration_since(start_time);

        if x >= 7 && x <= 8 && y >= 7 && y <= 8 || runtime > std::time::Duration::from_secs(10) {
            println!("Won!");

            let maze = mouse.maze();

            for y in 0..maze2::HEIGHT {
                for x in 0..maze2::WIDTH {
                    let (cell, north_edge, south_edge, east_edge, west_edge) =
                        maze.get(x, (maze2::HEIGHT - 1) - y);

                    print!("{}", cell.text());

                    if x != maze2::WIDTH - 1 {
                        print!("\t");
                    } else {
                        print!("\n");
                    }
                }
            }

            break;
        }

        mouse.run(1.0/60.0);
    }
    */

    let mut window: PistonWindow = WindowSettings::new(
        nav_string,
        [16 * CELL_SIZE as u32, 16 * CELL_SIZE as u32],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    window.set_ups(60);
    window.set_max_fps(20);

    let font = "/usr/share/fonts/TTF/FiraMono-Regular.ttf";
    let factory = window.factory.clone();
    let mut glyphs =
        Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    while let Some(event) = window.next() {
        if let Some(u) = event.update_args() {

            let (x, y, d) = mouse.maze_location();

            if x >= 7 && x <= 8 && y >= 7 && y <= 8 {
                println!("Won!");

                let maze = mouse.maze();

                for y in 0..maze2::HEIGHT {
                    for x in 0..maze2::WIDTH {
                        let (cell, north_edge, south_edge, east_edge, west_edge) =
                            maze.get(x, (maze2::HEIGHT - 1) - y);

                        print!("{}", cell.text());

                        if x != maze2::WIDTH - 1 {
                            print!("\t");
                        } else {
                            print!("\n");
                        }
                    }
                }

                //window.set_should_close(true);
            }

            //mouse.run(u.dt);
        }

        if let Some(r) = event.render_args() {
            window.draw_2d(&event, |context, graphics| {
                clear([1.0; 4], graphics);

                let drawings = draw_maze(mouse.maze());

                for ((string, x, y), rects) in drawings {
                    for (color, rect) in rects {
                        rectangle(color, rect, context.transform, graphics);
                    }
                    let transform = context.transform.trans(x, y);
                    /*
                    text(
                        [0.0, 0.0, 0.0, 1.0],
                        10,
                        &string,
                        &mut glyphs,
                        transform,
                        graphics,
                    )
                    .unwrap();
                    */
                }

                let (mouse_x, mouse_y, mouse_dir) = mouse.world_location();

                let transform = context
                    .transform
                    .trans(
                        CELL_SIZE / 2.0 + mouse_x,
                        CELL_SIZE / 2.0
                            + (maze2::HEIGHT - 1) as f64 * CELL_SIZE
                            - mouse_y,
                    )
                    .rot_deg(mouse_dir);

                rectangle(
                    [0.0, 1.0, 0.0, 1.0],
                    [
                        -mouse::WIDTH / 2.0,
                        -mouse::LENGTH / 2.0,
                        mouse::WIDTH,
                        mouse::LENGTH,
                    ],
                    transform,
                    graphics,
                );
            });
        }
    }
}
