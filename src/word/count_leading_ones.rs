use word::Word;

/// Count the number of leading ones in the binary representation of `x`.
///
/// # Keywords:
///
/// Count leading ones.
///
/// # Intrinsics:
/// - ARMv8: cls.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b1111_1111_1100_1000u16;
///
/// assert_eq!(n.count_leading_ones(), 10);
/// assert_eq!(count_leading_ones(n), 10);
/// ```
#[inline]
pub fn count_leading_ones<T: Word>(x: T) -> T {
    T::leading_zeros(!x) // TODO: use ARMv8 cls intrinsic
}

/// Method version of [`count_leading_ones`](fn.count_leading_ones.html).
pub trait CountLeadingOnes {
    #[inline]
    fn count_leading_ones(self) -> Self;
}

impl<T: Word> CountLeadingOnes for T {
    #[inline]
    fn count_leading_ones(self) -> Self {
        count_leading_ones(self)
    }
}
