use word::{Word, ToWord, UnsignedWord};

/// Sets all bits of `x` at position >= `bit`.
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
/// let s = 0b1110_0010u8;
/// assert_eq!(n.set_bits_geq(5u8), s);
/// assert_eq!(set_bits_geq(n, 5u8), s);
/// ```
#[inline]
pub fn set_bits_geq<T: Word, U: UnsignedWord>(x: T, bit: U) -> T {
    debug_assert!(T::bit_size() > bit.to());
    x | !((T::one() << bit.to()) - T::one())
}

/// Method version of [`set_bits_geq`](fn.set_bits_geq.html).
pub trait SetBitsGeq {
    #[inline]
    fn set_bits_geq<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> SetBitsGeq for T {
    #[inline]
    fn set_bits_geq<U: UnsignedWord>(self, n: U) -> Self {
        set_bits_geq(self, n)
    }
}
