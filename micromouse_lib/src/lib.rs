
#![no_std]

pub mod remote;

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
pub enum AbsoluteMove {
    North,
    South,
    East,
    West,
}

pub fn absolute_to_realative(direction: Direction, absolute_move: AbsoluteMove) -> RelativeMove {
    match (direction, absolute_move) {
        (Direction::North, AbsoluteMove::North) => RelativeMove::Forward,
        (Direction::North, AbsoluteMove::South) => RelativeMove::Backward,
        (Direction::North, AbsoluteMove::East) => RelativeMove::Right,
        (Direction::North, AbsoluteMove::West) => RelativeMove::Left,

        (Direction::South, AbsoluteMove::North) => RelativeMove::Backward,
        (Direction::South, AbsoluteMove::South) => RelativeMove::Forward,
        (Direction::South, AbsoluteMove::East) => RelativeMove::Left,
        (Direction::South, AbsoluteMove::West) => RelativeMove::Right,

        (Direction::East, AbsoluteMove::North) => RelativeMove::Left,
        (Direction::East, AbsoluteMove::South) => RelativeMove::Right,
        (Direction::East, AbsoluteMove::East) => RelativeMove::Forward,
        (Direction::East, AbsoluteMove::West) => RelativeMove::Backward,

        (Direction::West, AbsoluteMove::North) => RelativeMove::Right,
        (Direction::West, AbsoluteMove::South) => RelativeMove::Left,
        (Direction::West, AbsoluteMove::East) => RelativeMove::Backward,
        (Direction::West, AbsoluteMove::West) => RelativeMove::Forward,
    }
}

#[test]
fn test_absolute_to_realative() {
        assert_eq!(absolute_to_realative(Direction::North, AbsoluteMove::North), RelativeMove::Forward);
        assert_eq!(absolute_to_realative(Direction::North, AbsoluteMove::South), RelativeMove::Backward);
        assert_eq!(absolute_to_realative(Direction::North, AbsoluteMove::East), RelativeMove::Right);
        assert_eq!(absolute_to_realative(Direction::North, AbsoluteMove::West), RelativeMove::Left);

        assert_eq!(absolute_to_realative(Direction::South, AbsoluteMove::North), RelativeMove::Backward);
        assert_eq!(absolute_to_realative(Direction::South, AbsoluteMove::South), RelativeMove::Forward);
        assert_eq!(absolute_to_realative(Direction::South, AbsoluteMove::East), RelativeMove::Left);
        assert_eq!(absolute_to_realative(Direction::South, AbsoluteMove::West), RelativeMove::Right);

        assert_eq!(absolute_to_realative(Direction::East, AbsoluteMove::North), RelativeMove::Left);
        assert_eq!(absolute_to_realative(Direction::East, AbsoluteMove::South), RelativeMove::Right);
        assert_eq!(absolute_to_realative(Direction::East, AbsoluteMove::East), RelativeMove::Forward);
        assert_eq!(absolute_to_realative(Direction::East, AbsoluteMove::West), RelativeMove::Backward);

        assert_eq!(absolute_to_realative(Direction::West, AbsoluteMove::North), RelativeMove::Right);
        assert_eq!(absolute_to_realative(Direction::West, AbsoluteMove::South), RelativeMove::Left);
        assert_eq!(absolute_to_realative(Direction::West, AbsoluteMove::East), RelativeMove::Backward);
        assert_eq!(absolute_to_realative(Direction::West, AbsoluteMove::West), RelativeMove::Forward);
}

#[derive(Debug, PartialEq)]
pub enum RelativeMove {
    Forward,
    Backward,
    Left,
    Right,
}

pub enum DiagonalEnd {
    Left45,
    Left135,
    Right45,
    Right135,
}

pub enum CombinedMove {
    Straight {cells: u8},
    Left90,
    Left180,
    Right90,
    Right180,
    Around,
    Diagonal {start: DiagonalEnd, cells: u8, end: DiagonalEnd},
}

pub struct VelocityProfile {
    
}



