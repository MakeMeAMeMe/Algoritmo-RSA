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
pub fn euclides_estendido(a: &BigInt, b: &BigInt) -> Tripla { // O(n^2)
    let mut mdc: BigInt = a.clone(); // O(n)
    let mut mdc_old: BigInt = b.clone(); // O(n)

    let mut x: BigInt = One::one(); // O(n)
    let mut x_old: BigInt = Zero::zero(); // O(n)
    let mut y: BigInt = Zero::zero(); // O(n)
    let mut y_old: BigInt = One::one(); // O(n)

    while mdc_old != Zero::zero() { // O(k) * O(n^2 + ... + n) = O(k*n^2)
        let q = (&mdc).div(&mdc_old); // O(n^2)

        let diff_mdc = &q * &mdc_old; // O(n^1.585)
        mdc_old = replace(&mut mdc, mdc_old) - &diff_mdc; // O(n) + O(n)

        let diff_x = &q * &x_old; // O(n^1.5)
        x_old = replace(&mut x, x_old) - &diff_x; // O(n) + O(n)

        let diff_y = &q * &y_old; // O(n^1.585)
        y_old = replace(&mut y, y_old) - &diff_y; // O(n) + O(n)
    }

    let tripla = Tripla {
        mdc: mdc,
        x: x,
        y: y,
    };
    
    return tripla;
}

pub fn inverso_modular(a: &BigInt, m: &BigInt) -> BigInt { //O(n^2)
    let anwser: Tripla = euclides_estendido(a, m); // O(n^2)
    return (anwser.x.mod_floor(&m) + m).mod_floor(m); // O(n)
}
