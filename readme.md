# Portable high-level bitwise manipulation algorithms

[![crates.io version][crate-shield]][crate] [![Travis build status][travis-shield]][travis] [![Coveralls.io code coverage][coveralls-shield]][coveralls] [![Docs][docs-shield]][docs] [![License][license-shield]][license]

> We do what we must because we can.

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
  assert_eq!(w.parallel_bits_deposit(u), 0b_1001_0011u8);
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

The giants that came before us:

- Matthew  Fioravante's
  [N3864 A constexpr bitwise operations library for C++][n3864_link] proposal
  and accompanying library: [stdcxx-bitops][stdcxx_bitops_link].
- Henry S. Warren's [Hacker's Delight][hackers_delight_link].
- Jeroen Baert's [libmorton][libmorton_link].
- The [Chess Programming Wiki][chess_programming_wiki_link], in particular
  the [bit-twiddling][chess_programming_bit_twiddling_link] section.
- [Real-Time Collision Detection][real_time_collision_detection_link].
- My own's [bitintr][bitintr_link] library.
- Jasper Neumann's [programming pages][jasper_neumann_site_link].
- Jörg Arndt's [Matters Computational: Ideas, Algorithms, Source Code][matters_computational_link].

## Contribution

Yes please! Just note that all contributions shall be licensed as above without
any additional terms or conditions. The following people have contributed code to this library:

- [Gonzalo Brito Gadeschi](https://github.com/gnzlbg) (main author).
- [Brian L. Troutwine](https://github.com/blt).

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
[n3864_link]: https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2014/n3864.html
[stdcxx_bitops_link]: https://github.com/fmatthew5876/stdcxx-bitops
[libmorton_link]: https://github.com/Forceflow/libmorton
[chess_programming_wiki_link]: https://www.chessprogramming.org
[chess_programming_bit_twiddling_link]: https://www.chessprogramming.org/Bit-Twiddling
[real_time_collision_detection_link]: https://realtimecollisiondetection.net/
[hackers_delight_link]: https://hackersdelight.org/
[jasper_neumann_site_link]: https://programming.sirrida.de/index.php
[matters_computational_link]: https://www.jjj.de/fxt/fxtbook.pdf
