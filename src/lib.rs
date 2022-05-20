#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
#![feature(array_from_fn)]

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
macro_rules! funcs {
    ($($g:expr),*) => {
        [$(Box::new($g),)*]
    }
}

#[macro_export]
macro_rules! defined_eq {
    ($actual:expr, $expected:expr) => {
        assert_eq!($actual, Some($expected))
    };
}
