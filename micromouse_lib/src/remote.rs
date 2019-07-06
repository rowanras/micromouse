
use arrayvec::ArrayVec;
use arrayvec::Array;

pub trait ToBytes {
    fn to_bytes<B: Array>(&self, bytes: &mut ArrayVec<B>);
}

pub struct Remote<'a, T> {
    fields: &'a [(&'a Fn(T) -> u8, &'a Fn(T, u8) -> T)],
    buffer: ArrayVec<[usize; 256]>,
}

impl<'a, T> Remote<'a, T> {
    pub fn new (fields: &'a [(&Fn(T) -> u8, &Fn(T, u8) -> T)]) -> Remote<'a, T> {
        Remote { fields, buffer: ArrayVec::new() }
    }
}


