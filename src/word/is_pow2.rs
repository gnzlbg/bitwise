use word::Word;

/// Is `x` a power of 2.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert!(!0.is_pow2());
/// assert!(2.is_pow2());
/// assert!(!is_pow2(3));
/// assert!(4.is_pow2());
/// assert!(!5.is_pow2());
/// assert!(!6.is_pow2());
/// assert!(!7.is_pow2());
/// assert!(8.is_pow2());
/// ```
#[inline]
pub fn is_pow2<T: Word>(x: T) -> bool {
    x > T::zero() && ((x & (x - T::one())) == T::zero())
}

/// Method version of [`is_pow2`](fn.is_pow2.html).
pub trait IsPow2 {
    #[inline]
    fn is_pow2(self) -> bool;
}

impl<T: Word> IsPow2 for T {
    #[inline]
    fn is_pow2(self) -> bool {
        is_pow2(self)
    }
}

#[cfg(test)]
mod tests {
    use word::*;
    use quickcheck::{TestResult, QuickCheck};

    macro_rules! prop_is_pow2_tests {
        ($($name:ident: $WordType:ty,)*) => {
            $(
                #[test]
                fn $name() {
                    fn inner(x: $WordType) -> TestResult {
                        if x <= 0 {
                            return TestResult::discard();
                        }

                        // Determine if y is a power of two in the
                        // keep-dividing-until-we-hit-the-floor method.
                        let mut y = x;
                        while ((y % 2) == 0) && y > 1 {
                            y /= 2;
                        }

                        let rem = y == 1;
                        let res = x.is_pow2();
                        TestResult::from_bool(rem == res)
                    }
                    QuickCheck::new().quickcheck(inner as fn($WordType) -> TestResult);
                }
            )*
        }
    }

    prop_is_pow2_tests! {
        prop_is_pow2_u8: u8,
        prop_is_pow2_i8: i8,
        prop_is_pow2_u16: u16,
        prop_is_pow2_i16: i16,
        prop_is_pow2_u32: u32,
        prop_is_pow2_i32: i32,
        prop_is_pow2_u64: u64,
        prop_is_pow2_i64: i64,
        prop_is_pow2_usize: usize,
        prop_is_pow2_isize: isize,
    }
}
