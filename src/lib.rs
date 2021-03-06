//! `bitwise` offers portable high-level manipulation algorithms
//! ([@github](https://github.com/gnzlbg/bitwise),
//! [@crates.io](https://crates.io/crates/bitwise)).
//!
//! The algorithms:
//!
//! - have descriptive names to ease reading code that performs bit manipulations,
//! - often optimize to perfect assembly code (and _always_ on nightly by using
//!   the [`bitintr`](https://crates.io/crates/bitintr) crate),
//! - works on ~~stable~~ unstable only :( due to specialization for now.
//!
//! ## Example
//!
//! ```rust
//! extern crate bitwise;
//! use bitwise::word::*;
//!
//! fn main() {
//!     let u = outer_perfect_shuffle(0b_1001_1111u8);
//!     let v = inner_perfect_shuffle(0b_1001_1111u8);
//!     let w = u.copy_bit(4u8, v, 3u8);
//!     assert_eq!(w.parallel_bits_deposit(u), 0b_1001_0011u8);
//! }
//! ```

#![cfg_attr(RUSTC_IS_NIGHTLY, feature(specialization))]
#![cfg_attr(RUSTC_IS_NIGHTLY, feature(cfg_target_feature))]
#![cfg_attr(RUSTC_IS_NIGHTLY, feature(test))]

#![no_std]
extern crate core as std;

#[cfg(RUSTC_IS_NIGHTLY)]
extern crate test;

#[cfg(test)]
extern crate quickcheck;

/// Low-level bitwise manipulation intrinsics.
pub extern crate bitintr;

pub mod word;

pub use word::*;
