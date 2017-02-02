use word::{Word, ToWord, UnsignedWord};

/// Shift the bits of `x` to the right by `n`; the high-order bits of the result
/// are set to the value of the most significant bit of `x`.
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
/// let a = 0b0111_0000u8;
/// let b = 0b1001_0000u8;
///
/// assert_eq!(a.shift_arithmetic_right(4u8), 0b0000_0111u8);
/// assert_eq!(shift_arithmetic_right(b, 4u8), 0b1111_1001u8);
/// b.shift_arithmetic_right(u8::bit_size() - 1);
///
/// ```
#[inline]
pub fn shift_arithmetic_right<T: Word, U: UnsignedWord>(x: T, n: U) -> T {
    debug_assert!(n <= T::bit_size().to());
    (x.to_signed() >> n.to()).to()

}

/// Method version of [`shift_arithmetic_right`](fn.shift_arithmetic_right.html).
pub trait SAR {
    #[inline]
    fn shift_arithmetic_right<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> SAR for T {
    #[inline]
    fn shift_arithmetic_right<U: UnsignedWord>(self, n: U) -> Self {
        shift_arithmetic_right(self, n)
    }
}
