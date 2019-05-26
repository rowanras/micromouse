use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;

use crate::mouse::Direction;
use crate::Visualize;

const F_MOVES: [Option<Move>; 2] = [Some(Move::Forward), None];
const L_MOVES: [Option<Move>; 2] = [Some(Move::TurnLeft), Some(Move::Forward)];
const R_MOVES: [Option<Move>; 2] = [Some(Move::TurnRight), Some(Move::Forward)];
const B_MOVES: [Option<Move>; 2] =
    [Some(Move::TurnAround), Some(Move::Forward)];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    TurnLeft,
    TurnRight,
    TurnAround,
    Forward,
}

#[derive(Debug)]
pub struct MoveOptions {
    pub forward: bool,
    pub left: bool,
    pub right: bool,
}

pub trait Navigate {
    type Cell: Visualize + Copy;
    fn navigate(
        &mut self,
        x: usize,
        y: usize,
        d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2];

    fn get_cell(&self, x: i32, y: i32) -> Self::Cell;
}

impl<N: Navigate> Navigate for Box<N> {
    type Cell = N::Cell;
    fn navigate(
        &mut self,
        x: usize,
        y: usize,
        d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        self.as_mut().navigate(x, y, d, move_options)
    }

    fn get_cell(&self, x: i32, y: i32) -> Self::Cell {
        self.as_ref().get_cell(x, y)
    }
}

pub struct LeftWall {}

impl LeftWall {
    pub fn new() -> LeftWall {
        LeftWall {}
    }
}

impl Navigate for LeftWall {
    type Cell = ();
    fn navigate(
        &mut self,
        _x: usize,
        _y: usize,
        _d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        if move_options.left {
            L_MOVES
        } else if move_options.forward {
            F_MOVES
        } else if move_options.right {
            R_MOVES
        } else {
            B_MOVES
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> Self::Cell {
        ()
    }
}

pub struct RandomNavigate {
    rng: SmallRng,
}

impl RandomNavigate {
    pub fn new(seed: [u8; 16]) -> RandomNavigate {
        RandomNavigate {
            rng: SmallRng::from_seed(seed),
        }
    }
}

impl Navigate for RandomNavigate {
    type Cell = ();
    fn navigate(
        &mut self,
        _x: usize,
        _y: usize,
        _d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        match (move_options.left, move_options.forward, move_options.right) {
            (true, true, true) => match self.rng.gen_range(0, 3) {
                0 => L_MOVES,
                1 => R_MOVES,
                _ => F_MOVES,
            },

            (true, false, true) => match self.rng.gen_range(0, 2) {
                0 => L_MOVES,
                _ => R_MOVES,
            },

            (false, true, true) => match self.rng.gen_range(0, 2) {
                0 => R_MOVES,
                _ => F_MOVES,
            },

            (true, true, false) => match self.rng.gen_range(0, 2) {
                0 => L_MOVES,
                _ => F_MOVES,
            },

            (false, true, false) => F_MOVES,

            (true, false, false) => L_MOVES,

            (false, false, true) => R_MOVES,

            (false, false, false) => B_MOVES,
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> Self::Cell {
        ()
    }
}

pub struct DeadEndNavigate {
    cells: [[bool; 16]; 16],
}

impl DeadEndNavigate {
    pub fn new() -> DeadEndNavigate {
        DeadEndNavigate {
            cells: [[false; 16]; 16],
        }
    }
}

impl Navigate for DeadEndNavigate {
    type Cell = bool;

    fn get_cell(&self, x: i32, y: i32) -> bool {
        if x >= 0 && x <= 15 && y >= 0 && y <= 15 {
            self.cells[x as usize][y as usize]
        } else {
            true
        }
    }

    fn navigate(
        &mut self,
        x: usize,
        y: usize,
        d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        let x = x as i32;
        let y = y as i32;

        let ux = if x < 0 {
            0
        } else if x > 15 {
            15
        } else {
            x
        } as usize;

        let uy = if y < 0 {
            0
        } else if y > 15 {
            15
        } else {
            y
        } as usize;

        // win condition
        if x >= 7 && x <= 8 && y >= 7 && y <= 8 {
            [Some(Move::TurnLeft), Some(Move::TurnLeft)]
        } else {
            let left_blocked = !move_options.left
                || match d {
                    Direction::North => self.get_cell(x - 1, y),
                    Direction::South => self.get_cell(x + 1, y),
                    Direction::East => self.get_cell(x, y + 1),
                    Direction::West => self.get_cell(x, y - 1),
                };

            let front_blocked = !move_options.forward
                || match d {
                    Direction::North => self.get_cell(x, y + 1),
                    Direction::South => self.get_cell(x, y - 1),
                    Direction::East => self.get_cell(x + 1, y),
                    Direction::West => self.get_cell(x - 1, y),
                };

            let right_blocked = !move_options.right
                || match d {
                    Direction::North => self.get_cell(x + 1, y),
                    Direction::South => self.get_cell(x - 1, y),
                    Direction::East => self.get_cell(x, y - 1),
                    Direction::West => self.get_cell(x, y + 1),
                };

            let rear_blocked = match d {
                Direction::North => self.get_cell(x, y - 1),
                Direction::South => self.get_cell(x, y + 1),
                Direction::East => self.get_cell(x - 1, y),
                Direction::West => self.get_cell(x + 1, y),
            };

            let num_blocked =
                [front_blocked, left_blocked, right_blocked, rear_blocked]
                    .iter()
                    .filter(|&c| *c)
                    .count();

            if num_blocked == 3 {
                self.cells[ux][uy] = true;
            }

            let c = self.cells[ux][uy];

            if move_options.forward && !front_blocked {
                F_MOVES
            } else if move_options.left && !left_blocked {
                L_MOVES
            } else if move_options.right && !right_blocked {
                R_MOVES
            } else {
                B_MOVES
            }
        }
    }
}

pub struct CountingNavigate {
    cells: [[u8; 16]; 16],
}

impl CountingNavigate {
    pub fn new() -> CountingNavigate {
        CountingNavigate {
            cells: [[0; 16]; 16],
        }
    }
}

impl Navigate for CountingNavigate {
    type Cell = u8;

    fn get_cell(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x <= 15 && y >= 0 && y <= 15 {
            self.cells[x as usize][y as usize]
        } else {
            255
        }
    }

    fn navigate(
        &mut self,
        x: usize,
        y: usize,
        d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        let x = x as i32;
        let y = y as i32;
        let ux = if x < 0 {
            0
        } else if x > 15 {
            15
        } else {
            x
        } as usize;
        let uy = if y < 0 {
            0
        } else if y > 15 {
            15
        } else {
            y
        } as usize;

        if self.cells[ux][uy] < 255 {
            self.cells[ux][uy] += 1;
        }

        let c = self.cells[ux][uy];

        // win condition
        if x >= 7 && x <= 8 && y >= 7 && y <= 8 {
            [Some(Move::TurnLeft), Some(Move::TurnLeft)]
        } else {
            let left_cell = match d {
                Direction::North => self.get_cell(x - 1, y),
                Direction::South => self.get_cell(x + 1, y),
                Direction::East => self.get_cell(x, y + 1),
                Direction::West => self.get_cell(x, y - 1),
            };

            let front_cell = match d {
                Direction::North => self.get_cell(x, y + 1),
                Direction::South => self.get_cell(x, y - 1),
                Direction::East => self.get_cell(x + 1, y),
                Direction::West => self.get_cell(x - 1, y),
            };

            let right_cell = match d {
                Direction::North => self.get_cell(x + 1, y),
                Direction::South => self.get_cell(x - 1, y),
                Direction::East => self.get_cell(x, y - 1),
                Direction::West => self.get_cell(x, y + 1),
            };

            if move_options.forward
                && if move_options.left {
                    front_cell <= left_cell
                } else {
                    true
                }
                && if move_options.right {
                    front_cell <= right_cell
                } else {
                    true
                }
            {
                F_MOVES
            } else if move_options.left
                && if move_options.forward {
                    left_cell <= front_cell
                } else {
                    true
                }
                && if move_options.right {
                    left_cell <= right_cell
                } else {
                    true
                }
            {
                L_MOVES
            } else if move_options.right
                && if move_options.forward {
                    right_cell <= front_cell
                } else {
                    true
                }
                && if move_options.left {
                    right_cell <= left_cell
                } else {
                    true
                }
            {
                R_MOVES
            } else {
                B_MOVES
            }
        }
    }
}

pub struct CountingDeadEndNavigate {
    cells: [[u8; 16]; 16],
}

impl CountingDeadEndNavigate {
    pub fn new() -> CountingDeadEndNavigate {
        CountingDeadEndNavigate {
            cells: [[0; 16]; 16],
        }
    }
}

impl Navigate for CountingDeadEndNavigate {
    type Cell = u8;

    fn get_cell(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x <= 15 && y >= 0 && y <= 15 {
            self.cells[x as usize][y as usize]
        } else {
            255
        }
    }

    fn navigate(
        &mut self,
        x: usize,
        y: usize,
        d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        let x = x as i32;
        let y = y as i32;
        let ux = if x < 0 {
            0
        } else if x > 15 {
            15
        } else {
            x
        } as usize;
        let uy = if y < 0 {
            0
        } else if y > 15 {
            15
        } else {
            y
        } as usize;

        if self.cells[ux][uy] < 255 {
            self.cells[ux][uy] += 1;
        }

        // win condition
        if x >= 7 && x <= 8 && y >= 7 && y <= 8 {
            [Some(Move::TurnLeft), Some(Move::TurnLeft)]
        } else {
            let left_cell = match d {
                Direction::North => self.get_cell(x - 1, y),
                Direction::South => self.get_cell(x + 1, y),
                Direction::East => self.get_cell(x, y + 1),
                Direction::West => self.get_cell(x, y - 1),
            };

            let front_cell = match d {
                Direction::North => self.get_cell(x, y + 1),
                Direction::South => self.get_cell(x, y - 1),
                Direction::East => self.get_cell(x + 1, y),
                Direction::West => self.get_cell(x - 1, y),
            };

            let right_cell = match d {
                Direction::North => self.get_cell(x + 1, y),
                Direction::South => self.get_cell(x - 1, y),
                Direction::East => self.get_cell(x, y - 1),
                Direction::West => self.get_cell(x, y + 1),
            };

            let rear_cell = match d {
                Direction::North => self.get_cell(x, y - 1),
                Direction::South => self.get_cell(x, y + 1),
                Direction::East => self.get_cell(x - 1, y),
                Direction::West => self.get_cell(x + 1, y),
            };

            let front_blocked = front_cell == 255 || !move_options.forward;
            let left_blocked = left_cell == 255 || !move_options.left;
            let right_blocked = right_cell == 255 || !move_options.right;
            let rear_blocked = rear_cell == 255;

            let num_blocked =
                [front_blocked, left_blocked, right_blocked, rear_blocked]
                    .iter()
                    .filter(|&c| *c)
                    .count();

            if num_blocked == 3 {
                self.cells[ux][uy] = 255;
            }

            if move_options.forward
                && if move_options.left {
                    front_cell <= left_cell
                } else {
                    true
                }
                && if move_options.right {
                    front_cell <= right_cell
                } else {
                    true
                }
            {
                F_MOVES
            } else if move_options.left
                && if move_options.forward {
                    left_cell <= front_cell
                } else {
                    true
                }
                && if move_options.right {
                    left_cell <= right_cell
                } else {
                    true
                }
            {
                L_MOVES
            } else if move_options.right
                && if move_options.forward {
                    right_cell <= front_cell
                } else {
                    true
                }
                && if move_options.left {
                    right_cell <= left_cell
                } else {
                    true
                }
            {
                R_MOVES
            } else {
                B_MOVES
            }
        }
    }
}

pub struct FloodFillNavigate {
    cells: [[u8; 16]; 16],
}

impl FloodFillNavigate {
    pub fn new() -> FloodFillNavigate {
        let mut cells = [[0; 16]; 16];

        for x in 0..16 {
            let lx = if x <= 7 { 7 - x } else { x - 8 };
            for y in 0..16 {
                let ly = if y <= 7 { 7 - y } else { y - 8 };
                cells[x][y] = (lx + ly) as u8;
            }
        }

        FloodFillNavigate { cells }
    }
}

impl Navigate for FloodFillNavigate {
    type Cell = u8;

    fn get_cell(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x <= 15 && y >= 0 && y <= 15 {
            self.cells[x as usize][y as usize]
        } else {
            255
        }
    }

    fn navigate(
        &mut self,
        x: usize,
        y: usize,
        d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        let x = x as i32;
        let y = y as i32;
        let ux = if x < 0 {
            0
        } else if x > 15 {
            15
        } else {
            x
        } as usize;
        let uy = if y < 0 {
            0
        } else if y > 15 {
            15
        } else {
            y
        } as usize;

        if self.cells[ux][uy] < 255 {
            self.cells[ux][uy] += 1;
        }

        // win condition
        if x >= 7 && x <= 8 && y >= 7 && y <= 8 {
            [Some(Move::TurnLeft), Some(Move::TurnLeft)]
        } else {
            let left_cell = match d {
                Direction::North => self.get_cell(x - 1, y),
                Direction::South => self.get_cell(x + 1, y),
                Direction::East => self.get_cell(x, y + 1),
                Direction::West => self.get_cell(x, y - 1),
            };

            let front_cell = match d {
                Direction::North => self.get_cell(x, y + 1),
                Direction::South => self.get_cell(x, y - 1),
                Direction::East => self.get_cell(x + 1, y),
                Direction::West => self.get_cell(x - 1, y),
            };

            let right_cell = match d {
                Direction::North => self.get_cell(x + 1, y),
                Direction::South => self.get_cell(x - 1, y),
                Direction::East => self.get_cell(x, y - 1),
                Direction::West => self.get_cell(x, y + 1),
            };

            if move_options.forward
                && if move_options.left {
                    front_cell <= left_cell
                } else {
                    true
                }
                && if move_options.right {
                    front_cell <= right_cell
                } else {
                    true
                }
            {
                F_MOVES
            } else if move_options.left
                && if move_options.forward {
                    left_cell <= front_cell
                } else {
                    true
                }
                && if move_options.right {
                    left_cell <= right_cell
                } else {
                    true
                }
            {
                L_MOVES
            } else if move_options.right
                && if move_options.forward {
                    right_cell <= front_cell
                } else {
                    true
                }
                && if move_options.left {
                    right_cell <= left_cell
                } else {
                    true
                }
            {
                R_MOVES
            } else {
                B_MOVES
            }
        }
    }
}

pub struct FloodFillDeadEndNavigate {
    cells: [[u8; 16]; 16],
}

impl FloodFillDeadEndNavigate {
    pub fn new() -> FloodFillDeadEndNavigate {
        let mut cells = [[0; 16]; 16];

        for x in 0..16 {
            let lx = if x <= 7 { 7 - x } else { x - 8 };
            for y in 0..16 {
                let ly = if y <= 7 { 7 - y } else { y - 8 };
                cells[x][y] = (lx + ly) as u8;
            }
        }

        FloodFillDeadEndNavigate { cells }
    }
}

impl Navigate for FloodFillDeadEndNavigate {
    type Cell = u8;

    fn get_cell(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x <= 15 && y >= 0 && y <= 15 {
            self.cells[x as usize][y as usize]
        } else {
            255
        }
    }

    fn navigate(
        &mut self,
        x: usize,
        y: usize,
        d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        let x = x as i32;
        let y = y as i32;
        let ux = if x < 0 {
            0
        } else if x > 15 {
            15
        } else {
            x
        } as usize;
        let uy = if y < 0 {
            0
        } else if y > 15 {
            15
        } else {
            y
        } as usize;

        if self.cells[ux][uy] < 255 {
            self.cells[ux][uy] += 1;
        }

        // win condition
        if x >= 7 && x <= 8 && y >= 7 && y <= 8 {
            [Some(Move::TurnLeft), Some(Move::TurnLeft)]
        } else {
            let left_cell = match d {
                Direction::North => self.get_cell(x - 1, y),
                Direction::South => self.get_cell(x + 1, y),
                Direction::East => self.get_cell(x, y + 1),
                Direction::West => self.get_cell(x, y - 1),
            };

            let front_cell = match d {
                Direction::North => self.get_cell(x, y + 1),
                Direction::South => self.get_cell(x, y - 1),
                Direction::East => self.get_cell(x + 1, y),
                Direction::West => self.get_cell(x - 1, y),
            };

            let right_cell = match d {
                Direction::North => self.get_cell(x + 1, y),
                Direction::South => self.get_cell(x - 1, y),
                Direction::East => self.get_cell(x, y - 1),
                Direction::West => self.get_cell(x, y + 1),
            };

            let rear_cell = match d {
                Direction::North => self.get_cell(x, y - 1),
                Direction::South => self.get_cell(x, y + 1),
                Direction::East => self.get_cell(x - 1, y),
                Direction::West => self.get_cell(x + 1, y),
            };

            let front_blocked = front_cell == 255 || !move_options.forward;
            let left_blocked = left_cell == 255 || !move_options.left;
            let right_blocked = right_cell == 255 || !move_options.right;
            let rear_blocked = rear_cell == 255;

            let num_blocked =
                [front_blocked, left_blocked, right_blocked, rear_blocked]
                    .iter()
                    .filter(|&c| *c)
                    .count();

            if num_blocked == 3 {
                self.cells[ux][uy] = 255;
            }

            if move_options.forward
                && if move_options.left {
                    front_cell <= left_cell
                } else {
                    true
                }
                && if move_options.right {
                    front_cell <= right_cell
                } else {
                    true
                }
            {
                F_MOVES
            } else if move_options.left
                && if move_options.forward {
                    left_cell <= front_cell
                } else {
                    true
                }
                && if move_options.right {
                    left_cell <= right_cell
                } else {
                    true
                }
            {
                L_MOVES
            } else if move_options.right
                && if move_options.forward {
                    right_cell <= front_cell
                } else {
                    true
                }
                && if move_options.left {
                    right_cell <= left_cell
                } else {
                    true
                }
            {
                R_MOVES
            } else {
                B_MOVES
            }
        }
    }
}

pub struct FloodFillSquareNavigate {
    cells: [[u8; 16]; 16],
}

impl FloodFillSquareNavigate {
    pub fn new() -> FloodFillSquareNavigate {
        let mut cells = [[0; 16]; 16];

        for x in 0..16 {
            let lx = if x <= 7 { 7 - x } else { x - 8 };
            for y in 0..16 {
                let ly = if y <= 7 { 7 - y } else { y - 8 };
                cells[x][y] = usize::max(lx, ly) as u8;
            }
        }

        FloodFillSquareNavigate { cells }
    }
}

impl Navigate for FloodFillSquareNavigate {
    type Cell = u8;

    fn get_cell(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x <= 15 && y >= 0 && y <= 15 {
            self.cells[x as usize][y as usize]
        } else {
            255
        }
    }

    fn navigate(
        &mut self,
        x: usize,
        y: usize,
        d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        let x = x as i32;
        let y = y as i32;
        let ux = if x < 0 {
            0
        } else if x > 15 {
            15
        } else {
            x
        } as usize;
        let uy = if y < 0 {
            0
        } else if y > 15 {
            15
        } else {
            y
        } as usize;

        if self.cells[ux][uy] < 255 {
            self.cells[ux][uy] += 1;
        }

        // win condition
        if x >= 7 && x <= 8 && y >= 7 && y <= 8 {
            [Some(Move::TurnLeft), Some(Move::TurnLeft)]
        } else {
            let left_cell = match d {
                Direction::North => self.get_cell(x - 1, y),
                Direction::South => self.get_cell(x + 1, y),
                Direction::East => self.get_cell(x, y + 1),
                Direction::West => self.get_cell(x, y - 1),
            };

            let front_cell = match d {
                Direction::North => self.get_cell(x, y + 1),
                Direction::South => self.get_cell(x, y - 1),
                Direction::East => self.get_cell(x + 1, y),
                Direction::West => self.get_cell(x - 1, y),
            };

            let right_cell = match d {
                Direction::North => self.get_cell(x + 1, y),
                Direction::South => self.get_cell(x - 1, y),
                Direction::East => self.get_cell(x, y - 1),
                Direction::West => self.get_cell(x, y + 1),
            };

            if move_options.forward
                && if move_options.left {
                    front_cell <= left_cell
                } else {
                    true
                }
                && if move_options.right {
                    front_cell <= right_cell
                } else {
                    true
                }
            {
                F_MOVES
            } else if move_options.left
                && if move_options.forward {
                    left_cell <= front_cell
                } else {
                    true
                }
                && if move_options.right {
                    left_cell <= right_cell
                } else {
                    true
                }
            {
                L_MOVES
            } else if move_options.right
                && if move_options.forward {
                    right_cell <= front_cell
                } else {
                    true
                }
                && if move_options.left {
                    right_cell <= left_cell
                } else {
                    true
                }
            {
                R_MOVES
            } else {
                B_MOVES
            }
        }
    }
}

pub struct FloodFillSquareDeadEndNavigate {
    cells: [[u8; 16]; 16],
}

impl FloodFillSquareDeadEndNavigate {
    pub fn new() -> FloodFillSquareDeadEndNavigate {
        let mut cells = [[0; 16]; 16];

        for x in 0..16 {
            let lx = if x <= 7 { 7 - x } else { x - 8 };
            for y in 0..16 {
                let ly = if y <= 7 { 7 - y } else { y - 8 };
                cells[x][y] = usize::max(lx, ly) as u8;
            }
        }

        FloodFillSquareDeadEndNavigate { cells }
    }
}

impl Navigate for FloodFillSquareDeadEndNavigate {
    type Cell = u8;

    fn get_cell(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x <= 15 && y >= 0 && y <= 15 {
            self.cells[x as usize][y as usize]
        } else {
            255
        }
    }

    fn navigate(
        &mut self,
        x: usize,
        y: usize,
        d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        let x = x as i32;
        let y = y as i32;
        let ux = if x < 0 {
            0
        } else if x > 15 {
            15
        } else {
            x
        } as usize;
        let uy = if y < 0 {
            0
        } else if y > 15 {
            15
        } else {
            y
        } as usize;

        if self.cells[ux][uy] < 255 {
            self.cells[ux][uy] += 1;
        }

        // win condition
        if x >= 7 && x <= 8 && y >= 7 && y <= 8 {
            [Some(Move::TurnLeft), Some(Move::TurnLeft)]
        } else {
            let left_cell = match d {
                Direction::North => self.get_cell(x - 1, y),
                Direction::South => self.get_cell(x + 1, y),
                Direction::East => self.get_cell(x, y + 1),
                Direction::West => self.get_cell(x, y - 1),
            };

            let front_cell = match d {
                Direction::North => self.get_cell(x, y + 1),
                Direction::South => self.get_cell(x, y - 1),
                Direction::East => self.get_cell(x + 1, y),
                Direction::West => self.get_cell(x - 1, y),
            };

            let right_cell = match d {
                Direction::North => self.get_cell(x + 1, y),
                Direction::South => self.get_cell(x - 1, y),
                Direction::East => self.get_cell(x, y - 1),
                Direction::West => self.get_cell(x, y + 1),
            };

            let rear_cell = match d {
                Direction::North => self.get_cell(x, y - 1),
                Direction::South => self.get_cell(x, y + 1),
                Direction::East => self.get_cell(x - 1, y),
                Direction::West => self.get_cell(x + 1, y),
            };

            let front_blocked = front_cell == 255 || !move_options.forward;
            let left_blocked = left_cell == 255 || !move_options.left;
            let right_blocked = right_cell == 255 || !move_options.right;
            let rear_blocked = rear_cell == 255;

            let num_blocked =
                [front_blocked, left_blocked, right_blocked, rear_blocked]
                    .iter()
                    .filter(|&c| *c)
                    .count();

            if num_blocked == 3 {
                self.cells[ux][uy] = 255;
            }

            if move_options.forward
                && if move_options.left {
                    front_cell <= left_cell
                } else {
                    true
                }
                && if move_options.right {
                    front_cell <= right_cell
                } else {
                    true
                }
            {
                F_MOVES
            } else if move_options.left
                && if move_options.forward {
                    left_cell <= front_cell
                } else {
                    true
                }
                && if move_options.right {
                    left_cell <= right_cell
                } else {
                    true
                }
            {
                L_MOVES
            } else if move_options.right
                && if move_options.forward {
                    right_cell <= front_cell
                } else {
                    true
                }
                && if move_options.left {
                    right_cell <= left_cell
                } else {
                    true
                }
            {
                R_MOVES
            } else {
                B_MOVES
            }
        }
    }
}

const CENTER_LEFT: [[Option<Move>; 2]; 3] = [F_MOVES, L_MOVES, R_MOVES];
const CENTER_RIGHT: [[Option<Move>; 2]; 3] = [F_MOVES, R_MOVES, L_MOVES];
const LEFT: [[Option<Move>; 2]; 3] = [L_MOVES, F_MOVES, R_MOVES];
const RIGHT: [[Option<Move>; 2]; 3] = [R_MOVES, F_MOVES, L_MOVES];

pub struct TwelvePartitionNavigate {
    cells: [[u8; 16]; 16],
}

impl TwelvePartitionNavigate {
    pub fn new() -> TwelvePartitionNavigate {
        TwelvePartitionNavigate {
            cells: [[0; 16]; 16],
        }
    }
}

impl Navigate for TwelvePartitionNavigate {
    type Cell = u8;

    fn get_cell(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x <= 15 && y >= 0 && y <= 15 {
            self.cells[x as usize][y as usize]
        } else {
            255
        }
    }

    fn navigate(
        &mut self,
        x: usize,
        y: usize,
        d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        let x = x as i32;
        let y = y as i32;
        let ux = if x < 0 {
            0
        } else if x > 15 {
            15
        } else {
            x
        } as usize;
        let uy = if y < 0 {
            0
        } else if y > 15 {
            15
        } else {
            y
        } as usize;

        if self.cells[ux][uy] < 255 {
            self.cells[ux][uy] += 1;
        }

        // win condition
        if x >= 7 && x <= 8 && y >= 7 && y <= 8 {
            [Some(Move::TurnLeft), Some(Move::TurnLeft)]
        } else {
            let left_cell = match d {
                Direction::North => self.get_cell(x - 1, y),
                Direction::South => self.get_cell(x + 1, y),
                Direction::East => self.get_cell(x, y + 1),
                Direction::West => self.get_cell(x, y - 1),
            };

            let front_cell = match d {
                Direction::North => self.get_cell(x, y + 1),
                Direction::South => self.get_cell(x, y - 1),
                Direction::East => self.get_cell(x + 1, y),
                Direction::West => self.get_cell(x - 1, y),
            };

            let right_cell = match d {
                Direction::North => self.get_cell(x + 1, y),
                Direction::South => self.get_cell(x - 1, y),
                Direction::East => self.get_cell(x, y - 1),
                Direction::West => self.get_cell(x, y + 1),
            };

            let rear_cell = match d {
                Direction::North => self.get_cell(x, y - 1),
                Direction::South => self.get_cell(x, y + 1),
                Direction::East => self.get_cell(x - 1, y),
                Direction::West => self.get_cell(x + 1, y),
            };

            let front_blocked = front_cell == 255 || !move_options.forward;
            let left_blocked = left_cell == 255 || !move_options.left;
            let right_blocked = right_cell == 255 || !move_options.right;
            let rear_blocked = rear_cell == 255;

            let num_blocked =
                [front_blocked, left_blocked, right_blocked, rear_blocked]
                    .iter()
                    .filter(|&c| *c)
                    .count();

            if num_blocked == 3 {
                self.cells[ux][uy] = 255;
            }

            let possibilities = match (x, y) {
                (x, y) if x < 7 && y < 7 => match d {
                    Direction::North => CENTER_RIGHT,
                    Direction::South => LEFT,
                    Direction::East => CENTER_LEFT,
                    Direction::West => RIGHT,
                },

                (x, y) if x > 8 && y < 7 => match d {
                    Direction::North => CENTER_LEFT,
                    Direction::South => RIGHT,
                    Direction::East => LEFT,
                    Direction::West => CENTER_RIGHT,
                },

                (x, y) if x > 8 && y > 8 => match d {
                    Direction::North => LEFT,
                    Direction::South => CENTER_RIGHT,
                    Direction::East => RIGHT,
                    Direction::West => CENTER_LEFT,
                },

                (x, y) if x < 7 && y > 8 => match d {
                    Direction::North => RIGHT,
                    Direction::South => CENTER_LEFT,
                    Direction::East => CENTER_RIGHT,
                    Direction::West => LEFT,
                },

                (7, y) if y < 7 => match d {
                    Direction::North => CENTER_RIGHT,
                    Direction::South => LEFT,
                    Direction::East => LEFT,
                    Direction::West => RIGHT,
                },

                (8, y) if y < 7 => match d {
                    Direction::North => CENTER_LEFT,
                    Direction::South => RIGHT,
                    Direction::East => LEFT,
                    Direction::West => RIGHT,
                },

                (x, 7) if x > 8 => match d {
                    Direction::North => LEFT,
                    Direction::South => RIGHT,
                    Direction::East => CENTER_RIGHT,
                    Direction::West => LEFT,
                },

                (x, 8) if x > 8 => match d {
                    Direction::North => LEFT,
                    Direction::South => RIGHT,
                    Direction::East => CENTER_LEFT,
                    Direction::West => RIGHT,
                },

                (8, y) if y > 8 => match d {
                    Direction::North => LEFT,
                    Direction::South => CENTER_RIGHT,
                    Direction::East => RIGHT,
                    Direction::West => LEFT,
                },

                (7, y) if y > 8 => match d {
                    Direction::North => RIGHT,
                    Direction::South => CENTER_LEFT,
                    Direction::East => RIGHT,
                    Direction::West => LEFT,
                },

                (x, 8) if x < 7 => match d {
                    Direction::North => RIGHT,
                    Direction::South => LEFT,
                    Direction::East => CENTER_RIGHT,
                    Direction::West => LEFT,
                },

                (x, 7) if x < 7 => match d {
                    Direction::North => RIGHT,
                    Direction::South => LEFT,
                    Direction::East => CENTER_LEFT,
                    Direction::West => RIGHT,
                },

                (x, y) if x >= 7 && x <= 8 && y >= 7 && y <= 8 => [
                    [Some(Move::TurnAround), Some(Move::TurnAround)],
                    [Some(Move::TurnAround), Some(Move::TurnAround)],
                    [Some(Move::TurnAround), Some(Move::TurnAround)],
                ],

                (_, _) => panic!("Invalid location!"),
            };

            let mut moves = B_MOVES;

            // filter by walls
            let possibilities_iter =
                possibilities.iter().filter(|&moves| match moves {
                    &F_MOVES => move_options.forward,
                    &L_MOVES => move_options.left,
                    &R_MOVES => move_options.right,
                    _ => true,
                });

            let &min = [
                if move_options.forward {
                    front_cell
                } else {
                    255
                },
                if move_options.left { left_cell } else { 255 },
                if move_options.right { right_cell } else { 255 },
            ]
            .iter()
            .min()
            .unwrap();

            for &possibile_moves in possibilities_iter {
                let value = match possibile_moves {
                    F_MOVES => front_cell,
                    L_MOVES => left_cell,
                    R_MOVES => right_cell,
                    _ => 0,
                };

                if value == min {
                    moves = possibile_moves;
                    break;
                }
            }

            moves
        }
    }
}
