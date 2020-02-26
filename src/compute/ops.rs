use super::{Computable, Compute};
use crate::{
    args,
    recursive::{PrimitiveRecursive, Recursive},
};
use generic_array::{
    functional::FunctionalSequence,
    sequence::{Concat, Lengthen, Shorten},
    ArrayLength, GenericArray,
};
use std::{
    marker::PhantomData,
    ops::{Add, Sub},
};
use typenum::{bit::B1, consts::*, Add1, Sub1, Sum, Unsigned};

pub struct Cn<'g, F, N, M>
where
    F: Recursive<M>,
    N: Unsigned,
    M: ArrayLength<&'g dyn Computable<N>>,
{
    f: F,
    gs: GenericArray<&'g dyn Computable<N>, M>,
}

impl<'g, F, N, M> Cn<'g, F, N, M>
where
    F: Recursive<M>,
    N: Unsigned,
    M: ArrayLength<&'g dyn Computable<N>>,
{
    pub fn new(f: F, gs: GenericArray<&'g dyn Computable<N>, M>) -> Self {
        Cn { f, gs }
    }
}

// TODO: Right now I'm unable to correctly mark primitive recursive compositions
// as primitive recursive (instead, they're marked as recursive only)
// impl<'g, F, N, M> PrimitiveRecursive<N> for Cn<'g, F, N, M>
// where
//     F: PrimitiveRecursive<M>,
//     N: ArrayLength<&'g dyn Computable<N>>,
//     M: ArrayLength<&'g dyn Computable<N>>,
// {
// }

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
    M: ArrayLength<&'g dyn Computable<N>> + ArrayLength<usize> + ArrayLength<Option<usize>>,
{
    fn call(&self, x: GenericArray<usize, N>) -> Option<usize> {
        (&self.gs)
            .map(|g| g.call(x.clone()))
            .into_iter()
            .collect::<Option<Vec<usize>>>()
            .and_then(|x| self.f.call(GenericArray::clone_from_slice(&x)))
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
    fn call(&self, x: GenericArray<usize, N>) -> Option<usize> {
        let (x, y) = x.pop_back();
        let mut output = self.f.call(x.clone());
        for y in 0..y {
            match output {
                Some(out) => output = self.g.call(x.clone().concat(args![y, out])),
                None => return None,
            }
        }
        output
    }
}

pub struct Mn<F, N>
where
    F: Recursive<N>,
    N: Unsigned,
{
    f: F,
    n: PhantomData<N>,
}

impl<F, N> Mn<F, N>
where
    F: Recursive<N>,
    N: Unsigned,
{
    pub fn new(f: F) -> Self {
        Mn { f, n: PhantomData }
    }
}

impl<F, N> Recursive<N> for Mn<F, Add1<N>>
where
    F: Recursive<Add1<N>>,
    N: Unsigned + Add<B1>,
    Add1<N>: Unsigned,
{
}

impl<F, N> Compute<N> for Mn<F, Add1<N>>
where
    F: Compute<Add1<N>>,
    N: ArrayLength<usize> + Add<B1>,
    Add1<N>: ArrayLength<usize> + Sub<B1, Output = N>,
    Sub1<Add1<N>>: ArrayLength<usize>,
{
    fn call(&self, x: GenericArray<usize, N>) -> Option<usize> {
        for y in 0.. {
            match self.f.call(x.clone().append(y)) {
                None => return None,
                Some(0) => return Some(y),
                Some(_) => continue,
            }
        }
        return None;
    }
}

#[test]
fn test_cn() {
    use crate::*;

    let cn = Cn::new(S, funcs![U2; &id![U2, U1]]);
    defined_eq!(cn.call(args![0, 1]), 1);

    let cn = Cn::new(S, funcs![U2; &id![U2, U2]]);
    defined_eq!(cn.call(args![0, 1]), 2);

    let cn = Cn::new(id![U2, U2], funcs![U1; &id![U1, U1], &S]);
    defined_eq!(cn.call(args![1]), 2);

    let cn = Cn::new(id![U2, U1], funcs![U1; &id![U1, U1], &S]);
    defined_eq!(cn.call(args![1]), 1);
}

#[test]
fn test_pr() {
    use crate::*;

    // pr = Pr[z, id^3_3]
    let pr = Pr::new(Z, id![U3, U3]);
    defined_eq!(pr.call(args![1, 0]), 0);
    defined_eq!(pr.call(args![5, 0]), 0);
    defined_eq!(pr.call(args![5, 1]), 0);
    defined_eq!(pr.call(args![5, 2]), 0);
    defined_eq!(pr.call(args![5, 50]), 0);

    // pr = Pr[s, id^3_3]
    let pr = Pr::new(S, id![U3, U3]);
    defined_eq!(pr.call(args![0, 0]), 1);
    defined_eq!(pr.call(args![1, 0]), 2);
    defined_eq!(pr.call(args![4, 0]), 5);
    defined_eq!(pr.call(args![4, 9]), 5);

    // pr = Pr[id, id^3_3]
    let pr = Pr::new(id![U1, U1], id![U3, U3]);
    defined_eq!(pr.call(args![4, 0]), 4);
    defined_eq!(pr.call(args![4, 9]), 4);

    // pr = Pr[id, id^3_1]
    // should always output x
    let pr = Pr::new(id![U1, U1], id![U3, U1]);
    defined_eq!(pr.call(args![4, 0]), 4);
    defined_eq!(pr.call(args![4, 9]), 4);

    // pr = Pr[id, id^3_2]
    // should always output y for s(y) if y > 0, x otherwise
    // because h(x, s(y)) = g(x, y, h(x, y))
    let pr = Pr::new(id![U1, U1], id![U3, U2]);
    defined_eq!(pr.call(args![4, 0]), 4);
    defined_eq!(pr.call(args![4, 9]), 8);
}

#[test]
fn test_mn() {
    use crate::*;

    let mn = Mn::new(id![U2, U2]);
    defined_eq!(mn.call(args![5]), 0);

    let mn = Mn::new(id![U2, U1]);
    defined_eq!(mn.call(args![0]), 0);
    // undefined for input != args![0]
}
