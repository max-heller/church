use crate::{
    funcs::basic::{Id, Succ, Zero},
    recursive::Recursive,
};
use generic_array::{ArrayLength, GenericArray};
use typenum::{consts::U1, NonZero, Unsigned};

pub mod ops;

pub trait Computable<N>: Recursive<N> + Compute<N>
where
    N: ArrayLength<usize>,
{
}

impl<T, N> Computable<N> for T
where
    T: Recursive<N> + Compute<N>,
    N: ArrayLength<usize>,
{
}

pub trait Compute<N>: Recursive<N>
where
    N: ArrayLength<usize>,
{
    fn call(&self, x: &GenericArray<usize, N>) -> Option<usize>;
}

impl Compute<U1> for Zero {
    fn call(&self, _: &GenericArray<usize, U1>) -> Option<usize> {
        Some(0)
    }
}

impl Compute<U1> for Succ {
    fn call(&self, x: &GenericArray<usize, U1>) -> Option<usize> {
        Some(x[0] + 1)
    }
}

impl<N, K> Compute<N> for Id<N, K>
where
    N: ArrayLength<usize> + NonZero,
    K: Unsigned + NonZero,
{
    fn call(&self, x: &GenericArray<usize, N>) -> Option<usize> {
        Some(x[K::USIZE - 1])
    }
}
