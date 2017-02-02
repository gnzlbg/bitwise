use word::{Word, ToWord, UnsignedWord};

/// Clears all bits of `x` at position <= `bit`.
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
/// let n = 0b1111_0010u8;
/// let s = 0b1100_0000u8;
/// assert_eq!(n.clear_bits_leq(5u8), s);
/// assert_eq!(clear_bits_leq(n, 5u8), s);
/// ```
#[inline]
pub fn clear_bits_leq<T: Word, U: UnsignedWord>(x: T, bit: U) -> T {
    debug_assert!(T::bit_size() > bit.to());
    x & !((T::one() << (T::one() + bit.to())) - T::one())
}

/// Method version of [`clear_bits_leq`](fn.clear_bits_leq.html).
pub trait ClearBitsLeq {
    #[inline]
    fn clear_bits_leq<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> ClearBitsLeq for T {
    #[inline]
    fn clear_bits_leq<U: UnsignedWord>(self, n: U) -> Self {
        clear_bits_leq(self, n)
    }
}
