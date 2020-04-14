use std::error::Error;
use std::fmt;

/// Defines types of errors that might occur when parsing fractions.
#[derive(Debug, PartialEq)]
pub enum FractionParseError<E> {
    IncorrectForm,
    ZeroDenominator,
    NumParseError(E)
}

impl<E> FractionParseError<E> {
    /// Converts from `FractionParseError<E>` to `Option<E>`.
    ///
    /// Converts `self` into an `Option<E>`, consuming `self`,
    /// and discarding errors other than `NumParseError`
    pub fn num_parse_error(self) -> Option<E> {
        match self {
            FractionParseError::NumParseError(err) => Some(err),
            _ => None
        }
    }

    /// Converts from `&FractionParseError<E>` to `Option<&E>`.
    ///
    /// Produces a new `Option`, containing a reference to the `NumParseError` if it occurred,
    /// leaving the original in place.
    pub fn as_num_parse_error(&self) -> Option<&E> {
        match self {
            FractionParseError::NumParseError(err) => Some(err),
            _ => None
        }
    }

    /// Returns `true` if an underlying parse error occurred.
    pub fn is_num_parse_error(&self) -> bool {
        matches!(self, FractionParseError::NumParseError(_))
    }

    /// Returns `true` if an incorrect form error occurred.
    pub fn is_incorrect_form(&self) -> bool {
        matches!(self, FractionParseError::IncorrectForm)
    }

    /// Returns `true` if a zero denominator error occurred.
    pub fn is_zero_denominator(&self) -> bool {
        matches!(self, FractionParseError::ZeroDenominator)
    }
}

impl<E: fmt::Display> fmt::Display for FractionParseError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FractionParseError::IncorrectForm =>
                write!(f, "Incorrectly formed fraction (format should be <D>/<N>)"),
            FractionParseError::ZeroDenominator =>
                write!(f, "Fraction denominator cannot be zero"),
            FractionParseError::NumParseError(err) =>
                write!(f, "Error when parsing fraction: {}", err)
        }
    }
}

impl<E: Error + 'static> Error for FractionParseError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FractionParseError::NumParseError(err) => Some(err),
            _ => None
        }
    }
}
