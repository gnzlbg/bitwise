use word::{Word, ToWord, UnsignedWord};

/// Flips all bits of `x` at position >= `bit`.
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
/// let n = 0b1001_0010u8;
/// let s = 0b0111_0010u8;
/// assert_eq!(n.flip_bits_geq(5u8), s);
/// assert_eq!(flip_bits_geq(n, 5u8), s);
/// ```
pub fn flip_bits_geq<T: Word, U: UnsignedWord>(x: T, bit: U) -> T {
    debug_assert!(T::bit_size() > bit.to());
    x ^ !((T::one() << bit.to()) - T::one())

}

/// Method version of [`flip_bits_geq`](fn.flip_bits_geq.html).
pub trait FlipBitsGeq {
    fn flip_bits_geq<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> FlipBitsGeq for T {
    fn flip_bits_geq<U: UnsignedWord>(self, n: U) -> Self {
        flip_bits_geq(self, n)
    }
}
