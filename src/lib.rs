mod fractions;
pub use crate::fractions::Fraction;
pub use crate::fractions::parse_error::FractionParseError;

pub type Fraction8 = Fraction<i8>;
pub type Fraction16 = Fraction<i16>;
pub type Fraction64 = Fraction<i64>;

#[cfg(test)]
mod tests;
