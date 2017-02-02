use word::Word;

/// Round `x` to the previous power of 2.
///
/// # Panics
///
/// If `x <= 0`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert_eq!(2.floor_pow2(), 2);
/// assert_eq!(floor_pow2(3), 2);
/// assert_eq!(4.floor_pow2(), 4);
/// assert_eq!(5.floor_pow2(), 4);
/// assert_eq!(6.floor_pow2(), 4);
/// assert_eq!(7.floor_pow2(), 4);
/// assert_eq!(8.floor_pow2(), 8);
/// ```
#[inline]
pub fn floor_pow2<T: Word>(x: T) -> T {
    debug_assert!(x > T::zero());
    let mut x = x;
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
    x - (x >> T::one())
}

/// Method version of [`floor_pow2`](fn.floor_pow2.html).
pub trait FloorPow2 {
    #[inline]
    fn floor_pow2(self) -> Self;
}

impl<T: Word> FloorPow2 for T {
    #[inline]
    fn floor_pow2(self) -> Self {
        floor_pow2(self)
    }
}
