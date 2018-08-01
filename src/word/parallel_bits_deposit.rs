use word::{Word, ToWord};

/// Parallel bits deposit of `mask` into `x`.
///
/// # Keywords:
///
/// Scatter.
///
/// # Intrinsics:
/// - BMI 2.0: pdep.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n  = 0b1011_1110_1001_0011u16;
///
/// let m0 = 0b0110_0011_1000_0101u16;
/// let s0 = 0b0000_0010_0000_0101u16;
///
/// let m1 = 0b1110_1011_1110_1111u16;
/// let s1 = 0b1110_1001_0010_0011u16;
///
/// assert_eq!(n.parallel_bits_deposit(m0), s0);
/// assert_eq!(parallel_bits_deposit(n, m1), s1);
/// ```
#[inline]
pub fn parallel_bits_deposit<T: Word, U: Word>(x: T, mask: U) -> T {
    x.pdep(mask.to())
}

/// Method version of [`parallel_bits_deposit`](fn.parallel_bits_deposit.html).
pub trait ParallelBitsDeposit {
    #[inline]
    fn parallel_bits_deposit<U: Word>(self, U) -> Self;
}

impl<T: Word> ParallelBitsDeposit for T {
    #[inline]
    fn parallel_bits_deposit<U: Word>(self, mask: U) -> Self {
        parallel_bits_deposit(self, mask)
    }
}
