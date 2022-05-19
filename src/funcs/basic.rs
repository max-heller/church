use crate::{recursive::Recursive, Unsigned};

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

impl Recursive<1, true> for Zero {}
impl Recursive<1, true> for Succ {}
impl<const N: Unsigned, const K: Unsigned> Recursive<N, true> for Id<N, K> {}
