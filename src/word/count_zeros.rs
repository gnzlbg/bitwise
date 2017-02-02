use word::Word;

/// Number of zeros in the binary representation of `x`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0100_1100u8;
///
/// assert_eq!(n.count_zeros(), 5);
/// assert_eq!(count_zeros(n), 5);
/// ```
#[inline]
pub fn count_zeros<T: Word>(x: T) -> T {
    T::count_zeros(x)
}

/// Method version of [`count_zeros`](fn.count_zeros.html).
pub trait CountZeros {
    #[inline]
    fn count_zeros(self) -> Self;
}

impl<T: Word> CountZeros for T {
    #[inline]
    fn count_zeros(self) -> Self {
        count_zeros(self)
    }
}
