mod auxiliary;

mod fractions;
pub use crate::fractions::Fraction;

pub type Fraction8 = Fraction<i8>;
pub type Fraction16 = Fraction<i16>;
pub type Fraction64 = Fraction<i64>;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;
