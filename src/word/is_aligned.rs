use word::{Word, UnsignedWord, ToWord};

/// Is `x` aligned to `alignment` bytes.
///
/// Returns true if `x == 0` or `x` is a multiple of `alignment`, where
/// `alignment >= 1`.
///
/// # Panics
///
/// If `alignment < 1`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert!(2.is_aligned(1u8));
/// assert!(is_aligned(2, 2u8));
/// assert!(!2.is_aligned(4u8));
/// assert!(!2.is_aligned(8u8));
///
/// assert!(3.is_aligned(1u8));
/// assert!(!3.is_aligned(2u8));
/// assert!(!3.is_aligned(4u8));
/// assert!(!3.is_aligned(8u8));
///
/// assert!(4.is_aligned(1u8));
/// assert!(4.is_aligned(2u8));
/// assert!(4.is_aligned(4u8));
/// assert!(!4.is_aligned(8u8));
/// ```
#[inline]
pub fn is_aligned<T: Word, U: UnsignedWord>(x: T, alignment: U) -> bool {
    debug_assert!(U::one() <= alignment);
    (x & (alignment - U::one()).to()) == T::zero()
}

/// Method version of [`is_aligned`](fn.is_aligned.html).
pub trait IsAligned {
    #[inline]
    fn is_aligned<U: UnsignedWord>(self, U) -> bool;
}

impl<T: Word> IsAligned for T {
    #[inline]
    fn is_aligned<U: UnsignedWord>(self, u: U) -> bool {
        is_aligned(self, u)
    }
}
