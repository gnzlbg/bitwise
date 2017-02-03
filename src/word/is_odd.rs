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
