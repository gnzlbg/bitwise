use word::Word;
use word::reverse_bit_groups::*;

/// Reverses the pairs of bits of `x`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert_eq!(0b1011_0010u8.reverse_bit_pairs(), 0b1000_1110u8);
/// assert_eq!(reverse_bit_pairs(0b1011_0010_1010_1001u16), 0b0110_1010_1000_1110u16);
/// ```
#[inline]
pub fn reverse_bit_pairs<T: Word>(x: T) -> T {
    reverse_bit_groups(x, 2u8, 1u8)
}

/// Method version of [`reverse_bit_pairs`](fn.reverse_bit_pairs.html).
pub trait ReverseBitPairs: Word {
    #[inline]
    fn reverse_bit_pairs(self) -> Self;
}

impl<T: Word> ReverseBitPairs for T {
    #[inline]
    fn reverse_bit_pairs(self) -> T {
        reverse_bit_pairs(self)
    }
}
