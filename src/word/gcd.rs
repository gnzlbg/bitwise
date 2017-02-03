//! Greatest Common Divisor algorithms that use bit manipulation.
//!
//! Note: this is not intended to provide _the_ fastest GCD implementation
//! possible, but only to provide those algorithms that are implemented using
//! bit manipulation.
//!
//! A fast GCD implementation will probably want to switch algorithms depending
//! on the size of the input, and will involve other algorithms like Lehmer's
//! and probably parallelization. The `benches/gcd.rs` benchmarks might be
//! useful for making these decisions.

use word::Word;

pub mod test_util {
    use super::Word;

    pub fn invariant<T: Word>(x: T, y: T, gcd: T) -> bool {
        if x == T::zero() {
            gcd == y
        } else if y == T::zero() {
            gcd == x
        } else {
            x % gcd == T::zero() && y % gcd == T::zero()
        }
    }


    pub fn run_u8<F: Fn(u8, u8) -> u8>(from: u8, to: u8, f: F) {
        (from..to)
            .map(|x| {
                let xs: [u8; 1] = [x];
                (from..to)
                    .zip(xs.iter().cycle())
                    .map(|(x, &y)| {
                        let gcd = f(x, y);
                        assert!(invariant(x, y, gcd))
                    })
                    .count();
            })
            .count();
    }

    pub fn run_u16<F: Fn(u16, u16) -> u16>(from: u16, to: u16, f: F) {
        (from..to)
            .map(|x| {
                let xs: [u16; 1] = [x];
                (from..to)
                    .zip(xs.iter().cycle())
                    .map(|(x, &y)| {
                        let gcd = f(x, y);
                        assert!(invariant(x, y, gcd))
                    })
                    .count();
            })
            .count();
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_u8() {
        test_util::run_u8(0, u8::max_value(), euclid::recursive);
        test_util::run_u8(0, u8::max_value(), euclid::iterative);
        test_util::run_u8(0, u8::max_value(), steins::recursive);
        test_util::run_u8(0, u8::max_value(), steins::iterative);
        test_util::run_u8(0, u8::max_value(), steins::iterative_xor);
    }

}

pub mod euclid {
    //! Recursive implementation of the GCD algorithm.
    use word::Word;

    #[inline]
    pub fn recursive<T: Word>(x: T, y: T) -> T {
        if y == T::zero() {
            x
        } else {
            recursive(y, x % y)
        }
    }

    #[inline]
    pub fn iterative<T: Word>(x: T, y: T) -> T {
        let mut x = x;
        let mut y = y;
        while y != T::zero() {
            let t = y;
            y = x % y;
            x = t;
        }
        x
    }
}

pub mod steins {
    use std;
    use word::{Word, IsEven, IsOdd};
    #[inline] pub fn recursive<T: Word>(x: T, y: T) -> T {
        match (x, y) {
            (x, y) if (x == y) => x,
            (x, y) if (x == T::zero()) => y,
            (x, y) if (y == T::zero()) => x,
            (x, y) => {
                match (x.is_odd(), y.is_odd()) {
                    (false, false) => recursive(x >> T::one(), y >> T::one()) << T::one(),
                    (false, true) => recursive(x >> T::one(), y),
                    (true, false) => recursive(x, y >> T::one()),
                    (true, true) => {
                        if x >= y {
                            recursive((x - y) >> T::one(), x)
                        } else {
                            recursive((y - x) >> T::one(), x)
                        }
                    }
                }
            }

        }
    }

    #[inline]pub fn iterative<T: Word>(x: T, y: T) -> T {
        if x == T::zero() { return y; }
        if y == T::zero() { return x; }

        let mut x = x;
        let mut y = y;
        let mut shift = T::zero();
        while (x | y).is_even() {
            x = x >> T::one();
            y = y >> T::one();
            shift = shift + T::one();
        }

        while x.is_even() {
            x = x >> T::one();
        }


        loop {
            while y.is_even() {
                y = y >> T::one();
            }

            if x > y {
                std::mem::swap(&mut x, &mut y);
            }
            y = y - x;

            if y == T::zero() { break; }

        }
        x << shift
    }


    #[inline]pub fn iterative_xor<T: Word>(x: T, y: T) -> T {
        let mut x = x;
        let mut y = y;
        while y != T::zero() {
            x = x % y;
            y = y ^ x;
            x = x ^ y;
            y = y ^ x;
        }
        x
    }
}

/*


/// Greatest Common Divisor (GCD) of `x` and `y`.
///
///
/// # Keywords:
///
/// Greatest Common Divisor, GCD. 
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// ```
#[inline]
pub fn greatest_common_divisor<T: Word>(x: T, y: T) -> T {
    
}

/// Method version of [`greatest_common_divisor`](fn.greatest_common_divisor.html).
pub trait GCD {
    #[inline]
    fn greatest_common_divisor(self, Self) -> Self;
}

impl<T: Word> GCD for T {
    #[inline]
    fn greatest_common_divisor(self, y: Self) -> Self {
        greatest_common_divisor(self, y)
    }
}
*/
