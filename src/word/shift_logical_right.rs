use word::{Word, ToWord};

/// Shift the bits of `x` to the right `n`; the
/// high-order bits of the result are cleared.
///
/// # Panics
///
/// If `n > bit_size()`.
///
/// # Examples
///
///
/// ```
/// use bitwise::word::*;
///
/// let a = 0b0111_0000u8;
/// let b = 0b1001_0000u8;
///
/// assert_eq!(a.shift_logical_right(4), 0b0000_0111u8);
/// assert_eq!(shift_logical_right(b, 4), 0b0000_1001u8);
/// b.shift_logical_right((u8::bit_size() - 1));
/// ```
pub fn shift_logical_right<T: Word, U: Word>(x: T, n: U) -> T {
    debug_assert!(n <= T::bit_size().to());
    (x.to_unsigned() >> n.to_unsigned().to()).to()
}

/// Method version of [`shift_logical_right`](fn.shift_logical_right.html).
pub trait SLR {
    fn shift_logical_right<U: Word>(self, n: U) -> Self;
}

impl<T: Word> SLR for T {
    fn shift_logical_right<U: Word>(self, n: U) -> Self {
        shift_logical_right(self, n)
    }
}
