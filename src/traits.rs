//! High-level traits to translate the low-level API to idiomatic Rust.

use atof::*;
use atoi::*;
use error::Error;
use lib;

// TODO(ahuszagh) Need to re-conceive this?

cfg_if! {
    if #[cfg(any(feature = "std", feature = "alloc"))] {
        use ftoa::*;
        use itoa::*;
    }
}

// FROM STRING

/// Trait for types that are deserializable from strings or bytes.
pub trait FromBytes: Sized {
    /// Deserialize from byte slice.
    fn from_bytes(bytes: &[u8], base: u8) -> Self;

    /// Error-checking deserialize from byte slice.
    fn try_from_bytes(bytes: &[u8], base: u8) -> Result<Self, Error>;
}

macro_rules! from_bytes {
    ($t:ty, $bytes_cb:ident, $try_bytes_cb:ident) => (
        impl FromBytes for $t {
            #[inline(always)]
            fn from_bytes(bytes: &[u8], base: u8) -> $t
            {
                $bytes_cb(bytes, base)
            }

            #[inline(always)]
            fn try_from_bytes(bytes: &[u8], base: u8) -> Result<$t, Error>
            {
                $try_bytes_cb(bytes, base)
            }
        }
    )
}

from_bytes!(u8, atou8_bytes, try_atou8_bytes);
from_bytes!(u16, atou16_bytes, try_atou16_bytes);
from_bytes!(u32, atou32_bytes, try_atou32_bytes);
from_bytes!(u64, atou64_bytes, try_atou64_bytes);
from_bytes!(usize, atousize_bytes, try_atousize_bytes);
from_bytes!(i8, atoi8_bytes, try_atoi8_bytes);
from_bytes!(i16, atoi16_bytes, try_atoi16_bytes);
from_bytes!(i32, atoi32_bytes, try_atoi32_bytes);
from_bytes!(i64, atoi64_bytes, try_atoi64_bytes);
from_bytes!(isize, atoisize_bytes, try_atoisize_bytes);
from_bytes!(f32, atof32_bytes, try_atof32_bytes);
from_bytes!(f64, atof64_bytes, try_atof64_bytes);

// NTOA

cfg_if! {
    if #[cfg(any(feature = "std", feature = "alloc"))] {
        /// Trait for types that are serializable to string or bytes.
        pub trait ToBytes: Sized {
            /// Serialize to string.
            // TODO(ahuszagh) Needs to actually export bytes...
            // Just wrap it even higher level to string.
            fn to_bytes(&self, base: u8) -> lib::String;
        }

        macro_rules! ntoa_impl {
            ($t:ty, $string_cb:ident) => (
                impl ToBytes for $t {
                    #[inline(always)]
                    fn to_bytes(&self, base: u8) -> lib::String
                    {
                        $string_cb(*self, base)
                    }
                }
            )
        }

        ntoa_impl!(u8, u8toa_string);
        ntoa_impl!(u16, u16toa_string);
        ntoa_impl!(u32, u32toa_string);
        ntoa_impl!(u64, u64toa_string);
        ntoa_impl!(usize, usizetoa_string);
        ntoa_impl!(i8, i8toa_string);
        ntoa_impl!(i16, i16toa_string);
        ntoa_impl!(i32, i32toa_string);
        ntoa_impl!(i64, i64toa_string);
        ntoa_impl!(isize, isizetoa_string);
        ntoa_impl!(f32, f32toa_string);
        ntoa_impl!(f64, f64toa_string);
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use error::invalid_digit;
    use super::*;

    macro_rules! deserialize_int {
        ($($t:tt)*) => ($({
            assert_eq!($t::from_bytes(b"0", 10), 0);
            assert_eq!($t::try_from_bytes(b"0", 10), Ok(0));
            assert_eq!($t::try_from_bytes(b"", 10), Err(invalid_digit(0)));
            assert_eq!($t::try_from_bytes(b"1a", 10), Err(invalid_digit(1)));
        })*)
    }

    macro_rules! deserialize_float {
        ($($t:tt)*) => ($({
            assert_eq!($t::from_bytes(b"0.0", 10), 0.0);
            assert_eq!($t::try_from_bytes(b"0.0", 10), Ok(0.0));
            assert_eq!($t::try_from_bytes(b"0.0a", 10), Err(invalid_digit(3)));
        })*)
    }

    #[test]
    fn aton_test() {
        deserialize_int! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }
        deserialize_float! { f32 f64 }
    }

    cfg_if! {
        if #[cfg(any(feature = "std", feature = "alloc"))] {
            macro_rules! serialize_int {
                ($($t:tt)*) => ($({
                    let x: $t = 0;
                    assert_eq!(x.to_bytes(10), "0");
                })*)
            }

            macro_rules! serialize_float {
                ($($t:tt)*) => ($({
                    let x: $t = 0.0;
                    assert_eq!(x.to_bytes(10), "0.0");
                })*)
            }

            #[test]
            fn ntoa_test() {
                serialize_int! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }
                serialize_float! { f32 f64 }
            }
        }
    }
}
