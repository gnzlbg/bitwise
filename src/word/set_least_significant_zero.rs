use word::Word;
use bitintr::x86::tbm;

/// Set least significant 0 bit of `x`.
///
/// # Intrinsics:
/// - TBM: blcs.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0101;
/// let s = 0b0111;
///
/// assert_eq!(n.set_least_significant_zero(), s);
/// assert_eq!(set_least_significant_zero(n), s);
/// ```
pub fn set_least_significant_zero<T: Word>(x: T) -> T {
    tbm::blcs(x)
}

/// Method version of [`set_least_significant_zero`](fn.set_least_significant_zero.html).
pub trait SetLeastSignificantZero {
    fn set_least_significant_zero(self) -> Self;
}

impl<T: Word> SetLeastSignificantZero for T {
    fn set_least_significant_zero(self) -> Self {
        set_least_significant_zero(self)
    }
}
