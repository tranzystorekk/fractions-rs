use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign};

use num::{Integer, Signed, abs};
use num::integer::lcm;
use crate::auxiliary::{normalize_sign, reduce};

/// Structure representing a common fraction,
/// ie. one where the numerator is an integer
/// and the denominator is a positive integer.
///
/// All operations automatically
/// transform the fraction to its most reduced form,
/// e.g. 14/24 will become 7/12.
///
/// If the fraction is negative, its sign is kept in the numerator.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Fraction<T = i32> {
    numerator: T,
    denominator: T
}

impl<T: Integer + Signed + Copy> Fraction<T> {
    /// Create a new fraction from numerator and denominator.
    ///
    /// # Panics
    ///
    /// Panics if the denominator is zero.
    pub fn new(numerator: T, denominator: T) -> Fraction<T> {
        if denominator.is_zero() {
            panic!("Fraction cannot have a zero denominator");
        }

        let (n, d) = normalize_sign(numerator, denominator);
        let (numerator, denominator) = reduce(n, d);

        Fraction::<T> {
            numerator,
            denominator
        }
    }

    pub fn numerator(&self) -> T {
        self.numerator
    }

    pub fn denominator(&self) -> T {
        self.denominator
    }

    /// Returns a tuple in the form `(numerator, denominator)`.
    pub fn get_as_tuple(&self) -> (T, T) {
        (self.numerator, self.denominator)
    }

    /// Returns `true` if the fraction is proper,
    /// i.e. the absolute value of the numerator
    /// is lower than the denominator.
    pub fn is_proper(&self) -> bool {
        abs(self.numerator) < self.denominator
    }

    /// Returns a new fraction that is the inverse of this fraction, i.e. 1/f.
    ///
    /// # Panics
    ///
    /// Panics if the original fraction is a zero.
    pub fn reciprocal(&self) -> Fraction<T> {
        if self.numerator.is_zero() {
            panic!("Cannot reverse a zero");
        }

        let (numerator, denominator) = normalize_sign(self.denominator, self.numerator);

        Fraction::<T> {
            numerator,
            denominator
        }
    }
}

impl<T: Integer + Signed + Copy> Fraction<T> {
    fn add_impl(&self, other: &Self) -> (T, T) {
        let denom = lcm(self.denominator, other.denominator);
        let num = (self.numerator * denom) / self.denominator
            + (other.numerator * denom) / other.denominator;

        reduce(num, denom)
    }

    fn sub_impl(&self, other: &Self) -> (T, T) {
        let denom = lcm(self.denominator, other.denominator);
        let num = (self.numerator * denom) / self.denominator
            - (other.numerator * denom) / other.denominator;

        reduce(num, denom)
    }

    fn mul_impl(&self, other: &Self) -> (T, T) {
        reduce(self.numerator * other.numerator, self.denominator * other.denominator)
    }

    fn div_impl(&self, other: &Self) -> (T, T) {
        reduce(self.numerator * other.denominator, self.denominator * other.numerator)
    }
}

impl<T: fmt::Display> fmt::Display for Fraction<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl<T> From<Fraction<T>> for f32
    where f32: From<T> {

    fn from(f: Fraction<T>) -> Self {
        f32::from(f.numerator) / f32::from(f.denominator)
    }
}

impl<T> From<Fraction<T>> for f64
    where f64: From<T> {

    fn from(f: Fraction<T>) -> Self {
        f64::from(f.numerator) / f64::from(f.denominator)
    }
}

impl<T: PartialOrd + Integer + Copy> PartialOrd for Fraction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self.numerator * other.denominator;
        let b = other.numerator * self.denominator;

        a.partial_cmp(&b)
    }
}

impl<T: Integer + Signed + Copy> Add for Fraction<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (numerator, denominator) = self.add_impl(&other);

        Fraction::<T> {
            numerator,
            denominator
        }
    }
}

impl<T: Integer + Signed + Copy> AddAssign for Fraction<T> {
    fn add_assign(&mut self, other: Self) {
        let (numerator, denominator) = self.add_impl(&other);

        *self = Self {
            numerator,
            denominator
        };
    }
}

impl<T: Integer + Signed + Copy> Sub for Fraction<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (numerator, denominator) = self.sub_impl(&other);

        Fraction::<T> {
            numerator,
            denominator
        }
    }
}

impl<T: Integer + Signed + Copy> SubAssign for Fraction<T> {
    fn sub_assign(&mut self, other: Self) {
        let (numerator, denominator) = self.sub_impl(&other);

        *self = Self {
            numerator,
            denominator
        }
    }
}

impl<T: Integer + Signed + Copy> Neg for Fraction<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Fraction::<T> {
            numerator: -self.numerator,
            ..self
        }
    }
}

impl<T: Integer + Signed + Copy> Mul for Fraction<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let (numerator, denominator) = self.mul_impl(&rhs);

        Fraction::<T> {
            numerator,
            denominator
        }
    }
}

impl<T: Integer + Signed + Copy> MulAssign for Fraction<T> {
    fn mul_assign(&mut self, rhs: Self) {
        let (numerator, denominator) = self.mul_impl(&rhs);

        *self = Self {
            numerator,
            denominator
        }
    }
}

impl<T: Integer + Signed + Copy> Div for Fraction<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if rhs.numerator.is_zero() {
            panic!("Cannot divide by zero");
        }

        let (numerator, denominator) = self.div_impl(&rhs);

        Fraction::<T> {
            numerator,
            denominator
        }
    }
}

impl<T: Integer + Signed + Copy> DivAssign for Fraction<T> {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.numerator.is_zero() {
            panic!("Cannot divide by zero");
        }

        let (numerator, denominator) = self.div_impl(&rhs);

        *self = Self {
            numerator,
            denominator
        }
    }
}
