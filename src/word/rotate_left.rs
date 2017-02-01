use word::{Word, ToWord, UnsignedWord};

/// Shifts the bits of `x` to the left by `n` wrapping the truncated bits to the
/// end of the result.
///
/// # Panics
///
/// If `n > bit_size()`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0x0123456789ABCDEFu64;
/// let m = 0x3456789ABCDEF012u64;
///
/// assert_eq!(n.rotate_left(12), m);
/// rotate_left(n, u64::bit_size());
/// ```
pub fn rotate_left<T: Word, U: UnsignedWord>(x: T, n: U) -> T {
    debug_assert!(n <= T::bit_size().to());
    T::rotate_left(x, n.to())
}

/// Method version of [`rotate_left`](fn.rotate_left.html).
pub trait RotateLeft {
    fn rotate_left<U: UnsignedWord>(self, n: U) -> Self;
}

impl<T: Word> RotateLeft for T {
    fn rotate_left<U: UnsignedWord>(self, n: U) -> Self {
        rotate_left(self, n)
    }
}
