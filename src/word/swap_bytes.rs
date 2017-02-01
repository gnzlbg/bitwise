use word::Word;

/// Reverses the order of the bytes of `x`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0x0123456789ABCDEFu64;
/// let m = 0xEFCDAB8967452301u64;
///
/// assert_eq!(n.swap_bytes(), m);
/// assert_eq!(swap_bytes(n), m);
/// ```
pub fn swap_bytes<T: Word>(x: T) -> T {
    T::swap_bytes(x)
}

/// Method version of [`swap_bytes`](fn.swap_bytes.html).
pub trait SwapBytes {
    fn swap_bytes(self) -> Self;
}

impl<T: Word> SwapBytes for T {
    fn swap_bytes(self) -> Self {
        swap_bytes(self)
    }
}
