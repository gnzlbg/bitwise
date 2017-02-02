use word::Word;

/// Convert integer from little endian to the target's endianness.
///
/// On little endian this is a no-op. On big endian the bytes are swapped.
///
/// # Examples
///
/// ```
/// use bitwise::word::*;
///
/// let n = 0x0123456789ABCDEFu64;
///
/// if cfg!(target_endian = "little") {
///     assert_eq!(from_le(n), n);
///     assert_eq!(n.from_le(), n);
/// } else {
///     assert_eq!(from_le(n), n.swap_bytes());
///     assert_eq!(n.from_le(), n.swap_bytes());
/// }
/// ```
#[inline]
pub fn from_le<T: Word>(x: T) -> T {
    T::from_le(x)
}
