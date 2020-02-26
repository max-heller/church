#![feature(impl_trait_in_bindings)]

pub mod compute;
pub mod funcs;
pub mod recursive;

pub use funcs::*;
pub use recursive::*;

#[macro_export]
macro_rules! funcs {
    ($N:ty; $($g:expr),*) => {
        generic_array::arr![&'_ dyn crate::compute::Computable<$N>; $($g,)*]
    }
}

#[macro_export]
macro_rules! args {
    ($($x:expr),*) => {
        generic_array::arr![usize; $($x,)*]
    }
}

#[macro_export]
macro_rules! defined_eq {
    ($actual:expr, $expected:expr) => {
        assert_eq!($actual, Some($expected))
    };
}
