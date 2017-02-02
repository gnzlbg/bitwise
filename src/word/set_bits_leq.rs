use word::{Word, ToWord, UnsignedWord};

/// Sets all bits of `x` at position <= `bit`.
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
/// let n = 0b1000_0010u8;
/// let s = 0b1011_1111u8;
/// assert_eq!(n.set_bits_leq(5u8), s);
/// assert_eq!(set_bits_leq(n, 5u8), s);
/// ```
#[inline]
pub fn set_bits_leq<T: Word, U: UnsignedWord>(x: T, bit: U) -> T {
    debug_assert!(T::bit_size() > bit.to());
    x | ((T::one() << (T::one() + bit.to())) - T::one())
}

/// Method version of [`set_bits_leq`](fn.set_bits_leq.html).
pub trait SetBitsLeq {
    #[inline]
    fn set_bits_leq<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> SetBitsLeq for T {
    #[inline]
    fn set_bits_leq<U: UnsignedWord>(self, n: U) -> Self {
        set_bits_leq(self, n)
    }
}
