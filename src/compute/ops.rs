use super::Compute;
use crate::recursive::{PrimitiveRecursive, Recursive};
use generic_array::{
    arr,
    sequence::{Concat, Shorten},
    ArrayLength, GenericArray,
};
use std::marker::PhantomData;
use std::ops::{Add, Sub};
use typenum::{bit::B1, consts::*, Add1, Sub1, Sum, Unsigned};

pub struct Cn<
    'g,
    F: Recursive<M>,
    N: ArrayLength<&'g dyn Recursive<N>>,
    M: ArrayLength<&'g dyn Recursive<N>>,
> {
    f: PhantomData<F>,
    g: GenericArray<&'g dyn Recursive<N>, M>,
}

impl<
        'g,
        F: PrimitiveRecursive<M>,
        N: ArrayLength<&'g dyn PrimitiveRecursive<N>> + ArrayLength<&'g dyn Recursive<N>>,
        M: ArrayLength<&'g dyn PrimitiveRecursive<N>> + ArrayLength<&'g dyn Recursive<N>>,
    > PrimitiveRecursive<N> for Cn<'g, F, N, M>
{
}

impl<
        'g,
        F: Recursive<M>,
        N: ArrayLength<&'g dyn Recursive<N>>,
        M: ArrayLength<&'g dyn Recursive<N>>,
    > Recursive<N> for Cn<'g, F, N, M>
{
}

impl<
        'g,
        F: Recursive<M>,
        N: ArrayLength<&'g dyn Recursive<N>> + ArrayLength<usize>,
        M: ArrayLength<&'g dyn Recursive<N>>,
    > Compute<N> for Cn<'g, F, N, M>
{
    fn compute(x: GenericArray<usize, N>) -> usize {
        unimplemented!()
    }
}

pub struct Pr<F: Recursive<Sub1<N>>, G: Recursive<Add1<N>>, N: Unsigned + Sub<B1> + Add<B1>>
where
    Sub1<N>: Unsigned,
    Add1<N>: Unsigned,
{
    f: PhantomData<F>,
    g: PhantomData<G>,
    n: PhantomData<N>,
}

impl<
        F: Recursive<M>,
        G: Recursive<Add1<N>>,
        N: Unsigned + Sub<B1, Output = M> + Add<B1>,
        M: Unsigned + Add<B1, Output = N>,
    > Pr<F, G, N>
where
    Sub1<N>: Unsigned,
    Add1<N>: Unsigned,
{
    pub fn new(_: F, _: G) -> Self {
        Pr {
            f: PhantomData,
            g: PhantomData,
            n: PhantomData,
        }
    }
}

impl<
        F: PrimitiveRecursive<Sub1<N>>,
        G: PrimitiveRecursive<Add1<N>>,
        N: Unsigned + Sub<B1> + Add<B1>,
    > PrimitiveRecursive<N> for Pr<F, G, N>
where
    Sub1<N>: Unsigned,
    Add1<N>: Unsigned,
{
}

impl<F: Recursive<Sub1<N>>, G: Recursive<Add1<N>>, N: Unsigned + Sub<B1> + Add<B1>> Recursive<N>
    for Pr<F, G, N>
where
    Sub1<N>: Unsigned,
    Add1<N>: Unsigned,
{
}

impl<
        F: Recursive<Sub1<N>> + Compute<Sub1<N>>,
        G: Recursive<Add1<N>> + Compute<Add1<N>>,
        N: ArrayLength<usize> + Sub<B1> + Add<B1>,
    > Compute<N> for Pr<F, G, N>
where
    Add1<N>: Unsigned + ArrayLength<usize>,
    Sub1<N>: Unsigned + ArrayLength<usize> + Add<B1, Output = N> + Add<U2, Output = Add1<N>>,
    Sum<Sub1<N>, U2>: ArrayLength<usize>,
{
    fn compute(x: GenericArray<usize, N>) -> usize {
        let (x, y) = x.pop_back();
        let mut output = F::compute(x.clone());
        for y in 0..y {
            output = G::compute(x.clone().concat(arr![usize; y, output]));
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
