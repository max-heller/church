use crate::recursive::{PrimitiveRecursive, Recursive};
use std::{marker::PhantomData, ops::Sub};
use typenum::{consts::U1, Diff, NonZero, Unsigned};

#[derive(Debug)]
pub struct Zero;
pub const Z: Zero = Zero;

#[derive(Debug)]
pub struct Succ;
pub const S: Succ = Succ;

pub struct Id<N, K>
where
    N: Unsigned + NonZero,
    K: Unsigned + NonZero,
{
    n: PhantomData<N>,
    k: PhantomData<K>,
}

impl<N, K> std::fmt::Debug for Id<N, K>
where
    N: Unsigned + NonZero,
    K: Unsigned + NonZero,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id<{}, {}>", N::USIZE, K::USIZE)
    }
}

impl<N, K> Id<N, K>
where
    N: Unsigned + NonZero + Sub<K>,
    K: Unsigned + NonZero,
    Diff<N, K>: Unsigned,
{
    pub fn new() -> Self {
        Id {
            n: PhantomData,
            k: PhantomData,
        }
    }
}

#[macro_export]
macro_rules! id {
    ($N:ty, $K:ty) => {
        Id::<$N, $K>::new()
    };
}

impl PrimitiveRecursive<U1> for Zero {}
impl PrimitiveRecursive<U1> for Succ {}
impl<N, K> PrimitiveRecursive<N> for Id<N, K>
where
    N: Unsigned + NonZero,
    K: Unsigned + NonZero,
{
}
impl Recursive<U1> for Zero {}
impl Recursive<U1> for Succ {}

impl<N, K> Recursive<N> for Id<N, K>
where
    N: Unsigned + NonZero,
    K: Unsigned + NonZero,
{
}
