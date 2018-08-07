use word::Word;

/// Clear the trailing bits set of `x`.
///
/// If `x` is zero, returns `0`.
///
/// # Intrinsics:
/// - TBM: blcfill.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0110_1111;
/// let s = 0b0110_0000;
///
/// assert_eq!(n.clear_trailing_ones(), s);
/// assert_eq!(clear_trailing_ones(0), 0);
/// ```
#[inline]
pub fn clear_trailing_ones<T: Word>(x: T) -> T {
    x.blcfill()
}

/// Method version of [`clear_trailing_ones`](fn.clear_trailing_ones.html).
pub trait ClearTrailingOnes {
    #[inline]
    fn clear_trailing_ones(self) -> Self;
}

impl<T: Word> ClearTrailingOnes for T {
    #[inline]
    fn clear_trailing_ones(self) -> Self {
        clear_trailing_ones(self)
    }
}
