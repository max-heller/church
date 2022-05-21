use crate::{
    funcs::basic::{Id, Succ, Zero},
    Primitive, Unsigned,
};

pub mod ops;

pub trait PrimitivelyComputable<const N: Unsigned>: Primitive + Compute<N> {}

impl<T, const N: Unsigned> PrimitivelyComputable<N> for T where T: Primitive + Compute<N> {}

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

impl<const N: Unsigned> Compute<N> for Box<dyn Compute<N>> {
    fn call(&self, x: &[usize; N]) -> Option<usize> {
        self.as_ref().call(x)
    }
}
