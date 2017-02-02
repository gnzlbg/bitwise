use word::Word;
use bitintr;

/// Set the trailing 0's of `x`.
///
/// If all bits of `x` are set, returns `x`.
///
/// # Intrinsics:
/// - TBM: blsfill.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0110_0000u8;
/// let s = 0b0111_1111u8;
///
/// assert_eq!(n.set_trailing_zeros(), s);
/// assert_eq!(set_trailing_zeros(0b1111_1111u8), 0b1111_1111u8);
/// ```
pub fn set_trailing_zeros<T: Word>(x: T) -> T {
    bitintr::x86::tbm::blsfill(x)
}

/// Method version of [`set_trailing_zeros`](fn.set_trailing_zeros.html).
pub trait SetTrailingZeros {
    fn set_trailing_zeros(self) -> Self;
}

impl<T: Word> SetTrailingZeros for T {
    fn set_trailing_zeros(self) -> Self {
        set_trailing_zeros(self)
    }
}
