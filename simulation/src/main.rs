extern crate piston_window;

pub mod maze;
mod navigate;

use piston_window::*;

use maze::Maze;
use maze::Edge;
use maze::Location;
use maze::FullCellEdges;

use navigate::Move;
use navigate::MoveOptions;
use navigate::Navigate;

const CELL_SIZE: f64 = 10.0;
const WALL_SIZE: f64 = 2.0;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn edge_to_opacity(edge: Edge) -> f32 {
    match edge {
        Edge::Closed => 1.0,
        Edge::Unknown => 0.5,
        Edge::Open => 0.0,
    }
}

fn main() {

    let maze = Maze::new((), Edge::Open);

    let mut current_x = 0;
    let mut current_y = 0;
    let mut facing = Direction::Right;

    let mut counter = 0;

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true)
            .build()
            .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            if counter >= 1000 {
                counter = 0;
            } else {
                counter += 1;
            }

            clear([1.0; 4], graphics);

            for cell_x in 0..16 {
                let x = cell_x as f64 * CELL_SIZE;
                for cell_y in 0..16 {
                    let y = cell_y as f64 * CELL_SIZE;

                    let location = Location { x: cell_x, y: cell_y };

                    let (
                        _,
                        FullCellEdges {
                            north_edge,
                            east_edge,
                            south_edge,
                            west_edge
                        }
                    ) = maze.cell(location)
                        .expect(&format!("Invalid coordinate: {}, {}", cell_x, cell_y));

                    rectangle([1.0, 0.0, 0.0, edge_to_opacity(north_edge)],
                              [x, y+CELL_SIZE, CELL_SIZE, WALL_SIZE],
                              context.transform,
                              graphics);

                    rectangle([0.0, 1.0, 0.0, edge_to_opacity(east_edge)],
                              [x+CELL_SIZE, y, WALL_SIZE, CELL_SIZE],
                              context.transform,
                              graphics);

                    rectangle([0.0, 0.0, 1.0, edge_to_opacity(south_edge)],
                              [x, y, CELL_SIZE, WALL_SIZE],
                              context.transform,
                              graphics);

                    rectangle([1.0, 1.0, 0.0, edge_to_opacity(west_edge)],
                              [x, y, WALL_SIZE, CELL_SIZE],
                              context.transform,
                              graphics);
                }
            }

            /*
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [10.0, 20.0, 100.0, 150.0],
                      context.transform,
                      graphics);
                      */
        });
    }
}
