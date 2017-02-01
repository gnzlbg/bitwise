use word::{Word, ToWord, UnsignedWord};

/// Flip the `bit` of `x`.
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
/// assert_eq!(flip_bit(n, 7u8), 0b0011_0010u8);
/// assert_eq!(n.flip_bit(6u8), 0b1111_0010u8);
/// assert_eq!(n.flip_bit(5u8), 0b1001_0010u8);
/// ```
pub fn flip_bit<T: Word, U: UnsignedWord>(x: T, bit: U) -> T {
    debug_assert!(T::bit_size() > bit.to());
    x ^ (T::one() << bit.to())
}

/// Method version of [`flip_bit`](fn.flip_bit.html).
pub trait FlipBit {
    fn flip_bit<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> FlipBit for T {
    fn flip_bit<U: UnsignedWord>(self, n: U) -> Self {
        flip_bit(self, n)
    }
}
