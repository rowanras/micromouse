
//! A simulated mouse

use crate::CELL_SIZE;

use crate::maze2::Maze;
use crate::maze2::Edge;

use crate::navigate::Navigate;
use crate::navigate::Move;
use crate::navigate::MoveOptions;

pub const WIDTH: f64 = 8.0;
pub const LENGTH: f64 = 10.0;

const LINEAR_SPEED: f64 = CELL_SIZE / 120.0;
const TURN_SPEED: f64 = 90.0 / 120.0;

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn turn_left(&mut self) {
        *self = match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_around(&mut self) {
        *self = match self {
            Direction::North => Direction::South,
            Direction::East => Direction::East,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn rotation(&self) -> f64 {
        match self {
            Direction::North => 0.0,
            Direction::East => 270.0,
            Direction::South => 180.0,
            Direction::West => 90.0,
        }
    }
}

#[derive(Debug)]
enum MouseState {
    MoveLinear(f64, f64),
    MoveTurn(f64, f64),
    NextMove,
    Decision,
}

pub struct Mouse<N: Navigate> {
    local_x: f64,
    local_y: f64,
    cell_x: usize,
    cell_y: usize,
    local_direction: f64,
    direction: Direction,
    state: MouseState,
    paused: bool,
    moves: Vec<Move>,
    maze: Maze<()>,
    nav: N
}

impl<N: Navigate> Mouse<N> {
    pub fn new(nav: N, maze: Maze<()>) -> Mouse<N> {
        Mouse {
            local_x: 0.0,
            local_y: 0.0,
            cell_x: 0,
            cell_y: 0,
            local_direction: 0.0,
            direction: Direction::North,
            state: MouseState::NextMove,
            paused: true,
            moves: Vec::new(),
            maze,
            nav,
        }
    }

    pub fn start(&mut self) {
        self.paused = false;
    }

    pub fn stop(&mut self) {
        self.paused = true;
    }

    pub fn world_location(&self) -> (f64, f64, f64) {
        (
            self.cell_x as f64 * CELL_SIZE + self.local_x,
            self.cell_y as f64 * CELL_SIZE + self.local_y,
            self.direction.rotation() + self.local_direction,
        )
    }

    pub fn maze(&self) -> &Maze<()> {
        &self.maze
    }

    pub fn run(&mut self, dt: f64) {
        match self.state {
            MouseState::Decision => {
                let (_, north_edge, south_edge, east_edge, west_edge) =
                    self.maze.get(self.cell_x, self.cell_y);

                let left_edge = match self.direction {
                    Direction::North => west_edge,
                    Direction::South => east_edge,
                    Direction::East => north_edge,
                    Direction::West => south_edge,
                };

                let front_edge = match self.direction {
                    Direction::North => north_edge,
                    Direction::South => south_edge,
                    Direction::East => east_edge,
                    Direction::West => west_edge,
                };

                let right_edge = match self.direction {
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

                let moves = self.nav.navigate(move_options);

                println!("{:?}", moves);

                for m in moves.into_iter() {
                    if let Some(m) = m {
                        self.moves.insert(0, m.clone());
                    }
                }

                self.state = MouseState::NextMove;
            }

            MouseState::NextMove => {
                self.state = if let Some(next_move) = self.moves.pop() {
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
                    match self.direction {
                        Direction::North => self.cell_y += cells_moved,
                        Direction::South => self.cell_y -= cells_moved,
                        Direction::East => self.cell_x += cells_moved,
                        Direction::West => self.cell_x -= cells_moved,
                    }
                    self.local_x = 0.0;
                    self.local_y = 0.0;
                    self.state = MouseState::NextMove;
                } else {
                    match self.direction {
                        Direction::North => self.local_y = new_value,
                        Direction::South => self.local_y = -new_value,
                        Direction::East => self.local_x = new_value,
                        Direction::West => self.local_x = -new_value,
                    }
                    self.state = MouseState::MoveLinear(target, new_value);
                }
            }

            MouseState::MoveTurn(target, value) => {
                let new_value = value
                    + TURN_SPEED * if target > 0.0 { 1.0 } else { -1.0 };

                if new_value.abs() > target.abs() {
                    let turns = (target / 90.0).abs().round() as usize;

                    for _ in 0..turns {
                        if target > 0.0 {
                            self.direction.turn_right()
                        } else {
                            self.direction.turn_left()
                        };
                    }

                    self.local_direction = 0.0;
                    self.state = MouseState::NextMove;
                } else {
                    self.local_direction = new_value;
                    self.state = MouseState::MoveTurn(target, new_value);
                }
            }
        }
    }
}

