pub struct Nil;

pub struct Cons<H, T, const LEN: usize> {
    head: H,
    tail: T,
}

#[doc(hidden)]
pub const fn cons<H, T, const LEN: usize>(head: H, tail: T) -> Cons<H, T, { LEN + 1 }>
where
    T: Length<LEN>,
{
    Cons { head, tail }
}

impl<H, T, const LEN: usize> Cons<H, T, LEN> {
    pub(crate) fn head(&self) -> &H {
        &self.head
    }

    pub(crate) fn tail(&self) -> &T {
        &self.tail
    }
}

#[macro_export]
macro_rules! hlist {
    () => { $crate::hlist::Nil };
    ($x:expr) => { $crate::hlist::cons($x, $crate::hlist::Nil) };
    ($x:expr, $($xs:expr),*) => {
        $crate::hlist::cons($x, $crate::hlist![$($xs),*])
    };
}

pub trait Length<const N: usize> {}

impl Length<0> for Nil {}

impl<const N: usize, H, T> Length<N> for Cons<H, T, N> {}
