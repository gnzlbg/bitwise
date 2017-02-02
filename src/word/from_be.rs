use word::Word;

/// Convert integer from big endian to the target's endianness.
///
/// On big endian this is a no-op. On little endian the bytes are swapped.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0x0123456789ABCDEFu64;
///
/// if cfg!(target_endian = "big") {
///     assert_eq!(n.from_be(), n);
///     assert_eq!(from_be(n), n);
/// } else {
///     assert_eq!(n.from_be(), n.swap_bytes());
///     assert_eq!(from_be(n), n.swap_bytes());
/// }
/// ```
#[inline]
pub fn from_be<T: Word>(x: T) -> T {
    T::from_be(x)
}
