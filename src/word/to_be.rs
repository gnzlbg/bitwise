use word::Word;

/// Convert `x` to big endian from the target's endianness.
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
///     assert_eq!(n.to_be(), n);
///     assert_eq!(to_be(n), n);
/// } else {
///     assert_eq!(n.to_be(), n.swap_bytes());
///     assert_eq!(to_be(n), n.swap_bytes());
/// }
/// ```
pub fn to_be<T: Word>(x: T) -> T {
    T::to_be(x)
}
