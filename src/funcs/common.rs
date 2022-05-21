use crate::{hlist::*, *};

pub fn const_n(n: usize) -> impl Compute<1> {
    let mut f: Box<dyn Compute<1>> = Box::new(cn![id![1]; Z]);
    for _ in 0..n {
        f = Box::new(cn![S; f]);
    }
    f
}

pub type Sum = Pr<Id<1>, Cn<Succ, Cons<Id<3>, Nil, 1>>>;

pub const SUM: Sum = pr(id![1], cn![S; id![3]]);

pub type Product = Pr<Zero, Cn<Sum, Cons<Id<1>, Cons<Id<3>, Nil, 1>, 2>>>;

pub const PRODUCT: Product = pr(Z, cn![SUM; id![1], id![3]]);

pub type Power = Pr<Cn<Succ, Cons<Zero, Nil, 1>>, Cn<Product, Cons<Id<1>, Cons<Id<3>, Nil, 1>, 2>>>;

pub const POWER: Power = pr(cn![S; Z], cn![PRODUCT; id![1], id![3]]);

pub type Superpower =
    Pr<Cn<Succ, Cons<Zero, Nil, 1>>, Cn<Power, Cons<Id<1>, Cons<Id<3>, Nil, 1>, 2>>>;

pub const SUPERPOWER: Superpower = pr(cn![S; Z], cn![POWER; id![1], id![3]]);

const fn one_place_recursive_func<F, G>(
    f: F,
    g: G,
) -> Cn<Pr<F, G>, Cons<Id<1>, Cons<Id<1>, Nil, 1>, 2>>
where
    F: Compute<1>,
    G: Compute<3>,
{
    cn![pr(f, g); id![1], id![1]]
}

pub type Predecessor = Cn<Pr<Zero, Id<2>>, Cons<Id<1>, Cons<Id<1>, Nil, 1>, 2>>;

pub const PREDECESSOR: Predecessor = one_place_recursive_func(Z, id![2]);

pub type Difference = Pr<Id<1>, Cn<Predecessor, Cons<Id<3>, Nil, 1>>>;

pub const DIFFERENCE: Difference = pr(id![1], cn![PREDECESSOR; id![3]]);

pub type Antisignum = Cn<Difference, Cons<Cn<Succ, Cons<Zero, Nil, 1>>, Cons<Id<1>, Nil, 1>, 2>>;

pub const ANTISIGNUM: Antisignum = cn![DIFFERENCE; cn![S; Z], id![1]];

pub type Signum = Cn<Difference, Cons<Cn<Succ, Cons<Zero, Nil, 1>>, Cons<Antisignum, Nil, 1>, 2>>;

pub const SIGNUM: Signum = cn![DIFFERENCE; cn![S; Z], ANTISIGNUM];

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn const_n_is_n(n: u8, x: u8) -> bool {
        let (n, x) = (n as usize, x as usize);
        Some(n) == const_n(n).call(&[x])
    }

    #[quickcheck]
    fn sum_is_sum(a: u8, b: u8) -> bool {
        let (a, b) = (a as usize, b as usize);
        Some(a + b) == SUM.call(&[a, b])
    }

    #[quickcheck]
    fn product_is_product(a: u8, b: u8) -> bool {
        let (a, b) = (a as usize, b as usize);
        Some(a * b) == PRODUCT.call(&[a, b])
    }

    #[test]
    fn test_power() {
        defined_eq!(POWER.call(&[2, 3]), 8);
    }

    #[quickcheck]
    fn predecessor_is_predecessor(x: u8) -> bool {
        let x = x as usize;
        Some(x.saturating_sub(1)) == PREDECESSOR.call(&[x])
    }

    #[quickcheck]
    fn difference_is_difference(x: u8, y: u8) -> bool {
        let (x, y) = (x as usize, y as usize);
        Some(x.saturating_sub(y)) == DIFFERENCE.call(&[x, y])
    }

    #[quickcheck]
    fn antisignum_is_antisignum(x: u8) -> bool {
        let x = x as usize;
        Some(1usize.saturating_sub(x)) == ANTISIGNUM.call(&[x])
    }

    #[quickcheck]
    fn signum_is_signum(x: u8) -> bool {
        let x = x as usize;
        Some(1usize.saturating_sub(1usize.saturating_sub(x))) == SIGNUM.call(&[x])
    }
}
