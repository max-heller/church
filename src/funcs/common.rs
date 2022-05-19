use super::basic::*;
use crate::{compute::ops::*, compute::Computable, *};

pub fn const_n(n: usize) -> impl Computable<1> {
    let mut f = cn![id![1, 1]; Z];
    for _ in 0..n {
        f = cn![
            id![1, 1];
            cn![S; const_n(n - 1)]
        ];
    }
    f
}

pub fn sum() -> impl Computable<2> {
    pr![id![1, 1], cn![S; id![3, 3]]]
}

pub fn product() -> impl Computable<2> {
    pr![Z, cn![sum(); id![3, 1], id![3, 3]]]
}

pub fn power() -> impl Computable<2> {
    pr![cn![S; Z], cn![product(); id![3, 1], id![3, 3]]]
}

pub fn superpower() -> impl Computable<2> {
    pr![cn![S; Z], cn![power(); id![3, 1], id![3, 3]]]
}

fn one_place_recursive_func<F, G>(f: F, g: G) -> impl Computable<1>
where
    F: Computable<1>,
    G: Computable<3>,
{
    cn![pr![f, g]; id![1, 1], id![1, 1]]
}

pub fn predecessor() -> impl Computable<1> {
    one_place_recursive_func(Z, id![3, 2])
}

pub fn difference() -> impl Computable<2> {
    pr!(id![1, 1], cn![predecessor(); id![3, 3]])
}

pub fn antisignum() -> impl Computable<1> {
    cn![difference(); cn![S; Z], id![1, 1]]
}

pub fn signum() -> impl Computable<1> {
    cn![difference(); cn![S; Z], antisignum()]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{compute::Compute, defined_eq};
    use quickcheck_macros::quickcheck;

    // TODO: takes far too long, likely due to constructing function each time?
    // #[quickcheck]
    // fn const_n_is_n(n: usize, x: usize) -> bool {
    //     Some(n) == const_n(n).call(&[x])
    // }

    #[quickcheck]
    fn sum_is_sum(a: usize, b: usize) -> bool {
        Some(a + b) == sum().call(&[a, b])
    }

    #[quickcheck]
    fn product_is_product(a: usize, b: usize) -> bool {
        Some(a * b) == product().call(&[a, b])
    }

    #[test]
    fn test_power() {
        let pow = power();
        defined_eq!(pow.call(&[2, 3]), 8);
    }

    #[quickcheck]
    fn predecessor_is_predecessor(x: usize) -> bool {
        Some(x.saturating_sub(1)) == predecessor().call(&[x])
    }

    #[quickcheck]
    fn difference_is_difference(x: usize, y: usize) -> bool {
        Some(x.saturating_sub(y)) == difference().call(&[x, y])
    }

    #[quickcheck]
    fn antisignum_is_antisignum(x: usize) -> bool {
        Some(1usize.saturating_sub(x)) == antisignum().call(&[x])
    }

    #[quickcheck]
    fn signum_is_signum(x: usize) -> bool {
        Some(1usize.saturating_sub(1usize.saturating_sub(x))) == signum().call(&[x])
    }
}
