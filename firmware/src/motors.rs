pub mod control;
pub mod left;
pub mod right;

pub trait Motor {
    fn change_velocity(&mut self, speed: i32);
}

pub trait Encoder {
    fn count(&self) -> i32;
}
