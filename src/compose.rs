use crate::{
    hlist::{Cons, Nil},
    Compute, PrimitiveRecursive, Recursive,
};

#[derive(Default)]
pub struct Cn<F, GS> {
    f: F,
    gs: GS,
}

impl<F, GS> Cn<F, GS> {
    pub const fn new(f: F, gs: GS) -> Self {
        Cn { f, gs }
    }
}

#[macro_export]
macro_rules! cn {
    ($f:expr; $($g:expr),*) => {{
        $crate::Cn::new($f, $crate::hlist![$($g),*])
    }}
}

impl<F, GS> Recursive for Cn<F, GS>
where
    F: Recursive,
    GS: Recursive,
{
}

impl<F, GS> PrimitiveRecursive for Cn<F, GS>
where
    F: PrimitiveRecursive,
    GS: PrimitiveRecursive,
{
}

trait HComputable<const N: usize, const M: usize> {
    fn compute(&self, x: &[usize; N], out: &mut [usize; M]) -> Option<()>;
}

impl<const N: usize> HComputable<N, 0> for Nil {
    fn compute(&self, _: &[usize; N], _: &mut [usize; 0]) -> Option<()> {
        Some(())
    }
}

impl<const N: usize, const M: usize, H, T> HComputable<N, M> for Cons<H, T, M>
where
    H: Compute<N>,
    T: HComputable<N, { M - 1 }>,
{
    fn compute(&self, x: &[usize; N], out: &mut [usize; M]) -> Option<()> {
        let (first, rest) = array_split_first_mut(out);
        *first = self.head().call(x)?;
        self.tail().compute(x, rest)
    }
}

impl<const N: usize, const M: usize, F, H, T> Compute<N> for Cn<F, Cons<H, T, M>>
where
    F: Compute<M>,
    Cons<H, T, M>: HComputable<N, M>,
{
    fn call(&self, x: &[usize; N]) -> Option<usize> {
        let mut buf = [0; M];
        self.gs.compute(x, &mut buf)?;
        self.f.call(&buf)
    }
}

#[derive(Default)]
pub struct Pr<F, G> {
    f: F,
    g: G,
}

pub const fn pr<F, G>(f: F, g: G) -> Pr<F, G> {
    Pr { f, g }
}

impl<F, G> PrimitiveRecursive for Pr<F, G>
where
    F: PrimitiveRecursive,
    G: PrimitiveRecursive,
{
}

impl<F, G> Recursive for Pr<F, G>
where
    F: Recursive,
    G: Recursive,
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

#[inline]
fn array_split_first_mut<T, const N: usize>(arr: &mut [T; N]) -> (&mut T, &mut [T; N - 1]) {
    match arr.as_mut_slice() {
        [first, rest @ ..] => (first, rest.try_into().unwrap()),
        _ => unreachable!(),
    }
}

fn array_split_last<T, const N: usize>(arr: &[T; N]) -> (&T, &[T; N - 1]) {
    match arr.as_slice() {
        [rest @ .., last] => (last, rest.try_into().unwrap()),
        _ => unreachable!(),
    }
}

impl<F, G, const N: usize> Compute<N> for Pr<F, G>
where
    F: Compute<{ N - 1 }>,
    G: Compute<{ N - 1 + 2 }>,
{
    fn call(&self, x: &[usize; N]) -> Option<usize> {
        let (&y, x) = array_split_last(x);
        let mut output = self.f.call(x)?;
        let mut input = concat(x, &[0, 0]);
        for y in 0..y {
            input[N - 1] = y;
            input[N] = output;
            output = self.g.call(&input)?;
        }
        Some(output)
    }
}

#[derive(Default)]
pub struct Mn<F> {
    f: F,
}

pub const fn mn<F>(f: F) -> Mn<F> {
    Mn { f }
}

impl<F> Recursive for Mn<F> {}

impl<F, const N: usize> Compute<N> for Mn<F>
where
    F: Compute<{ N + 1 }>,
{
    fn call(&self, x: &[usize; N]) -> Option<usize> {
        let mut input = concat(x, &[0]);
        for y in 0.. {
            input[N] = y;
            if let 0 = self.f.call(&input)? {
                return Some(y);
            }
        }
        return None;
    }
}

#[test]
fn test_cn() {
    use crate::*;

    type T = Cn<Succ, Cons<Id<1>, Nil, 1>>;
    let f: T = cn![S; id![1]];
    defined_eq!(f.call(&[0, 1]), 1);

    static_assertions::assert_impl_all!(T: Compute<2>, PrimitiveRecursive, Recursive);

    let f = cn![S; id![2]];
    defined_eq!(f.call(&[0, 1]), 2);

    let f = cn![id![2]; id![1], S];
    defined_eq!(f.call(&[1]), 2);

    let f = cn![id![1]; id![1], S];
    defined_eq!(f.call(&[1]), 1);

    type U = Cn<Succ, Cons<Mn<Id<2>>, Nil, 1>>;
    let f: U = cn![S; mn(id![2])];
    f.call(&[0]);
    static_assertions::assert_impl_all!(U: Compute<1>, Recursive);
    static_assertions::assert_not_impl_any!(U: PrimitiveRecursive);
}

#[test]
fn test_pr() {
    use crate::*;

    // pr = Pr[z, id^3_3]
    type T = Pr<Zero, Id<3>>;
    let f: T = pr(Z, id![3]);
    defined_eq!(f.call(&[1, 0]), 0);
    defined_eq!(f.call(&[5, 0]), 0);
    defined_eq!(f.call(&[5, 1]), 0);
    defined_eq!(f.call(&[5, 2]), 0);
    defined_eq!(f.call(&[5, 50]), 0);

    static_assertions::assert_impl_all!(T: Compute<2>, Recursive, PrimitiveRecursive);

    // pr = Pr[s, id^3_3]
    let f = pr(S, id![3]);
    defined_eq!(f.call(&[0, 0]), 1);
    defined_eq!(f.call(&[1, 0]), 2);
    defined_eq!(f.call(&[4, 0]), 5);
    defined_eq!(f.call(&[4, 9]), 5);

    // pr = Pr[id, id^3_3]
    let f = pr(id![1], id![3]);
    defined_eq!(f.call(&[4, 0]), 4);
    defined_eq!(f.call(&[4, 9]), 4);

    // pr = Pr[id, id^3_1]
    // should always output x
    let f = pr(id![1], id![1]);
    defined_eq!(f.call(&[4, 0]), 4);
    defined_eq!(f.call(&[4, 9]), 4);

    // pr = Pr[id, id^3_2]
    // should always output y for s(y) if y > 0, x otherwise
    // because h(x, s(y)) = g(x, y, h(x, y))
    let f = pr(id![1], id![2]);
    defined_eq!(f.call(&[4, 0]), 4);
    defined_eq!(f.call(&[4, 9]), 8);
}

#[test]
fn test_mn() {
    use crate::*;

    let f = mn(id![2]);
    defined_eq!(f.call(&[5]), 0);

    type T = Mn<Id<1>>;
    let f: T = mn(id![1]);
    defined_eq!(f.call(&[0]), 0);
    // undefined for input != &[0]

    static_assertions::assert_impl_all!(T: Compute<1>, Recursive);
    static_assertions::assert_not_impl_any!(T: PrimitiveRecursive);
}
