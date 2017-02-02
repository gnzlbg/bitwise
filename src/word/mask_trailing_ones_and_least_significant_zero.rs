use word::Word;
use bitintr;

/// Returns mask with all trailing 1's of `x` and the least
/// significant 0 bit set.
///
/// # Intrinsics:
/// - TBM: blcmsk.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0101_1111u8;
/// let s = 0b0011_1111u8;
///
/// assert_eq!(n.mask_trailing_ones_and_least_significant_zero(), s);
/// assert_eq!(mask_trailing_ones_and_least_significant_zero(n), s);
/// ```
#[inline]
pub fn mask_trailing_ones_and_least_significant_zero<T: Word>(x: T) -> T {
    bitintr::x86::tbm::blcmsk(x)
}

/// Method version of [`mask_trailing_zeros_and_least_significant_zero`](fn.mask_trailing_zeros_and_least_significant_zero.html).
pub trait MaskTrailingOnesAndLeastSignificantZero {
    #[inline]
    fn mask_trailing_ones_and_least_significant_zero(self) -> Self;
}

impl<T: Word> MaskTrailingOnesAndLeastSignificantZero for T {
    #[inline]
    fn mask_trailing_ones_and_least_significant_zero(self) -> Self {
        mask_trailing_ones_and_least_significant_zero(self)
    }
}
