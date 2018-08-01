use word::{Word, ToWord, UnsignedWord};

/// Clears all bits of `x` at position >= `bit`.
///
/// # Panics
///
/// If `bit >= bit_size()`.
///
/// # Intrinsics:
/// - BMI 2.0: bzhi.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert_eq!(0b1111_0010u8.clear_bits_geq(5u8), 0b0001_0010u8);
/// assert_eq!(clear_bits_geq(0b1111_0010u8, 5u8), 0b0001_0010u8);
/// ```
#[inline]
pub fn clear_bits_geq<T: Word, U: UnsignedWord>(x: T, bit: U) -> T {
    debug_assert!(T::bit_size() > bit.to());
    x.bzhi(bit.to())
}

/// Method version of [`clear_bits_geq`](fn.clear_bits_geq.html).
pub trait ClearBitsGeq {
    #[inline]
    fn clear_bits_geq<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> ClearBitsGeq for T {
    #[inline]
    fn clear_bits_geq<U: UnsignedWord>(self, n: U) -> Self {
        clear_bits_geq(self, n)
    }
}
