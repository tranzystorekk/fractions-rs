use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign};

#[macro_export]
macro_rules! frac {
    ( $n:expr, $d:expr ) => {
        Fraction::new($n, $d)
    };
    ( $n:expr ) => {
        Fraction::new($n, 1)
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Fraction {
    numerator: i32,
    denominator: i32
}

fn minmax(a: i32, b: i32) -> (i32, i32) {
    if b < a {
        (b, a)
    } else {
        (a, b)
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 1;
    }

    let (mut lower, mut greater) = minmax(a, b);

    let mut modulo = greater % lower;
    while modulo != 0 {
        greater = lower;
        lower = modulo;

        modulo = greater % lower;
    }

    lower
}

fn lcm(a: i32, b: i32) -> i32 {
    let gcd = gcd(a, b);

    (a * b) / gcd
}

fn reduce(a: i32, b:i32) -> (i32, i32) {
    let gcd = gcd(a.abs(), b);

    (a / gcd, b / gcd)
}

impl Fraction {
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

    pub fn get_numerator(&self) -> i32 {
        self.numerator
    }

    pub fn get_denominator(&self) -> i32 {
        self.denominator
    }

    pub fn get_as_tuple(&self) -> (i32, i32) {
        (self.numerator, self.denominator)
    }

    pub fn is_proper(&self) -> bool {
        self.numerator.abs() < self.denominator
    }

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
