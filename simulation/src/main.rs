extern crate piston_window;

mod navigate;

use piston_window::*;

use navigate::Navigate;
use navigate::Move;
use navigate::MoveOptions;

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

fn main() {
    let cells = [
        ["TLrb", "Tlrb", "Tlrb", "Tlrb", "Tlrb", "Tlrb", "Tlrb", "Tlrb", "Tlrb", "Tlrb", "Tlrb", "Tlrb", "Tlrb", "Tlrb", "Tlrb", "TlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlrb", "tlRb"],
        ["tLrB", "tlrB", "tlrB", "tlrB", "tlrB", "tlrB", "tlrB", "tlrB", "tlrB", "tlrB", "tlrB", "tlrB", "tlrB", "tlrB", "tlrB", "tlRB"],
    ];

    let mut current_x = 0;
    let mut current_y = 0;
    let mut facing = Direction::Right;

    let mut counter = 0;

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
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

                    let s = cells[cell_y][cell_x];

                    for c in s.chars() {
                        let dims = match c {
                            'T' => Some([
                                        x,
                                        y,
                                        CELL_SIZE,
                                        WALL_SIZE,
                            ]),
                            'L' => Some([
                                        x,
                                        y,
                                        WALL_SIZE,
                                        CELL_SIZE,
                            ]),
                            'R' => Some([
                                        x + CELL_SIZE - WALL_SIZE,
                                        y,
                                        WALL_SIZE,
                                        CELL_SIZE,
                            ]),
                            'B' => Some([
                                        x,
                                        y + CELL_SIZE - WALL_SIZE,
                                        CELL_SIZE,
                                        WALL_SIZE,
                            ]),
                            _ => None,
                        };

                        if let Some(d) = dims {
                            rectangle(
                                [1.0, 0.0, 0.0, 1.0],
                                d,
                                context.transform,
                                graphics
                            );
                        }
                    }
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
