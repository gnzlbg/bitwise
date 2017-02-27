use word::{Word};

/// Is `x` odd.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert!(!0.is_odd());
/// assert!(1.is_odd());
/// assert!(!is_odd(2));
/// assert!(3.is_odd());
/// assert!(!4.is_odd());
/// ```
#[inline]
pub fn is_odd<T: Word>(x: T) -> bool {
    x & T::one() == T::one()
}

/// Method version of [`is_odd`](fn.is_odd.html).
pub trait IsOdd {
    #[inline]
    fn is_odd(self) -> bool;
}

impl<T: Word> IsOdd for T {
    #[inline]
    fn is_odd(self) -> bool {
        is_odd(self)
    }
}

#[cfg(test)]
mod tests {
    use word::*;
    use quickcheck::{TestResult, QuickCheck};

    macro_rules! prop_is_odd_tests {
        ($($name:ident: $WordType:ty,)*) => {
            $(
                #[test]
                fn $name() {
                    fn inner(x: $WordType) -> TestResult {
                        let rem = (x % 2) != 0;
                        let res = x.is_odd();
                        TestResult::from_bool(rem == res)
                    }
                    QuickCheck::new().quickcheck(inner as fn($WordType) -> TestResult);
                }
            )*
        }
    }

    prop_is_odd_tests! {
        prop_is_odd_u8: u8,
        prop_is_odd_i8: i8,
        prop_is_odd_u16: u16,
        prop_is_odd_i16: i16,
        prop_is_odd_u32: u32,
        prop_is_odd_i32: i32,
        prop_is_odd_u64: u64,
        prop_is_odd_i64: i64,
        prop_is_odd_usize: usize,
        prop_is_odd_isize: isize,
    }
}
