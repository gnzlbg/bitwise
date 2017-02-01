use word::Word;

/// Round `x` to the next power of 2.
///
/// # Panics
///
/// If the next power of 2 cannot be represented by `T`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert_eq!(2.ceil_pow2(), 2);
/// assert_eq!(ceil_pow2(3), 4);
/// assert_eq!(4.ceil_pow2(), 4);
/// assert_eq!(5.ceil_pow2(), 8);
/// assert_eq!(6.ceil_pow2(), 8);
/// assert_eq!(7.ceil_pow2(), 8);
/// assert_eq!(8.ceil_pow2(), 8);
/// assert_eq!(2u32.pow(30).ceil_pow2(), 2u32.pow(30));
/// assert_eq!((2u32.pow(30) + 1).ceil_pow2(), 2u32.pow(31));
/// assert_eq!(2u32.pow(31).ceil_pow2(), 2u32.pow(31));
/// // panics:
/// // assert_eq!((2u32.pow(31) + 1).ceil_pow2(), 2u32.pow(32));
/// ```
pub fn ceil_pow2<T: Word>(x: T) -> T {
    let mut x = x - T::one();
    let s = T::byte_size();
    x = x | (x >> T::one());
    x = x | (x >> T::from_u8(2));
    x = x | (x >> T::from_u8(4));
    if s > T::one() {
        x = x | (x >> T::from_u8(8));
        if s > T::from_u8(2) {
            x = x | (x >> T::from_u8(16));
            if s > T::from_u8(4) {
                x = x | (x >> T::from_u8(32));
            }
        }
    }
    x + T::one()
}

/// Method version of [`ceil_pow2`](fn.ceil_pow2.html).
pub trait CeilPow2 {
    fn ceil_pow2(self) -> Self;
}

impl<T: Word> CeilPow2 for T {
    fn ceil_pow2(self) -> Self {
        ceil_pow2(self)
    }
}
