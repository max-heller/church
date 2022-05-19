use super::{Computable, Compute};
use crate::{recursive::Recursive, Unsigned};

pub struct Cn<F, const N: Unsigned, const M: Unsigned, const PRIMITIVE: bool = true> {
    f: F,
    gs: [Box<dyn Computable<N, PRIMITIVE>>; M],
}

impl<F, const N: Unsigned, const M: Unsigned, const PRIMITIVE: bool> std::fmt::Debug
    for Cn<F, N, M, PRIMITIVE>
where
    F: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cn[{:?}, {:?}]", self.f, self.gs)
    }
}

impl<F, const N: Unsigned, const M: Unsigned, const PRIMITIVE: bool> Cn<F, N, M, PRIMITIVE> {
    pub fn new(f: F, gs: [Box<dyn Computable<N, PRIMITIVE>>; M]) -> Self {
        Cn { f, gs }
    }
}

#[macro_export]
macro_rules! cn {
    ($f:expr; $($g:expr),*) => {
        Cn::<_, _, _, true>::new($f, funcs![$($g),*])
    }
}

impl<F: Recursive<M, PRIMITIVE>, const N: Unsigned, const M: Unsigned, const PRIMITIVE: bool>
    Recursive<N, PRIMITIVE> for Cn<F, N, M, PRIMITIVE>
{
}

impl<F: Compute<M>, const N: Unsigned, const M: Unsigned, const PRIMITIVE: bool> Compute<N>
    for Cn<F, N, M, PRIMITIVE>
{
    fn call(&self, x: &[usize; N]) -> Option<usize> {
        self.gs
            .iter()
            .map(|g| g.call(x))
            .collect::<Option<Vec<_>>>()
            .and_then(|x| <[usize; M]>::try_from(x).ok())
            .and_then(|x| self.f.call(&x))
    }
}

pub struct Pr<F, G, const N: Unsigned, const PRIMITIVE: bool = true> {
    f: F,
    g: G,
}

impl<F, G, const N: Unsigned, const PRIMITIVE: bool> std::fmt::Debug for Pr<F, G, N, PRIMITIVE>
where
    F: std::fmt::Debug,
    G: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pr[{:?}, {:?}]", self.f, self.g)
    }
}

impl<F, G, const N: Unsigned, const PRIMITIVE: bool> Pr<F, G, N, PRIMITIVE> {
    pub fn new(f: F, g: G) -> Self {
        Pr { f, g }
    }
}

#[macro_export]
macro_rules! pr {
    ($f:expr, $g:expr) => {
        Pr::<_, _, _, true>::new($f, $g)
    };
}

impl<F, G, const N: Unsigned, const PRIMITIVE: bool> Recursive<{ N + 1 }, PRIMITIVE>
    for Pr<F, G, N, PRIMITIVE>
where
    F: Recursive<N, PRIMITIVE>,
    G: Recursive<{ N + 2 }, PRIMITIVE>,
{
}

// https://stackoverflow.com/a/67085709
pub fn concat<T: Copy + Default, const A: usize, const B: usize>(
    a: &[T; A],
    b: &[T; B],
) -> [T; A + B] {
    let mut whole: [T; A + B] = [Default::default(); A + B];
    let (one, two) = whole.split_at_mut(A);
    one.copy_from_slice(a);
    two.copy_from_slice(b);
    whole
}

impl<F, G, const N: Unsigned, const PRIMITIVE: bool> Compute<{ N + 1 }> for Pr<F, G, N, PRIMITIVE>
where
    F: Compute<N>,
    G: Compute<{ N + 2 }>,
{
    fn call(&self, x: &[usize; N + 1]) -> Option<usize> {
        let (y, x) = x.split_last()?;
        let x: &[usize; N] = x.try_into().unwrap();
        let mut output = self.f.call(x);
        let mut input = concat(x, &[0, 0]);
        for y in 0..*y {
            match output {
                Some(out) => {
                    input[N] = y;
                    input[N + 1] = out;
                    output = self.g.call(&input);
                }
                None => return None,
            }
        }
        output
    }
}

pub struct Mn<F, const N: Unsigned, const PRIMITIVE: bool = true> {
    f: F,
}

impl<F, const N: Unsigned, const PRIMITIVE: bool> std::fmt::Debug for Mn<F, N, PRIMITIVE>
where
    F: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mn[{:?}]", self.f)
    }
}

impl<F, const N: Unsigned, const PRIMITIVE: bool> Mn<F, N, PRIMITIVE> {
    pub fn new(f: F) -> Self {
        Mn { f }
    }
}

#[macro_export]
macro_rules! mn {
    ($f:expr) => {
        Mn::<_, _, true>::new($f)
    };
}

impl<F, const N: Unsigned, const PRIMITIVE: bool> Recursive<N, PRIMITIVE>
    for Mn<F, { N + 1 }, PRIMITIVE>
where
    F: Recursive<{ N + 1 }, PRIMITIVE>,
{
}

impl<F, const N: Unsigned, const PRIMITIVE: bool> Compute<N> for Mn<F, { N + 1 }, PRIMITIVE>
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

    let f = cn![S; id![2, 1]];
    defined_eq!(f.call(&[0, 1]), 1);

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
    let f = pr![Z, id![3, 3]];
    defined_eq!(f.call(&[1, 0]), 0);
    defined_eq!(f.call(&[5, 0]), 0);
    defined_eq!(f.call(&[5, 1]), 0);
    defined_eq!(f.call(&[5, 2]), 0);
    defined_eq!(f.call(&[5, 50]), 0);

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

    let f = mn![id![2, 1]];
    defined_eq!(f.call(&[0]), 0);
    // undefined for input != &[0]
}
