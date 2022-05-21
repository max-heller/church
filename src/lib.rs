#![feature(generic_const_exprs)]

pub mod compute;
pub mod funcs;
pub mod recursive;

pub use funcs::*;
pub use recursive::*;

pub type Unsigned = usize;

pub(crate) struct Assert<const X: bool>;
pub(crate) trait True {}
impl True for Assert<true> {}

#[macro_export]
macro_rules! defined_eq {
    ($actual:expr, $expected:expr) => {
        assert_eq!($actual, Some($expected))
    };
}

pub struct Nil;

pub struct Cons<H, T, const LEN: usize> {
    head: H,
    tail: T,
}

pub fn cons<H, T, const LEN: usize>(head: H, tail: T) -> Cons<H, T, { LEN + 1 }>
where
    T: Length<LEN>,
{
    Cons { head, tail }
}

#[doc(hidden)]
#[macro_export]
macro_rules! funcs {
    () => { $crate::Nil };
    ($x:expr) => { $crate::cons($x, $crate::Nil) };
    ($x:expr, $($xs:expr),*) => {
        $crate::cons($x, $crate::funcs![$($xs),*])
    };
}

pub trait Length<const N: usize> {}

impl Length<0> for Nil {}
impl<const N: usize, H, T> Length<N> for Cons<H, T, N> {}
