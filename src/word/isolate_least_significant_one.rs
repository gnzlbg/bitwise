use word::Word;
use bitintr;

/// Returns mask with the least significant set bit of `x` set to 1.
///
/// If `x` is 0 returns 0.
///
/// # Intrinsics:
/// - BMI 1.0: blsi.
/// - TBM: blsic, not.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0110;
/// let s = 0b0010;
///
/// assert_eq!(n.isolate_least_significant_one(), s);
/// assert_eq!(isolate_least_significant_one(0), 0);
/// ```
pub fn isolate_least_significant_one<T: Word>(x: T) -> T {
    // the software fallback of `blsi` should generate the right code when bmi
    // is not available (TODO: check this, otherwise switch depending on target
    // features):
    bitintr::x86::bmi::blsi(x)
}

/// Method version of [`isolate_least_significant_one`](fn.isolate_least_significant_one.html).
pub trait IsolateLeastSignificantOne {
    fn isolate_least_significant_one(self) -> Self;
}

impl<T: Word> IsolateLeastSignificantOne for T {
    fn isolate_least_significant_one(self) -> Self {
        isolate_least_significant_one(self)
    }
}
