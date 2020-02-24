use crate::recursive::{PrimitiveRecursive, Recursive};
use std::marker::PhantomData;
use typenum::{consts::U1, NonZero, Unsigned};

pub struct Zero {}
pub struct Succ {}
pub struct Id<N, K>
where
    N: Unsigned + NonZero,
    K: Unsigned + NonZero,
{
    n: PhantomData<N>,
    k: PhantomData<K>,
}

impl<N, K> Id<N, K>
where
    N: Unsigned + NonZero,
    K: Unsigned + NonZero,
{
    pub fn new() -> Self {
        Id {
            n: PhantomData,
            k: PhantomData,
        }
    }
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
