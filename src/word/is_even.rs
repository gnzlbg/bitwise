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
