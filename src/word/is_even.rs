use word::Word;

/// Is `x` even.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert!(0.is_even());
/// assert!(!1.is_even());
/// assert!(is_even(2));
/// assert!(!3.is_even());
/// assert!(4.is_even());
/// ```
#[inline]
pub fn is_even<T: Word>(x: T) -> bool {
    !x & T::one() == T::one()
}

/// Method version of [`is_even`](fn.is_even.html).
pub trait IsEven {
    #[inline]
    fn is_even(self) -> bool;
}

impl<T: Word> IsEven for T {
    #[inline]
    fn is_even(self) -> bool {
        is_even(self)
    }
}

#[cfg(test)]
mod tests {
    use word::*;
    use quickcheck::{TestResult, QuickCheck};

    macro_rules! prop_is_even_tests {
        ($($name:ident: $WordType:ty,)*) => {
            $(
                #[test]
                fn $name() {
                    fn inner(x: $WordType) -> TestResult {
                        let rem = (x % 2) == 0;
                        let res = x.is_even();
                        TestResult::from_bool(rem == res)
                    }
                    QuickCheck::new().quickcheck(inner as fn($WordType) -> TestResult);
                }
            )*
        }
    }

    prop_is_even_tests! {
        prop_is_even_u8: u8,
        prop_is_even_i8: i8,
        prop_is_even_u16: u16,
        prop_is_even_i16: i16,
        prop_is_even_u32: u32,
        prop_is_even_i32: i32,
        prop_is_even_u64: u64,
        prop_is_even_i64: i64,
    }
}
