use word::{Word, ToWord};

/// Extract bits [`start`, `start + length`) from `x` into the lower bits
/// of the result.
///
/// # Keywords:
///
/// Gather bit range.
///
/// # Intrinsics:
/// - BMI 1.0: bextr.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n  = 0b1011_1110_1001_0011u16;
///
/// assert_eq!(n.extract_bits(1u8, 4u8), 0b1001);
/// ```
#[inline]
pub fn extract_bits<T: Word, U: Word>(x: T, start: U, length: U) -> T {
    x.bextr(start.to(), length.to())
}

/// Method version of [`extract_bits`](fn.extract_bits.html).
pub trait ExtractBits {
    #[inline]
    fn extract_bits<U: Word>(self, U, U) -> Self;
}

impl<T: Word> ExtractBits for T {
    #[inline]
    fn extract_bits<U: Word>(self, start: U, length: U) -> Self {
        extract_bits(self, start, length)
    }
}
