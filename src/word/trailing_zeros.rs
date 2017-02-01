use word::Word;

/// Number of trailing zeros in the binary representation of `x`.
///
/// # Keywords:
///
/// Count trailing zeros.
///
/// # Intrinsics:
/// - BMI 1.0: tzcnt.
/// - gcc/llvm builtin: `x == 0 ? mem::size_of(x) * 8 : __builtin_ctz(x)`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0010_1000u16;
///
/// assert_eq!(n.trailing_zeros(), 3);
/// assert_eq!(trailing_zeros(n), 3);
/// ```
pub fn trailing_zeros<T: Word>(x: T) -> T {
    T::trailing_zeros(x)
}

/// Method version of [`trailing_zeros`](fn.trailing_zeros.html).
pub trait TrailingZeros {
    fn trailing_zeros(self) -> Self;
}

impl<T: Word> TrailingZeros for T {
    fn trailing_zeros(self) -> Self {
        trailing_zeros(self)
    }
}
