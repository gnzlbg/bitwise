use word::Word;

/// Number of set bits in `x` mod 2; i.e. 1 if the
/// number of set bits in `x` is odd, zero otherwise
///
/// # Intrinsics:
/// -gcc/llvm: `__builtin_parity(x)`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n0 = 0b0000;  // 0 -> even => parity = 0
/// let n1 = 0b0001;  // 1 -> odd  => partiy = 1
/// let n2 = 0b0010;  // 1 -> odd  => parity = 1
/// let n3 = 0b0011;  // 2 -> even => parity = 0
///
/// assert_eq!(n0.parity(), 0);
/// assert_eq!(n1.parity(), 1);
/// assert_eq!(n2.parity(), 1);
/// assert_eq!(n3.parity(), 0);
/// assert_eq!(parity(n2), 1);
/// ```
#[inline]
pub fn parity<T: Word>(x: T) -> T {
    x.count_ones() & T::one()
}

/// Method version of [`parity`](fn.parity.html).
pub trait Parity {
    #[inline]
    fn parity(self) -> Self;
}

impl<T: Word> Parity for T {
    #[inline]
    fn parity(self) -> Self {
        parity(self)
    }
}
