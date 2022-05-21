use crate::{recursive::Recursive, Primitive, Unsigned};

#[derive(Debug)]
pub struct Zero;
pub const Z: Zero = Zero;

#[derive(Debug)]
pub struct Succ;
pub const S: Succ = Succ;

pub struct Id<const N: Unsigned, const K: Unsigned>;

impl<const N: Unsigned, const K: Unsigned> std::fmt::Debug for Id<N, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id<{}, {}>", N, K)
    }
}

#[macro_export]
macro_rules! id {
    ($N:expr, $K:expr) => {
        Id::<$N, $K>
    };
}

impl Recursive for Zero {}
impl Primitive for Zero {}
impl Recursive for Succ {}
impl Primitive for Succ {}
impl<const N: Unsigned, const K: Unsigned> Recursive for Id<N, K> {}
impl<const N: Unsigned, const K: Unsigned> Primitive for Id<N, K> {}
