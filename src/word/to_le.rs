use word::Word;

/// Convert `x` to little endian from the target's endianness.
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
///     assert_eq!(n.to_le(), n);
///     assert_eq!(to_le(n), n);
/// } else {
///     assert_eq!(n.to_le(), n.swap_bytes());
///     assert_eq!(to_le(n), n.swap_bytes());
/// }
/// ```
pub fn to_le<T: Word>(x: T) -> T {
    T::to_le(x)
}
