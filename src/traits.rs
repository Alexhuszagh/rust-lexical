//! High-level traits to translate the low-level API to idiomatic Rust.

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::string::String;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::vec::Vec;

use atof::*;
use atoi::*;

#[cfg(any(feature = "std", feature = "alloc"))]
use ftoa::*;

#[cfg(any(feature = "std", feature = "alloc"))]
use itoa::*;

// ATON

/// Trait for types that are deserializable from strings or bytes.
pub trait Aton: Sized {
    /// Deserialize from byte slice.
    fn deserialize_from_bytes(bytes: &[u8], base: u8) -> Self;

    /// Error-checking deserialize from byte slice.
    fn try_deserialize_from_bytes(bytes: &[u8], base: u8) -> Result<Self, usize>;
}

macro_rules! aton_impl {
    ($t:ty, $bytes_cb:ident, $try_bytes_cb:ident) => (
        impl Aton for $t {
            #[inline(always)]
            fn deserialize_from_bytes(bytes: &[u8], base: u8) -> $t
            {
                $bytes_cb(bytes, base)
            }

            #[inline(always)]
            fn try_deserialize_from_bytes(bytes: &[u8], base: u8) -> Result<$t, usize>
            {
                $try_bytes_cb(bytes, base)
            }
        }
    )
}

aton_impl!(u8, atou8_bytes, try_atou8_bytes);
aton_impl!(u16, atou16_bytes, try_atou16_bytes);
aton_impl!(u32, atou32_bytes, try_atou32_bytes);
aton_impl!(u64, atou64_bytes, try_atou64_bytes);
aton_impl!(i8, atoi8_bytes, try_atoi8_bytes);
aton_impl!(i16, atoi16_bytes, try_atoi16_bytes);
aton_impl!(i32, atoi32_bytes, try_atoi32_bytes);
aton_impl!(i64, atoi64_bytes, try_atoi64_bytes);
aton_impl!(f32, atof32_bytes, try_atof32_bytes);
aton_impl!(f64, atof64_bytes, try_atof64_bytes);

// NTOA

/// Trait for types that are serializable to string or bytes.
#[cfg(any(feature = "std", feature = "alloc"))]
pub trait Ntoa: Sized {
    /// Serialize to string.
    fn serialize_to_string(&self, base: u8) -> String;
}

#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! ntoa_impl {
    ($t:ty, $string_cb:ident) => (
        impl Ntoa for $t {
            #[inline(always)]
            fn serialize_to_string(&self, base: u8) -> String
            {
                $string_cb(*self, base)
            }
        }
    )
}

#[cfg(any(feature = "std", feature = "alloc"))]
ntoa_impl!(u8, u8toa_string);

#[cfg(any(feature = "std", feature = "alloc"))]
ntoa_impl!(u16, u16toa_string);

#[cfg(any(feature = "std", feature = "alloc"))]
ntoa_impl!(u32, u32toa_string);

#[cfg(any(feature = "std", feature = "alloc"))]
ntoa_impl!(u64, u64toa_string);

#[cfg(any(feature = "std", feature = "alloc"))]
ntoa_impl!(i8, i8toa_string);

#[cfg(any(feature = "std", feature = "alloc"))]
ntoa_impl!(i16, i16toa_string);

#[cfg(any(feature = "std", feature = "alloc"))]
ntoa_impl!(i32, i32toa_string);

#[cfg(any(feature = "std", feature = "alloc"))]
ntoa_impl!(i64, i64toa_string);

#[cfg(any(feature = "std", feature = "alloc"))]
ntoa_impl!(f32, f32toa_string);

#[cfg(any(feature = "std", feature = "alloc"))]
ntoa_impl!(f64, f64toa_string);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! deserialize_int {
        ($($t:tt)*) => ($({
            assert_eq!($t::deserialize_from_bytes(b"0", 10), 0);
            assert_eq!($t::try_deserialize_from_bytes(b"0", 10), Ok(0));
            assert_eq!($t::try_deserialize_from_bytes(b"", 10), Err(0));
            assert_eq!($t::try_deserialize_from_bytes(b"1a", 10), Err(1));
        })*)
    }

    macro_rules! deserialize_float {
        ($($t:tt)*) => ($({
            assert_eq!($t::deserialize_from_bytes(b"0.0", 10), 0.0);
            assert_eq!($t::try_deserialize_from_bytes(b"0.0", 10), Ok(0.0));
            assert_eq!($t::try_deserialize_from_bytes(b"0.0a", 10), Err(3));
        })*)
    }

    #[test]
    fn aton_test() {
        deserialize_int! { u8 u16 u32 u64 i8 i16 i32 i64 }
        deserialize_float! { f32 f64 }
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    macro_rules! serialize_int {
        ($($t:tt)*) => ($({
            let x: $t = 0;
            assert_eq!(x.serialize_to_string(10), "0");
        })*)
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    macro_rules! serialize_float {
        ($($t:tt)*) => ($({
            let x: $t = 0.0;
            assert_eq!(x.serialize_to_string(10), "0.0");
        })*)
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn ntoa_test() {
        serialize_int! { u8 u16 u32 u64 i8 i16 i32 i64 }
        serialize_float! { f32 f64 }
    }
}
