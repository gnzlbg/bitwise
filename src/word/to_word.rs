use word::Word;

/// Unsigned word.
pub trait UnsignedWord: Word {}
impl UnsignedWord for u8 {}
impl UnsignedWord for u16 {}
impl UnsignedWord for u32 {}
impl UnsignedWord for u64 {}

/// From-like trait for words.
pub trait FromWord<T> {
    /// Converts a `T` to `Self`.
    #[inline]
    fn from(T) -> Self;
}

macro_rules! impl_from_word_from_to {
    ($From:ty, $To:ty) => (
        impl FromWord<$From> for $To {
            #[inline] fn from(x: $From) -> Self {
                x as Self
            }
        }
    )
}

macro_rules! impl_from_word_multiple {
    ($From:ty, $To:ty) => (
        impl_from_word_from_to!($From, $To);
    );
    ($From:ty, $To:ty $(, $Rest:ty)+) => (
        impl_from_word_multiple!($From, $To);
        impl_from_word_multiple!($From, $($Rest),*);
    )
}

macro_rules! impl_from_word {
    ($From:ty) => (
        impl_from_word_multiple!($From, i8, i16, i32, i64,
                                 u8, u16, u32, u64);
    )
}

impl_from_word!(i8);
impl_from_word!(i16);
impl_from_word!(i32);
impl_from_word!(i64);
impl_from_word!(u8);
impl_from_word!(u16);
impl_from_word!(u32);
impl_from_word!(u64);

impl<T: Word, U: Word> FromWord<T> for U {
    #[inline]
    default fn from(x: T) -> Self {
        ToWord::to(x)
    }
}

/// Into-like trait for words.
pub trait ToWord<T> {
    /// Converts self to `T`.
    #[inline]
    fn to(self) -> T;
}

// From implies Into
impl<T, U> ToWord<U> for T
    where U: FromWord<T>
{
    #[inline]
    fn to(self) -> U {
        U::from(self)
    }
}
