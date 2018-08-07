use word::Word;

/// Count the number of trailing zeros in the binary representation of `x`.
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
/// assert_eq!(n.count_trailing_zeros(), 3);
/// assert_eq!(count_trailing_zeros(n), 3);
/// ```
#[inline]
pub fn count_trailing_zeros<T: Word>(x: T) -> T {
    x.tzcnt()
}

/// Method version of [`count_trailing_zeros`](fn.count_trailing_zeros.html).

pub trait CountTrailingZeros {
    #[inline]
    fn count_trailing_zeros(self) -> Self;
}

impl<T: Word> CountTrailingZeros for T {
    #[inline]
    fn count_trailing_zeros(self) -> Self {
        count_trailing_zeros(self)
    }
}
