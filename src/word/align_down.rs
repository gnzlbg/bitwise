use word::{Word, UnsignedWord, ToWord, IsPow2};

/// Align `x` down to `alignment`.
///
/// Returns `n`, where `n` is the greatest number <= `x`
/// and `is_aligned(n, alignment)`.
///
/// # Panics
///
/// `alignment` must be a power of two.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert_eq!(2.align_down(1u8), 2);
/// assert_eq!(align_down(2, 2u8), 2);
/// assert_eq!(2.align_down(4u8), 0);
/// assert_eq!(2.align_down(8u8), 0);
///
/// assert_eq!(3.align_down(1u8), 3);
/// assert_eq!(3.align_down(2u8), 2);
/// assert_eq!(3.align_down(4u8), 0);
/// assert_eq!(3.align_down(8u8), 0);
///
/// assert_eq!(4.align_down(1u8), 4);
/// assert_eq!(4.align_down(2u8), 4);
/// assert_eq!(4.align_down(4u8), 4);
/// assert_eq!(4.align_down(8u8), 0);
/// ```
#[inline]
pub fn align_down<T: Word, U: UnsignedWord>(x: T, alignment: U) -> T {
    debug_assert!(alignment.is_pow2());
    (x & (!(alignment - U::one())).to()).to()
}

/// Method version of [`align_down`](fn.align_down.html).
pub trait AlignDown {
    #[inline]
    fn align_down<U: UnsignedWord>(self, U) -> Self;
}

impl<T: Word> AlignDown for T {
    #[inline]
    fn align_down<U: UnsignedWord>(self, u: U) -> Self {
        align_down(self, u)
    }
}
