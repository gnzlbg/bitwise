use word::Word;
use bitintr;

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
pub fn mask_trailing_zeros<T: Word>(x: T) -> T {
    // the software fallback of `tzmsk` should generate the right code when tbm
    // is not available (TODO: check this, otherwise switch depending on target
    // features):
    bitintr::x86::tbm::tzmsk(x)
}

/// Method version of [`mask_trailing_zeros`](fn.mask_trailing_zeros.html).
pub trait MaskTrailingZeros {
    fn mask_trailing_zeros(self) -> Self;
}

impl<T: Word> MaskTrailingZeros for T {
    fn mask_trailing_zeros(self) -> Self {
        mask_trailing_zeros(self)
    }
}
