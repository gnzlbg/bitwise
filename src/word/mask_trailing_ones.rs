use word::Word;
use bitintr;

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
pub fn mask_trailing_ones<T: Word>(x: T) -> T {
    // the software fallback of `t1mskc` should generate the right code when tbm
    // is not available (TODO: check this, otherwise switch depending on target
    // features):
    !bitintr::x86::tbm::t1mskc(x)
}

/// Method version of [`mask_trailing_ones`](fn.mask_trailing_ones.html).
pub trait MaskTrailingOnes {
    fn mask_trailing_ones(self) -> Self;
}

impl<T: Word> MaskTrailingOnes for T {
    fn mask_trailing_ones(self) -> Self {
        mask_trailing_ones(self)
    }
}
