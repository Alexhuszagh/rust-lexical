//! High-level traits to translate the low-level API to idiomatic Rust.

use lexical_core::{self, ErrorCode};
use lib;
use error::*;

// FROM BYTES

/// Trait for numerical types that can be parsed from bytes.
pub trait FromBytes: Sized {
    /// Deserialize from byte slice.
    fn from_bytes(bytes: &[u8], radix: u8) -> Self;

    /// Error-checking deserialize from byte slice.
    fn try_from_bytes(bytes: &[u8], radix: u8) -> Result<Self, Error>;
}

macro_rules! from_bytes {
    ($t:ty, $bytes_cb:ident, $try_bytes_cb:ident) => (
        impl FromBytes for $t {
            #[inline(always)]
            fn from_bytes(bytes: &[u8], radix: u8) -> $t
            {
                // We reverse the argument order, since the low-level API
                // always uses (radix: u8, first: *const u8, last: *const u8)
                lexical_core::$bytes_cb(radix, bytes)
            }

            #[inline(always)]
            fn try_from_bytes(bytes: &[u8], radix: u8) -> Result<$t, Error>
            {
                // We reverse the argument order, since the low-level API
                // always uses (radix: u8, first: *const u8, last: *const u8)
                let result = lexical_core::$try_bytes_cb(radix, bytes);
                match result.error.code {
                    ErrorCode::Success  => Ok(result.value),
                    ErrorCode::Overflow => Err(overflow()),
                    ErrorCode::InvalidDigit => Err(invalid_digit(result.error.index)),
                    ErrorCode::Empty => Err(empty()),
                    _ => unimplemented!(),
                }
            }
        }
    )
}

from_bytes!(u8, atou8_slice, try_atou8_slice);
from_bytes!(u16, atou16_slice, try_atou16_slice);
from_bytes!(u32, atou32_slice, try_atou32_slice);
from_bytes!(u64, atou64_slice, try_atou64_slice);
from_bytes!(usize, atousize_slice, try_atousize_slice);
from_bytes!(i8, atoi8_slice, try_atoi8_slice);
from_bytes!(i16, atoi16_slice, try_atoi16_slice);
from_bytes!(i32, atoi32_slice, try_atoi32_slice);
from_bytes!(i64, atoi64_slice, try_atoi64_slice);
from_bytes!(isize, atoisize_slice, try_atoisize_slice);
from_bytes!(f32, atof32_slice, try_atof32_slice);
from_bytes!(f64, atof64_slice, try_atof64_slice);

// FROM BYTES LOSSY

/// Trait for floating-point types that can be parsed using lossy algorithms from bytes.
pub trait FromBytesLossy: FromBytes {
    /// Deserialize from byte slice.
    fn from_bytes_lossy(bytes: &[u8], radix: u8) -> Self;

    /// Error-checking deserialize from byte slice.
    fn try_from_bytes_lossy(bytes: &[u8], radix: u8) -> Result<Self, Error>;
}

macro_rules! from_bytes_lossy {
    ($t:ty, $bytes_cb:ident, $try_bytes_cb:ident) => (
        impl FromBytesLossy for $t {
            #[inline(always)]
            fn from_bytes_lossy(bytes: &[u8], radix: u8) -> $t
            {
                // We reverse the argument order, since the low-level API
                // always uses (radix: u8, first: *const u8, last: *const u8)
                lexical_core::$bytes_cb(radix, bytes)
            }

            #[inline(always)]
            fn try_from_bytes_lossy(bytes: &[u8], radix: u8) -> Result<$t, Error>
            {
                // We reverse the argument order, since the low-level API
                // always uses (radix: u8, first: *const u8, last: *const u8)
                let result = lexical_core::$try_bytes_cb(radix, bytes);
                match result.error.code {
                    ErrorCode::Success  => Ok(result.value),
                    ErrorCode::Overflow => Err(overflow()),
                    ErrorCode::InvalidDigit => Err(invalid_digit(result.error.index)),
                    ErrorCode::Empty => Err(empty()),
                    _ => unimplemented!(),
                }
            }
        }
    )
}

from_bytes_lossy!(f32, atof32_lossy_slice, try_atof32_lossy_slice);
from_bytes_lossy!(f64, atof64_lossy_slice, try_atof64_lossy_slice);

// TO BYTES

/// Trait for numerical types that can be serialized to bytes.
pub trait ToBytes: Sized {
    /// Serialize to string.
    fn to_bytes(&self, radix: u8) -> lib::Vec<u8>;
}

macro_rules! to_bytes {
    ($t:ty, $string_cb:ident, $capacity:ident) => (
        impl ToBytes for $t {
            #[inline(always)]
            fn to_bytes(&self, radix: u8) -> lib::Vec<u8>
            {
                let mut buf = lib::Vec::<u8>::with_capacity(lexical_core::$capacity);
                let len = unsafe {
                    let first = buf.as_mut_ptr();
                    let slc = lib::slice::from_raw_parts_mut(first, buf.capacity());
                    let slc = lexical_core::$string_cb(*self, radix, slc);
                    slc.len()
                };
                unsafe {
                    buf.set_len(len);
                }
                buf
            }
        }
    )
}

to_bytes!(u8, u8toa_slice, MAX_U8_SIZE);
to_bytes!(u16, u16toa_slice, MAX_U16_SIZE);
to_bytes!(u32, u32toa_slice, MAX_U32_SIZE);
to_bytes!(u64, u64toa_slice, MAX_U64_SIZE);
to_bytes!(u128, u128toa_slice, MAX_U128_SIZE);
to_bytes!(usize, usizetoa_slice, MAX_USIZE_SIZE);
to_bytes!(i8, i8toa_slice, MAX_I8_SIZE);
to_bytes!(i16, i16toa_slice, MAX_I16_SIZE);
to_bytes!(i32, i32toa_slice, MAX_I32_SIZE);
to_bytes!(i64, i64toa_slice, MAX_I64_SIZE);
to_bytes!(i128, i128toa_slice, MAX_I128_SIZE);
to_bytes!(isize, isizetoa_slice, MAX_ISIZE_SIZE);
to_bytes!(f32, f32toa_slice, MAX_F32_SIZE);
to_bytes!(f64, f64toa_slice, MAX_F64_SIZE);

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
            assert_eq!($t::try_from_bytes(b"", 10), Err(empty()));
            assert_eq!($t::try_from_bytes(b"1a", 10), Err(invalid_digit(1)));
        })*)
    }

    macro_rules! deserialize_float {
        ($($t:tt)*) => ($({
            assert_eq!($t::from_bytes(b"0.0", 10), 0.0);
            assert_eq!($t::from_bytes_lossy(b"0.0", 10), 0.0);
            assert_eq!($t::try_from_bytes(b"0.0", 10), Ok(0.0));
            assert_eq!($t::try_from_bytes(b"0.0a", 10), Err(invalid_digit(3)));
            assert_eq!($t::try_from_bytes(b"", 10), Err(empty()));
            assert_eq!($t::try_from_bytes_lossy(b"0.0", 10), Ok(0.0));
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
            assert_eq!(x.to_bytes(10), b"0".to_vec());
        })*)
    }

    macro_rules! serialize_float {
        ($($t:tt)*) => ($({
            let x: $t = 0.0;
            assert_eq!(x.to_bytes(10), b"0.0".to_vec());
        })*)
    }

    #[test]
    fn to_bytes_test() {
        serialize_int! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }
        serialize_float! { f32 f64 }
    }
}
