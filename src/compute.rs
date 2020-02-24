use crate::basic::{Id, Succ, Zero};
use crate::recursive::Recursive;
use generic_array::{ArrayLength, GenericArray};
use typenum::{consts::U1, NonZero, Unsigned};
mod ops;

trait Computable<N: ArrayLength<usize>>: Recursive<N> + Compute<N> {}

trait Compute<N: ArrayLength<usize>>: Recursive<N> {
    fn compute(x: GenericArray<usize, N>) -> usize;
    fn call(&self, x: GenericArray<usize, N>) -> usize {
        Self::compute(x)
    }
}

trait Compose<
    'g,
    N: 'g + ArrayLength<usize>,
    F: Compute<M>,
    M: ArrayLength<usize> + ArrayLength<&'g dyn Computable<N>>,
>: Compute<M>
{
    fn compose(
        &self,
        x: GenericArray<usize, N>,
        gs: GenericArray<&'g dyn Computable<N>, M>,
    ) -> usize {
        self.compute(gs.map(|g| g.compute(x)))
    }
}

impl Compute<U1> for Zero {
    fn compute(_: GenericArray<usize, U1>) -> usize {
        0
    }
}

impl Compute<U1> for Succ {
    fn compute(x: GenericArray<usize, U1>) -> usize {
        x[0] + 1
    }
}

impl<N: ArrayLength<usize> + NonZero, K: Unsigned + NonZero> Compute<N> for Id<N, K> {
    fn compute(x: GenericArray<usize, N>) -> usize {
        x[K::USIZE - 1]
    }
}
