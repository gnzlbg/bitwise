use word::{Word, ToWord, UnsignedWord};

/// Flips all bits of `x` at position <= `bit`.
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
/// let s = 0b1000_1101u8;
/// assert_eq!(n.flip_bits_leq(5u8), s);
/// assert_eq!(flip_bits_leq(n, 5u8), s);
/// ```
#[inline]
pub fn flip_bits_leq<T: Word, U: UnsignedWord>(x: T, bit: U) -> T {
    debug_assert!(T::bit_size() > bit.to());
    x ^ ((T::one() << (T::one() + bit.to())) - T::one())
}

/// Method version of [`flip_bits_leq`](fn.flip_bits_leq.html).
pub trait FlipBitsLeq {
    #[inline]
    fn flip_bits_leq<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> FlipBitsLeq for T {
    #[inline]
    fn flip_bits_leq<U: UnsignedWord>(self, n: U) -> Self {
        flip_bits_leq(self, n)
    }
}
