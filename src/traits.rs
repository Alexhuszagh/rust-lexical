//! High-level traits to translate the low-level API to idiomatic Rust.

use lexical_core::{self, Result};
use lib::{slice, Vec};

// HELPERS

/// Get a vector as a slice, including the capacity.
#[inline]
unsafe fn vector_as_slice<'a, T>(buf: &'a mut Vec<T>)
    -> &'a mut [T]
{
    let first = buf.as_mut_ptr();
    slice::from_raw_parts_mut(first, buf.capacity())
}

// FROM BYTES

/// Trait for numerical types that can be parsed from bytes.
pub trait FromLexical: Sized {
    /// Deserialize number from byte slice.
    fn from_lexical(bytes: &[u8]) -> Result<Self>;

    /// Deserialize number from a subset of a byte slice.
    fn from_lexical_partial(bytes: &[u8]) -> Result<(Self, usize)>;

    /// Deserialize number from byte slice with radix.
    #[cfg(feature = "radix")]
    fn from_lexical_radix(bytes: &[u8], radix: u8) -> Result<Self>;

    /// Deserialize number from a subset of a byte slice with radix.
    #[cfg(feature = "radix")]
    fn from_lexical_partial_radix(bytes: &[u8], radix: u8) -> Result<(Self, usize)>;
}

macro_rules! from_lexical {
    ($t:ty, $decimal_cb:ident, $radix_cb:ident, $decimal_partial_cb:ident, $radix_partial_cb:ident) => (
        impl FromLexical for $t {
            #[inline]
            fn from_lexical(bytes: &[u8]) -> Result<$t>
            {
                lexical_core::$decimal_cb(bytes)
            }

            #[inline]
            fn from_lexical_partial(bytes: &[u8]) -> Result<($t, usize)>
            {
                lexical_core::$decimal_partial_cb(bytes)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_lexical_radix(bytes: &[u8], radix: u8) -> Result<$t>
            {
                lexical_core::$radix_cb(radix, bytes)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_lexical_partial_radix(bytes: &[u8], radix: u8) -> Result<($t, usize)>
            {
                lexical_core::$radix_partial_cb(radix, bytes)
            }
        }
    )
}

from_lexical!(u8, atou8, atou8_radix, atou8_partial, atou8_partial_radix);
from_lexical!(u16, atou16, atou16_radix, atou16_partial, atou16_partial_radix);
from_lexical!(u32, atou32, atou32_radix, atou32_partial, atou32_partial_radix);
from_lexical!(u64, atou64, atou64_radix, atou64_partial, atou64_partial_radix);
from_lexical!(usize, atousize, atousize_radix, atousize_partial, atousize_partial_radix);
from_lexical!(i8, atoi8, atoi8_radix, atoi8_partial, atoi8_partial_radix);
from_lexical!(i16, atoi16, atoi16_radix, atoi16_partial, atoi16_partial_radix);
from_lexical!(i32, atoi32, atoi32_radix, atoi32_partial, atoi32_partial_radix);
from_lexical!(i64, atoi64, atoi64_radix, atoi64_partial, atoi64_partial_radix);
from_lexical!(isize, atoisize, atoisize_radix, atoisize_partial, atoisize_partial_radix);
from_lexical!(f32, atof32, atof32_radix, atof32_partial, atof32_partial_radix);
from_lexical!(f64, atof64, atof64_radix, atof64_partial, atof64_partial_radix);

#[cfg(has_i128)]
from_lexical!(u128, atou128, atou128_radix, atou128_partial, atou128_partial_radix);

#[cfg(has_i128)]
from_lexical!(i128, atoi128, atoi128_radix, atoi128_partial, atoi128_partial_radix);

// FROM BYTES LOSSY

/// Trait for floating-point types that can be parsed using lossy algorithms from bytes.
pub trait FromLexicalLossy: FromLexical {
    /// Deserialize float lossily from byte slice.
    fn from_lexical_lossy(bytes: &[u8]) -> Result<Self>;

    /// Deserialize float lossily from a subset of a byte slice.
    fn from_lexical_partial_lossy(bytes: &[u8]) -> Result<(Self, usize)>;

    /// Deserialize float lossily from byte slice with radix.
    #[cfg(feature = "radix")]
    fn from_lexical_lossy_radix(bytes: &[u8], radix: u8) -> Result<Self>;

    /// Deserialize float lossily from a subset of a byte slice with radix.
    #[cfg(feature = "radix")]
    fn from_lexical_partial_lossy_radix(bytes: &[u8], radix: u8) -> Result<(Self, usize)>;
}

macro_rules! from_lexical_lossy {
    ($t:ty, $decimal_cb:ident, $radix_cb:ident, $decimal_partial_cb:ident, $radix_partial_cb:ident) => (
        impl FromLexicalLossy for $t {
            #[inline]
            fn from_lexical_lossy(bytes: &[u8]) -> Result<$t>
            {
                lexical_core::$decimal_cb(bytes)
            }

            #[inline]
            fn from_lexical_partial_lossy(bytes: &[u8]) -> Result<($t, usize)>
            {
                lexical_core::$decimal_partial_cb(bytes)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_lexical_lossy_radix(bytes: &[u8], radix: u8) -> Result<$t>
            {
                lexical_core::$radix_cb(radix, bytes)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_lexical_partial_lossy_radix(bytes: &[u8], radix: u8) -> Result<($t, usize)>
            {
                lexical_core::$radix_partial_cb(radix, bytes)
            }
        }
    )
}

from_lexical_lossy!(f32, atof32_lossy, atof32_lossy_radix, atof32_partial_lossy, atof32_partial_lossy_radix);
from_lexical_lossy!(f64, atof64_lossy, atof64_lossy_radix, atof64_partial_lossy, atof64_partial_lossy_radix);

// TO BYTES

/// Trait for numerical types that can be serialized to bytes.
pub trait ToLexical: Sized {
    /// Serialize to string.
    fn to_lexical(&self) -> Vec<u8>;

    /// Serialize to string with radix.
    #[cfg(feature = "radix")]
    fn to_lexical_radix(&self, radix: u8) -> Vec<u8>;
}

macro_rules! to_lexical {
    ($t:ty, $decimal_cb:ident, $radix_cb:ident, $capacity:ident) => (
        impl ToLexical for $t {
            #[inline]
            fn to_lexical(&self) -> Vec<u8> {
                unsafe {
                    let mut buf = Vec::<u8>::with_capacity(lexical_core::$capacity);
                    let len = lexical_core::$decimal_cb(*self, vector_as_slice(&mut buf)).len();
                    buf.set_len(len);
                    buf
                }
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn to_lexical_radix(&self, radix: u8) -> Vec<u8> {
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

to_lexical!(u8, u8toa, u8toa_radix, MAX_U8_SIZE);
to_lexical!(u16, u16toa, u16toa_radix, MAX_U16_SIZE);
to_lexical!(u32, u32toa, u32toa_radix, MAX_U32_SIZE);
to_lexical!(u64, u64toa, u64toa_radix, MAX_U64_SIZE);
to_lexical!(usize, usizetoa, usizetoa_radix, MAX_USIZE_SIZE);
to_lexical!(i8, i8toa, i8toa_radix, MAX_I8_SIZE);
to_lexical!(i16, i16toa, i16toa_radix, MAX_I16_SIZE);
to_lexical!(i32, i32toa, i32toa_radix, MAX_I32_SIZE);
to_lexical!(i64, i64toa, i64toa_radix, MAX_I64_SIZE);
to_lexical!(isize, isizetoa, isizetoa_radix, MAX_ISIZE_SIZE);
to_lexical!(f32, f32toa, f32toa_radix, MAX_F32_SIZE);
to_lexical!(f64, f64toa, f64toa_radix, MAX_F64_SIZE);

#[cfg(has_i128)]
to_lexical!(u128, u128toa, u128toa_radix, MAX_U128_SIZE);

#[cfg(has_i128)]
to_lexical!(i128, i128toa, i128toa_radix, MAX_I128_SIZE);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use lexical_core::ErrorCode;
    use super::*;

    macro_rules! deserialize_int {
        ($($t:tt)*) => ($({
            assert_eq!($t::from_lexical(b"0"), Ok(0));
            assert_eq!($t::from_lexical(b""), Err(ErrorCode::Empty.into()));
            assert_eq!($t::from_lexical(b"1a"), Err((ErrorCode::InvalidDigit, 1).into()));

            #[cfg(feature = "radix")]
            assert_eq!($t::from_lexical_radix(b"0", 10), Ok(0));
        })*)
    }

    macro_rules! deserialize_float {
        ($($t:tt)*) => ($({
            assert_eq!($t::from_lexical(b"0.0"), Ok(0.0));
            assert_eq!($t::from_lexical(b"0.0a").err().unwrap().code, ErrorCode::InvalidDigit);
            assert_eq!($t::from_lexical(b""), Err(ErrorCode::Empty.into()));
            assert_eq!($t::from_lexical_lossy(b"0.0"), Ok(0.0));

            #[cfg(feature = "radix")]
            assert_eq!($t::from_lexical_radix(b"0.0", 10), Ok(0.0));

            #[cfg(feature = "radix")]
            assert_eq!($t::from_lexical_lossy_radix(b"0.0", 10), Ok(0.0));
        })*)
    }

    #[test]
    fn from_lexical_test() {
        deserialize_int! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }
        deserialize_float! { f32 f64 }
    }

    macro_rules! serialize_int {
        ($($t:tt)*) => ($({
            let x: $t = 0;
            assert_eq!(x.to_lexical().to_vec(), b"0".to_vec());

            #[cfg(feature = "radix")]
            assert_eq!(x.to_lexical_radix(10).to_vec(), b"0".to_vec());
        })*)
    }

    #[cfg(feature = "trim_floats")]
    macro_rules! serialize_float {
        ($($t:tt)*) => ($({
            let x: $t = 0.0;
            assert_eq!(x.to_lexical().to_vec(), b"0".to_vec());

            #[cfg(feature = "radix")]
            assert_eq!(x.to_lexical_radix(10).to_vec(), b"0".to_vec());
        })*)
    }

    #[cfg(not(feature = "trim_floats"))]
    macro_rules! serialize_float {
        ($($t:tt)*) => ($({
            let x: $t = 0.0;
            assert_eq!(x.to_lexical().to_vec(), b"0.0".to_vec());

            #[cfg(feature = "radix")]
            assert_eq!(x.to_lexical_radix(10).to_vec(), b"0.0".to_vec());
        })*)
    }

    #[test]
    fn to_lexical_test() {
        serialize_int! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }
        serialize_float! { f32 f64 }
    }
}
