use word::Word;

/// Number of ones in the binary representation of `x`.
///
/// # Keywords:
///
/// Population count, popcount, hamming weight, sideways sum.
///
/// # Intrinsics:
/// - ABM: popcnt.
/// - SSE4.2: popcnt.
/// - NEON: vcnt.
/// - PowerPC: popcntb.
/// - gcc/llvm builtin: `__builtin_popcount(x)`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0100_1100u8;
///
/// assert_eq!(n.count_ones(), 3);
/// assert_eq!(count_ones(n), 3);
/// ```
#[inline]
pub fn count_ones<T: Word>(x: T) -> T {
    x.popcnt()
}

/// Method version of [`count_ones`](fn.count_ones.html).
pub trait CountOnes {
    #[inline]
    fn count_ones(self) -> Self;
}

impl<T: Word> CountOnes for T {
    #[inline]
    fn count_ones(self) -> Self {
        count_ones(self)
    }
}
