use word::Word;

/// Returns mask with the trailing 1's of `self` set.
///
/// If `x` is zero, returns `0`.
///
/// # Intrinsics:
/// - TBM: t1mskc, not.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert_eq!(0b0101_1111u8.mask_trailing_ones(), 0b0001_1111u8);
/// assert_eq!(mask_trailing_ones(0), 0);
/// ```
#[inline]
pub fn mask_trailing_ones<T: Word>(x: T) -> T {
    !x.t1mskc()
}

/// Method version of [`mask_trailing_ones`](fn.mask_trailing_ones.html).
pub trait MaskTrailingOnes {
    #[inline]
    fn mask_trailing_ones(self) -> Self;
}

impl<T: Word> MaskTrailingOnes for T {
    #[inline]
    fn mask_trailing_ones(self) -> Self {
        mask_trailing_ones(self)
    }
}
