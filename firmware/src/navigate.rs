use core::fmt::Write;

use ignore_result::Ignore;

use rand::rngs::SmallRng;
use rand::Rng;
use rand::SeedableRng;

use crate::plan::Direction;

use crate::plan::Move;
use crate::plan::MoveOptions;

use crate::uart::Uart;
use crate::uart::Command;

pub trait Navigate: Command {
    fn navigate(&mut self, x: i32, y: i32, dir: Direction, move_options: MoveOptions) -> [Option<Move>; 2];
}

pub struct LessRandomNavigate {
    rng: SmallRng,
    cells: [[u8; 16]; 16],
}

impl LessRandomNavigate {
    pub fn new(seed: [u8; 16]) -> LessRandomNavigate {
        LessRandomNavigate {
            rng: SmallRng::from_seed(seed),
            cells: [[0; 16]; 16],
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

impl Navigate for LessRandomNavigate {
    fn navigate(&mut self, x: i32, y: i32, d: Direction, move_options: MoveOptions) -> [Option<Move>; 2] {

        let ux = if x < 0 { 0 } else if x > 15 { 15 } else { x } as usize;
        let uy = if y < 0 { 0 } else if y > 15 { 15 } else { y } as usize;

        if self.cells[ux][uy] < 255 {
            self.cells[ux][uy] += 1;
        }

        let left_cell = match d {
            Direction::Up => self.get_cell(x-1, y),
            Direction::Down => self.get_cell(x+1, y),
            Direction::Right => self.get_cell(x, y+1),
            Direction::Left => self.get_cell(x, y-1),
        };

        let front_cell = match d {
            Direction::Up => self.get_cell(x, y+1),
            Direction::Down => self.get_cell(x, y-1),
            Direction::Right => self.get_cell(x+1, y),
            Direction::Left => self.get_cell(x-1, y),
        };

        let right_cell = match d {
            Direction::Up => self.get_cell(x+1, y),
            Direction::Down => self.get_cell(x-1, y),
            Direction::Right => self.get_cell(x, y-1),
            Direction::Left => self.get_cell(x, y+1),
        };

        if
            move_options.forward &&
            front_cell <= left_cell &&
            front_cell <= right_cell
        {
            [Some(Move::Forward), None]
        } else if
            move_options.left &&
            left_cell <= front_cell &&
            left_cell <= right_cell
        {
            [Some(Move::TurnLeft), Some(Move::Forward)]
        } else if
            move_options.right &&
            right_cell <= front_cell &&
            right_cell <= left_cell
        {
            [Some(Move::TurnRight), Some(Move::Forward)]
        } else {
            [Some(Move::TurnAround), Some(Move::Forward)]
        }

        /*
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
        */
    }
}

impl Command for LessRandomNavigate {
    fn keyword_command(&self) -> &str {
        "nav"
    }

    fn handle_command<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        uart: &mut Uart,
        mut args: I,
    ) {
        let command = args.next();

        match command {
            Some("cells") => writeln!(uart, "{:?}", self.cells).ignore(),
            c => writeln!(uart, "lrn: unknown command: {:?}", c).ignore(),
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
    fn navigate(&mut self, _x: i32, _y: i32, _d: Direction, move_options: MoveOptions) -> [Option<Move>; 2] {
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

impl Command for RandomNavigate {
    fn keyword_command(&self) -> &str {
        "nav"
    }

    fn handle_command<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        uart: &mut Uart,
        mut args: I,
    ) {
        let command = args.next();

        match command {
            _ => writeln!(uart, "rn: unknown command").ignore(),
        }
    }
}
