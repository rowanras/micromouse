pub mod left;
pub mod right;

pub trait Motor {
    fn change_power(&mut self, power: i32);
}

pub trait Encoder {
    fn count(&self) -> i32;
    fn reset(&mut self);
}
