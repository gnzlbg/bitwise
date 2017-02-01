use word::Word;
use bitintr::x86::abm;

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
pub fn count_ones<T: Word>(x: T) -> T {
    abm::popcnt(x)
}

/// Method version of [`count_ones`](fn.count_ones.html).
pub trait CountOnes {
    fn count_ones(self) -> Self;
}

impl<T: Word> CountOnes for T {
    fn count_ones(self) -> Self {
        count_ones(self)
    }
}
