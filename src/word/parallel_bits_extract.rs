use word::{Word, ToWord};
use bitintr;

/// Parallel bits extract of `mask` from `x`.
///
/// # Keywords:
///
/// Gather.
///
/// # Intrinsics:
/// - BMI 2.0: pext.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n  = 0b1011_1110_1001_0011u16;
///
/// let m0 = 0b0110_0011_1000_0101u16;
/// let s0 = 0b0000_0000_0011_0101u16;
///
/// let m1 = 0b1110_1011_1110_1111u16;
/// let s1 = 0b0001_0111_0100_0011u16;
///
/// assert_eq!(n.parallel_bits_extract(m0), s0);
/// assert_eq!(parallel_bits_extract(n, m1), s1);
/// ```
#[inline]
pub fn parallel_bits_extract<T: Word, U: Word>(x: T, mask: U) -> T {
    bitintr::x86::bmi2::pext(x, mask.to())
}

/// Method version of [`parallel_bits_extract`](fn.parallel_bits_extract.html).
pub trait ParallelBitsExtract {
    #[inline]
    fn parallel_bits_extract<U: Word>(self, U) -> Self;
}

impl<T: Word> ParallelBitsExtract for T {
    #[inline]
    fn parallel_bits_extract<U: Word>(self, mask: U) -> Self {
        parallel_bits_extract(self, mask)
    }
}
