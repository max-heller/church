pub mod basic;
pub mod compute;
pub mod recursive;

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
