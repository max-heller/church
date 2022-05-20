use super::{Computable, Compute};
use crate::{recursive::Recursive, Unsigned};

pub struct Cn<F, GS> {
    f: F,
    gs: GS,
}

impl<F, GS> std::fmt::Debug for Cn<F, GS>
where
    F: std::fmt::Debug,
    GS: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cn[{:?}, {:?}]", self.f, self.gs)
    }
}

impl<F, GS> Cn<F, GS> {
    pub fn new(f: F, gs: GS) -> Self {
        Cn { f, gs }
    }
}

#[macro_export]
macro_rules! cn {
    ($f:expr; $($g:expr),*) => {
        Cn::<_, [Box<dyn Computable<_, true>>; _]>::new($f, funcs![$($g),*])
    }
}

impl<F: Recursive<M, PRIMITIVE>, const N: Unsigned, const M: Unsigned, const PRIMITIVE: bool>
    Recursive<N, PRIMITIVE> for Cn<F, [Box<dyn Computable<N, PRIMITIVE>>; M]>
{
}

impl<F: Compute<M>, const N: Unsigned, const M: Unsigned, const PRIMITIVE: bool> Compute<N>
    for Cn<F, [Box<dyn Computable<N, PRIMITIVE>>; M]>
{
    fn call(&self, x: &[usize; N]) -> Option<usize> {
        std::array::try_from_fn(|i| self.gs[i].call(x)).and_then(|x| self.f.call(&x))
    }
}

pub struct Pr<F, G, const PRIMITIVE: bool> {
    f: F,
    g: G,
}

impl<F, G, const PRIMITIVE: bool> std::fmt::Debug for Pr<F, G, PRIMITIVE>
where
    F: std::fmt::Debug,
    G: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pr[{:?}, {:?}]", self.f, self.g)
    }
}

impl<F, G, const PRIMITIVE: bool> Pr<F, G, PRIMITIVE> {
    pub fn new(f: F, g: G) -> Self {
        Pr { f, g }
    }
}

#[macro_export]
macro_rules! pr {
    ($f:expr, $g:expr) => {
        Pr::<_, _, true>::new($f, $g)
    };
}

impl<F, G, const N: Unsigned, const PRIMITIVE: bool> Recursive<N, PRIMITIVE> for Pr<F, G, PRIMITIVE>
where
    F: Recursive<{ N - 1 }, PRIMITIVE>,
    G: Recursive<{ N + 1 }, PRIMITIVE>,
{
}

// https://stackoverflow.com/a/67085709
fn concat<T: Copy + Default, const A: usize, const B: usize>(a: &[T; A], b: &[T; B]) -> [T; A + B] {
    let mut whole: [T; A + B] = [Default::default(); A + B];
    let (one, two) = whole.split_at_mut(A);
    one.copy_from_slice(a);
    two.copy_from_slice(b);
    whole
}

fn array_split_last<T, const N: usize>(arr: &[T; N]) -> Option<(&T, &[T; N - 1])> {
    let (last, rest) = arr.split_last()?;
    Some((last, rest.try_into().unwrap()))
}

impl<F, G, const N: Unsigned, const PRIMITIVE: bool> Compute<N> for Pr<F, G, PRIMITIVE>
where
    F: Compute<{ N - 1 }>,
    G: Compute<{ N - 1 + 2 }>,
{
    fn call(&self, x: &[usize; N]) -> Option<usize> {
        let (y, x) = array_split_last(x).unwrap();
        let mut output = self.f.call(x);
        let mut input = concat(x, &[0, 0]);
        for y in 0..*y {
            match output {
                Some(out) => {
                    input[N - 1] = y;
                    input[N] = out;
                    output = self.g.call(&input);
                }
                None => return None,
            }
        }
        output
    }
}

pub struct Mn<F> {
    f: F,
}

impl<F> std::fmt::Debug for Mn<F>
where
    F: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mn[{:?}]", self.f)
    }
}

impl<F> Mn<F> {
    pub fn new(f: F) -> Self {
        Mn { f }
    }
}

#[macro_export]
macro_rules! mn {
    ($f:expr) => {
        Mn::new($f)
    };
}

impl<F, const N: Unsigned> Recursive<N, false> for Mn<F> where F: Recursive<{ N + 1 }, false> {}

impl<F, const N: Unsigned> Compute<N> for Mn<F>
where
    F: Compute<{ N + 1 }>,
{
    fn call(&self, x: &[usize; N]) -> Option<usize> {
        let mut input = concat(x, &[0]);
        for y in 0.. {
            input[N] = y;
            match self.f.call(&input) {
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

    type T = Cn<Succ, [Box<dyn Computable<2, true>>; 1]>;
    let f: T = cn![S; id![2, 1]];
    defined_eq!(f.call(&[0, 1]), 1);

    static_assertions::assert_impl_all!(T: Compute<2>, Recursive<2, false>,  Computable<2, false>);
    static_assertions::assert_impl_all!(T: Recursive<2, true>,  Computable<2, true>);

    let f = cn![S; id![2, 2]];
    defined_eq!(f.call(&[0, 1]), 2);

    let f = cn![id![2, 2]; id![1, 1], S];
    defined_eq!(f.call(&[1]), 2);

    let f = cn![id![2, 1]; id![1, 1], S];
    defined_eq!(f.call(&[1]), 1);
}

#[test]
fn test_pr() {
    use crate::*;

    // pr = Pr[z, id^3_3]
    type T = Pr<Zero, Id<3, 3>, true>;
    let f: T = pr![Z, id![3, 3]];
    defined_eq!(f.call(&[1, 0]), 0);
    defined_eq!(f.call(&[5, 0]), 0);
    defined_eq!(f.call(&[5, 1]), 0);
    defined_eq!(f.call(&[5, 2]), 0);
    defined_eq!(f.call(&[5, 50]), 0);

    static_assertions::assert_impl_all!(T: Compute<2>, Recursive<2, false>,  Computable<2, false>);
    static_assertions::assert_impl_all!(T: Recursive<2, true>,  Computable<2, true>);

    // pr = Pr[s, id^3_3]
    let f = pr![S, id![3, 3]];
    defined_eq!(f.call(&[0, 0]), 1);
    defined_eq!(f.call(&[1, 0]), 2);
    defined_eq!(f.call(&[4, 0]), 5);
    defined_eq!(f.call(&[4, 9]), 5);

    // pr = Pr[id, id^3_3]
    let f = pr![id![1, 1], id![3, 3]];
    defined_eq!(f.call(&[4, 0]), 4);
    defined_eq!(f.call(&[4, 9]), 4);

    // pr = Pr[id, id^3_1]
    // should always output x
    let f = pr![id![1, 1], id![3, 1]];
    defined_eq!(f.call(&[4, 0]), 4);
    defined_eq!(f.call(&[4, 9]), 4);

    // pr = Pr[id, id^3_2]
    // should always output y for s(y) if y > 0, x otherwise
    // because h(x, s(y)) = g(x, y, h(x, y))
    let f = pr![id![1, 1], id![3, 2]];
    defined_eq!(f.call(&[4, 0]), 4);
    defined_eq!(f.call(&[4, 9]), 8);
}

#[test]
fn test_mn() {
    use crate::*;

    let f = mn![id![2, 2]];
    defined_eq!(f.call(&[5]), 0);

    type T = Mn<Id<2, 1>>;
    let f: T = mn![id![2, 1]];
    defined_eq!(f.call(&[0]), 0);
    // undefined for input != &[0]

    static_assertions::assert_impl_all!(T: Compute<1>, Recursive<1, false>, Computable<1, false>);
    static_assertions::assert_not_impl_any!(T: Recursive<1, true>, Computable<1, true>);
}
