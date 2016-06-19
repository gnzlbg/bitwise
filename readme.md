# Bitwise manipulation algorithms

[![Travis build status][travis-shield]][travis] [![Coveralls.io code coverage][coveralls-shield]][coveralls] [![Docs][docs-shield]][docs]

<!---
clippy linting doesn't seem to work
[![Clippy Linting Result](https://clippy.bashy.io/github/gnzlbh/bitwise/master/badge.svg)]
-->

The algorithms are implemented for single words and slices of words in the
`Word` and `Words` traits, respectively.

The algorithms are heavily inspired and/or directly taken from Matthew
Fioravante's
[N3864 A constexpr bitwise operations library for C++](http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2014/n3864.html)
proposal and accompanying library:
[stdcxx-bitops](https://github.com/fmatthew5876/stdcxx-bitops).

<!-- Links -->
[travis-shield]: https://img.shields.io/travis/gnzlbg/bitwise.svg?style=flat-square
[travis]: https://travis-ci.org/gnzlbg/bitwise
[coveralls-shield]: https://img.shields.io/coveralls/gnzlbg/bitwise.svg?style=flat-square
[coveralls]: https://coveralls.io/github/gnzlbg/bitwise
[docs-shield]: https://img.shields.io/badge/docs-online-blue.svg?style=flat-square
[docs]: https://gnzlbg.github.io/bitwise
