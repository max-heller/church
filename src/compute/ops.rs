use super::Compute;
use crate::{recursive::Recursive, Assert, Cons, Nil, Primitive, True, Unsigned};

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

#[doc(hidden)]
#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! cn {
    ($f:expr; $($g:expr),*) => {{
        // const M: usize = $crate::count!($($g)*);
        Cn::new($f, $crate::funcs![$($g),*])
    }}
}

impl<F, GS> Recursive for Cn<F, GS>
where
    F: Recursive,
    GS: Recursive,
{
}
impl<F, GS> Primitive for Cn<F, GS>
where
    F: Primitive,
    GS: Primitive,
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

fn array_split_first_mut<T, const N: usize>(arr: &mut [T; N]) -> Option<(&mut T, &mut [T; N - 1])> {
    let (first, rest) = arr.split_first_mut()?;
    Some((first, rest.try_into().unwrap()))
}

impl<const N: usize, const M: usize, H, T> HComputable<N, M> for Cons<H, T, M>
where
    H: Compute<N>,
    T: HComputable<N, { M - 1 }>,
{
    fn compute(&self, x: &[usize; N], out: &mut [usize; M]) -> Option<()> {
        let (first, rest) = array_split_first_mut(out)?;
        *first = self.head.call(x)?;
        self.tail.compute(x, rest)
    }
}

impl<const N: Unsigned, const M: Unsigned, F, H, T> Compute<N> for Cn<F, Cons<H, T, M>>
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

pub struct Pr<F, G> {
    f: F,
    g: G,
}

impl<F, G> std::fmt::Debug for Pr<F, G>
where
    F: std::fmt::Debug,
    G: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pr[{:?}, {:?}]", self.f, self.g)
    }
}

impl<F, G> Pr<F, G> {
    pub fn new(f: F, g: G) -> Self {
        Pr { f, g }
    }
}

#[macro_export]
macro_rules! pr {
    ($f:expr, $g:expr) => {
        Pr::new($f, $g)
    };
}

impl<F, G> Primitive for Pr<F, G>
where
    F: Primitive,
    G: Primitive,
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

fn array_split_last<T, const N: usize>(arr: &[T; N]) -> Option<(&T, &[T; N - 1])> {
    let (last, rest) = arr.split_last()?;
    Some((last, rest.try_into().unwrap()))
}

impl<F, G, const N: Unsigned> Compute<N> for Pr<F, G>
where
    Assert<{ N > 0 }>: True,
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

impl<F> Recursive for Mn<F> {}

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

    type T = Cn<Succ, Cons<Id<1>, Nil, 1>>;
    let f: T = cn![S; id![1]];
    defined_eq!(f.call(&[0, 1]), 1);

    static_assertions::assert_impl_all!(T: Compute<2>, Primitive, Recursive);

    let f = cn![S; id![2]];
    defined_eq!(f.call(&[0, 1]), 2);

    let f = cn![id![2]; id![1], S];
    defined_eq!(f.call(&[1]), 2);

    let f = cn![id![1]; id![1], S];
    defined_eq!(f.call(&[1]), 1);

    type U = Cn<Succ, Cons<Mn<Id<2>>, Nil, 1>>;
    let f: U = cn![S; mn![id![2]]];
    f.call(&[0]);
    static_assertions::assert_impl_all!(U: Compute<1>, Recursive);
    static_assertions::assert_not_impl_any!(U: Primitive);
}

#[test]
fn test_pr() {
    use crate::*;

    // pr = Pr[z, id^3_3]
    type T = Pr<Zero, Id<3>>;
    let f: T = pr![Z, id![3]];
    defined_eq!(f.call(&[1, 0]), 0);
    defined_eq!(f.call(&[5, 0]), 0);
    defined_eq!(f.call(&[5, 1]), 0);
    defined_eq!(f.call(&[5, 2]), 0);
    defined_eq!(f.call(&[5, 50]), 0);

    static_assertions::assert_impl_all!(T: Compute<2>, Recursive, Primitive);

    // pr = Pr[s, id^3_3]
    let f = pr![S, id![3]];
    defined_eq!(f.call(&[0, 0]), 1);
    defined_eq!(f.call(&[1, 0]), 2);
    defined_eq!(f.call(&[4, 0]), 5);
    defined_eq!(f.call(&[4, 9]), 5);

    // pr = Pr[id, id^3_3]
    let f = pr![id![1], id![3]];
    defined_eq!(f.call(&[4, 0]), 4);
    defined_eq!(f.call(&[4, 9]), 4);

    // pr = Pr[id, id^3_1]
    // should always output x
    let f = pr![id![1], id![1]];
    defined_eq!(f.call(&[4, 0]), 4);
    defined_eq!(f.call(&[4, 9]), 4);

    // pr = Pr[id, id^3_2]
    // should always output y for s(y) if y > 0, x otherwise
    // because h(x, s(y)) = g(x, y, h(x, y))
    let f = pr![id![1], id![2]];
    defined_eq!(f.call(&[4, 0]), 4);
    defined_eq!(f.call(&[4, 9]), 8);
}

#[test]
fn test_mn() {
    use crate::*;

    let f = mn![id![2]];
    defined_eq!(f.call(&[5]), 0);

    type T = Mn<Id<1>>;
    let f: T = mn![id![1]];
    defined_eq!(f.call(&[0]), 0);
    // undefined for input != &[0]

    static_assertions::assert_impl_all!(T: Compute<1>, Recursive);
    static_assertions::assert_not_impl_any!(T: Primitive);
}
