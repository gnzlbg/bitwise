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
