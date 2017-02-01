//! Bitwise manipulation algorithms for `Word`s and sequences of `Words`.

#![cfg_attr(RUSTC_IS_NIGHTLY, feature(specialization))]
#![cfg_attr(RUSTC_IS_NIGHTLY, feature(cfg_target_feature))]
#![cfg_attr(RUSTC_IS_NIGHTLY, feature(test))]

#[cfg(RUSTC_IS_NIGHTLY)]
extern crate test;

extern crate bitintr;

pub mod word;

pub use word::*;
