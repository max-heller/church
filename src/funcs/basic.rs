use crate::{recursive::Recursive, Primitive};

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
impl Primitive for Zero {}
impl Recursive for Succ {}
impl Primitive for Succ {}
impl<const K: usize> Recursive for Id<K> {}
impl<const K: usize> Primitive for Id<K> {}
