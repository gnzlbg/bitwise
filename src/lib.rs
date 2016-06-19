#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]

//! Bitwise manipulation algorithms for Words and WordSlices.
//!
//! See the list of available algorithms in the `Word` and `Words` traits.

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[cfg(feature = "llvmint")]
extern crate llvmint;

use std::ops::{Add, Sub, Mul, Div};
use std::ops::{Not, BitAnd, BitOr, BitXor, Shl, Shr};
use std::mem;

/// Bitwise manipulation algorithms for Words
pub trait Word
    : Sized
    + Copy
    + Not<Output=Self>
    + BitOr<Output=Self>
    + BitXor<Output=Self>
    + Add<Output=Self>
    + Sub<Output=Self>
    + Mul<Output=Self>
    + Div<Output=Self>
    + Shr<u32, Output=Self>
    + Shl<u32, Output=Self>
    + BitAnd<Output=Self>
    + Eq + PartialOrd

{
/// Signed Word Type of the same size as Self.
    type Signed : Word;
/// Unsigned Word Type of the same size as Self.
    type Unsigned : Word;

/// Size of the word in bytes.
    fn size() -> usize;

/// Transmutes the integer into an unsigned integer of the
/// same size (bitwise loss-less).
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// assert_eq!((8i32).to_unsigned().to_signed(), 8i32);
/// assert_eq!((-1i32).to_unsigned().to_signed(), -1i32);
/// ```
    fn to_unsigned(self) -> Self::Unsigned;

/// Transmutes an integer into a signed integer of the
/// same size (bitwise loss-less).
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// assert_eq!((8i32).to_unsigned().to_signed(), 8i32);
/// assert_eq!((-1i32).to_unsigned().to_signed(), -1i32);
/// ```
    fn to_signed(self) -> Self::Signed;

/// Returns an integer of value one.
    fn one() -> Self;
/// Returns an integer of value zero.
    fn zero() -> Self;

/// Returns the number of ones in the binary representation of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0100_1100u8;
///
/// assert_eq!(n.count_ones(), 3);
/// ```
    fn count_ones(self) -> usize;

/// Returns the number of zeros in the binary representation of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0100_1100u8;
///
/// assert_eq!(n.count_zeros(), 5);
/// ```
    fn count_zeros(self) -> usize;

/// Returns the number of leading zeros in the binary representation
/// of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0010_1000u16;
///
/// assert_eq!(n.leading_zeros(), 10);
/// ```
    fn leading_zeros(self) -> usize;

/// Returns the number of trailing zeros in the binary representation
/// of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0010_1000u16;
///
/// assert_eq!(n.trailing_zeros(), 3);
/// ```
    fn trailing_zeros(self) -> usize;

/// Shifts the bits to the left by a specified amount amount, `n`, wrapping
/// the truncated bits to the end of the resulting integer.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0x0123456789ABCDEFu64;
/// let m = 0x3456789ABCDEF012u64;
///
/// assert_eq!(n.rotate_left(12), m);
/// ```
    fn rotate_left(self, n: u32) -> Self;

/// Shifts the bits to the right by a specified amount amount, `n`, wrapping
/// the truncated bits to the beginning of the resulting integer.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0x0123456789ABCDEFu64;
/// let m = 0xDEF0123456789ABCu64;
///
/// assert_eq!(n.rotate_right(12), m);
/// ```
    fn rotate_right(self, n: u32) -> Self;

/// Reverses the byte order of the integer.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0x0123456789ABCDEFu64;
/// let m = 0xEFCDAB8967452301u64;
///
/// assert_eq!(n.swap_bytes(), m);
/// ```
    fn swap_bytes(self) -> Self;

/// Convert an integer from big endian to the target's endianness.
///
/// On big endian this is a no-op. On little endian the bytes are swapped.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0x0123456789ABCDEFu64;
///
/// if cfg!(target_endian = "big") {
///     assert_eq!(u64::from_be(n), n)
/// } else {
///     assert_eq!(u64::from_be(n), n.swap_bytes())
/// }
/// ```
    fn from_be(x: Self) -> Self;

/// Convert an integer from little endian to the target's endianness.
///
/// On little endian this is a no-op. On big endian the bytes are swapped.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0x0123456789ABCDEFu64;
///
/// if cfg!(target_endian = "little") {
///     assert_eq!(u64::from_le(n), n)
/// } else {
///     assert_eq!(u64::from_le(n), n.swap_bytes())
/// }
/// ```
    fn from_le(x: Self) -> Self;

/// Convert `self` to big endian from the target's endianness.
///
/// On big endian this is a no-op. On little endian the bytes are swapped.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0x0123456789ABCDEFu64;
///
/// if cfg!(target_endian = "big") {
///     assert_eq!(n.to_be(), n)
/// } else {
///     assert_eq!(n.to_be(), n.swap_bytes())
/// }
/// ```
    fn to_be(self) -> Self;

/// Convert `self` to little endian from the target's endianness.
///
/// On little endian this is a no-op. On big endian the bytes are swapped.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0x0123456789ABCDEFu64;
///
/// if cfg!(target_endian = "little") {
///     assert_eq!(n.to_le(), n)
/// } else {
///     assert_eq!(n.to_le(), n.swap_bytes())
/// }
/// ```
    fn to_le(self) -> Self;

/// Raises self to the power of `exp`, using exponentiation by squaring.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// assert_eq!(2i32.pow(4), 16);
/// ```
    fn pow(self, mut exp: u32) -> Self;

/// Returns the number of leading ones in the binary representation
/// of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1111_1111_1100_1000u16;
///
/// assert_eq!(n.leading_ones(), 10);
/// ```
    fn leading_ones(self) -> usize {
       Self::leading_zeros(!self)
    }

/// Returns the number of trailing ones in the binary representation
/// of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0010_0111u16;
///
/// assert_eq!(n.trailing_ones(), 3);
/// ```
    fn trailing_ones(self) -> usize {
       Self::trailing_zeros(!self)
    }

/// Returns the number of 1 bits in `self` mod 2, that is, returns 1 if the
/// number of 1 bits in `self` is odd, and zero otherwise
///
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n0 = 0b0000;  // 0 -> even => parity = 0
/// let n1 = 0b0001;  // 1 -> odd  => partiy = 1
/// let n2 = 0b0010;  // 1 -> odd  => parity = 1
/// let n3 = 0b0011;  // 2 -> even => parity = 0
///
/// assert_eq!(n0.parity(), 0);
/// assert_eq!(n1.parity(), 1);
/// assert_eq!(n2.parity(), 1);
/// assert_eq!(n3.parity(), 0);
/// ```
    fn parity(self) -> usize {
// TODO: use intrinsics depending on size:
//   - __builtin_parity, __builtin_parityl, __builtin_parityll
        self.count_ones() & 1
    }

/// Reset least significant 1 bit of `self`; returns 0 if `self` is 0.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0110;
/// let s = 0b0100;
///
/// assert_eq!(n.reset_least_significant_one(), s);
/// ```
    fn reset_least_significant_one(self) -> Self {
        self & (self - Self::one())
    }

/// Set least significant 0 bit of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0101;
/// let s = 0b0111;
///
/// assert_eq!(n.set_least_significant_zero(), s);
/// ```
    fn set_least_significant_zero(self) -> Self {
        self | (self + Self::one())
    }

/// Isolate least significant 1 bit of `self` and returns it; returns 0
/// if `self` is 0.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0110;
/// let s = 0b0010;
///
/// assert_eq!(n.isolate_least_significant_one(), s);
/// ```
    fn isolate_least_significant_one(self) -> Self {
// note: self & -self is intended, which is rewritten as:
        self & (Self::zero() - self)
    }

/// Set the least significant zero bit of `self` to 1 and all of the
/// rest to 0.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0101;
/// let s = 0b0010;
///
/// assert_eq!(n.isolate_least_significant_zero(), s);
/// ```
    fn isolate_least_significant_zero(self) -> Self {
        (!self) & (self + Self::one())
    }

/// Reset the trailing 1's in `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0110_1111;
/// let s = 0b0110_0000;
///
/// assert_eq!(n.reset_trailing_ones(), s);
/// ```
    fn reset_trailing_ones(self) -> Self {
        self & (self + Self::one())
    }

/// Set all of the trailing 0's in `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0110_0000u8;
/// let s = 0b0111_1111u8;
///
/// assert_eq!(n.set_trailing_zeros(), s);
/// ```
    fn set_trailing_zeros(self) -> Self {
        self | (self - Self::one())
    }

/// Returns a mask with all of the trailing 0's of `self` set.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0110_0000u8;
/// let s = 0b0001_1111u8;
///
/// assert_eq!(n.mask_trailing_zeros(), s);
/// ```
    fn mask_trailing_zeros(self) -> Self {
        (!self) & (self - Self::one())
    }

/// Returns a mask with all of the trailing 1's of `self` set.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0101_1111u8;
/// let s = 0b0001_1111u8;
///
/// assert_eq!(n.mask_trailing_ones(), s);
/// ```
    fn mask_trailing_ones(self) -> Self {
        !((!self) | (self + Self::one()))
    }

/// Returns a mask with all of the trailing 0's of `self` set and the least
/// significant 1 bit set.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0101_0000u8;
/// let s = 0b0001_1111u8;
///
/// assert_eq!(n.mask_trailing_zeros_and_least_significant_one(), s);
/// ```
    fn mask_trailing_zeros_and_least_significant_one(self) -> Self {
        (self - Self::one()) ^ self
    }

/// Returns a mask with all of the trailing 1's of `self` set and the least
/// significant 0 bit set.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0101_1111u8;
/// let s = 0b0011_1111u8;
///
/// assert_eq!(n.mask_trailing_ones_and_least_significant_zero(), s);
/// ```
    fn mask_trailing_ones_and_least_significant_zero(self) -> Self {
        self ^ (self + Self::one())
    }

/// Reverses the bits of `self` by `subword_bits` and `group_subwords`:
///
/// - `subword_bits`: the bits will be reversed in grous:
///   1 (single bits), 2 (pair-wise), 4 (nibbles), 
/// - `group_subwords`: the subword size is 8 bits: `mem::size_of::<u8>()`,
///   the bits will be reversed within each subword.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0101_1101_1010_0101_u16;
///
/// // Single bits:
/// let s0 = 0b1010_0101_1011_1010u16;
/// assert_eq!(n.reverse_bit_groups(1, 1), s0);
///
/// // Bit pairs:
/// let s1 = 0b0101_1010_0111_0101u16;
/// assert_eq!(n.reverse_bit_groups(2, 1), s1);
///
/// // Bit nibbles:
/// let s2 = 0b0101_1010_1101_0101u16;
/// assert_eq!(n.reverse_bit_groups(4, 1), s2);
///
/// // Single bits: group_subwords = 2
/// let s3 = 0b1011_1010_1010_0101u16;
/// assert_eq!(n.reverse_bit_groups(1, 2), s3);
///
/// // Bit pairs: group_subwords = 2
/// let s4 = 0b0111_0101_0101_1010u16;
/// assert_eq!(n.reverse_bit_groups(2, 2), s4);
///
/// // Bit nibbles: group_subwords = 2
/// let s5 = 0b1101_0101_0101_1010u16;
/// assert_eq!(n.reverse_bit_groups(4, 2), s5);
/// ```
    fn reverse_bit_groups(self, subword_bits: u32, group_subwords: u32) -> Self;

/// Reverses the bits of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1011_0010u8;
/// let s = 0b0100_1101u8;
/// assert_eq!(n.reverse_bits(), s);
///
/// let n1 = 0b1011_0010_1010_1001u16;
/// let s1 = 0b1001_0101_0100_1101u16;
/// assert_eq!(n1.reverse_bits(), s1);
/// ```
    fn reverse_bits(self) -> Self {
        self.reverse_bit_groups(1, 1)
    }

/// Reverses the pairs of bits of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1011_0010u8;
/// let s = 0b1000_1110u8;
/// assert_eq!(n.reverse_bit_pairs(), s);
///
/// let n1 = 0b1011_0010_1010_1001u16;
/// let s1 = 0b0110_1010_1000_1110u16;
/// assert_eq!(n1.reverse_bit_pairs(), s1);
/// ```
    fn reverse_bit_pairs(self) -> Self {
        self.reverse_bit_groups(2, 1)
    }

/// Reverses the nibbles of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1011_0010u8;
/// let s = 0b0010_1011u8;
/// assert_eq!(n.reverse_bit_nibbles(), s);
///
/// let n1 = 0b1011_0010_1010_1001u16;
/// let s1 = 0b1001_1010_0010_1011u16;
/// assert_eq!(n1.reverse_bit_nibbles(), s1);
/// ```
    fn reverse_bit_nibbles(self) -> Self {
        self.reverse_bit_groups(4, 1)
    }

/// Reverses the bytes of `self`.
///
/// - `bytes_per_block`: number of bytes per block to reverse.
/// - `blocks_per_group`: number of blocks per group of blocks.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0101_1101_1010_0101_u16;
///
/// // Single bytes:
/// let s0 = 0b1010_0101_0101_1101u16;
/// assert_eq!(n.reverse_byte_groups(1, 1), s0);
///
/// // Single bytes: group_subwords = 2
/// let s3 = 0b0101_1101_1010_0101u16;
/// assert_eq!(n.reverse_byte_groups(1, 2), s3);
/// ```
    fn reverse_byte_groups(self, bytes_per_block: u32, blocks_per_group: u32) -> Self {
        self.reverse_bit_groups(8 * bytes_per_block, blocks_per_group)
    }

/// Reverses the bytes of `self` (equivalent to swap bytes).
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1011_0010u8;
/// let s = 0b1011_0010u8;
/// assert_eq!(n.reverse_bytes(), s);
/// assert_eq!(n.swap_bytes(), s);
///
/// let n1 = 0b1011_0010_1010_1001u16;
/// let s1 = 0b1010_1001_1011_0010u16;
/// assert_eq!(n1.reverse_bytes(), s1);
/// assert_eq!(n1.swap_bytes(), s1);
/// ```
    fn reverse_bytes(self) -> Self {
        self.swap_bytes()
    }

/// Sets the `bit` of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n  = 0b1011_0010u8;
/// let s0 = 0b1111_0010u8;
/// let s1 = 0b1011_0011u8;
/// let s2 = 0b1011_1010u8;
/// assert_eq!(n.set_bit(6), s0);
/// assert_eq!(n.set_bit(0), s1);
/// assert_eq!(n.set_bit(3), s2);
/// ```
     fn set_bit(self, bit: u32) -> Self {
         self | (Self::one() << bit)
    }

/// Resets the `bit` of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1011_0010u8;
/// let s0 = 0b0011_0010u8;
/// let s1 = 0b1011_0000u8;
/// let s2 = 0b1001_0010u8;
/// assert_eq!(n.reset_bit(7), s0);
/// assert_eq!(n.reset_bit(1), s1);
/// assert_eq!(n.reset_bit(5), s2);
/// ```
    fn reset_bit(self, bit: u32) -> Self {
         self & !(Self::one() << bit)
    }

/// Flip the `bit` of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1011_0010u8;
/// let s0 = 0b0011_0010u8;
/// let s1 = 0b1111_0010u8;
/// let s2 = 0b1001_0010u8;
/// assert_eq!(n.flip_bit(7), s0);
/// assert_eq!(n.flip_bit(6), s1);
/// assert_eq!(n.flip_bit(5), s2);
/// ```
    fn flip_bit(self, bit: u32) -> Self {
         self ^ (Self::one() << bit)
    }

/// Test the `bit` of `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1011_0010u8;
/// assert_eq!(n.test_bit(7), true);
/// assert_eq!(n.test_bit(6), false);
/// assert_eq!(n.test_bit(5), true);
/// ```
    fn test_bit(self, bit: u32) -> bool {
        self & (Self::one() << bit) != Self::zero()
    }

/// Resets all bits of `self` at position >= `bit`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1111_0010u8;
/// let s = 0b0001_0010u8;
/// assert_eq!(n.reset_bits_geq(5), s);
/// ```
    fn reset_bits_geq(self, bit: u32) -> Self {
       self & ((Self::one() << bit) - Self::one())
    }

/// Resets all bits of `self` at position <= `bit`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1111_0010u8;
/// let s = 0b1100_0000u8;
/// assert_eq!(n.reset_bits_leq(5), s);
/// ```
    fn reset_bits_leq(self, bit: u32) -> Self {
       self & !((Self::one() << (bit + 1)) - Self::one())
    }

/// Sets all bits of `self` at position >= `bit`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1000_0010u8;
/// let s = 0b1110_0010u8;
/// assert_eq!(n.set_bits_geq(5), s);
/// ```
    fn set_bits_geq(self, bit: u32) -> Self {
       self | !((Self::one() << bit) - Self::one())
    }

/// Sets all bits of `self` at position <= `bit`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1000_0010u8;
/// let s = 0b1011_1111u8;
/// assert_eq!(n.set_bits_leq(5), s);
/// ```
    fn set_bits_leq(self, bit: u32) -> Self {
       self | ((Self::one() << (bit + 1)) - Self::one())
    }

/// Flip all bits of `self` at position >= `bit`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1001_0010u8;
/// let s = 0b0111_0010u8;
/// assert_eq!(n.flip_bits_geq(5), s);
/// ```
    fn flip_bits_geq(self, bit: u32) -> Self {
       self ^ !((Self::one() << bit) - Self::one())
    }

/// Flip all bits of `self` at position <= `bit`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1011_0010u8;
/// let s = 0b1000_1101u8;
/// assert_eq!(n.flip_bits_leq(5), s);
/// ```
    fn flip_bits_leq(self, bit: u32) -> Self {
       self ^ ((Self::one() << (bit + 1)) - Self::one() )
    }

/// Is `self` a power of 2.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// assert!(2.is_pow2());
/// assert!(!3.is_pow2());
/// assert!(4.is_pow2());
/// assert!(!5.is_pow2());
/// assert!(!6.is_pow2());
/// assert!(!7.is_pow2());
/// assert!(8.is_pow2());
/// ```
    fn is_pow2(self) -> bool {
       self > Self::zero()
       && ((self & (self - Self::one())) == Self::zero())
    }

/// Round `self` to the next power of 2.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// assert_eq!(2.ceil_pow2(), 2);
/// assert_eq!(3.ceil_pow2(), 4);
/// assert_eq!(4.ceil_pow2(), 4);
/// assert_eq!(5.ceil_pow2(), 8);
/// assert_eq!(6.ceil_pow2(), 8);
/// assert_eq!(7.ceil_pow2(), 8);
/// assert_eq!(8.ceil_pow2(), 8);
/// ```
    fn ceil_pow2(self)-> Self {
        let mut x = self - Self::one();
        let s = Self::size();
        x = x | (x >> 1);
        x = x | (x >> 2);
        x = x | (x >> 4);
        if s > 1 {
            x = x | (x >> 8);
            if s > 2 {
                x = x | (x >> 16);
                if s > 4 {
                    x = x | (x >> 32);
                }
            }
        }
        x + Self::one()
    }

/// Round `self` to the previous power of 2.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// assert_eq!(2.floor_pow2(), 2);
/// assert_eq!(3.floor_pow2(), 2);
/// assert_eq!(4.floor_pow2(), 4);
/// assert_eq!(5.floor_pow2(), 4);
/// assert_eq!(6.floor_pow2(), 4);
/// assert_eq!(7.floor_pow2(), 4);
/// assert_eq!(8.floor_pow2(), 8);
/// ```
    fn floor_pow2(self) -> Self {
        let mut x = self;
        let s = Self::size();
        x = x | (x >> 1);
        x = x | (x >> 2);
        x = x | (x >> 4);
        if s > 1 {
            x = x | (x >> 8);
            if s > 2 {
                x = x | (x >> 16);
                if s > 4 {
                    x = x | (x >> 32);
                }
            }
        }
        x - (x >> 1)
    }

/// Is `self` aligned to `alignment` bytes.
///
/// Returns true if `self == 0` or `self` is a multiple of `alignment`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// assert!(2.is_aligned(1));
/// assert!(2.is_aligned(2));
/// assert!(!2.is_aligned(4));
/// assert!(!2.is_aligned(8));
///
/// assert!(3.is_aligned(1));
/// assert!(!3.is_aligned(2));
/// assert!(!3.is_aligned(4));
/// assert!(!3.is_aligned(8));
///
/// assert!(4.is_aligned(1));
/// assert!(4.is_aligned(2));
/// assert!(4.is_aligned(4));
/// assert!(!4.is_aligned(8));
/// ```
    fn is_aligned(self, alignment: u32) -> bool;

/// Align `self` up to `alignment`.
///
/// Returns `n`, where `n` is the least number >= `self`
/// and `is_aligned(n, alignment)`.
///
/// # Panics
///
/// `alignment` must be a power of two which is asserted in debug builds.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// assert_eq!(2.align_up(1), 2);
/// assert_eq!(2.align_up(2), 2);
/// assert_eq!(2.align_up(4), 4);
/// assert_eq!(2.align_up(8), 8);
///
/// assert_eq!(3.align_up(1), 3);
/// assert_eq!(3.align_up(2), 4);
/// assert_eq!(3.align_up(4), 4);
/// assert_eq!(3.align_up(8), 8);
///
/// assert_eq!(4.align_up(1), 4);
/// assert_eq!(4.align_up(2), 4);
/// assert_eq!(4.align_up(4), 4);
/// assert_eq!(4.align_up(8), 8);
/// ```
    fn align_up(self, alignment: u32) -> Self;

/// Align `self` down to `alignment`.
///
/// Returns `n`, where `n` is the greatest number <= `self`
/// and `is_aligned(n, alignment)`.
///
/// # Panics
///
/// `alignment` must be a power of two which is asserted in debug builds.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// assert_eq!(2.align_down(1), 2);
/// assert_eq!(2.align_down(2), 2);
/// assert_eq!(2.align_down(4), 0);
/// assert_eq!(2.align_down(8), 0);
///
/// assert_eq!(3.align_down(1), 3);
/// assert_eq!(3.align_down(2), 2);
/// assert_eq!(3.align_down(4), 0);
/// assert_eq!(3.align_down(8), 0);
///
/// assert_eq!(4.align_down(1), 4);
/// assert_eq!(4.align_down(2), 4);
/// assert_eq!(4.align_down(4), 4);
/// assert_eq!(4.align_down(8), 0);
/// ```
    fn align_down(self, alignment: u32) -> Self;

/// Outer Perfect Shuffle of `self`.
///
/// See also:
/// [Hacker's Delight: shuffling bits](http://icodeguru.com/Embedded/Hacker's-Delight/047.htm).
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0110_0101_1101_1011_1111_1001_0110_0011u32;
/// //        abcd efgh ijkl mnop ABCD EFGH IJKL MNOP,
/// let s = 0b0111_1101_0110_0011_1011_0110_1000_1111u32;
/// //        aAbB cCdD eEfF gGhH iIjJ kKlL mMnN oOpP,
///
/// assert_eq!(n.outer_perfect_shuffle(), s);
/// ```
    fn outer_perfect_shuffle(self) -> Self;

/// Outer Perfect Unshuffle of `self`.
///
/// See also:
/// [Hacker's Delight: shuffling bits](http://icodeguru.com/Embedded/Hacker's-Delight/047.htm).
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0111_1101_0110_0011_1011_0110_1000_1111u32;
/// //        aAbB cCdD eEfF gGhH iIjJ kKlL mMnN oOpP,
/// let s = 0b0110_0101_1101_1011_1111_1001_0110_0011u32;
/// //        abcd efgh ijkl mnop ABCD EFGH IJKL MNOP,
///
/// assert_eq!(n.outer_perfect_unshuffle(), s);
/// ```
    fn outer_perfect_unshuffle(self) -> Self;

/// Inner Perfect Shuffle of `self`.
///
/// See also:
/// [Hacker's Delight: shuffling bits](http://icodeguru.com/Embedded/Hacker's-Delight/047.htm).
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b0110_0101_1101_1011_1111_1001_0110_0011u32;
/// //        abcd efgh ijkl mnop ABCD EFGH IJKL MNOP,
/// let s = 0b1011_1110_1001_0011_0111_1001_0100_1111u32;
/// //        AaBb CcDd EeFf GgHh IiJj KkLl MmNn OoPp
///
/// assert_eq!(n.inner_perfect_shuffle(), s);
/// ```
    fn inner_perfect_shuffle(self) -> Self {
        let hwb = Self::size() * 8 / 2;
        self.reverse_bit_groups(hwb as u32, 1).outer_perfect_shuffle()
   }

/// Inner Perfect Unshuffle of `self`.
///
/// See also:
/// [Hacker's Delight: shuffling bits](http://icodeguru.com/Embedded/Hacker's-Delight/047.htm).
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n = 0b1011_1110_1001_0011_0111_1001_0100_1111u32;
/// //        AaBb CcDd EeFf GgHh IiJj KkLl MmNn OoPp
/// let s = 0b0110_0101_1101_1011_1111_1001_0110_0011u32;
/// //        abcd efgh ijkl mnop ABCD EFGH IJKL MNOP,
///
/// assert_eq!(n.inner_perfect_unshuffle(), s);
/// ```
    fn inner_perfect_unshuffle(self) -> Self {
        let hwb = Self::size() * 8 / 2;
        self.outer_perfect_unshuffle().reverse_bit_groups(hwb as u32, 1)
    }

/// Parallel bits deposit of `mask` into `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n  = 0b1011_1110_1001_0011u16;
///
/// let m0 = 0b0110_0011_1000_0101u16;
/// let s0 = 0b0000_0010_0000_0101u16;
///
/// let m1 = 0b1110_1011_1110_1111u16;
/// let s1 = 0b1110_1001_0010_0011u16;
///
/// assert_eq!(n.parallel_bits_deposit(m0), s0);
/// assert_eq!(n.parallel_bits_deposit(m1), s1);
/// ```
    fn parallel_bits_deposit(self, mask_: Self) -> Self;

/// Parallel bits extract of `mask` from `self`.
///
/// # Examples
///
/// ```
/// use bitwise::Word;
///
/// let n  = 0b1011_1110_1001_0011u16;
///
/// let m0 = 0b0110_0011_1000_0101u16;
/// let s0 = 0b0000_0000_0011_0101u16;
///
/// let m1 = 0b1110_1011_1110_1111u16;
/// let s1 = 0b0001_0111_0100_0011u16;
///
/// assert_eq!(n.parallel_bits_extract(m0), s0);
/// assert_eq!(n.parallel_bits_extract(m1), s1);
/// ```
    fn parallel_bits_extract(self, mask_: Self) -> Self;

}

macro_rules! bitwise_word_impl {
    ($T:ty, $AS:ty, $AU:ty) => (
        impl Word for $T {
            type Signed = $AS;
            type Unsigned = $AU;

            fn size() -> usize { mem::size_of::<$T>() }

            fn to_unsigned(self) -> $AU
            {
                self as $AU
            }

            fn to_signed(self) -> $AS
            {
                self as $AS
            }

            fn zero() -> $T { 0 as $T }
            fn leading_zeros(self) -> usize {
                <$T>::leading_zeros(self) as usize
            }

            fn trailing_zeros(self) -> usize {
                <$T>::trailing_zeros(self) as usize
            }
            fn count_ones(self) -> usize {
              <$T>::count_ones(self) as usize
            }
            fn count_zeros(self) -> usize {
              <$T>::count_zeros(self) as usize
            }
            fn one() -> $T { 1 as $T }
            fn rotate_left(self, n: u32) -> Self {
                <$T>::rotate_left(self, n)
            }
            fn rotate_right(self, n: u32) -> Self {
                <$T>::rotate_right(self, n)
            }
            fn swap_bytes(self) -> Self {
                <$T>::swap_bytes(self)
            }

            fn from_be(x: Self) -> Self {
                <$T>::from_be(x)
            }

            fn from_le(x: Self) -> Self {
                <$T>::from_le(x)
            }

            fn to_be(self) -> Self {
                <$T>::to_be(self)
            }

            fn to_le(self) -> Self {
                <$T>::to_le(self)
            }

            fn pow(self, exp: u32) -> Self {
                <$T>::pow(self, exp)
            }

            fn reverse_bit_groups(self, subword_bits: u32, group_subwords: u32) -> Self {
// Adapted from Matthew Fioravante's stdcxx-bitops, which
// is released under the MIT's License here:
// https://github.com/fmatthew5876/stdcxx-bitops

              let mut x: Self::Unsigned = self.to_unsigned();
              let width: u32  = <Self as Word>::size() as u32;
              let group_sz: u32 = width * 8 / group_subwords;
              let k: u32 = group_sz - subword_bits;
              {
                  let mut up0 = |i: u32, l: u64, r: u64| {
                      if k & i > 0 {
                          x = ((x & (l as Self::Unsigned)) << (i as Self::Unsigned))
                              | ((x & (r as Self::Unsigned)) >> (i as Self::Unsigned));
                      }
                  };

                  up0(1, 0x5555555555555555u64, 0xAAAAAAAAAAAAAAAAu64);
                  up0(2, 0x3333333333333333u64, 0xCCCCCCCCCCCCCCCCu64);
                  up0(4, 0x0F0F0F0F0F0F0F0Fu64, 0xF0F0F0F0F0F0F0F0u64);
              }

              {
                  let mut up1 = |i: u32, s: u32, l: u64, r: u64| {
                      if width > i && (k & s > 0) {
                          x = ((x & (l as Self::Unsigned)) << (s as Self::Unsigned))
                              | ((x & (r as Self::Unsigned)) >> (s as Self::Unsigned));
                      }
                  };

                  up1(1, 8, 0x00FF00FF00FF00FFu64, 0xFF00FF00FF00FF00u64);
                  up1(2, 16, 0x0000FFFF0000FFFFu64, 0xFFFF0000FFFF0000u64);
                  up1(4, 32, 0x00000000FFFFFFFFu64, 0xFFFFFFFF00000000u64);
              }
              x as Self
            }

            fn align_up(self, alignment: u32) -> Self {
                assert!(alignment.is_pow2());
                let x = self.to_unsigned();
                let a = alignment as Self::Unsigned;
                ((x + (a - 1)) & !(a - 1)) as Self
            }

            fn align_down(self, alignment: u32) -> Self {
                assert!(alignment.is_pow2());
                self & (!(alignment - 1) as Self)
            }

            #[allow(exceeding_bitshifts)] fn outer_perfect_shuffle(self) -> Self {
                let mut x = self;
                let mut t;
                let s = <Self as Word>::size();
                if s > 4 {
                    t = (x ^ (x >> 16)) & 0x00000000FFFF0000u64 as Self;
                    x = x ^ t ^ (t << 16);
                }
                if s > 2 {
                    t = (x ^ (x >> 8)) & 0x0000FF000000FF00u64 as Self;
                    x = x ^ t ^ (t << 8);
                }
                if s > 1 {
                    t = (x ^ (x >> 4)) & 0x00F000F000F000F0u64 as Self;
                    x = x ^ t ^ (t << 4);
                }
                t = (x ^ (x >> 2)) & 0x0C0C0C0C0C0C0C0Cu64 as Self;
                x = x ^ t ^ (t << 2);
                t = (x ^ (x >> 1)) & 0x2222222222222222u64 as Self;
                x = x ^ t ^ (t << 1);
                x
            }

            #[allow(exceeding_bitshifts)] fn outer_perfect_unshuffle(self) -> Self {
                let mut x = self;
                let s = <Self as Word>::size();
                let mut t = (x ^ (x >> 1)) & (0x2222222222222222u64 as Self);
                x = x ^ t ^ (t << 1);
                t = (x ^ (x >>  2)) & (0x0C0C0C0C0C0C0C0Cu64 as Self);
                x = x ^ t ^ (t << 2);
                if s > 1 {
                    t = (x ^ (x >> 4)) & (0x00F000F000F000F0u64 as Self);
                    x = x ^ t ^ (t << 4);
                }
                if s > 2 {
                    t = (x ^ (x >> 8)) & (0x0000FF000000FF00u64 as Self);
                    x = x ^ t ^ (t << 8);
                }
                if s > 4 {
                    t = (x ^ (x >> 16)) & (0x00000000FFFF0000u64 as Self);
                    x = x ^ t ^ (t << 16);
                }
                x
            }

            fn is_aligned(self, alignment: u32) -> bool {
                debug_assert!(alignment as i32 - 1 >= 0);
                (self & ((alignment - 1) as Self)) == Self::zero()
            }

            #[cfg(not(feature = "bmi2"))]
            fn parallel_bits_deposit(self, mask_: Self) -> Self
            {
                let mut res = Self::zero();
                let mut mask = mask_;
                let mut bb = Self::one();
                loop {
                    if mask == Self::zero() {
                        break;
                    }
                    if (self & bb) != Self::zero() {
                        res |= mask & mask.wrapping_neg();
                    }
                    mask &= mask - Self::one();
                    bb += bb;
                }
                res
            }

            #[cfg(not(feature = "bmi2"))]
            fn parallel_bits_extract(self, mask_: Self) -> Self {
                let mut res = Self::zero();
                let mut mask = mask_;
                let mut bb = Self::one();
                loop {
                    if mask == Self::zero() {
                        break;
                    }
                    if self & mask & (mask.wrapping_neg()) != Self::zero() {
                        res |= bb;
                    }
                    mask &= mask - Self::one();
                    bb += bb;
                }
                res
            }

            #[cfg(feature = "bmi2")]
            fn parallel_bits_deposit(self, mask_: Self) -> Self {
                match <Self as Word>::size() {
                    0...32 => unsafe {
                        llvmint::x86::bmi_pdep_32(self as i32, mask_ as i32) as Self
                    },
                    64 => unsafe {
                        llvmint::x86::bmi_pdep_64(self as i64, mask_ as i64) as Self
                    },
                     _ => unreachable!()
                }
            }
            #[cfg(feature = "bmi2")]
            fn parallel_bits_extract(self, mask_: Self) -> Self {
                match <Self as Word>::size() {
                    0...32 => unsafe {
                        llvmint::x86::bmi_pext_32(self as i32, mask_ as i32) as Self
                    },
                    64 => unsafe {
                        llvmint::x86::bmi_pext_64(self as i64, mask_ as i64) as Self
                    },
                     _ => unreachable!()
                }
            }
        }
    )
}

bitwise_word_impl!(u8, i8, u8);
bitwise_word_impl!(u16, i16, u16);
bitwise_word_impl!(u32, i32, u32);
bitwise_word_impl!(u64, i64, u64);
bitwise_word_impl!(usize, isize, usize);
bitwise_word_impl!(i8, i8, u8);
bitwise_word_impl!(i16, i16, u16);
bitwise_word_impl!(i32, i32, u32);
bitwise_word_impl!(i64, i64, u64);
bitwise_word_impl!(isize, isize, usize);

/// Bitwise manimpulation algorithms for Word sequences.
pub trait Words {
    /// Returns the number of ones in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitwise::Words;
    ///
    /// let n = 0b0100_1100u8;
    ///
    /// assert_eq!(n.count_ones(), 3);
    ///
    /// let ns0 = [0u8, 1u8, 0b0100_1100u8];
    /// let ns1 = [1u64, 0u64, 0b0100_1100u64];
    ///
    /// assert_eq!(ns0.count_ones(), 4);
    /// assert_eq!(ns1.count_ones(), 4);
    /// ```
    fn count_ones(&self) -> usize;

    /// Returns the number of zeros in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitwise::Words;
    ///
    /// let n = 0b0100_1100u8;
    ///
    /// assert_eq!(n.count_zeros(), 5);
    ///
    /// let ns = [0u8, 1u8, 0b0100_1100u8];
    ///
    /// assert_eq!(ns.count_zeros(), 8 + 7 + 5);
    /// ```
    fn count_zeros(&self) -> usize;

    /// Returns the number of leading zeros in the binary representation of
    /// `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitwise::Words;
    ///
    /// let n = 0b0010_1000u16;
    ///
    /// assert_eq!(n.leading_zeros(), 10);
    ///
    /// let ns = [0u8, 0b0010_1000u8, 1u8];
    ///
    /// assert_eq!(ns.leading_zeros(), 10);
    /// ```
    fn leading_zeros(&self) -> usize;

    /// Size of the word sequence.
    fn size(&self) -> usize;
}

impl<T: Word> Words for T {
    fn size(&self) -> usize {
        <Self as Word>::size() as usize
    }
    fn count_ones(&self) -> usize {
        <Self as Word>::count_ones(*self) as usize
    }
    fn count_zeros(&self) -> usize {
        <Self as Word>::count_zeros(*self) as usize
    }
    fn leading_zeros(&self) -> usize {
        <Self as Word>::leading_zeros(*self) as usize
    }
}

impl<T: Words> Words for [T] {
    fn size(&self) -> usize {
        self.iter().fold(0usize, |sum, i| sum + i.size())
    }
    fn count_ones(&self) -> usize {
        self.iter().fold(0usize, |sum, i| sum + i.count_ones())
    }
    fn count_zeros(&self) -> usize {
        self.iter().fold(0usize, |sum, i| sum + i.count_zeros())
    }
    fn leading_zeros(&self) -> usize {
        // TODO: transmute into the largest possible word size and count leading
        // zeros on that, e.g., given [u32; 5]
        // => [u64; 2].leading_zeros() ?+ [u32; 1].leading_zeros()
        let mut sum: usize = 0;
        for i in self {
            let tmp: usize = i.leading_zeros();
            sum += tmp;
            if tmp != i.size() * 8 {
                break;
            }
        }
        sum
    }
}
