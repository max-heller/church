use super::{Computable, Compute};
use crate::recursive::{PrimitiveRecursive, Recursive};
use generic_array::{
    arr,
    functional::FunctionalSequence,
    sequence::{Concat, Shorten},
    ArrayLength, GenericArray,
};
use std::marker::PhantomData;
use std::ops::{Add, Sub};
use typenum::{bit::B1, consts::*, Add1, Sub1, Sum, Unsigned};

pub struct Cn<'g, F, N, M>
where
    F: Recursive<M>,
    N: ArrayLength<&'g dyn Computable<N>>,
    M: ArrayLength<&'g dyn Computable<N>>,
{
    f: F,
    gs: GenericArray<&'g dyn Computable<N>, M>,
}

impl<'g, F, N, M> PrimitiveRecursive<N> for Cn<'g, F, N, M>
where
    F: PrimitiveRecursive<M>,
    N: ArrayLength<&'g dyn Computable<N>>,
    M: ArrayLength<&'g dyn Computable<N>>,
{
}

impl<'g, F, N, M> Recursive<N> for Cn<'g, F, N, M>
where
    F: Recursive<M>,
    N: ArrayLength<&'g dyn Computable<N>>,
    M: ArrayLength<&'g dyn Computable<N>>,
{
}

impl<'g, F, N, M> Compute<N> for Cn<'g, F, N, M>
where
    F: Recursive<M> + Compute<M>,
    N: ArrayLength<&'g dyn Computable<N>> + ArrayLength<usize>,
    M: ArrayLength<&'g dyn Computable<N>> + ArrayLength<usize>,
{
    fn call(&self, x: GenericArray<usize, N>) -> usize {
        self.f.call((&self.gs).map(|g| g.call(x.clone())))
    }
}

pub struct Pr<F, G, N>
where
    F: Recursive<Sub1<N>>,
    G: Recursive<Add1<N>>,
    N: Unsigned + Sub<B1> + Add<B1>,
    Sub1<N>: Unsigned,
    Add1<N>: Unsigned,
{
    f: F,
    g: G,
    n: PhantomData<N>,
}

impl<F, G, N, M> Pr<F, G, N>
where
    F: Recursive<M>,
    G: Recursive<Add1<N>>,
    N: Unsigned + Sub<B1, Output = M> + Add<B1>,
    M: Unsigned + Add<B1, Output = N>,
    Sub1<N>: Unsigned,
    Add1<N>: Unsigned,
{
    pub fn new(f: F, g: G) -> Self {
        Pr {
            f,
            g,
            n: PhantomData,
        }
    }
}

impl<F, G, N> PrimitiveRecursive<N> for Pr<F, G, N>
where
    F: PrimitiveRecursive<Sub1<N>>,
    G: PrimitiveRecursive<Add1<N>>,
    N: Unsigned + Sub<B1> + Add<B1>,
    Sub1<N>: Unsigned,
    Add1<N>: Unsigned,
{
}

impl<F, G, N> Recursive<N> for Pr<F, G, N>
where
    F: Recursive<Sub1<N>>,
    G: Recursive<Add1<N>>,
    N: Unsigned + Sub<B1> + Add<B1>,
    Sub1<N>: Unsigned,
    Add1<N>: Unsigned,
{
}

impl<F, G, N> Compute<N> for Pr<F, G, N>
where
    F: Recursive<Sub1<N>> + Compute<Sub1<N>>,
    G: Recursive<Add1<N>> + Compute<Add1<N>>,
    N: ArrayLength<usize> + Sub<B1> + Add<B1>,
    Add1<N>: Unsigned + ArrayLength<usize>,
    Sub1<N>: Unsigned + ArrayLength<usize> + Add<B1, Output = N> + Add<U2, Output = Add1<N>>,
    Sum<Sub1<N>, U2>: ArrayLength<usize>,
{
    fn call(&self, x: GenericArray<usize, N>) -> usize {
        let (x, y) = x.pop_back();
        let mut output = self.f.call(x.clone());
        for y in 0..y {
            output = self.g.call(x.clone().concat(arr![usize; y, output]));
        }
        output
    }
}

#[test]
fn test_pr() {
    use crate::basic::*;

    // pr = Pr[z, id^3_3]
    let pr = Pr::new(Zero {}, Id::<U3, U3>::new());
    assert_eq!(0, pr.call(arr![usize; 1, 0]));
    assert_eq!(0, pr.call(arr![usize; 5, 0]));
    assert_eq!(0, pr.call(arr![usize; 5, 1]));
    assert_eq!(0, pr.call(arr![usize; 5, 2]));
    assert_eq!(0, pr.call(arr![usize; 5, 50]));

    // pr = Pr[s, id^3_3]
    let pr = Pr::new(Succ {}, Id::<U3, U3>::new());
    assert_eq!(1, pr.call(arr![usize; 0, 0]));
    assert_eq!(2, pr.call(arr![usize; 1, 0]));
    assert_eq!(5, pr.call(arr![usize; 4, 0]));
    assert_eq!(5, pr.call(arr![usize; 4, 9]));

    // pr = Pr[id, id^3_3]
    let pr = Pr::new(Id::<U1, U1>::new(), Id::<U3, U3>::new());
    assert_eq!(4, pr.call(arr![usize; 4, 0]));
    assert_eq!(4, pr.call(arr![usize; 4, 9]));

    // pr = Pr[id, id^3_1]
    // should always output x
    let pr = Pr::new(Id::<U1, U1>::new(), Id::<U3, U1>::new());
    assert_eq!(4, pr.call(arr![usize; 4, 0]));
    assert_eq!(4, pr.call(arr![usize; 4, 9]));

    // pr = Pr[id, id^3_2]
    // should always output y for s(y) if y > 0, x otherwise
    // because h(x, s(y)) = g(x, y, h(x, y))
    let pr = Pr::new(Id::<U1, U1>::new(), Id::<U3, U2>::new());
    assert_eq!(4, pr.call(arr![usize; 4, 0]));
    assert_eq!(8, pr.call(arr![usize; 4, 9]));
}

//pub struct Mn<F: Recursive<N>, const N: usize> {
//    f: F,
//}

//impl<F: Recursive<N>, const N: usize> Recursive<{ N - 1 }> for Mn<F, N> {}
