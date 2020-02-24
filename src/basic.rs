use crate::recursive::{PrimitiveRecursive, Recursive};
use std::marker::PhantomData;
use typenum::{consts::U1, NonZero, Unsigned};

pub struct Zero {}
pub struct Succ {}
pub struct Id<N: Unsigned + NonZero, K: Unsigned + NonZero> {
    n: PhantomData<N>,
    k: PhantomData<K>,
}

impl<N: Unsigned + NonZero, K: Unsigned + NonZero> Id<N, K> {
    pub fn new() -> Self {
        Id {
            n: PhantomData,
            k: PhantomData,
        }
    }
}

impl PrimitiveRecursive<U1> for Zero {}
impl PrimitiveRecursive<U1> for Succ {}
impl<N: Unsigned + NonZero, K: Unsigned + NonZero> PrimitiveRecursive<N> for Id<N, K> {}
impl Recursive<U1> for Zero {}
impl Recursive<U1> for Succ {}
impl<N: Unsigned + NonZero, K: Unsigned + NonZero> Recursive<N> for Id<N, K> {}
