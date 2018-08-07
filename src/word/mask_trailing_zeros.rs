use word::Word;

/// Returns mask with the trailing 0's of `x` set.
///
/// If all bits of `x` are set, returns `0`.
///
/// # Intrinsics:
/// - TBM: tzmsk.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert_eq!(0b0110_0000u8.mask_trailing_zeros(), 0b0001_1111u8);
/// assert_eq!(mask_trailing_zeros(0b1111_1111u8), 0);
/// ```
#[inline]
pub fn mask_trailing_zeros<T: Word>(x: T) -> T {
    x.tzmsk()
}

/// Method version of [`mask_trailing_zeros`](fn.mask_trailing_zeros.html).
pub trait MaskTrailingZeros {
    #[inline]
    fn mask_trailing_zeros(self) -> Self;
}

impl<T: Word> MaskTrailingZeros for T {
    #[inline]
    fn mask_trailing_zeros(self) -> Self {
        mask_trailing_zeros(self)
    }
}
