use word::{Word, UnsignedWord, ToWord, IsPow2};

/// Align `x` up to `alignment`.
///
/// Returns `n`, where `n` is the least number >= `x`
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
/// assert_eq!(2.align_up(1u8), 2);
/// assert_eq!(align_up(2, 2u8), 2);
/// assert_eq!(2.align_up(4u8), 4);
/// assert_eq!(2.align_up(8u8), 8);
///
/// assert_eq!(3.align_up(1u8), 3);
/// assert_eq!(3.align_up(2u8), 4);
/// assert_eq!(3.align_up(4u8), 4);
/// assert_eq!(3.align_up(8u8), 8);
///
/// assert_eq!(4.align_up(1u8), 4);
/// assert_eq!(4.align_up(2u8), 4);
/// assert_eq!(4.align_up(4u8), 4);
/// assert_eq!(4.align_up(8u8), 8);
/// ```
#[inline]
pub fn align_up<T: Word, U: UnsignedWord>(x: T, alignment: U) -> T {
    debug_assert!(alignment.is_pow2());
    let x = x.to_unsigned();
    let a = alignment - U::one();
    ((x + a.to()) & (!a).to()).to()
}

/// Method version of [`align_up`](fn.align_up.html).
pub trait AlignUp {
    #[inline]
    fn align_up<U: UnsignedWord>(self, U) -> Self;
}

impl<T: Word> AlignUp for T {
    #[inline]
    fn align_up<U: UnsignedWord>(self, u: U) -> Self {
        align_up(self, u)
    }
}
