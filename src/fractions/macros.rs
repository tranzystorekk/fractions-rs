/// Construct a new fraction from numerator and denominator:
///
/// ```
/// use fractions::frac;
/// use fractions::Fraction;
///
/// let f = frac!(1, 5); // one fifth
/// ```
///
/// Or from integer:
///
/// ```
/// use fractions::frac;
/// use fractions::Fraction;
///
/// let f = frac!(5); // denominator defaults to 1
/// ```
#[macro_export]
macro_rules! frac {
    ( $n:expr, $d:expr ) => {
        Fraction::new($n, $d)
    };
    ( $n:expr ) => {
        Fraction::new($n, 1)
    };
}
