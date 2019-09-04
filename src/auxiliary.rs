use num::{Integer, Signed};
use num::integer::gcd;

pub fn normalize_sign<T: Signed>(n: T, d: T) -> (T, T) {
    if d.is_negative() {
        (-n, -d)
    } else {
        (n, d)
    }
}

pub fn reduce<T: Integer + Copy>(a: T, b: T) -> (T, T) {
    if a.is_zero() {
        return (T::zero(), T::one());
    }

    let gcd = gcd(a, b);

    (a / gcd, b / gcd)
}
