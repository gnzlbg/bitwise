extern crate bitwise;
use bitwise::word::*;

fn main() {
    let u = outer_perfect_shuffle(0b_1001_1111u8);
    let v = inner_perfect_shuffle(0b_1001_1111u8);
    let w = u.copy_bit(4u8, v, 3u8);
    let z = w.parallel_bits_deposit(u);
}
