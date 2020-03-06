use typenum::Unsigned;

pub trait PrimitiveRecursive<N: Unsigned>: Recursive<N> {}
pub trait Recursive<N: Unsigned>: std::fmt::Debug {}
