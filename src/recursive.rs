use crate::{Cons, Nil};

pub trait Recursive {}
pub trait Primitive: Recursive {}

impl<H, T, const LEN: usize> Recursive for Cons<H, T, LEN>
where
    H: Recursive,
    T: Recursive,
{
}

impl<H, T, const LEN: usize> Primitive for Cons<H, T, LEN>
where
    H: Primitive,
    T: Primitive,
{
}

impl Recursive for Nil {}
impl Primitive for Nil {}
