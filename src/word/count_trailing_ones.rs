use word::Word;

/// Number of trailing ones in the binary representation of `x`.
///
/// # Keywords:
///
/// Count trailing ones.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0010_0111u16;
///
/// assert_eq!(n.count_trailing_ones(), 3);
/// assert_eq!(count_trailing_ones(n), 3);
/// ```
#[inline]
pub fn count_trailing_ones<T: Word>(x: T) -> T {
    T::trailing_zeros(!x)
}

/// Method version of [`count_trailing_ones`](fn.count_trailing_ones.html).
pub trait CountTrailingOnes {
    #[inline]
    fn count_trailing_ones(self) -> Self;
}

impl<T: Word> CountTrailingOnes for T {
    #[inline]
    fn count_trailing_ones(self) -> Self {
        count_trailing_ones(self)
    }
}
