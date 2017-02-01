use word::{Word, ToWord, UnsignedWord};

/// Test the `bit` of `x`.
///
/// # Panics
///
/// If `bit >= bit_size()`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b1011_0010u8;
/// assert_eq!(test_bit(n, 7u8), true);
/// assert_eq!(n.test_bit(6u8), false);
/// assert_eq!(n.test_bit(5u8), true);
/// ```
pub fn test_bit<T: Word, U: UnsignedWord>(x: T, bit: U) -> bool {
    debug_assert!(T::bit_size() > bit.to());
    x & (T::one() << bit.to()) != T::zero()
}

/// Method version of [`test_bit`](fn.test_bit.html).
pub trait TestBit {
    fn test_bit<U: UnsignedWord>(self, n: U) -> bool;
}

impl<T: Word> TestBit for T {
    fn test_bit<U: UnsignedWord>(self, n: U) -> bool {
        test_bit(self, n)
    }
}
