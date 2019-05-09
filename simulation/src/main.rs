extern crate piston_window;

mod maze2;
mod navigate;
mod mouse;

use std::fs::File;
use std::io::Read;

use piston_window::*;

use maze2::Edge;
use maze2::Maze;

use navigate::LeftWall;
use navigate::RandomNavigate;
use navigate::Navigate;
use navigate::Move;
use navigate::MoveOptions;

use mouse::Direction;
use mouse::Mouse;

pub const CELL_SIZE: f64 = 20.0;
pub const WALL_SIZE: f64 = 1.0;

trait ToColor {
    fn to_color(&self) -> [f64; 4];
}

fn edge_to_opacity(edge: Edge) -> f32 {
    match edge {
        Edge::Closed => 1.0,
        Edge::Unknown => 0.5,
        Edge::Open => 0.1,
    }
}

fn draw_maze<C: ToColor + Copy>(maze: &Maze<C>) -> Vec<([f32; 4], [f64; 4])> {
    let mut drawings = Vec::new();
    for cell_x in 0..16 {
        let x = cell_x as f64 * CELL_SIZE;
        for cell_y in 0..16 {
            let y = cell_y as f64 * CELL_SIZE;

            let (_, north_edge, south_edge, east_edge, west_edge) =
                maze.get(cell_x, 15 - cell_y);

            drawings.push((
                [0.0, 0.0, 0.0, edge_to_opacity(south_edge)],
                [x, y + CELL_SIZE - WALL_SIZE, CELL_SIZE, WALL_SIZE],
            ));
            drawings.push((
                [0.0, 0.0, 0.0, edge_to_opacity(east_edge)],
                [x + CELL_SIZE - WALL_SIZE, y, WALL_SIZE, CELL_SIZE],
            ));
            drawings.push((
                [0.0, 0.0, 0.0, edge_to_opacity(north_edge)],
                [x, y, CELL_SIZE, WALL_SIZE],
            ));
            drawings.push((
                [0.0, 0.0, 0.0, edge_to_opacity(west_edge)],
                [x, y, WALL_SIZE, CELL_SIZE],
            ));
        }
    }

    drawings
}

impl ToColor for () {
    fn to_color(&self) -> [f64; 4] {
        [0.0; 4]
    }
}

fn main() {
    //let maze = Maze::new((), Edge::Open);
    let mut file =
        File::open("micromouse_maze_tool/mazefiles/binary/APEC2016.MAZ")
            .unwrap();
    let mut bytes = [0; 256];
    file.read_exact(&mut bytes).unwrap();
    let maze = Maze::from_file((), bytes);

    //let mut navigation = LeftWall::new();
    let nav = RandomNavigate::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    let mut mouse = Mouse::new(nav, maze);

    let mut window: PistonWindow = WindowSettings::new(
        "Maze",
        [16 * CELL_SIZE as u32, 16 * CELL_SIZE as u32],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    while let Some(event) = window.next() {
        if let Some(u) = event.update_args() {
            mouse.run(u.dt);
        }

        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            let drawings = draw_maze(mouse.maze());

            for (color, rect) in drawings {
                rectangle(color, rect, context.transform, graphics);
            }

            let (mouse_x, mouse_y, mouse_dir) = mouse.world_location();

            let transform = context
                .transform
                .trans(
                    CELL_SIZE / 2.0 + mouse_x,
                    CELL_SIZE / 2.0 + (maze2::HEIGHT-1) as f64 * CELL_SIZE - mouse_y,
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
