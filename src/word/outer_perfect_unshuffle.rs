use word::Word;

/// Outer Perfect Unshuffle of `x`.
///
/// See also:
/// [Hacker's Delight: shuffling bits](http://icodeguru.com/Embedded/Hacker's-Delight/047.htm).
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0111_1101_0110_0011_1011_0110_1000_1111u32;
/// //        aAbB cCdD eEfF gGhH iIjJ kKlL mMnN oOpP,
/// let s = 0b0110_0101_1101_1011_1111_1001_0110_0011u32;
/// //        abcd efgh ijkl mnop ABCD EFGH IJKL MNOP,
///
/// assert_eq!(n.outer_perfect_unshuffle(), s);
/// assert_eq!(outer_perfect_unshuffle(n), s);
/// ```
#[inline]
pub fn outer_perfect_unshuffle<T: Word>(x: T) -> T {
    let mut x = x;
    let s = T::byte_size();
    let t = (x ^ (x >> T::one())) & T::from_u64(0x2222222222222222u64);
    x = x ^ t ^ (t << T::one());
    let t = (x ^ (x >> T::from_u8(2))) & T::from_u64(0x0C0C0C0C0C0C0C0Cu64);
    x = x ^ t ^ (t << T::from_u8(2));
    if s > T::one() {
        let t = (x ^ (x >> T::from_u8(4))) & T::from_u64(0x00F000F000F000F0u64);
        x = x ^ t ^ (t << T::from_u8(4));
    }
    if s > T::from_u8(2) {
        let t = (x ^ (x >> T::from_u8(8))) & T::from_u64(0x0000FF000000FF00u64);
        x = x ^ t ^ (t << T::from_u8(8));
    }
    if s > T::from_u8(4) {
        let t = (x ^ (x >> T::from_u8(16))) & T::from_u64(0x00000000FFFF0000u64);
        x = x ^ t ^ (t << T::from_u8(16));
    }
    x
}

/// Method version of [`outer_perfect_unshuffle`](fn.outer_perfect_unshuffle.html).
pub trait OuterPerfectUnshuffle {
    #[inline]
    fn outer_perfect_unshuffle(self) -> Self;
}

impl<T: Word> OuterPerfectUnshuffle for T {
    #[inline]
    fn outer_perfect_unshuffle(self) -> Self {
        outer_perfect_unshuffle(self)
    }
}
