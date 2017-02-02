use word::{Word, ToWord, UnsignedWord};

/// Shift the bits of `x` to the left by `n`.
///
/// Same as [`shift_logical_left`](fn.shift_logical_left), provided for
/// symmetry.
///
/// # Panics
///
/// If `n > bit_size()`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let a = 0b0000_1010u8;
/// let b = 0b0000_1001u8;
///
/// assert_eq!(a.shift_arithmetic_left(4u8), 0b1010_0000u8);
/// assert_eq!(shift_arithmetic_left(b, 4u8), 0b1001_0000u8);
/// b.shift_arithmetic_left(u8::bit_size() - 1);
/// ```
#[inline]
pub fn shift_arithmetic_left<T: Word, U: UnsignedWord>(x: T, n: U) -> T {
    debug_assert!(n <= T::bit_size().to());
    (x.to_unsigned() << n.to()).to()
}

/// Method version of [`shift_arithmetic_left`](fn.shift_arithmetic_left.html).
pub trait SAL {
    #[inline]
    fn shift_arithmetic_left<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> SAL for T {
    #[inline]
    fn shift_arithmetic_left<U: UnsignedWord>(self, n: U) -> Self {
        shift_arithmetic_left(self, n)
    }
}
