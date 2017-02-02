//! Algorithms for single words (u8...u64).

mod to_word;

pub use bitintr::Int as Word;

pub use self::to_word::*;

mod count_zeros;
pub use self::count_zeros::*;

mod count_ones;
pub use self::count_ones::*;

mod count_leading_zeros;
pub use self::count_leading_zeros::*;

mod count_leading_ones;
pub use self::count_leading_ones::*;

mod count_trailing_zeros;
pub use self::count_trailing_zeros::*;

mod count_trailing_ones;
pub use self::count_trailing_ones::*;

mod shift_logical_left;
pub use self::shift_logical_left::*;

mod shift_logical_right;
pub use self::shift_logical_right::*;

mod shift_arithmetic_left;
pub use self::shift_arithmetic_left::*;

mod shift_arithmetic_right;
pub use self::shift_arithmetic_right::*;

mod rotate_left;
pub use self::rotate_left::*;

mod rotate_right;
pub use self::rotate_right::*;

mod swap_bytes;
pub use self::swap_bytes::*;

mod from_be;
pub use self::from_be::*;

mod from_le;
pub use self::from_le::*;

mod to_be;
pub use self::to_be::*;

mod to_le;
pub use self::to_le::*;

mod pow;
pub use self::pow::*;

mod parity;
pub use self::parity::*;

mod clear_least_significant_one;
pub use self::clear_least_significant_one::*;

mod set_least_significant_zero;
pub use self::set_least_significant_zero::*;

mod isolate_least_significant_one;
pub use self::isolate_least_significant_one::*;

mod isolate_least_significant_zero;
pub use self::isolate_least_significant_zero::*;

mod clear_trailing_ones;
pub use self::clear_trailing_ones::*;

mod set_trailing_zeros;
pub use self::set_trailing_zeros::*;

mod mask_trailing_zeros;
pub use self::mask_trailing_zeros::*;

mod mask_trailing_ones;
pub use self::mask_trailing_ones::*;

mod mask_trailing_zeros_and_least_significant_one;
pub use self::mask_trailing_zeros_and_least_significant_one::*;

mod mask_trailing_ones_and_least_significant_zero;
pub use self::mask_trailing_ones_and_least_significant_zero::*;

mod set_bit;
pub use self::set_bit::*;

mod clear_bit;
pub use self::clear_bit::*;

mod flip_bit;
pub use self::flip_bit::*;

mod test_bit;
pub use self::test_bit::*;

mod copy_bit;
pub use self::copy_bit::*;

mod reverse_bit_groups;
pub use self::reverse_bit_groups::*;

mod reverse_bits;
pub use self::reverse_bits::*;

mod reverse_bit_pairs;
pub use self::reverse_bit_pairs::*;

mod reverse_bit_nibbles;
pub use self::reverse_bit_nibbles::*;

mod reverse_byte_groups;
pub use self::reverse_byte_groups::*;

mod reverse_bytes;
pub use self::reverse_bytes::*;

mod clear_bits_geq;
pub use self::clear_bits_geq::*;

mod clear_bits_leq;
pub use self::clear_bits_leq::*;

mod set_bits_geq;
pub use self::set_bits_geq::*;

mod set_bits_leq;
pub use self::set_bits_leq::*;

mod flip_bits_geq;
pub use self::flip_bits_geq::*;

mod flip_bits_leq;
pub use self::flip_bits_leq::*;

mod is_pow2;
pub use self::is_pow2::*;

mod ceil_pow2;
pub use self::ceil_pow2::*;

mod floor_pow2;
pub use self::floor_pow2::*;

mod is_aligned;
pub use self::is_aligned::*;

mod align_up;
pub use self::align_up::*;

mod align_down;
pub use self::align_down::*;

mod outer_perfect_shuffle;
pub use self::outer_perfect_shuffle::*;

mod outer_perfect_unshuffle;
pub use self::outer_perfect_unshuffle::*;

mod inner_perfect_shuffle;
pub use self::inner_perfect_shuffle::*;

mod inner_perfect_unshuffle;
pub use self::inner_perfect_unshuffle::*;

mod parallel_bits_deposit;
pub use self::parallel_bits_deposit::*;

mod parallel_bits_extract;
pub use self::parallel_bits_extract::*;

mod extract_bits;
pub use self::extract_bits::*;

pub mod morton;
pub use morton::decode_2d as morton_decode_2d;
pub use morton::decode_3d as morton_decode_3d;
pub use morton::encode_2d as morton_encode_2d;
pub use morton::encode_3d as morton_encode_3d;
