use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;

use crate::mouse::Direction;

#[derive(Debug, Copy, Clone)]
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
    fn navigate(
        &mut self,
        x: usize,
        y: usize,
        d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2];
}

pub struct LeftWall {}

impl LeftWall {
    pub fn new() -> LeftWall {
        LeftWall {}
    }
}

impl Navigate for LeftWall {
    fn navigate(
        &mut self,
        _x: usize,
        _y: usize,
        _d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        if move_options.left {
            [Some(Move::TurnLeft), Some(Move::Forward)]
        } else if move_options.forward {
            [Some(Move::Forward), None]
        } else if move_options.right {
            [Some(Move::TurnRight), Some(Move::Forward)]
        } else {
            [Some(Move::TurnAround), Some(Move::Forward)]
        }
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
    fn navigate(
        &mut self,
        _x: usize,
        _y: usize,
        _d: Direction,
        move_options: MoveOptions,
    ) -> [Option<Move>; 2] {
        match (move_options.left, move_options.forward, move_options.right) {
            (true, true, true) => match self.rng.gen_range(0, 3) {
                0 => [Some(Move::TurnLeft), Some(Move::Forward)],
                1 => [Some(Move::TurnRight), Some(Move::Forward)],
                _ => [Some(Move::Forward), None],
            },

            (true, false, true) => match self.rng.gen_range(0, 2) {
                0 => [Some(Move::TurnLeft), Some(Move::Forward)],
                _ => [Some(Move::TurnRight), Some(Move::Forward)],
            },

            (false, true, true) => match self.rng.gen_range(0, 2) {
                0 => [Some(Move::TurnRight), Some(Move::Forward)],
                _ => [Some(Move::Forward), None],
            },

            (true, true, false) => match self.rng.gen_range(0, 2) {
                0 => [Some(Move::TurnLeft), Some(Move::Forward)],
                _ => [Some(Move::Forward), None],
            },

            (false, true, false) => [Some(Move::Forward), None],

            (true, false, false) => [Some(Move::TurnLeft), Some(Move::Forward)],

            (false, false, true) => {
                [Some(Move::TurnRight), Some(Move::Forward)]
            }

            (false, false, false) => {
                [Some(Move::TurnAround), Some(Move::Forward)]
            }
        }
    }
}

pub struct CountingNavigate {
    cells: [[u8; 16]; 16],
    left_cell: u8,
    right_cell: u8,
    front_cell: u8,
}

impl CountingNavigate {
    pub fn new() -> CountingNavigate {
        CountingNavigate {
            cells: [[0; 16]; 16],
            left_cell: 0,
            right_cell: 0,
            front_cell: 0,
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> u8 {
        if x >= 0 && x <= 15 && y >= 0 && y <= 15 {
            self.cells[x as usize][y as usize]
        } else {
            255
        }
    }
}

impl Navigate for CountingNavigate {
    fn navigate(&mut self, x: usize, y: usize, d: Direction, move_options: MoveOptions) -> [Option<Move>; 2] {

        let x = x as i32;
        let y = y as i32;
        let ux = if x < 0 { 0 } else if x > 15 { 15 } else { x } as usize;
        let uy = if y < 0 { 0 } else if y > 15 { 15 } else { y } as usize;

        if self.cells[ux][uy] < 255 {
            self.cells[ux][uy] += 1;
        }

        // win condition
        if x >= 7 && x <= 8 && y >= 7 && y <= 8 {
            [Some(Move::TurnLeft), Some(Move::TurnLeft)]
        } else {

            let left_cell = match d {
                Direction::North => self.get_cell(x-1, y),
                Direction::South => self.get_cell(x+1, y),
                Direction::East => self.get_cell(x, y+1),
                Direction::West => self.get_cell(x, y-1),
            };

            self.left_cell = left_cell;

            let front_cell = match d {
                Direction::North => self.get_cell(x, y+1),
                Direction::South => self.get_cell(x, y-1),
                Direction::East => self.get_cell(x+1, y),
                Direction::West => self.get_cell(x-1, y),
            };

            self.left_cell = left_cell;

            let right_cell = match d {
                Direction::North => self.get_cell(x+1, y),
                Direction::South => self.get_cell(x-1, y),
                Direction::East => self.get_cell(x, y-1),
                Direction::West=> self.get_cell(x, y+1),
            };

            self.right_cell = right_cell;

            if
                move_options.forward &&
                if move_options.left { front_cell <= left_cell } else { true } &&
                if move_options.right { front_cell <= right_cell } else { true }
            {
                [Some(Move::Forward), None]
            } else if
                move_options.left &&
                if move_options.forward { left_cell <= front_cell } else { true } &&
                if move_options.right { left_cell <= right_cell } else { true }
            {
                [Some(Move::TurnLeft), Some(Move::Forward)]
            } else if
                move_options.right &&
                if move_options.forward { right_cell <= front_cell } else { true } &&
                if move_options.left { right_cell <= left_cell } else { true }
            {
                [Some(Move::TurnRight), Some(Move::Forward)]
            } else {
                [Some(Move::TurnAround), Some(Move::Forward)]
            }
        }
    }
}
