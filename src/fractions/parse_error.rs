use std::error::Error;
use std::fmt;

/// Defines types of errors that might occur when parsing fractions.
#[derive(Debug, PartialEq)]
pub enum FractionParseErr<E> {
    IncorrectForm,
    ZeroDenominator,
    NumParseErr(E)
}

impl<E> FractionParseErr<E> {
    /// Returns the underlying parse error (wrapped) if it occurred, `None` otherwise.
    pub fn num_parse_err(self) -> Option<E> {
        if let FractionParseErr::NumParseErr(err) = self {
            Some(err)
        } else {
            None
        }
    }

    /// Returns `true` if an underlying parse error occurred, false otherwise.
    pub fn is_num_parse_err(&self) -> bool {
        if let FractionParseErr::NumParseErr(_) = self {
            true
        } else {
            false
        }
    }
}

impl<E: fmt::Display> fmt::Display for FractionParseErr<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FractionParseErr::IncorrectForm =>
                write!(f, "Incorrectly formed fraction (format should be <D>/<N>)"),
            FractionParseErr::ZeroDenominator =>
                write!(f, "Fraction denominator cannot be zero"),
            FractionParseErr::NumParseErr(err) =>
                write!(f, "Error when parsing fraction: {}", err)
        }
    }
}

impl<E: Error + 'static> Error for FractionParseErr<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let FractionParseErr::NumParseErr(err) = self {
            Some(err)
        } else {
            None
        }
    }
}
