use word::{Word, ToWord, UnsignedWord};

/// Clear the `bit` of `x`.
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
/// assert_eq!(clear_bit(n, 7u8), 0b0011_0010u8);
/// assert_eq!(n.clear_bit(1u8), 0b1011_0000u8);
/// assert_eq!(n.clear_bit(5u8), 0b1001_0010u8);
/// ```
#[inline]
pub fn clear_bit<T: Word, U: UnsignedWord>(x: T, bit: U) -> T {
    debug_assert!(T::bit_size() > bit.to());
    x & !(T::one() << bit.to())
}

/// Method version of [`clear_bit`](fn.clear_bit.html).
pub trait ClearBit {
    #[inline]
    fn clear_bit<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> ClearBit for T {
    #[inline]
    fn clear_bit<U: UnsignedWord>(self, n: U) -> Self {
        clear_bit(self, n)
    }
}
