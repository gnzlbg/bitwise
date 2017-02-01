use word::Word;

/// Reverses the bytes of `x`.
///
/// Equivalent to swap bytes.
///
/// # Intrinsics:
/// - x86_64: bswap.
/// - ARM: rev (v5), revsh (v5), rev16 (v6,v8), rev32(v8).
/// - gcc/llvm: `__builtin_bswap16/32/64`.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// assert_eq!(0b1011_0010u8.reverse_bytes(), 0b1011_0010u8);
/// assert_eq!(0b1011_0010_1010_1001u16.reverse_bytes(), 0b1010_1001_1011_0010u16);
/// ```
pub fn reverse_bytes<T: Word>(x: T) -> T {
    x.swap_bytes()
}

/// Method version of [`reverse_bytes`](fn.reverse_bytes.html).
pub trait ReverseBytes: Word {
    fn reverse_bytes(self) -> Self;
}

impl<T: Word> ReverseBytes for T {
    fn reverse_bytes(self) -> T {
        reverse_bytes(self)
    }
}
