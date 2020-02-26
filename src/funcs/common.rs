use super::basic::*;
use crate::{compute::ops::*, compute::Computable, funcs, id};
use typenum::consts::*;

// TODO: this is gross to make it type check, can I make it better?
pub fn const_n(n: usize) -> impl Computable<U1> {
    match n {
        0 => Cn::new(id![U1, U1], funcs![U1; &Z]),
        _ => Cn::new(
            id![U1, U1],
            funcs![U1; &Cn::new(S, funcs![U1; &const_n(n - 1)])],
        ),
    }
}

pub fn sum() -> impl Computable<U2> {
    Pr::new(id![U1, U1], Cn::new(S, funcs![U3; &id![U3, U3]]))
}

pub fn product() -> impl Computable<U2> {
    Pr::new(Z, Cn::new(sum(), funcs![U3; &id![U3, U1], &id![U3, U3]]))
}

pub fn power() -> impl Computable<U2> {
    Pr::new(
        Cn::new(S, funcs![U1; &Z]),
        Cn::new(product(), funcs![U3; &id![U3, U1], &id![U3, U3]]),
    )
}

pub fn superpower() -> impl Computable<U2> {
    Pr::new(
        Cn::new(S, funcs![U1; &Z]),
        Cn::new(power(), funcs![U3; &id![U3, U1], &id![U3, U3]]),
    )
}
