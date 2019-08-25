use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign};

use crate::auxilliary::{lcm, reduce};

/// Structure representing a rational fraction,
/// ie. one where the numerator is an integer
/// and the denominator is a positive integer.
///
/// All operations automatically
/// transform the fraction to its most reduced form,
/// e.g. 14/24 will become 7/12.
///
/// If the fraction is negative, its sign is kept in the numerator.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Fraction {
    numerator: i32,
    denominator: i32
}

impl Fraction {
    /// Create a new fraction from numerator and denominator.
    ///
    /// # Panics
    ///
    /// Panics if the denominator is zero.
    pub fn new(numerator: i32, denominator: i32) -> Fraction {
        if denominator == 0 {
            panic!("Fraction cannot have a zero denominator");
        }

        let (numerator, denominator) = if denominator < 0 {
            reduce(-numerator, -denominator)
        } else {
            reduce(numerator, denominator)
        };

        Fraction {
            numerator,
            denominator
        }
    }

    pub fn numerator(&self) -> i32 {
        self.numerator
    }

    pub fn denominator(&self) -> i32 {
        self.denominator
    }

    pub fn get_as_tuple(&self) -> (i32, i32) {
        (self.numerator, self.denominator)
    }

    /// Returns `true` if the fraction is proper,
    /// i.e. the absolute value of the numerator
    /// is lower than the denominator.
    pub fn is_proper(&self) -> bool {
        self.numerator.abs() < self.denominator
    }

    /// Returns a new fraction that is the reverse of this fraction, i.e. 1/f.
    ///
    /// # Panics
    ///
    /// Panics if the original fraction is a zero.
    pub fn reverse(&self) -> Fraction {
        if self.numerator == 0 {
            panic!("Cannot reverse a zero");
        }

        let (numerator, denominator) = if self.numerator < 0 {
            (-self.denominator, -self.numerator)
        } else {
            (self.denominator, self.numerator)
        };

        Fraction {
            numerator,
            denominator
        }
    }
}

impl Fraction {
    fn add_impl(&self, other: &Self) -> (i32, i32) {
        let denom = lcm(self.denominator, other.denominator);
        let num = (self.numerator * denom) / self.denominator
            + (other.numerator * denom) / other.denominator;

        reduce(num, denom)
    }

    fn sub_impl(&self, other: &Self) -> (i32, i32) {
        let denom = lcm(self.denominator, other.denominator);
        let num = (self.numerator * denom) / self.denominator
            - (other.numerator * denom) / other.denominator;

        reduce(num, denom)
    }

    fn mul_impl(&self, other: &Self) -> (i32, i32) {
        reduce(self.numerator * other.numerator, self.denominator * other.denominator)
    }

    fn div_impl(&self, other: &Self) -> (i32, i32) {
        reduce(self.numerator * other.denominator, self.denominator * other.numerator)
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Into<f32> for Fraction {
    fn into(self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (numerator, denominator) = self.add_impl(&other);

        Fraction {
            numerator,
            denominator
        }
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        let (numerator, denominator) = self.add_impl(&other);

        *self = Self {
            numerator,
            denominator
        };
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (numerator, denominator) = self.sub_impl(&other);

        Fraction {
            numerator,
            denominator
        }
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, other: Self) {
        let (numerator, denominator) = self.sub_impl(&other);

        *self = Self {
            numerator,
            denominator
        }
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self {
        Fraction {
            numerator: -self.numerator,
            ..self
        }
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let (numerator, denominator) = self.mul_impl(&rhs);

        Fraction {
            numerator,
            denominator
        }
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        let (numerator, denominator) = self.mul_impl(&rhs);

        *self = Self {
            numerator,
            denominator
        }
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if rhs.numerator == 0 {
            panic!("Cannot divide by zero");
        }

        let (numerator, denominator) = self.div_impl(&rhs);

        Fraction {
            numerator,
            denominator
        }
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.numerator == 0 {
            panic!("Cannot divide by zero");
        }

        let (numerator, denominator) = self.div_impl(&rhs);

        *self = Self {
            numerator,
            denominator
        }
    }
}
