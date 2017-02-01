use word::Word;

/// Number of leading ones in the binary representation of `x`.
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
/// assert_eq!(n.leading_ones(), 10);
/// assert_eq!(leading_ones(n), 10);
/// ```
pub fn leading_ones<T: Word>(x: T) -> T {
    T::leading_zeros(!x) // TODO: use ARMv8 cls intrinsic
}

/// Method version of [`leading_ones`](fn.leading_ones.html).
pub trait LeadingOnes {
    fn leading_ones(self) -> Self;
}

impl<T: Word> LeadingOnes for T {
    fn leading_ones(self) -> Self {
        leading_ones(self)
    }
}
