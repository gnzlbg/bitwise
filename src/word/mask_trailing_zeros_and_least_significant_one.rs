use word::Word;
use bitintr;

/// Returns mask with all of the trailing 0's of `x` and the least
/// significant 1 bit set.
///
/// # Intrinsics:
/// - BMI: blsmsk.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0101_0000u8;
/// let s = 0b0001_1111u8;
///
/// assert_eq!(n.mask_trailing_zeros_and_least_significant_one(), s);
/// assert_eq!(mask_trailing_zeros_and_least_significant_one(n), s);
/// ```
#[inline]
pub fn mask_trailing_zeros_and_least_significant_one<T: Word>(x: T) -> T {
    bitintr::x86::bmi::blsmsk(x)
}

/// Method version of [`mask_trailing_zeros_and_least_significant_one`](fn.mask_trailing_zeros_and_least_significant_one.html).
pub trait MaskTrailingZerosAndLeastSignificantOne {
    #[inline]
    fn mask_trailing_zeros_and_least_significant_one(self) -> Self;
}

impl<T: Word> MaskTrailingZerosAndLeastSignificantOne for T {
    #[inline]
    fn mask_trailing_zeros_and_least_significant_one(self) -> Self {
        mask_trailing_zeros_and_least_significant_one(self)
    }
}
