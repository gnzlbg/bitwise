use word::Word;

/// Number of leading zeros in the binary representation of `x`.
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
/// assert_eq!(n.leading_zeros(), 10);
/// assert_eq!(leading_zeros(n), 10);
/// ```
pub fn leading_zeros<T: Word>(x: T) -> T {
    T::leading_zeros(x)
}

/// Method version of [`leading_zeros`](fn.leading_zeros.html).
pub trait LeadingZeros {
    fn leading_zeros(self) -> Self;
}

impl<T: Word> LeadingZeros for T {
    fn leading_zeros(self) -> Self {
        leading_zeros(self)
    }
}
