use word::{Word, ToWord, UnsignedWord};

/// Shifts the bits of `x` to the right by `n` wrapping the truncated bits to
/// the beginning of the result.
///
/// # Panics
///
/// If `n > bit_size()`.
///
/// # Intrinsics:
/// - BMI 2.0: rorx.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0x0123456789ABCDEFu64;
/// let m = 0xDEF0123456789ABCu64;
///
/// assert_eq!(n.rotate_right(12), m);
/// rotate_right(n, u64::bit_size());
/// ```
#[inline]
pub fn rotate_right<T: Word, U: UnsignedWord>(x: T, n: U) -> T {
    debug_assert!(n <= T::bit_size().to());
    T::rotate_right(x, n.to())
}

/// Method version of [`rotate_right`](fn.rotate_right.html).
pub trait RotateRight {
    #[inline]
    fn rotate_right<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> RotateRight for T {
    #[inline]
    fn rotate_right<U: UnsignedWord>(self, n: U) -> Self {
        rotate_right(self, n)
    }
}
