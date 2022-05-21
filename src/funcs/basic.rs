use crate::{recursive::Recursive, PrimitiveRecursive};

#[derive(Debug)]
pub struct Zero;
pub const Z: Zero = Zero;

#[derive(Debug)]
pub struct Succ;
pub const S: Succ = Succ;

pub struct Id<const K: usize>;

impl<const K: usize> std::fmt::Debug for Id<K> {
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
impl PrimitiveRecursive for Zero {}

impl Recursive for Succ {}
impl PrimitiveRecursive for Succ {}

impl<const K: usize> Recursive for Id<K> {}
impl<const K: usize> PrimitiveRecursive for Id<K> {}
