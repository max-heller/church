use crate::hlist::{Cons, Nil};

pub trait Recursive {}
pub trait PrimitiveRecursive: Recursive {}

impl Recursive for Nil {}
impl PrimitiveRecursive for Nil {}

impl<H, T, const LEN: usize> Recursive for Cons<H, T, LEN>
where
    H: Recursive,
    T: Recursive,
{
}

impl<H, T, const LEN: usize> PrimitiveRecursive for Cons<H, T, LEN>
where
    H: PrimitiveRecursive,
    T: PrimitiveRecursive,
{
}
