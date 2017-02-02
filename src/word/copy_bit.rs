use word::{Word, ToWord, UnsignedWord, test_bit, set_bit, clear_bit};

/// Copys the `from_bit` of `x` into `y` at `to_bit`.
///
/// # Panics
///
/// If `bit >= bit_size() or pos >= bit_size()`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let a  = 0b1011_0010u8;
/// let b  = 0b1001_0010u8;
/// let c  = 0b1111_0010u8;
/// assert_eq!(a.copy_bit(5u8, b, 5u8), a);
/// assert_eq!(copy_bit(a, 6u8, c, 6u8), a);
/// ```
#[inline]
pub fn copy_bit<T: Word, U: UnsignedWord>(x: T, from_bit: U, y: T, to_bit: U) -> T {
    debug_assert!(T::bit_size() > from_bit.to());
    debug_assert!(T::bit_size() > to_bit.to());
    if test_bit(x, from_bit) {
        set_bit(y, to_bit)
    } else {
        clear_bit(y, to_bit)
    }
}

/// Method version of [`copy_bit`](fn.copy_bit.html).
pub trait CopyBit {
    #[inline]
    fn copy_bit<U: UnsignedWord>(self, f: U, Self, t: U) -> Self;
}

impl<T: Word> CopyBit for T {
    #[inline]
    fn copy_bit<U: UnsignedWord>(self, f: U, y: Self, t: U) -> Self {
        copy_bit(self, f, y, t)
    }
}
