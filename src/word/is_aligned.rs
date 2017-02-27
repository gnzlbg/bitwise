use word::{Word, UnsignedWord, ToWord};

/// Is `x` aligned to `alignment` bytes.
///
/// Returns true if `x == 0` or `x` is a multiple of `alignment`, where
/// `alignment >= 1`.
///
/// # Panics
///
/// If `alignment < 1`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert!(2.is_aligned(1u8));
/// assert!(is_aligned(2, 2u8));
/// assert!(!2.is_aligned(4u8));
/// assert!(!2.is_aligned(8u8));
///
/// assert!(3.is_aligned(1u8));
/// assert!(!3.is_aligned(2u8));
/// assert!(!3.is_aligned(4u8));
/// assert!(!3.is_aligned(8u8));
///
/// assert!(4.is_aligned(1u8));
/// assert!(4.is_aligned(2u8));
/// assert!(4.is_aligned(4u8));
/// assert!(!4.is_aligned(8u8));
/// ```
#[inline]
pub fn is_aligned<T: Word, U: UnsignedWord>(x: T, alignment: U) -> bool {
    debug_assert!(U::one() <= alignment);
    (x & (alignment - U::one()).to()) == T::zero()
}

/// Method version of [`is_aligned`](fn.is_aligned.html).
pub trait IsAligned {
    #[inline]
    fn is_aligned<U: UnsignedWord>(self, U) -> bool;
}

impl<T: Word> IsAligned for T {
    #[inline]
    fn is_aligned<U: UnsignedWord>(self, u: U) -> bool {
        is_aligned(self, u)
    }
}

#[cfg(test)]
mod tests {
    use word::*;
    use quickcheck::{TestResult, QuickCheck};

    macro_rules! prop_is_aligned_tests {
        ($($name:ident: ($WordType:ty, $UnsignedType:ty),)*) => {
            $(
                #[test]
                fn $name() {
                    fn inner(x: $WordType, alignment: $UnsignedType) -> TestResult {
                        if alignment < 1 {
                            return TestResult::discard();
                        }
                        if !alignment.is_pow2() {
                            return TestResult::discard();
                        }

                        let res = x.is_aligned(alignment);

                        if x == 0 || (x % alignment) == 0 { // zero or multiple
                            TestResult::from_bool(res)
                        } else {
                            TestResult::from_bool(!res)
                        }
                    }
                    QuickCheck::new().quickcheck(inner as fn($WordType, $UnsignedType) -> TestResult);
                }
            )*
        }
    }

    prop_is_aligned_tests! {
        prop_is_aligned_u8_u8: (u8, u8),
        prop_is_aligned_u16_u16: (u16, u16),
        prop_is_aligned_u32_u32: (u32, u32),
        prop_is_aligned_u64_u64: (u64, u64),
        prop_is_aligned_usize_usize: (usize, usize),
    }
}
