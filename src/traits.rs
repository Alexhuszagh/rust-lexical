//! High-level traits to translate the low-level API to idiomatic Rust.

use lexical_core::{self, ErrorCode};
use lib::{slice, Vec};
use error::*;

// HELPERS

/// Get a vector as a slice, including the capacity.
#[inline]
unsafe fn vector_as_slice<'a, T>(buf: &'a mut Vec<T>)
    -> &'a mut [T]
{
    let first = buf.as_mut_ptr();
    slice::from_raw_parts_mut(first, buf.capacity())
}

/// Convert a C-compatible result to an idiomatic Rust one.
#[inline]
fn convert_result<T>(result: lexical_core::Result<T>) -> Result<T, Error> {
    match result.error.code {
        ErrorCode::Success      => Ok(result.value),
        ErrorCode::Overflow     => Err(overflow()),
        ErrorCode::InvalidDigit => Err(invalid_digit(result.error.index)),
        ErrorCode::Empty        => Err(empty()),
        _                       => unimplemented!(),
    }
}

// FROM BYTES

/// Trait for numerical types that can be parsed from bytes.
pub trait FromBytes: Sized {
    /// Deserialize from byte slice.
    fn from_bytes(bytes: &[u8]) -> Self;

    /// Deserialize from byte slice with radix.
    #[cfg(feature = "radix")]
    fn from_bytes_radix(bytes: &[u8], radix: u8) -> Self;

    /// Error-checking deserialize from byte slice.
    fn try_from_bytes(bytes: &[u8]) -> Result<Self, Error>;

    /// Error-checking deserialize from byte slice with radix.
    #[cfg(feature = "radix")]
    fn try_from_bytes_radix(bytes: &[u8], radix: u8) -> Result<Self, Error>;
}

macro_rules! from_bytes {
    ($t:ty, $decimal_cb:ident, $radix_cb:ident, $try_decimal_cb:ident, $try_radix_cb:ident) => (
        impl FromBytes for $t {
            #[inline]
            fn from_bytes(bytes: &[u8]) -> $t
            {
                lexical_core::$decimal_cb(bytes)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_bytes_radix(bytes: &[u8], radix: u8) -> $t
            {
                lexical_core::$radix_cb(radix, bytes)
            }

            #[inline]
            fn try_from_bytes(bytes: &[u8]) -> Result<$t, Error>
            {
                convert_result(lexical_core::$try_decimal_cb(bytes))
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn try_from_bytes_radix(bytes: &[u8], radix: u8) -> Result<$t, Error>
            {
                convert_result(lexical_core::$try_radix_cb(radix, bytes))
            }
        }
    )
}

from_bytes!(u8, atou8_slice, atou8_radix_slice, try_atou8_slice, try_atou8_radix_slice);
from_bytes!(u16, atou16_slice, atou16_radix_slice, try_atou16_slice, try_atou16_radix_slice);
from_bytes!(u32, atou32_slice, atou32_radix_slice, try_atou32_slice, try_atou32_radix_slice);
from_bytes!(u64, atou64_slice, atou64_radix_slice, try_atou64_slice, try_atou64_radix_slice);
from_bytes!(u128, atou128_slice, atou128_radix_slice, try_atou128_slice, try_atou128_radix_slice);
from_bytes!(usize, atousize_slice, atousize_radix_slice, try_atousize_slice, try_atousize_radix_slice);
from_bytes!(i8, atoi8_slice, atoi8_radix_slice, try_atoi8_slice, try_atoi8_radix_slice);
from_bytes!(i16, atoi16_slice, atoi16_radix_slice, try_atoi16_slice, try_atoi16_radix_slice);
from_bytes!(i32, atoi32_slice, atoi32_radix_slice, try_atoi32_slice, try_atoi32_radix_slice);
from_bytes!(i64, atoi64_slice, atoi64_radix_slice, try_atoi64_slice, try_atoi64_radix_slice);
from_bytes!(i128, atoi128_slice, atoi128_radix_slice, try_atoi128_slice, try_atoi128_radix_slice);
from_bytes!(isize, atoisize_slice, atoisize_radix_slice, try_atoisize_slice, try_atoisize_radix_slice);
from_bytes!(f32, atof32_slice, atof32_radix_slice, try_atof32_slice, try_atof32_radix_slice);
from_bytes!(f64, atof64_slice, atof64_radix_slice, try_atof64_slice, try_atof64_radix_slice);

// FROM BYTES LOSSY

/// Trait for floating-point types that can be parsed using lossy algorithms from bytes.
pub trait FromBytesLossy: FromBytes {
    /// Deserialize from byte slice.
    fn from_bytes_lossy(bytes: &[u8]) -> Self;

    /// Deserialize from byte slice with radix.
    #[cfg(feature = "radix")]
    fn from_bytes_lossy_radix(bytes: &[u8], radix: u8) -> Self;

    /// Error-checking deserialize from byte slice.
    fn try_from_bytes_lossy(bytes: &[u8]) -> Result<Self, Error>;

    /// Error-checking deserialize from byte slice with radix.
    #[cfg(feature = "radix")]
    fn try_from_bytes_lossy_radix(bytes: &[u8], radix: u8) -> Result<Self, Error>;
}

macro_rules! from_bytes_lossy {
    ($t:ty, $decimal_cb:ident, $radix_cb:ident, $try_decimal_cb:ident, $try_radix_cb:ident) => (
        impl FromBytesLossy for $t {
            #[inline]
            fn from_bytes_lossy(bytes: &[u8]) -> $t
            {
                lexical_core::$decimal_cb(bytes)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_bytes_lossy_radix(bytes: &[u8], radix: u8) -> $t
            {
                lexical_core::$radix_cb(radix, bytes)
            }

            #[inline]
            fn try_from_bytes_lossy(bytes: &[u8]) -> Result<$t, Error>
            {
                convert_result(lexical_core::$try_decimal_cb(bytes))
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn try_from_bytes_lossy_radix(bytes: &[u8], radix: u8) -> Result<$t, Error>
            {
                convert_result(lexical_core::$try_radix_cb(radix, bytes))
            }
        }
    )
}

from_bytes_lossy!(f32, atof32_lossy_slice, atof32_lossy_radix_slice, try_atof32_lossy_slice, try_atof32_lossy_radix_slice);
from_bytes_lossy!(f64, atof64_lossy_slice, atof64_lossy_radix_slice, try_atof64_lossy_slice, try_atof64_lossy_radix_slice);

// TO BYTES

/// Trait for numerical types that can be serialized to bytes.
pub trait ToBytes: Sized {
    /// Serialize to string.
    fn to_bytes(&self) -> Vec<u8>;

    /// Serialize to string with radix.
    #[cfg(feature = "radix")]
    fn to_bytes_radix(&self, radix: u8) -> Vec<u8>;
}

macro_rules! to_bytes {
    ($t:ty, $decimal_cb:ident, $radix_cb:ident, $capacity:ident) => (
        impl ToBytes for $t {
            #[inline]
            fn to_bytes(&self) -> Vec<u8> {
                unsafe {
                    let mut buf = Vec::<u8>::with_capacity(lexical_core::$capacity);
                    let len = lexical_core::$decimal_cb(*self, vector_as_slice(&mut buf)).len();
                    buf.set_len(len);
                    buf
                }
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn to_bytes_radix(&self, radix: u8) -> Vec<u8> {
                unsafe {
                    let mut buf = Vec::<u8>::with_capacity(lexical_core::$capacity);
                    let len = lexical_core::$radix_cb(*self, radix, vector_as_slice(&mut buf)).len();
                    buf.set_len(len);
                    buf
                }
            }
        }
    )
}

to_bytes!(u8, u8toa_slice, u8toa_radix_slice, MAX_U8_SIZE);
to_bytes!(u16, u16toa_slice, u16toa_radix_slice, MAX_U16_SIZE);
to_bytes!(u32, u32toa_slice, u32toa_radix_slice, MAX_U32_SIZE);
to_bytes!(u64, u64toa_slice, u64toa_radix_slice, MAX_U64_SIZE);
to_bytes!(u128, u128toa_slice, u128toa_radix_slice, MAX_U128_SIZE);
to_bytes!(usize, usizetoa_slice, usizetoa_radix_slice, MAX_USIZE_SIZE);
to_bytes!(i8, i8toa_slice, i8toa_radix_slice, MAX_I8_SIZE);
to_bytes!(i16, i16toa_slice, i16toa_radix_slice, MAX_I16_SIZE);
to_bytes!(i32, i32toa_slice, i32toa_radix_slice, MAX_I32_SIZE);
to_bytes!(i64, i64toa_slice, i64toa_radix_slice, MAX_I64_SIZE);
to_bytes!(i128, i128toa_slice, i128toa_radix_slice, MAX_I128_SIZE);
to_bytes!(isize, isizetoa_slice, isizetoa_radix_slice, MAX_ISIZE_SIZE);
to_bytes!(f32, f32toa_slice, f32toa_radix_slice, MAX_F32_SIZE);
to_bytes!(f64, f64toa_slice, f64toa_radix_slice, MAX_F64_SIZE);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use error::invalid_digit;
    use super::*;

    macro_rules! deserialize_int {
        ($($t:tt)*) => ($({
            assert_eq!($t::from_bytes(b"0"), 0);
            assert_eq!($t::try_from_bytes(b"0"), Ok(0));
            assert_eq!($t::try_from_bytes(b""), Err(empty()));
            assert_eq!($t::try_from_bytes(b"1a"), Err(invalid_digit(1)));

            #[cfg(feature = "radix")]
            assert_eq!($t::from_bytes_radix(b"0", 10), 0);

            #[cfg(feature = "radix")]
            assert_eq!($t::try_from_bytes_radix(b"0", 10), Ok(0));
        })*)
    }

    macro_rules! deserialize_float {
        ($($t:tt)*) => ($({
            assert_eq!($t::from_bytes(b"0.0"), 0.0);
            assert_eq!($t::from_bytes_lossy(b"0.0"), 0.0);
            assert_eq!($t::try_from_bytes(b"0.0"), Ok(0.0));
            assert_eq!($t::try_from_bytes(b"0.0a"), Err(invalid_digit(3)));
            assert_eq!($t::try_from_bytes(b""), Err(empty()));
            assert_eq!($t::try_from_bytes_lossy(b"0.0"), Ok(0.0));

            #[cfg(feature = "radix")]
            assert_eq!($t::from_bytes_radix(b"0.0", 10), 0.0);

            #[cfg(feature = "radix")]
            assert_eq!($t::from_bytes_lossy_radix(b"0.0", 10), 0.0);

            #[cfg(feature = "radix")]
            assert_eq!($t::try_from_bytes_radix(b"0.0", 10), Ok(0.0));

            #[cfg(feature = "radix")]
            assert_eq!($t::try_from_bytes_lossy_radix(b"0.0", 10), Ok(0.0));
        })*)
    }

    #[test]
    fn from_bytes_test() {
        deserialize_int! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }
        deserialize_float! { f32 f64 }
    }

    macro_rules! serialize_int {
        ($($t:tt)*) => ($({
            let x: $t = 0;
            assert_eq!(x.to_bytes(), b"0".to_vec());

            #[cfg(feature = "radix")]
            assert_eq!(x.to_bytes_radix(10), b"0".to_vec());
        })*)
    }

    macro_rules! serialize_float {
        ($($t:tt)*) => ($({
            let x: $t = 0.0;
            assert_eq!(x.to_bytes(), b"0.0".to_vec());

            #[cfg(feature = "radix")]
            assert_eq!(x.to_bytes_radix(10), b"0.0".to_vec());
        })*)
    }

    #[test]
    fn to_bytes_test() {
        serialize_int! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }
        serialize_float! { f32 f64 }
    }
}
