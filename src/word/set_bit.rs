use word::{Word, ToWord, UnsignedWord};

/// Sets the `bit` of `x`.
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
/// let n  = 0b1011_0010u8;
/// assert_eq!(n.set_bit(6u8), 0b1111_0010u8);
/// assert_eq!(set_bit(n, 0u8), 0b1011_0011u8);
/// assert_eq!(n.set_bit(3u8), 0b1011_1010u8);
/// ```
pub fn set_bit<T: Word, U: UnsignedWord>(x: T, bit: U) -> T {
    debug_assert!(T::bit_size() > bit.to());
    x | (T::one() << bit.to())
}

/// Method version of [`set_bit`](fn.set_bit.html).
pub trait SetBit {
    fn set_bit<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> SetBit for T {
    fn set_bit<U: UnsignedWord>(self, n: U) -> Self {
        set_bit(self, n)
    }
}
