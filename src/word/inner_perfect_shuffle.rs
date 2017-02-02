use word::{Word, ReverseBitGroups, OuterPerfectShuffle};

/// Inner Perfect Shuffle of `x`.
///
/// See also:
/// [Hacker's Delight: shuffling bits](http://icodeguru.com/Embedded/Hacker's-Delight/047.htm).
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0110_0101_1101_1011_1111_1001_0110_0011u32;
/// //        abcd efgh ijkl mnop ABCD EFGH IJKL MNOP,
/// let s = 0b1011_1110_1001_0011_0111_1001_0100_1111u32;
/// //        AaBb CcDd EeFf GgHh IiJj KkLl MmNn OoPp
///
/// assert_eq!(n.inner_perfect_shuffle(), s);
/// assert_eq!(inner_perfect_shuffle(n), s);
/// ```
#[inline]
pub fn inner_perfect_shuffle<T: Word>(x: T) -> T {
    let hwb = T::bit_size().to_u8() / 2u8;
    x.reverse_bit_groups(hwb, 1u8).outer_perfect_shuffle()
}

/// Method version of [`inner_perfect_shuffle`](fn.inner_perfect_shuffle.html).
pub trait InnerPerfectShuffle {
    #[inline]
    fn inner_perfect_shuffle(self) -> Self;
}

impl<T: Word> InnerPerfectShuffle for T {
    #[inline]
    fn inner_perfect_shuffle(self) -> Self {
        inner_perfect_shuffle(self)
    }
}
