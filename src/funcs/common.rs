use super::basic::*;
use crate::{compute::ops::*, compute::Computable, *};
use typenum::consts::*;

// TODO: this is gross to make it type check, can I make it better?
pub fn const_n(n: usize) -> impl Computable<U1> {
    let mut f = cn![id![U1, U1]; U1; Z];
    for _ in 0..n {
        f = cn![
            id![U1, U1]; U1;
            cn![S; U1; const_n(n - 1)]
        ];
    }
    f
}

pub fn sum() -> impl Computable<U2> {
    pr![id![U1, U1], cn![S; U3; id![U3, U3]]]
}

pub fn product() -> impl Computable<U2> {
    pr![Z, cn![sum(); U3; id![U3, U1], id![U3, U3]]]
}

pub fn power() -> impl Computable<U2> {
    pr![cn![S; U1; Z], cn![product(); U3; id![U3, U1], id![U3, U3]]]
}

pub fn superpower() -> impl Computable<U2> {
    pr![cn![S; U1; Z], cn![power(); U3; id![U3, U1], id![U3, U3]]]
}

fn one_place_recursive_func<F, G>(f: F, g: G) -> impl Computable<U1>
where
    F: Computable<U1>,
    G: Computable<U3>,
{
    cn![pr![f, g]; U1; id![U1, U1], id![U1, U1]]
}

pub fn predecessor() -> impl Computable<U1> {
    one_place_recursive_func(Z, id![U3, U2])
}

pub fn difference() -> impl Computable<U2> {
    pr!(id![U1, U1], cn![predecessor(); U3; id![U3, U3]])
}

pub fn antisignum() -> impl Computable<U1> {
    cn![difference(); U1; cn![S; U1; Z], id![U1, U1]]
}

pub fn signum() -> impl Computable<U1> {
    cn![difference(); U1; cn![S; U1; Z], antisignum()]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{args, compute::Compute, defined_eq};
    use quickcheck_macros::quickcheck;

    // TODO: takes far too long, likely due to constructing function each time?
    // #[quickcheck]
    // fn const_n_is_n(n: usize, x: usize) -> bool {
    //     Some(n) == const_n(n).call(args![x])
    // }

    #[quickcheck]
    fn sum_is_sum(a: usize, b: usize) -> bool {
        Some(a + b) == sum().call(args![a, b])
    }

    #[quickcheck]
    fn product_is_product(a: usize, b: usize) -> bool {
        Some(a * b) == product().call(args![a, b])
    }

    #[test]
    fn test_power() {
        let pow = power();
        defined_eq!(pow.call(args![2, 3]), 8);
    }

    #[quickcheck]
    fn predecessor_is_predecessor(x: usize) -> bool {
        Some(x.saturating_sub(1)) == predecessor().call(args![x])
    }

    #[quickcheck]
    fn difference_is_difference(x: usize, y: usize) -> bool {
        Some(x.saturating_sub(y)) == difference().call(args![x, y])
    }

    #[test]
    fn antisignum_is_antisignum() {
        let asg = antisignum();
        defined_eq!(asg.call(args![0]), 1);
        // defined_eq!(asg.call(args![1]), 1);
    }

    // #[quickcheck]
    // fn antisignum_is_antisignum(x: usize) -> bool {
    //     Some(1usize.saturating_sub(x)) == antisignum().call(args![x])
    // }

    // #[quickcheck]
    // fn signum_is_signum(x: usize) -> bool {
    //     Some(1usize.saturating_sub(1usize.saturating_sub(x))) == signum().call(args![x])
    // }
}
