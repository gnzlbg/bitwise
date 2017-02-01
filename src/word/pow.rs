use word::{Word, ToWord, UnsignedWord};

/// Raises `x` to the power of `exp`.
///
/// Uses exponentiation by squaring.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert_eq!(pow(2i8, 6u8), 64i8);
/// ```
pub fn pow<T: Word, U: UnsignedWord>(x: T, exp: U) -> T {
    T::pow(x, exp.to())
}

// TODO: figure out a way to offer the trait version without clashing with the
// one in std:
/*
pub trait Pow {
    fn pow<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> Pow for T {
    fn pow<U: UnsignedWord>(self, n: U) -> Self {
        pow(self, n)
    }
}
*/
