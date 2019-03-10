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

