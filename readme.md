# Portable high-level bitwise manipulation algorithms

[![crates.io version][crate-shield]][crate] [![Travis build status][travis-shield]][travis] [![Coveralls.io code coverage][coveralls-shield]][coveralls] [![Docs][docs-shield]][docs] [![License][license-shield]][license]

> Work in progress.

The algorithms:

- have descriptive names to ease reading code that performs bit manipulations,
- often optimize to perfect assembly code (and _always_ on nightly by using
  the [bitintr][bitintr_link] crate),
- works on ~~stable~~ unstable only :( due to specialization for now.

## Example

```rust
extern crate bitwise;
use bitwise::word::*;

fn main() {
  let u = outer_perfect_shuffle(0b_1001_1111u8);
  let v = inner_perfect_shuffle(0b_1001_1111u8);
  let w = u.copy_bit(4u8, v, 3u8);
  let z = w.parallel_bits_deposit(u);
}
```

## Supported compilers

> The minimum required rustc version is >= **1.13.0**.

Requires unstable for now.

## Performance

Some algorithms like the Morton Z-Curve encoding/decoding routines switch
implementation at compile-time depending on target features (like BMI2 support).

## License

Licensed under the [MIT license][license].

## Acknowledgments

- Some of the algorithms are heavily inspired from Matthew Fioravante's 
  [N3864 A constexpr bitwise operations library for C++][n3864_link] proposal 
  and accompanying library: [stdcxx-bitops][stdcxx_bitops_link]. 
- Others are inspired from those in [Hacker's Delight](hackers_delight_link).
- The Morton Z-Curve encoding / decoding algorithms are adapted/modified from
  [libmorton][libmorton_link] (which resulted in bugfixes to libmorton).
- Some come from [Chess Programming Wiki][chess_programming_wiki_link].
- Others from [Real-Time Collision Detection][real_time_collision_detection_link].
- Many are adapted from the [bitintr][bitintr_link] protable bitwise
  manipulation intrinsics library.

## Contribution

Yes please! Just note that all contributions shall be licensed as above without
any additional terms or conditions.

<!-- Links -->
[travis-shield]: https://img.shields.io/travis/gnzlbg/bitwise.svg?style=flat-square
[travis]: https://travis-ci.org/gnzlbg/bitwise
[coveralls-shield]: https://img.shields.io/coveralls/gnzlbg/bitwise.svg?style=flat-square
[coveralls]: https://coveralls.io/github/gnzlbg/bitwise
[docs-shield]: https://img.shields.io/badge/docs-online-blue.svg?style=flat-square
[docs]: https://gnzlbg.github.io/bitwise
[license-shield]: https://img.shields.io/github/license/mashape/apistatus.svg?style=flat-square
[license]: https://github.com/gnzlbg/bitwise/blob/master/license.md
[crate-shield]: https://img.shields.io/crates/v/bitwise.svg?style=flat-square
[crate]: https://crates.io/crates/bitwise
[bitintr_link]: https://github.com/gnzlbg/bitintr
[n3864_link]: http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2014/n3864.html
[stdcxx_bitops_link]: https://github.com/fmatthew5876/stdcxx-bitops
[libmorton_link]: https://github.com/Forceflow/libmorton
[chess_programming_wiki_link]: https://chessprogramming.wikispaces.com/
[real_time_collision_detection_link]: http://realtimecollisiondetection.net/
