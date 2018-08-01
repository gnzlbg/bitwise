use word::Word;

/// Reverses the bits of `x`.
///
/// # Intrinsics:
/// - ARM: rbit (u32 ARMv7, u64 ARMv8).
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert_eq!(0b1011_0010u8.reverse_bits(), 0b0100_1101u8);
/// assert_eq!(reverse_bits(0b1011_0010_1010_1001u16), 0b1001_0101_0100_1101u16);
/// ```
#[inline]
pub fn reverse_bits<T: Word>(x: T) -> T {
    x.rbit()
}

/// Method version of [`reverse_bits`](fn.reverse_bits.html).
pub trait ReverseBits: Word {
    #[inline]
    fn reverse_bits(self) -> Self;
}

impl<T: Word> ReverseBits for T {
    #[inline]
    fn reverse_bits(self) -> T {
        reverse_bits(self)
    }
}
