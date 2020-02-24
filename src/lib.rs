use generic_array::ArrayLength;

pub mod basic;
pub mod compute;
pub mod recursive;

pub trait Unsigned: ArrayLength<usize> {}
