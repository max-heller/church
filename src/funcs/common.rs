use super::basic::*;
use crate::{compute::ops::*, compute::Compute, *};

pub fn const_n(n: usize) -> impl Compute<1> {
    let mut f: Box<dyn Compute<1>> = Box::new(cn![id![1, 1]; Z]);
    for _ in 0..n {
        f = Box::new(cn![S; f]);
    }
    f
}

pub fn sum() -> impl Compute<2> {
    pr![id![1, 1], cn![S; id![3, 3]]]
}

pub fn product() -> impl Compute<2> {
    pr![Z, cn![sum(); id![3, 1], id![3, 3]]]
}

pub fn power() -> impl Compute<2> {
    pr![cn![S; Z], cn![product(); id![3, 1], id![3, 3]]]
}

pub fn superpower() -> impl Compute<2> {
    pr![cn![S; Z], cn![power(); id![3, 1], id![3, 3]]]
}

fn one_place_recursive_func<F, G>(f: F, g: G) -> impl Compute<1>
where
    F: Compute<1>,
    G: Compute<3>,
{
    cn![pr![f, g]; id![1, 1], id![1, 1]]
}

pub fn predecessor() -> impl Compute<1> {
    one_place_recursive_func(Z, id![3, 2])
}

pub fn difference() -> impl Compute<2> {
    pr!(id![1, 1], cn![predecessor(); id![3, 3]])
}

pub fn antisignum() -> impl Compute<1> {
    cn![difference(); cn![S; Z], id![1, 1]]
}

pub fn signum() -> impl Compute<1> {
    cn![difference(); cn![S; Z], antisignum()]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{compute::Compute, defined_eq};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn const_n_is_n(n: u8, x: u8) -> bool {
        let (n, x) = (n as usize, x as usize);
        Some(n) == const_n(n).call(&[x])
    }

    #[quickcheck]
    fn sum_is_sum(a: u8, b: u8) -> bool {
        let (a, b) = (a as usize, b as usize);
        Some(a + b) == sum().call(&[a, b])
    }

    #[quickcheck]
    fn product_is_product(a: u8, b: u8) -> bool {
        let (a, b) = (a as usize, b as usize);
        Some(a * b) == product().call(&[a, b])
    }

    #[test]
    fn test_power() {
        let pow = power();
        defined_eq!(pow.call(&[2, 3]), 8);
    }

    #[quickcheck]
    fn predecessor_is_predecessor(x: u8) -> bool {
        let x = x as usize;
        Some(x.saturating_sub(1)) == predecessor().call(&[x])
    }

    #[quickcheck]
    fn difference_is_difference(x: u8, y: u8) -> bool {
        let (x, y) = (x as usize, y as usize);
        Some(x.saturating_sub(y)) == difference().call(&[x, y])
    }

    #[quickcheck]
    fn antisignum_is_antisignum(x: u8) -> bool {
        let x = x as usize;
        Some(1usize.saturating_sub(x)) == antisignum().call(&[x])
    }

    #[quickcheck]
    fn signum_is_signum(x: u8) -> bool {
        let x = x as usize;
        Some(1usize.saturating_sub(1usize.saturating_sub(x))) == signum().call(&[x])
    }
}
