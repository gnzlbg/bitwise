use word::Word;
use bitintr;

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
pub fn reverse_bits<T: Word>(x: T) -> T {
    bitintr::arm::v7::rbit(x)
}

/// Method version of [`reverse_bits`](fn.reverse_bits.html).
pub trait ReverseBits: Word {
    fn reverse_bits(self) -> Self;
}

impl<T: Word> ReverseBits for T {
    fn reverse_bits(self) -> T {
        reverse_bits(self)
    }
}
