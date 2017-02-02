use word::Word;
use bitintr::x86::abm;

/// Hamming distance between the binary representation of `x` and `y`.
///
/// The Hamming distance is the minimum number of bits that one needs to change
/// in `x` to make it equal to `y`.
///
/// # Keywords:
///
/// Hamming distance.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n   = 0b_1011_0111u8;
///
/// // distance of 1:
/// let d1a  = 0b_1111_0111u8;
/// let d1b = 0b_1011_0101u8;
///
/// // distance of 2:
/// let d2a = 0b_1111_1111u8;
/// let d2b = 0b_1010_0101u8;
///
/// // distance of 3:
/// let d3a  = 0b_1111_1110u8;
/// let d3b = 0b_0010_0101u8;
///
/// assert_eq!(hamming_distance(n, d1a), 1);
/// assert_eq!(hamming_distance(n, d1b), 1);
/// assert_eq!(hamming_distance(n, d2a), 2);
/// assert_eq!(hamming_distance(n, d2b), 2);
/// assert_eq!(hamming_distance(n, d3a), 3);
/// assert_eq!(hamming_distance(n, d3b), 3);
/// ```
#[inline]
pub fn hamming_distance<T: Word>(x: T, y: T) -> T {
    abm::popcnt(x ^ y)
}

/// Method version of [`hamming_distance`](fn.hamming_distance.html).
pub trait HammingDistance {
    #[inline]
    fn hamming_distance(self, Self) -> Self;
}

impl<T: Word> HammingDistance for T {
    #[inline]
    fn hamming_distance(self, y: Self) -> Self {
        hamming_distance(self, y)
    }
}
