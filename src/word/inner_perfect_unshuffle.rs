use word::{Word, ReverseBitGroups, OuterPerfectUnshuffle};

/// Inner Perfect Unshuffle of `x`.
///
/// See also:
/// [Hacker's Delight: shuffling bits](http://icodeguru.com/Embedded/Hacker's-Delight/047.htm).
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b1011_1110_1001_0011_0111_1001_0100_1111u32;
/// //        AaBb CcDd EeFf GgHh IiJj KkLl MmNn OoPp
/// let s = 0b0110_0101_1101_1011_1111_1001_0110_0011u32;
/// //        abcd efgh ijkl mnop ABCD EFGH IJKL MNOP,
///
/// assert_eq!(n.inner_perfect_unshuffle(), s);
/// assert_eq!(inner_perfect_unshuffle(n), s);
/// ```
#[inline]
pub fn inner_perfect_unshuffle<T: Word>(x: T) -> T {
    let hwb = T::bit_size().to_u8() / 2u8;
    x.outer_perfect_unshuffle().reverse_bit_groups(hwb, 1u8)
}

/// Method version of [`inner_perfect_unshuffle`](fn.inner_perfect_unshuffle.html).
pub trait InnerPerfectUnshuffle {
    #[inline]
    fn inner_perfect_unshuffle(self) -> Self;
}

impl<T: Word> InnerPerfectUnshuffle for T {
    #[inline]
    fn inner_perfect_unshuffle(self) -> Self {
        inner_perfect_unshuffle(self)
    }
}
