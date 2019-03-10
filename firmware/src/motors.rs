pub mod left;

#[derive(Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
}

impl core::ops::Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
        }
    }
}

pub trait Motor {
    fn change_speed(&mut self, speed: u32);
    fn change_direction(&mut self, direction: Direction);
}
