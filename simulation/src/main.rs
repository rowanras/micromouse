extern crate piston_window;

mod navigate;

use piston_window::*;

use navigate::Navigate;
use navigate::Move;
use navigate::MoveOptions;

const CELL_SIZE: f64 = 10.0;
const WALL_SIZE: f64 = 2.0;

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

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            for cell_x in 0..16 {
                let x = cell_x as f64 * CELL_SIZE;
                for cell_y in 0..16 {
                    let y = cell_y as f64 * CELL_SIZE;

                    let s = cells[cell_x][cell_y];

                    for c in s.chars() {
                        let dims = match c {
                            'T' => Some([
                                        x,
                                        y + CELL_SIZE,
                                        CELL_SIZE,
                                        WALL_SIZE,
                            ]),
                            'L' => Some([
                                        x,
                                        y + CELL_SIZE,
                                        WALL_SIZE,
                                        CELL_SIZE,
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
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
                      */
        });
    }
}
