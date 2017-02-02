use word::{Word, UnsignedWord, ToWord};
use word::is_pow2::*;

/// Reverses groups of bits within each subword of `x`.
///
/// * `group_bit_size` - The size (in bits) of the groups of bits to be
/// reversed.
/// * `no_subwords` - The number of subwords in `x`.
///
/// The word `x` is divided into `no_subwords`. The bit groups of size
/// `group_bit_size` are reversed within each subword. Bits within a bit group
/// are _not_ reversed.
///
/// The size in bits of a subword is thus `x::bit_size() / no_subword`. When
/// `no_suboword == 1` the subword equals the original word. When
/// `group_bit_size == 1` the single bits are reversed within a subword.
///
/// Both `group_bit_size` and `no_subwords` must be a power of 2 and
/// `x::bit_size() / no_subword % group_bit_size == 0` (that is, a subword must
/// be divisible into bitgroups).
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0b0101_1101_1010_0101_u16;
///
/// // Reverse bits of `n`:
/// assert_eq!(n.reverse_bit_groups(1u32, 1u32), 0b1010_0101_1011_1010u16);
///
/// // Reverse bit pairs of `n`:
/// assert_eq!(reverse_bit_groups(n, 2u32, 1u32), 0b0101_1010_0111_0101u16);
///
/// // Reverse bit nibbles (groups of 4 bits) of `n`:
/// assert_eq!(n.reverse_bit_groups(4u32, 1u32), 0b0101_1010_1101_0101u16);
///
/// // Reverse the bits within each two 8-bit subwords of `n`:
/// assert_eq!(n.reverse_bit_groups(1u32, 2u32), 0b1011_1010_1010_0101u16);
///
/// // Reverse the bit pairs within each two-8 bit subwords of `n`:
/// assert_eq!(n.reverse_bit_groups(2u32, 2u32), 0b0111_0101_0101_1010u16);
///
/// // Reverse the bit nibbles within each two 8-bit subwords of `n`:
/// assert_eq!(n.reverse_bit_groups(4u32, 2u32), 0b1101_0101_0101_1010u16);
///
/// // Reverse bits within each four 4-bit subwords of `n`:
/// assert_eq!(n.reverse_bit_groups(1u32, 4u32), 0b1010_1011_0101_1010u16);
///
/// // Reverse bit pairs within each four 4-bit subwords of `n`:
/// assert_eq!(n.reverse_bit_groups(2u32, 4u32), 0b0101_0111_1010_0101u16);
///
/// // Reverse bits within each 8 2-bit subwords of `n`:
/// assert_eq!(n.reverse_bit_groups(1u32, 8u32), 0b1010_1110_0101_1010u16);
/// ```
pub fn reverse_bit_groups<T: Word, U: UnsignedWord>(x: T, group_bit_size: U, no_subwords: U) -> T {
    // Adapted from Matthew Fioravante's stdcxx-bitops, which
    // is released under the MIT's License here:
    // https://github.com/fmatthew5876/stdcxx-bitops

    type TU<U> where U: Word = U::Unsigned; // TODO: fix warning ?

    debug_assert!(group_bit_size.is_pow2());
    debug_assert!(no_subwords.is_pow2());

    let mut y: TU<T> = x.to();
    let width = T::byte_size();
    let subword_bit_size = T::bit_size() / no_subwords.to();
    debug_assert!(subword_bit_size.to_u32() % group_bit_size.to_u32() == 0);
    let k = subword_bit_size - group_bit_size.to();
    {
        let mut up0 = |i, l, r| if k & i > 0.to() {
            y = ((y & l) << i.to()) | ((y & r) >> i.to());
        };

        up0(1.to(),
            0x5555555555555555u64.to(),
            0xAAAAAAAAAAAAAAAAu64.to());
        up0(2.to(),
            0x3333333333333333u64.to(),
            0xCCCCCCCCCCCCCCCCu64.to());
        up0(4.to(),
            0x0F0F0F0F0F0F0F0Fu64.to(),
            0xF0F0F0F0F0F0F0F0u64.to());
    }

    {
        let mut up1 = |i, s, l, r| if width > i && (k & s > 0.to()) {
            y = ((y & l) << s.to()) | ((y & r) >> s.to());
        };

        up1(1.to(),
            8.to(),
            0x00FF00FF00FF00FFu64.to(),
            0xFF00FF00FF00FF00u64.to());
        up1(2.to(),
            16.to(),
            0x0000FFFF0000FFFFu64.to(),
            0xFFFF0000FFFF0000u64.to());
        up1(4.to(),
            32.to(),
            0x00000000FFFFFFFFu64.to(),
            0xFFFFFFFF00000000u64.to());
    }
    y.to()
}

/// Method version of [`reverse_bit_groups`](fn.reverse_bit_groups.html).
pub trait ReverseBitGroups: Word {
    fn reverse_bit_groups<T: UnsignedWord>(self, x: T, y: T) -> Self;
}

impl<T: Word> ReverseBitGroups for T {
    fn reverse_bit_groups<U: UnsignedWord>(self, x: U, y: U) -> T {
        reverse_bit_groups(self, x, y)
    }
}
