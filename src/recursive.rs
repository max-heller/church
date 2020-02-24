use typenum::Unsigned;

pub trait PrimitiveRecursive<N: Unsigned>: Recursive<N> {}
pub trait Recursive<N: Unsigned> {}

// impl<T: PrimitiveRecursive<N>, N: Unsigned> Recursive<N> for T {}
