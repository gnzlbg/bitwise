#![allow(dead_code)]

#[macro_use]
extern crate bencher;

extern crate bitwise;

use bencher::Bencher;

use bitwise::word::gcd;



pub fn run_u8_small<F: Fn(u8, u8) -> u8>(b: &mut Bencher, f: F) {
    b.iter(|| gcd::test_util::run_u8(0, u8::max_value() / 3,
                                     |x, y| bencher::black_box(f(bencher::black_box(x),
                                                                 bencher::black_box(y)))));
}

pub fn run_u8_mid<F: Fn(u8, u8) -> u8>(b: &mut Bencher, f: F) {
    b.iter(|| gcd::test_util::run_u8(u8::max_value() / 3, u8::max_value() / 3 * 2,
                                     |x, y| bencher::black_box(f(bencher::black_box(x),
                                                                 bencher::black_box(y)))));
}

pub fn run_u8_large<F: Fn(u8, u8) -> u8>(b: &mut Bencher, f: F) {
    b.iter(|| gcd::test_util::run_u8(u8::max_value() / 3 * 2, u8::max_value(),
                                     |x, y| bencher::black_box(f(bencher::black_box(x),
                                                                 bencher::black_box(y)))));
}

fn u08_small_euclid_recursive(b: &mut Bencher) {
    run_u8_small(b, gcd::euclid::recursive)    
}

fn u08_small_euclid_iterative(b: &mut Bencher) {
    run_u8_small(b, gcd::euclid::iterative)    
}

fn u08_small_steins_recursive(b: &mut Bencher) {
    run_u8_small(b, gcd::steins::recursive)    
}

fn u08_small_steins_iterative(b: &mut Bencher) {
    run_u8_small(b, gcd::steins::iterative)    
}

fn u08_small_steins_iterative_xor(b: &mut Bencher) {
    run_u8_small(b, gcd::steins::iterative_xor)    
}


benchmark_group!(u08_small_g,
                 u08_small_euclid_recursive,
                 u08_small_euclid_iterative,
                 u08_small_steins_recursive,
                 u08_small_steins_iterative,
                 u08_small_steins_iterative_xor
                 );

fn u08_mid_euclid_recursive(b: &mut Bencher) {
    run_u8_mid(b, gcd::euclid::recursive)    
}

fn u08_mid_euclid_iterative(b: &mut Bencher) {
    run_u8_mid(b, gcd::euclid::iterative)    
}

fn u08_mid_steins_recursive(b: &mut Bencher) {
    run_u8_mid(b, gcd::steins::recursive)    
}

fn u08_mid_steins_iterative(b: &mut Bencher) {
    run_u8_mid(b, gcd::steins::iterative)    
}

fn u08_mid_steins_iterative_xor(b: &mut Bencher) {
    run_u8_mid(b, gcd::steins::iterative_xor)    
}


benchmark_group!(u08_mid_g,
                 u08_mid_euclid_recursive,
                 u08_mid_euclid_iterative,
                 u08_mid_steins_recursive,
                 u08_mid_steins_iterative,
                 u08_mid_steins_iterative_xor
);


fn u08_large_euclid_recursive(b: &mut Bencher) {
    run_u8_large(b, gcd::euclid::recursive)    
}

fn u08_large_euclid_iterative(b: &mut Bencher) {
    run_u8_large(b, gcd::euclid::iterative)    
}

fn u08_large_steins_recursive(b: &mut Bencher) {
    run_u8_large(b, gcd::steins::recursive)    
}

fn u08_large_steins_iterative(b: &mut Bencher) {
    run_u8_large(b, gcd::steins::iterative)    
}

fn u08_large_steins_iterative_xor(b: &mut Bencher) {
    run_u8_large(b, gcd::steins::iterative_xor)    
}


benchmark_group!(u08_large_g,
                 u08_large_euclid_recursive,
                 u08_large_euclid_iterative,
                 u08_large_steins_recursive,
                 u08_large_steins_iterative,
                 u08_large_steins_iterative_xor
);




pub fn run_u16_small<F: Fn(u16, u16) -> u16>(b: &mut Bencher, f: F) {
    b.iter(|| gcd::test_util::run_u16(0, u16::max_value() / 3,
                                     |x, y| bencher::black_box(f(bencher::black_box(x),
                                                                 bencher::black_box(y)))));
}

pub fn run_u16_mid<F: Fn(u16, u16) -> u16>(b: &mut Bencher, f: F) {
    b.iter(|| gcd::test_util::run_u16(u16::max_value() / 3, u16::max_value() / 3 * 2,
                                     |x, y| bencher::black_box(f(bencher::black_box(x),
                                                                 bencher::black_box(y)))));
}

pub fn run_u16_large<F: Fn(u16, u16) -> u16>(b: &mut Bencher, f: F) {
    b.iter(|| gcd::test_util::run_u16(u16::max_value() / 6 * 5 , u16::max_value(),
                                     |x, y| bencher::black_box(f(bencher::black_box(x),
                                                                 bencher::black_box(y)))));
}

fn u16_small_euclid_recursive(b: &mut Bencher) {
    run_u16_small(b, gcd::euclid::recursive)    
}

fn u16_small_euclid_iterative(b: &mut Bencher) {
    run_u16_small(b, gcd::euclid::iterative)    
}

fn u16_small_steins_recursive(b: &mut Bencher) {
    run_u16_small(b, gcd::steins::recursive)    
}

fn u16_small_steins_iterative(b: &mut Bencher) {
    run_u16_small(b, gcd::steins::iterative)    
}

fn u16_small_steins_iterative_xor(b: &mut Bencher) {
    run_u16_small(b, gcd::steins::iterative_xor)    
}
/*

benchmark_group!(u16_small_g,
                 u16_small_euclid_recursive,
                 u16_small_euclid_iterative,
                 u16_small_steins_recursive,
                 u16_small_steins_iterative,
                 u16_small_steins_iterative_xor
                 );

fn u16_mid_euclid_recursive(b: &mut Bencher) {
    run_u16_mid(b, gcd::euclid::recursive)    
}

fn u16_mid_euclid_iterative(b: &mut Bencher) {
    run_u16_mid(b, gcd::euclid::iterative)    
}

fn u16_mid_steins_recursive(b: &mut Bencher) {
    run_u16_mid(b, gcd::steins::recursive)    
}

fn u16_mid_steins_iterative(b: &mut Bencher) {
    run_u16_mid(b, gcd::steins::iterative)    
}

fn u16_mid_steins_iterative_xor(b: &mut Bencher) {
    run_u16_mid(b, gcd::steins::iterative_xor)    
}


benchmark_group!(u16_mid_g,
                 u16_mid_euclid_recursive,
                 u16_mid_euclid_iterative,
                 u16_mid_steins_recursive,
                 u16_mid_steins_iterative,
                 u16_mid_steins_iterative_xor
);


fn u16_large_euclid_recursive(b: &mut Bencher) {
    run_u16_large(b, gcd::euclid::recursive)    
}

fn u16_large_euclid_iterative(b: &mut Bencher) {
    run_u16_large(b, gcd::euclid::iterative)    
}

fn u16_large_steins_recursive(b: &mut Bencher) {
    run_u16_large(b, gcd::steins::recursive)    
}

fn u16_large_steins_iterative(b: &mut Bencher) {
    run_u16_large(b, gcd::steins::iterative)    
}

fn u16_large_steins_iterative_xor(b: &mut Bencher) {
    run_u16_large(b, gcd::steins::iterative_xor)    
}

benchmark_group!(u16_large_g,
                 //u16_large_euclid_recursive,
                 //u16_large_euclid_iterative,
                 //u16_large_steins_recursive,
                 //u16_large_steins_iterative,
                 u16_large_steins_iterative_xor
);
*/


benchmark_main!(u08_small_g, u08_mid_g, u08_large_g);
