#![feature(generic_const_exprs)]

mod compose;
mod compute;
mod funcs;
pub mod hlist;
mod recursive;

pub use compose::*;
pub use compute::*;
pub use funcs::*;
pub use recursive::*;

#[macro_export]
macro_rules! defined_eq {
    ($actual:expr, $expected:expr) => {
        assert_eq!($actual, Some($expected))
    };
}
