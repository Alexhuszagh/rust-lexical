//! High-level traits to translate the low-level API to idiomatic Rust.

use lexical_core::Result;
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
    /// Deserialize from byte slice.
    fn from_lexical(bytes: &[u8]) -> Result<Self>;

    /// Deserialize from byte slice with radix.
    #[cfg(feature = "radix")]
    fn from_lexical_radix(bytes: &[u8], radix: u8) -> Result<Self>;
}

macro_rules! from_lexical {
    ($t:ty, $decimal_cb:ident, $radix_cb:ident) => (
        impl FromLexical for $t {
            #[inline]
            fn from_lexical(bytes: &[u8]) -> Result<$t>
            {
                lexical_core::$decimal_cb(bytes)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_lexical_radix(bytes: &[u8], radix: u8) -> Result<$t>
            {
                lexical_core::$radix_cb(radix, bytes)
            }
        }
    )
}

from_lexical!(u8, atou8_slice, atou8_radix_slice);
from_lexical!(u16, atou16_slice, atou16_radix_slice);
from_lexical!(u32, atou32_slice, atou32_radix_slice);
from_lexical!(u64, atou64_slice, atou64_radix_slice);
from_lexical!(usize, atousize_slice, atousize_radix_slice);
from_lexical!(i8, atoi8_slice, atoi8_radix_slice);
from_lexical!(i16, atoi16_slice, atoi16_radix_slice);
from_lexical!(i32, atoi32_slice, atoi32_radix_slice);
from_lexical!(i64, atoi64_slice, atoi64_radix_slice);
from_lexical!(isize, atoisize_slice, atoisize_radix_slice);
from_lexical!(f32, atof32_slice, atof32_radix_slice);
from_lexical!(f64, atof64_slice, atof64_radix_slice);

#[cfg(has_i128)]
from_lexical!(u128, atou128_slice, atou128_radix_slice);

#[cfg(has_i128)]
from_lexical!(i128, atoi128_slice, atoi128_radix_slice);

// FROM BYTES LOSSY

/// Trait for floating-point types that can be parsed using lossy algorithms from bytes.
pub trait FromLexicalLossy: FromLexical {
    /// Deserialize from byte slice.
    fn from_lexical_lossy(bytes: &[u8]) -> Result<Self>;

    /// Deserialize from byte slice with radix.
    #[cfg(feature = "radix")]
    fn from_lexical_lossy_radix(bytes: &[u8], radix: u8) -> Result<Self>;
}

macro_rules! from_lexical_lossy {
    ($t:ty, $decimal_cb:ident, $radix_cb:ident) => (
        impl FromLexicalLossy for $t {
            #[inline]
            fn from_lexical_lossy(bytes: &[u8]) -> Result<$t>
            {
                lexical_core::$decimal_cb(bytes)
            }

            #[cfg(feature = "radix")]
            #[inline]
            fn from_lexical_lossy_radix(bytes: &[u8], radix: u8) -> Result<$t>
            {
                lexical_core::$radix_cb(radix, bytes)
            }
        }
    )
}

from_lexical_lossy!(f32, atof32_lossy_slice, atof32_lossy_radix_slice);
from_lexical_lossy!(f64, atof64_lossy_slice, atof64_lossy_radix_slice);

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

to_lexical!(u8, u8toa_slice, u8toa_radix_slice, MAX_U8_SIZE);
to_lexical!(u16, u16toa_slice, u16toa_radix_slice, MAX_U16_SIZE);
to_lexical!(u32, u32toa_slice, u32toa_radix_slice, MAX_U32_SIZE);
to_lexical!(u64, u64toa_slice, u64toa_radix_slice, MAX_U64_SIZE);
to_lexical!(usize, usizetoa_slice, usizetoa_radix_slice, MAX_USIZE_SIZE);
to_lexical!(i8, i8toa_slice, i8toa_radix_slice, MAX_I8_SIZE);
to_lexical!(i16, i16toa_slice, i16toa_radix_slice, MAX_I16_SIZE);
to_lexical!(i32, i32toa_slice, i32toa_radix_slice, MAX_I32_SIZE);
to_lexical!(i64, i64toa_slice, i64toa_radix_slice, MAX_I64_SIZE);
to_lexical!(isize, isizetoa_slice, isizetoa_radix_slice, MAX_ISIZE_SIZE);
to_lexical!(f32, f32toa_slice, f32toa_radix_slice, MAX_F32_SIZE);
to_lexical!(f64, f64toa_slice, f64toa_radix_slice, MAX_F64_SIZE);

#[cfg(has_i128)]
to_lexical!(u128, u128toa_slice, u128toa_radix_slice, MAX_U128_SIZE);

#[cfg(has_i128)]
to_lexical!(i128, i128toa_slice, i128toa_radix_slice, MAX_I128_SIZE);

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
            assert_eq!($t::from_lexical(b"0.0a"), Err((ErrorCode::InvalidDigit, 3).into()));
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
