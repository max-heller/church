use crate::{recursive::Recursive, Primitive, Unsigned};

#[derive(Debug)]
pub struct Zero;
pub const Z: Zero = Zero;

#[derive(Debug)]
pub struct Succ;
pub const S: Succ = Succ;

pub struct Id<const K: Unsigned>;

impl<const K: Unsigned> std::fmt::Debug for Id<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id<{}>", K)
    }
}

#[macro_export]
macro_rules! id {
    ($K:expr) => {
        Id::<$K>
    };
}

impl Recursive for Zero {}
impl Primitive for Zero {}
impl Recursive for Succ {}
impl Primitive for Succ {}
impl<const K: Unsigned> Recursive for Id<K> {}
impl<const K: Unsigned> Primitive for Id<K> {}
