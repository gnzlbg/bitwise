use word::Word;

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
#[inline]
pub fn isolate_least_significant_one<T: Word>(x: T) -> T {
    x.blsi()
}

/// Method version of [`isolate_least_significant_one`](fn.isolate_least_significant_one.html).
pub trait IsolateLeastSignificantOne {
    #[inline]
    fn isolate_least_significant_one(self) -> Self;
}

impl<T: Word> IsolateLeastSignificantOne for T {
    #[inline]
    fn isolate_least_significant_one(self) -> Self {
        isolate_least_significant_one(self)
    }
}
