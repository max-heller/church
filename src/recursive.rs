use crate::Unsigned;

pub trait PrimitiveRecursive<const N: Unsigned> = Recursive<N, true>;
pub trait Recursive<const N: Unsigned, const PRIMITIVE: bool>: std::fmt::Debug {}

impl<T: Recursive<N, true>, const N: Unsigned> Recursive<N, false> for T {}
