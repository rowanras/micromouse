
#![no_std]

pub trait BytesN {
    pub type Slice;
}

pub struct Bytes1 { }
impl BytesN for Bytes1 {
    type Slice = [u8; 1];
}

pub struct Bytes2 { }
impl BytesN for Bytes2 {
    type Slice = [u8; 2];
}

pub struct Bytes4 { }
impl BytesN for Bytes4 {
    type Slice = [u8; 4];
}

pub struct Bytes8 { }
impl BytesN for Bytes8 {
    type Slice = [u8; 8];
}

pub trait ToBytes<N: BytesN> {
    fn to_bytes(&self, &mut BytesN::Slice);
}

pub struct LeftMotorPowerMsg {
    power: u16
}

pub struct RightMotorMsg {
    power: u16
}

pub struct LeftEncoderMsg {
    ticks: u16
}

pub struct RightEncoderMsg {
    ticks: u16
}

pub struct LeftDistanceMsg {
    mm: u8
}

pub struct RightDistanceMsg {
    mm: u8
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct MazeLocation {
    x: u8,
    y: u8,
    direction: Direction,
}

pub strcut CellOffset {
    x: f64,
    y: f64,
    direction: 
}
