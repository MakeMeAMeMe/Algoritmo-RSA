use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};
use std::fmt;
use std::mem::replace;
use std::ops::Div;

pub struct Tripla {
    pub mdc: BigInt,
    pub x: BigInt,
    pub y: BigInt,
}

impl fmt::Display for Tripla {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(MDC: {mdc}, X: {x}, Y: {y})",
            mdc = self.mdc,
            x = self.x,
            y = self.y
        )
    }
}

// ax + by = mdc(a, b)
pub fn euclides_estendido(a: &BigInt, b: &BigInt) -> Tripla {
    let mut mdc: BigInt = a.clone();
    let mut mdc_old: BigInt = b.clone();

    let mut x: BigInt = One::one();
    let mut x_old: BigInt = Zero::zero();
    let mut y: BigInt = Zero::zero();
    let mut y_old: BigInt = One::one();

    while mdc_old != Zero::zero() {
        let q = (&mdc).div(&mdc_old);

        let diff_mdc = &q * &mdc_old;
        mdc_old = replace(&mut mdc, mdc_old) - &diff_mdc;

        let diff_x = &q * &x_old; 
        x_old = replace(&mut x, x_old) - &diff_x;

        let diff_y = &q * &y_old;
        y_old = replace(&mut y, y_old) - &diff_y;
    }

    let tripla = Tripla {
        mdc: mdc,
        x: x,
        y: y,
    };
    return tripla;
}

pub fn inverso_modular(a: &BigInt, m: &BigInt) -> BigInt {
    let anwser: Tripla = euclides_estendido(a, m);
    return (anwser.x.mod_floor(&m) + m).mod_floor(m);
}
