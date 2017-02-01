pub mod word;

pub use bitintr::Int as Word;

//pub use self::to_word::*;
pub use self::word::*;

pub mod count_zeros;
pub use self::count_zeros::*;

pub mod count_ones;
pub use self::count_ones::*;

pub mod leading_zeros;
pub use self::leading_zeros::*;

pub mod leading_ones;
pub use self::leading_ones::*;

pub mod trailing_zeros;
pub use self::trailing_zeros::*;

pub mod trailing_ones;
pub use self::trailing_ones::*;

pub mod shift_logical_left;
pub use self::shift_logical_left::*;

pub mod shift_logical_right;
pub use self::shift_logical_right::*;

pub mod shift_arithmetic_left;
pub use self::shift_arithmetic_left::*;

pub mod shift_arithmetic_right;
pub use self::shift_arithmetic_right::*;

pub mod rotate_left;
pub use self::rotate_left::*;

pub mod rotate_right;
pub use self::rotate_right::*;

pub mod swap_bytes;
pub use self::swap_bytes::*;

pub mod from_be;
pub use self::from_be::*;

pub mod from_le;
pub use self::from_le::*;

pub mod to_be;
pub use self::to_be::*;

pub mod to_le;
pub use self::to_le::*;

pub mod pow;
pub use self::pow::*;

pub mod parity;
pub use self::parity::*;

pub mod clear_least_significant_one;
pub use self::clear_least_significant_one::*;

pub mod set_least_significant_zero;
pub use self::set_least_significant_zero::*;

pub mod isolate_least_significant_one;
pub use self::isolate_least_significant_one::*;

pub mod isolate_least_significant_zero;
pub use self::isolate_least_significant_zero::*;

pub mod clear_trailing_ones;
pub use self::clear_trailing_ones::*;

pub mod set_trailing_zeros;
pub use self::set_trailing_zeros::*;

pub mod mask_trailing_zeros;
pub use self::mask_trailing_zeros::*;

pub mod mask_trailing_ones;
pub use self::mask_trailing_ones::*;

pub mod mask_trailing_zeros_and_least_significant_one;
pub use self::mask_trailing_zeros_and_least_significant_one::*;

pub mod mask_trailing_ones_and_least_significant_zero;
pub use self::mask_trailing_ones_and_least_significant_zero::*;

pub mod set_bit;
pub use self::set_bit::*;

pub mod clear_bit;
pub use self::clear_bit::*;

pub mod flip_bit;
pub use self::flip_bit::*;

pub mod test_bit;
pub use self::test_bit::*;

pub mod copy_bit;
pub use self::copy_bit::*;

pub mod reverse_bit_groups;
pub use self::reverse_bit_groups::*;

pub mod reverse_bits;
pub use self::reverse_bits::*;

pub mod reverse_bit_pairs;
pub use self::reverse_bit_pairs::*;

pub mod reverse_bit_nibbles;
pub use self::reverse_bit_nibbles::*;

pub mod reverse_byte_groups;
pub use self::reverse_byte_groups::*;

pub mod reverse_bytes;
pub use self::reverse_bytes::*;

pub mod clear_bits_geq;
pub use self::clear_bits_geq::*;

pub mod clear_bits_leq;
pub use self::clear_bits_leq::*;

pub mod set_bits_geq;
pub use self::set_bits_geq::*;

pub mod set_bits_leq;
pub use self::set_bits_leq::*;

pub mod flip_bits_geq;
pub use self::flip_bits_geq::*;

pub mod flip_bits_leq;
pub use self::flip_bits_leq::*;

pub mod is_pow2;
pub use self::is_pow2::*;

pub mod ceil_pow2;
pub use self::ceil_pow2::*;

pub mod floor_pow2;
pub use self::floor_pow2::*;

pub mod is_aligned;
pub use self::is_aligned::*;

pub mod align_up;
pub use self::align_up::*;

pub mod align_down;
pub use self::align_down::*;

pub mod outer_perfect_shuffle;
pub use self::outer_perfect_shuffle::*;

pub mod outer_perfect_unshuffle;
pub use self::outer_perfect_unshuffle::*;

pub mod inner_perfect_shuffle;
pub use self::inner_perfect_shuffle::*;

pub mod inner_perfect_unshuffle;
pub use self::inner_perfect_unshuffle::*;

pub mod parallel_bits_deposit;
pub use self::parallel_bits_deposit::*;

pub mod parallel_bits_extract;
pub use self::parallel_bits_extract::*;

pub mod morton;
pub use self::morton::*;
