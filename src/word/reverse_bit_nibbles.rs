use word::Word;
use word::reverse_bit_groups::*;

/// Reverses the nibbles of `x`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert_eq!(0b1011_0010u8.reverse_bit_nibbles(), 0b0010_1011u8);
/// assert_eq!(reverse_bit_nibbles(0b1011_0010_1010_1001u16), 0b1001_1010_0010_1011u16);
/// ```
#[inline]
pub fn reverse_bit_nibbles<T: Word>(x: T) -> T {
    reverse_bit_groups(x, 4u8, 1u8)
}

/// Method version of [`reverse_bit_nibbles`](fn.reverse_bit_nibbles.html).
pub trait ReverseBitNibbles: Word {
    #[inline]
    fn reverse_bit_nibbles(self) -> Self;
}

impl<T: Word> ReverseBitNibbles for T {
    #[inline]
    fn reverse_bit_nibbles(self) -> T {
        reverse_bit_nibbles(self)
    }
}
