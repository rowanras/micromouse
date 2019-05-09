extern crate piston_window;

mod maze2;
mod navigate;

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

const CELL_SIZE: f64 = 20.0;
const WALL_SIZE: f64 = 1.0;

const MOUSE_WIDTH: f64 = 8.0;
const MOUSE_LENGTH: f64 = 10.0;

const LINEAR_SPEED: f64 = CELL_SIZE / 120.0;
const TURN_SPEED: f64 = 90.0 / 120.0;

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_around(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::East,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn rotation(&self) -> f64 {
        match self {
            Direction::North => 0.0,
            Direction::East => 270.0,
            Direction::South => 180.0,
            Direction::West => 90.0,
        }
    }
}

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

#[derive(Debug)]
enum MouseState {
    MoveLinear(f64, f64),
    MoveTurn(f64, f64),
    NextMove,
    Decision,
    Stop,
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
    let mut navigation = RandomNavigate::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

    let mut mouse_x = 0.0;
    let mut mouse_y = 0.0;
    let mut x = 0;
    let mut y = 0;
    let mut facing = Direction::North;
    let mut mouse_turn = 0.0;

    let mut mouse_state = MouseState::Decision;

    let mut mouse_moves = Vec::new();

    let mut window: PistonWindow = WindowSettings::new(
        "Maze",
        [16 * CELL_SIZE as u32, 16 * CELL_SIZE as u32],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    while let Some(event) = window.next() {
        if let Some(u) = event.update_args() {
            match mouse_state {
                MouseState::Decision => {
                    println!(
                        "{}, {}, {}, {}, {:?}",
                        x, y, mouse_x, mouse_y, facing,
                    );
                    let (_, north_edge, south_edge, east_edge, west_edge) =
                        maze.get(x, y);

                    let left_edge = match facing {
                        Direction::North => west_edge,
                        Direction::South => east_edge,
                        Direction::East => north_edge,
                        Direction::West => south_edge,
                    };

                    let front_edge = match facing {
                        Direction::North => north_edge,
                        Direction::South => south_edge,
                        Direction::East => east_edge,
                        Direction::West => west_edge,
                    };

                    let right_edge = match facing {
                        Direction::North => east_edge,
                        Direction::South => west_edge,
                        Direction::East => south_edge,
                        Direction::West => north_edge,
                    };

                    let move_options = MoveOptions {
                        forward: front_edge == Edge::Open,
                        left: left_edge == Edge::Open,
                        right: right_edge == Edge::Open,
                    };

                    println!("{:?}", move_options);

                    let moves = navigation.navigate(move_options);

                    println!("{:?}", moves);

                    for m in moves.into_iter() {
                        if let Some(m) = m {
                            mouse_moves.insert(0, m.clone());
                        }
                    }

                    mouse_state = MouseState::NextMove;
                    //mouse_state = MouseState::Stop;
                }

                MouseState::NextMove => {
                    mouse_state = if let Some(next_move) = mouse_moves.pop() {
                        match next_move {
                            Move::Forward => {
                                MouseState::MoveLinear(CELL_SIZE, 0.0)
                            }
                            Move::TurnLeft => MouseState::MoveTurn(-90.0, 0.0),
                            Move::TurnRight => MouseState::MoveTurn(90.0, 0.0),
                            Move::TurnAround => {
                                MouseState::MoveTurn(180.0, 0.0)
                            }
                        }
                    } else {
                        MouseState::Decision
                    }
                }

                MouseState::MoveLinear(target, value) => {
                    let new_value = value
                        + LINEAR_SPEED * if target > 0.0 { 1.0 } else { -1.0 };

                    if new_value.abs() > target.abs() {
                        let cells_moved =
                            (target / CELL_SIZE).abs().round() as usize;
                        match facing {
                            Direction::North => y += cells_moved,
                            Direction::South => y -= cells_moved,
                            Direction::East => x += cells_moved,
                            Direction::West => x -= cells_moved,
                        }
                        mouse_x = 0.0;
                        mouse_y = 0.0;
                        mouse_state = MouseState::NextMove;
                    } else {
                        match facing {
                            Direction::North => mouse_y = new_value,
                            Direction::South => mouse_y = -new_value,
                            Direction::East => mouse_x = new_value,
                            Direction::West => mouse_x = -new_value,
                        }
                        mouse_state = MouseState::MoveLinear(target, new_value);
                    }
                }

                MouseState::MoveTurn(target, value) => {
                    let new_value = value
                        + TURN_SPEED * if target > 0.0 { 1.0 } else { -1.0 };

                    if new_value.abs() > target.abs() {
                        let turns = (target / 90.0).abs().round() as usize;

                        for i in 0..turns {
                            facing = if target > 0.0 {
                                facing.turn_right()
                            } else {
                                facing.turn_left()
                            };
                        }

                        mouse_turn = 0.0;
                        mouse_state = MouseState::NextMove;
                    } else {
                        mouse_turn = new_value;
                        mouse_state = MouseState::MoveTurn(target, new_value);
                    }
                }

                MouseState::Stop => {}
            }
        }

        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

            let drawings = draw_maze(&maze);

            for (color, rect) in drawings {
                rectangle(color, rect, context.transform, graphics);
            }

            let transform = context
                .transform
                .trans(
                    CELL_SIZE / 2.0 + x as f64 * CELL_SIZE + mouse_x,
                    CELL_SIZE / 2.0 + (15 - y) as f64 * CELL_SIZE - mouse_y,
                )
                .rot_deg(facing.rotation() + mouse_turn);

            rectangle(
                [0.0, 1.0, 0.0, 1.0],
                [
                    -MOUSE_WIDTH / 2.0,
                    -MOUSE_LENGTH / 2.0,
                    MOUSE_WIDTH,
                    MOUSE_LENGTH,
                ],
                transform,
                graphics,
            );
        });
    }
}
