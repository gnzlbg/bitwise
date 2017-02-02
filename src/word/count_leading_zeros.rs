use word::Word;
use bitintr;

/// Counts the number of leading zeros in the binary representation of `x`.
///
/// # Keywords:
///
/// Count leading zeros.
///
/// # Intrinsics:
/// - ABM: lzcnt.
/// - BMI 1.0: lzcnt.
/// - ARMv5: clz.
/// - PowerPC: cntlzd.
/// - gcc/llvm builtin: `x == 0 ? mem::size_of(x) * 8 : __builtin_clz(x)`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0010_1000u16;
///
/// assert_eq!(n.count_leading_zeros(), 10);
/// assert_eq!(count_leading_zeros(n), 10);
/// ```
#[inline]
pub fn count_leading_zeros<T: Word>(x: T) -> T {
    bitintr::x86::abm::lzcnt(x)
}

/// Method version of [`count_leading_zeros`](fn.count_leading_zeros.html).
pub trait CountLeadingZeros {
    #[inline]
    fn count_leading_zeros(self) -> Self;
}

impl<T: Word> CountLeadingZeros for T {
    #[inline]
    fn count_leading_zeros(self) -> Self {
        count_leading_zeros(self)
    }
}
