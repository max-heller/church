use crate::{
    funcs::basic::{Id, Succ, Zero},
    recursive::Recursive,
    Unsigned,
};

pub mod ops;

pub trait Computable<const N: Unsigned, const PRIMITIVE: bool = true>:
    Recursive<N, PRIMITIVE> + Compute<N>
{
}

impl<T, const N: Unsigned, const PRIMITIVE: bool> Computable<N, PRIMITIVE> for T where
    T: Recursive<N, PRIMITIVE> + Compute<N>
{
}

pub trait Compute<const N: Unsigned> {
    fn call(&self, x: &[usize; N]) -> Option<usize>;
}

impl Compute<1> for Zero {
    fn call(&self, _: &[usize; 1]) -> Option<usize> {
        Some(0)
    }
}

impl Compute<1> for Succ {
    fn call(&self, x: &[usize; 1]) -> Option<usize> {
        Some(x[0] + 1)
    }
}

impl<const N: Unsigned, const K: Unsigned> Compute<N> for Id<N, K> {
    fn call(&self, x: &[usize; N]) -> Option<usize> {
        Some(x[K - 1])
    }
}
