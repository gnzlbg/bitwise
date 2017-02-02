use word::{Word, UnsignedWord, ToWord};
use word::reverse_bit_groups::*;

/// Reverses groups of bytes within each subword of `x`
///
/// * `group_byte_size` - The size (in bytes) of the groups of bytes to be
/// reversed.
/// * `no_subwords` - The number of subwords in `x`.
///
/// The word `x` is divided into `no_subwords`. The byte groups of size
/// `group_byte_size` are reversed within each subword. Bits within a byte group
/// are _not_ reversed.
///
/// The size in bits of a subword is thus `x::bit_size() / no_subword`. When
/// `no_suboword == 1` the subword equals the original word. When
/// `group_byte_size == 1` bytes are reversed within a subword.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// // Single bytes:
/// assert_eq!(0b0101_1101_1010_0101_u16.reverse_byte_groups(1u32, 1u32), 0b1010_0101_0101_1101u16);
///
/// // Single bytes within two half-words:
/// assert_eq!(reverse_byte_groups(0b0101_1101_1010_0101_0101_1101_1010_0101u32, 1u32, 2u32), 0b1010_0101_0101_1101_1010_0101_0101_1101u32);
/// ```
#[inline]
pub fn reverse_byte_groups<T: Word, U: UnsignedWord>(x: T,
                                                     group_byte_size: U,
                                                     no_subwords: U)
                                                     -> T {
    reverse_bit_groups(x, group_byte_size * 8.to(), no_subwords)
}

/// Method version of [`reverse_byte_groups`](fn.reverse_byte_groups.html).
pub trait ReverseByteGroups: Word {
    #[inline]
    fn reverse_byte_groups<U: UnsignedWord>(self, group_byte_size: U, no_subwords: U) -> Self;
}

impl<T: Word> ReverseByteGroups for T {
    #[inline]
    fn reverse_byte_groups<U: UnsignedWord>(self, group_byte_size: U, no_subwords: U) -> T {
        reverse_byte_groups(self, group_byte_size, no_subwords)
    }
}
