use word::{Word, ToWord};

/// Shift the bits to the left by a specified amount, `n`.
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
/// let a = 0b0000_1010u8;
/// let b = 0b0000_1001u8;
///
/// assert_eq!(shift_logical_left(a, 4), 0b1010_0000u8);
/// assert_eq!(b.shift_logical_left(4), 0b1001_0000u8);
///
/// ```
pub fn shift_logical_left<T: Word, U: Word>(x: T, n: U) -> T {
    debug_assert!(n <= T::bit_size().to());
    (x.to_unsigned() << n.to_unsigned().to()).to()
}

/// Method version of [`shift_logical_left`](fn.shift_logical_left.html).
pub trait SLL {
    fn shift_logical_left<U: Word>(self, n: U) -> Self;
}

impl<T: Word> SLL for T {
    fn shift_logical_left<U: Word>(self, n: U) -> Self {
        shift_logical_left(self, n)
    }
}
