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
/// assert_eq!(n.trailing_ones(), 3);
/// assert_eq!(trailing_ones(n), 3);
/// ```
pub fn trailing_ones<T: Word>(x: T) -> T {
    T::trailing_zeros(!x)
}

/// Method version of [`trailing_ones`](fn.trailing_ones.html).
pub trait TrailingOnes {
    fn trailing_ones(self) -> Self;
}

impl<T: Word> TrailingOnes for T {
    fn trailing_ones(self) -> Self {
        trailing_ones(self)
    }
}
