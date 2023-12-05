use std::ops::{BitOrAssign, ShlAssign};

pub fn take_bytes_start<T>(data: &Vec<u8>, start: usize, number_of_bytes: usize) -> T
    where
        T: ShlAssign + Default + BitOrAssign + From<u8>
{
    let mut value: T = Default::default();
    for i in (start..(start + number_of_bytes)).rev() {
        value <<= T::from(8);
        value |= T::from(data[i]);
    }
    value
}