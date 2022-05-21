use crate::{
    funcs::basic::{Id, Succ, Zero},
    Primitive,
};

pub mod ops;

pub trait PrimitivelyComputable<const N: usize>: Primitive + Compute<N> {}

impl<T, const N: usize> PrimitivelyComputable<N> for T where T: Primitive + Compute<N> {}

pub trait Compute<const N: usize> {
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

impl<const N: usize, const K: usize> Compute<N> for Id<K> {
    fn call(&self, x: &[usize; N]) -> Option<usize> {
        Some(x[K - 1])
    }
}

impl<const N: usize> Compute<N> for Box<dyn Compute<N>> {
    fn call(&self, x: &[usize; N]) -> Option<usize> {
        self.as_ref().call(x)
    }
}
