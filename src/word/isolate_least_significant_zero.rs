use word::Word;
use bitintr;

/// Returns mask with the least significant zero bit of `x` set to 1.
///
/// All other bits of the mask are set to zero. If `x` contains only set bits,
/// returns 0.
///
/// # Intrinsics:
/// - TBM: blcic (or: blci, not).
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0101;
/// let s = 0b0010;
///
/// assert_eq!(n.isolate_least_significant_zero(), s);
/// assert_eq!(isolate_least_significant_zero(0b1111_1111u8), 0u8);
/// ```
pub fn isolate_least_significant_zero<T: Word>(x: T) -> T {
    // the software fallback of `blcic` should generate the right code when tbm
    // is not available (TODO: check this, otherwise switch depending on target
    // features):
    bitintr::x86::tbm::blcic(x)
}

/// Method version of [`isolate_least_significant_zero`](fn.isolate_least_significant_zero.html).
pub trait IsolateLeastSignificantZero {
    fn isolate_least_significant_zero(self) -> Self;
}

impl<T: Word> IsolateLeastSignificantZero for T {
    fn isolate_least_significant_zero(self) -> Self {
        isolate_least_significant_zero(self)
    }
}
