use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;

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
    fn navigate(&mut self, move_options: MoveOptions) -> [Option<Move>; 2];
}

pub struct LeftWall {}

impl LeftWall {
    pub fn new() -> LeftWall {
        LeftWall {}
    }
}

impl Navigate for LeftWall {
    fn navigate(&mut self, move_options: MoveOptions) -> [Option<Move>; 2] {
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
    fn navigate(&mut self, move_options: MoveOptions) -> [Option<Move>; 2] {
        match (move_options.left, move_options.forward, move_options.right) {
            (true, true, true) => {
                match self.rng.gen_range(0, 3) {
                    0 => [Some(Move::TurnLeft), Some(Move::Forward)],
                    1 => [Some(Move::TurnRight), Some(Move::Forward)],
                    _ => [Some(Move::Forward), None],
                }
            }

            (true, false, true) => {
                match self.rng.gen_range(0, 2) {
                    0 => [Some(Move::TurnLeft), Some(Move::Forward)],
                    _ => [Some(Move::TurnRight), Some(Move::Forward)],
                }
            }

            (false, true, true) => {
                match self.rng.gen_range(0, 2) {
                    0 => [Some(Move::TurnRight), Some(Move::Forward)],
                    _ => [Some(Move::Forward), None],
                }
            }

            (true, true, false) => {
                match self.rng.gen_range(0, 2) {
                    0 => [Some(Move::TurnLeft), Some(Move::Forward)],
                    _ => [Some(Move::Forward), None],
                }
            }

            (false, true, false) => {
                [Some(Move::Forward), None]
            }

            (true, false, false) => {
                [Some(Move::TurnLeft), Some(Move::Forward)]
            }

            (false, false, true) => {
                [Some(Move::TurnRight), Some(Move::Forward)]
            }

            (false, false, false) => {
                [Some(Move::TurnAround), Some(Move::Forward)]
            }
        }
    }
}
