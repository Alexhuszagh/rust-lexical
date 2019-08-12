//! Low-level API generator.
//!
//! Uses either the optimized base10 algorithm, the optimized generic
//! algorithm, or the naive algorithm.

use util::*;

/// Select the back-end
#[cfg(feature = "table")]
use super::base10::Base10;

#[cfg(all(feature = "table", feature = "radix"))]
use super::generic::Generic;

#[cfg(not(feature = "table"))]
use super::naive::Naive;

// HELPERS

// Wrapper to facilitate calling a backend that writes iteratively to
// the end of the buffer.
#[cfg(any(not(feature = "table"), feature = "radix"))]
macro_rules! write_backwards {
    ($value:ident, $radix:ident, $buffer:ident, $t:tt, $cb:ident) => ({
        // Create a temporary buffer, and copy into it.
        // Way faster than reversing a buffer in-place.
        // Need to ensure the buffer size is adequate for basen, but small
        // for the optimized base10 formatters.
        debug_assert_radix!($radix);
        let mut buffer: [u8; BUFFER_SIZE] = [b'\0'; BUFFER_SIZE];
        let digits;
        if cfg!(not(feature = "radix")) || $radix == 10 {
            digits = &mut buffer[..$t::MAX_SIZE_BASE10];
        } else {
            digits = &mut buffer[..$t::MAX_SIZE];
        }

        // Write backwards to buffer and copy output to slice.
        let offset = $value.$cb($radix, digits);
        debug_assert!(offset <= digits.len());
        copy_to_dst($buffer, &unchecked_index!(digits[offset..]))
    });
}

// FORWARD

// Forward itoa arguments to an optimized backend.
//  Preconditions: `value` must be non-negative and unsigned.
perftools_inline!{
#[cfg(all(feature = "table", feature = "radix"))]
pub(crate) fn itoa_positive<T>(value: T, radix: u32, buffer: &mut [u8])
    -> usize
    where T: Base10 + Generic + UnsignedInteger
{
    if radix == 10 {
        value.base10(buffer)
    } else {
        write_backwards!(value, radix, buffer, T, generic)
    }
}}

// Forward itoa arguments to a base10 optimized backend.
//  Preconditions: `value` must be non-negative and unsigned.
perftools_inline!{
#[cfg(all(feature = "table", not(feature = "radix")))]
pub(crate) fn itoa_positive<T>(value: T, _: u32, buffer: &mut [u8])
    -> usize
    where T: Base10
{
    value.base10(buffer)
}}

// Forward itoa arguments to a naive backend.
//  Preconditions: `value` must be non-negative and unsigned.
perftools_inline!{
#[cfg(not(feature = "table"))]
pub(crate) fn itoa_positive<T>(value: T, radix: u32, buffer: &mut [u8])
    -> usize
    where T: Naive + UnsignedInteger
{
    write_backwards!(value, radix, buffer, T, naive)
}}

// OPTIMIZED

// Generate unsigned wrappers for impl methods.
macro_rules! unsigned {
    ($name:ident, $t:ty) => (
        perftools_inline!{
        fn $name(value: $t, radix: u8, buffer: &mut [u8]) -> usize {
            itoa_positive(value, radix.as_u32(), buffer)
        }}
    );
}

unsigned!(u8toa_impl, u8);
unsigned!(u16toa_impl, u16);
unsigned!(u32toa_impl, u32);
unsigned!(u64toa_impl, u64);
unsigned!(usizetoa_impl, usize);
#[cfg(has_i128)]
unsigned!(u128toa_impl, u128);

// Generate signed wrappers for impl methods.
macro_rules! signed {
    ($name:ident, $type:ty, $wide:ty, $unsigned:ty) => (
        perftools_inline!{
        fn $name(value: $type, radix: u8, buffer: &mut [u8]) -> usize {
            if value < 0 {
                unchecked_index_mut!(buffer[0] = b'-');
                let value = (value as $wide).wrapping_neg() as $unsigned;
                itoa_positive(value, radix.as_u32(), &mut unchecked_index_mut!(buffer[1..])) + 1
            } else {
                itoa_positive(value as $unsigned, radix.as_u32(), buffer)
            }
        }}
    );
}

signed!(i8toa_impl, i8, i32, u32);
signed!(i16toa_impl, i16, i32, u32);
signed!(i32toa_impl, i32, i32, u32);
signed!(i64toa_impl, i64, i64, u64);
signed!(isizetoa_impl, isize, i64, u64);
#[cfg(has_i128)]
signed!(i128toa_impl, i128, i128, u128);

// LOW-LEVEL API
// -------------

// SLICE API
generate_to_slice_api!(u8toa, u8toa_radix, u8, u8toa_impl, MAX_U8_SIZE);
generate_to_slice_api!(u16toa, u16toa_radix, u16, u16toa_impl, MAX_U16_SIZE);
generate_to_slice_api!(u32toa, u32toa_radix, u32, u32toa_impl, MAX_U32_SIZE);
generate_to_slice_api!(u64toa, u64toa_radix, u64, u64toa_impl, MAX_U64_SIZE);
generate_to_slice_api!(usizetoa, usizetoa_radix, usize, usizetoa_impl, MAX_USIZE_SIZE);
#[cfg(has_i128)] generate_to_slice_api!(u128toa, u128toa_radix, u128, u128toa_impl, MAX_U128_SIZE);

generate_to_slice_api!(i8toa, i8toa_radix, i8, i8toa_impl, MAX_I8_SIZE);
generate_to_slice_api!(i16toa, i16toa_radix, i16, i16toa_impl, MAX_I16_SIZE);
generate_to_slice_api!(i32toa, i32toa_radix, i32, i32toa_impl, MAX_I32_SIZE);
generate_to_slice_api!(i64toa, i64toa_radix, i64, i64toa_impl, MAX_I64_SIZE);
generate_to_slice_api!(isizetoa, isizetoa_radix, isize, isizetoa_impl, MAX_ISIZE_SIZE);
#[cfg(has_i128)] generate_to_slice_api!(i128toa, i128toa_radix, i128, i128toa_impl, MAX_I128_SIZE);

pub(crate) mod itoa_ffi {

use super::*;

// RANGE API (FFI)
generate_to_range_api!(u8toa, u8toa_radix, u8, u8toa_impl, MAX_U8_SIZE);
generate_to_range_api!(u16toa, u16toa_radix, u16, u16toa_impl, MAX_U16_SIZE);
generate_to_range_api!(u32toa, u32toa_radix, u32, u32toa_impl, MAX_U32_SIZE);
generate_to_range_api!(u64toa, u64toa_radix, u64, u64toa_impl, MAX_U64_SIZE);
generate_to_range_api!(usizetoa, usizetoa_radix, usize, usizetoa_impl, MAX_USIZE_SIZE);
#[cfg(has_i128)] generate_to_range_api!(u128toa, u128toa_radix, u128, u128toa_impl, MAX_U128_SIZE);

generate_to_range_api!(i8toa, i8toa_radix, i8, i8toa_impl, MAX_I8_SIZE);
generate_to_range_api!(i16toa, i16toa_radix, i16, i16toa_impl, MAX_I16_SIZE);
generate_to_range_api!(i32toa, i32toa_radix, i32, i32toa_impl, MAX_I32_SIZE);
generate_to_range_api!(i64toa, i64toa_radix, i64, i64toa_impl, MAX_I64_SIZE);
generate_to_range_api!(isizetoa, isizetoa_radix, isize, isizetoa_impl, MAX_ISIZE_SIZE);
#[cfg(has_i128)] generate_to_range_api!(i128toa, i128toa_radix, i128, i128toa_impl, MAX_I128_SIZE);

}   // itoa_ffi

// TESTS
// -----

#[cfg(test)]
mod tests {
    use atoi::*;
    use util::test::*;
    use super::*;

    // GENERIC

    #[test]
    fn u8toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", u8toa(0, &mut buffer));
        assert_eq!(b"1", u8toa(1, &mut buffer));
        assert_eq!(b"5", u8toa(5, &mut buffer));
        assert_eq!(b"127", u8toa(127, &mut buffer));
        assert_eq!(b"128", u8toa(128, &mut buffer));
        assert_eq!(b"255", u8toa(255, &mut buffer));
        assert_eq!(b"255", u8toa(-1i8 as u8, &mut buffer));
    }

    #[test]
    fn i8toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", i8toa(0, &mut buffer));
        assert_eq!(b"1", i8toa(1, &mut buffer));
        assert_eq!(b"5", i8toa(5, &mut buffer));
        assert_eq!(b"127", i8toa(127, &mut buffer));
        assert_eq!(b"-128", i8toa(128u8 as i8, &mut buffer));
        assert_eq!(b"-1", i8toa(255u8 as i8, &mut buffer));
        assert_eq!(b"-1", i8toa(-1, &mut buffer));
    }

    #[test]
    fn u16toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", u16toa(0, &mut buffer));
        assert_eq!(b"1", u16toa(1, &mut buffer));
        assert_eq!(b"5", u16toa(5, &mut buffer));
        assert_eq!(b"32767", u16toa(32767, &mut buffer));
        assert_eq!(b"32768", u16toa(32768, &mut buffer));
        assert_eq!(b"65535", u16toa(65535, &mut buffer));
        assert_eq!(b"65535", u16toa(-1i16 as u16, &mut buffer));
    }

    #[test]
    fn i16toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", i16toa(0, &mut buffer));
        assert_eq!(b"1", i16toa(1, &mut buffer));
        assert_eq!(b"5", i16toa(5, &mut buffer));
        assert_eq!(b"32767", i16toa(32767, &mut buffer));
        assert_eq!(b"-32768", i16toa(32768u16 as i16, &mut buffer));
        assert_eq!(b"-1", i16toa(65535u16 as i16, &mut buffer));
        assert_eq!(b"-1", i16toa(-1, &mut buffer));
    }

    #[test]
    fn u32toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", u32toa(0, &mut buffer));
        assert_eq!(b"1", u32toa(1, &mut buffer));
        assert_eq!(b"5", u32toa(5, &mut buffer));
        assert_eq!(b"2147483647", u32toa(2147483647, &mut buffer));
        assert_eq!(b"2147483648", u32toa(2147483648, &mut buffer));
        assert_eq!(b"4294967295", u32toa(4294967295, &mut buffer));
        assert_eq!(b"4294967295", u32toa(-1i32 as u32, &mut buffer));
    }

    #[test]
    fn i32toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", i32toa(0, &mut buffer));
        assert_eq!(b"1", i32toa(1, &mut buffer));
        assert_eq!(b"5", i32toa(5, &mut buffer));
        assert_eq!(b"2147483647", i32toa(2147483647, &mut buffer));
        assert_eq!(b"-2147483648", i32toa(2147483648u32 as i32, &mut buffer));
        assert_eq!(b"-1", i32toa(4294967295u32 as i32, &mut buffer));
        assert_eq!(b"-1", i32toa(-1, &mut buffer));
    }

    #[test]
    fn u64toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", u64toa(0, &mut buffer));
        assert_eq!(b"1", u64toa(1, &mut buffer));
        assert_eq!(b"5", u64toa(5, &mut buffer));
        assert_eq!(b"9223372036854775807", u64toa(9223372036854775807, &mut buffer));
        assert_eq!(b"9223372036854775808", u64toa(9223372036854775808, &mut buffer));
        assert_eq!(b"18446744073709551615", u64toa(18446744073709551615, &mut buffer));
        assert_eq!(b"18446744073709551615", u64toa(-1i64 as u64, &mut buffer));
    }

    #[test]
    fn i64toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", i64toa(0, &mut buffer));
        assert_eq!(b"1", i64toa(1, &mut buffer));
        assert_eq!(b"5", i64toa(5, &mut buffer));
        assert_eq!(b"9223372036854775807", i64toa(9223372036854775807, &mut buffer));
        assert_eq!(b"-9223372036854775808", i64toa(9223372036854775808u64 as i64, &mut buffer));
        assert_eq!(b"-1", i64toa(18446744073709551615u64 as i64, &mut buffer));
        assert_eq!(b"-1", i64toa(-1, &mut buffer));
    }

    #[cfg(has_i128)]
    #[test]
    fn u128toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", u128toa(0, &mut buffer));
        assert_eq!(b"1", u128toa(1, &mut buffer));
        assert_eq!(b"5", u128toa(5, &mut buffer));
        assert_eq!(&b"170141183460469231731687303715884105727"[..], u128toa(170141183460469231731687303715884105727, &mut buffer));
        assert_eq!(&b"170141183460469231731687303715884105728"[..], u128toa(170141183460469231731687303715884105728, &mut buffer));
        assert_eq!(&b"340282366920938463463374607431768211455"[..], u128toa(340282366920938463463374607431768211455, &mut buffer));
        assert_eq!(&b"340282366920938463463374607431768211455"[..], u128toa(-1i128 as u128, &mut buffer));
    }

    #[cfg(has_i128)]
    #[test]
    fn i128toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", i128toa(0, &mut buffer));
        assert_eq!(b"1", i128toa(1, &mut buffer));
        assert_eq!(b"5", i128toa(5, &mut buffer));
        assert_eq!(&b"170141183460469231731687303715884105727"[..], i128toa(170141183460469231731687303715884105727, &mut buffer));
        assert_eq!(&b"-170141183460469231731687303715884105728"[..], i128toa(170141183460469231731687303715884105728u128 as i128, &mut buffer));
        assert_eq!(b"-1", i128toa(340282366920938463463374607431768211455u128 as i128, &mut buffer));
        assert_eq!(b"-1", i128toa(-1, &mut buffer));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn basen_test() {
        let data = [
            (2, "100101"),
            (3, "1101"),
            (4, "211"),
            (5, "122"),
            (6, "101"),
            (7, "52"),
            (8, "45"),
            (9, "41"),
            (10, "37"),
            (11, "34"),
            (12, "31"),
            (13, "2B"),
            (14, "29"),
            (15, "27"),
            (16, "25"),
            (17, "23"),
            (18, "21"),
            (19, "1I"),
            (20, "1H"),
            (21, "1G"),
            (22, "1F"),
            (23, "1E"),
            (24, "1D"),
            (25, "1C"),
            (26, "1B"),
            (27, "1A"),
            (28, "19"),
            (29, "18"),
            (30, "17"),
            (31, "16"),
            (32, "15"),
            (33, "14"),
            (34, "13"),
            (35, "12"),
            (36, "11"),
        ];

        let mut buffer = new_buffer();
        for (base, expected) in data.iter() {
            assert_eq!(expected.as_bytes(), i8toa_radix(37, *base, &mut buffer));
        }
    }

    // Extensive tests

    #[test]
    fn u8toa_pow2_test() {
        let mut buffer = new_buffer();
        let values: &[u8] = &[0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255];
        for &i in values.iter() {
            assert_eq!(i, atou8(u8toa(i, &mut buffer)).unwrap());
        }
    }

    #[test]
    fn u8toa_pow10_test() {
        let mut buffer = new_buffer();
        let values: &[u8] = &[0, 1, 5, 9, 10, 11, 15, 99, 100, 101, 105];
        for &i in values.iter() {
            assert_eq!(i, atou8(u8toa(i, &mut buffer)).unwrap());
        }
    }

    #[test]
    fn u16toa_pow2_test() {
        let mut buffer = new_buffer();
        let values: &[u16] = &[0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 511, 512, 513, 1023, 1024, 1025, 2047, 2048, 2049, 4095, 4096, 4097, 8191, 8192, 8193, 16383, 16384, 16385, 32767, 32768, 32769, 65535];
        for &i in values.iter() {
            assert_eq!(i, atou16(u16toa(i, &mut buffer)).unwrap());
        }
    }

    #[test]
    fn u16toa_pow10_test() {
        let mut buffer = new_buffer();
        let values: &[u16] = &[0, 1, 5, 9, 10, 11, 15, 99, 100, 101, 105, 999, 1000, 1001, 1005, 9999, 10000, 10001, 10005];
        for &i in values.iter() {
            assert_eq!(i, atou16(u16toa(i, &mut buffer)).unwrap());
        }
    }

    #[test]
    fn u32toa_pow2_test() {
        let mut buffer = new_buffer();
        let values: &[u32] = &[0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 511, 512, 513, 1023, 1024, 1025, 2047, 2048, 2049, 4095, 4096, 4097, 8191, 8192, 8193, 16383, 16384, 16385, 32767, 32768, 32769, 65535, 65536, 65537, 131071, 131072, 131073, 262143, 262144, 262145, 524287, 524288, 524289, 1048575, 1048576, 1048577, 2097151, 2097152, 2097153, 4194303, 4194304, 4194305, 8388607, 8388608, 8388609, 16777215, 16777216, 16777217, 33554431, 33554432, 33554433, 67108863, 67108864, 67108865, 134217727, 134217728, 134217729, 268435455, 268435456, 268435457, 536870911, 536870912, 536870913, 1073741823, 1073741824, 1073741825, 2147483647, 2147483648, 2147483649, 4294967295];
        for &i in values.iter() {
            assert_eq!(i, atou32(u32toa(i, &mut buffer)).unwrap());
        }
    }

    #[test]
    fn u32toa_pow10_test() {
        let mut buffer = new_buffer();
        let values: &[u32] = &[0, 1, 5, 9, 10, 11, 15, 99, 100, 101, 105, 999, 1000, 1001, 1005, 9999, 10000, 10001, 10005, 99999, 100000, 100001, 100005, 999999, 1000000, 1000001, 1000005, 9999999, 10000000, 10000001, 10000005, 99999999, 100000000, 100000001, 100000005, 999999999, 1000000000, 1000000001, 1000000005];
        for &i in values.iter() {
            assert_eq!(i, atou32(u32toa(i, &mut buffer)).unwrap());
        }
    }

    #[test]
    fn u64toa_pow2_test() {
        let mut buffer = new_buffer();
        let values: &[u64] = &[0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 511, 512, 513, 1023, 1024, 1025, 2047, 2048, 2049, 4095, 4096, 4097, 8191, 8192, 8193, 16383, 16384, 16385, 32767, 32768, 32769, 65535, 65536, 65537, 131071, 131072, 131073, 262143, 262144, 262145, 524287, 524288, 524289, 1048575, 1048576, 1048577, 2097151, 2097152, 2097153, 4194303, 4194304, 4194305, 8388607, 8388608, 8388609, 16777215, 16777216, 16777217, 33554431, 33554432, 33554433, 67108863, 67108864, 67108865, 134217727, 134217728, 134217729, 268435455, 268435456, 268435457, 536870911, 536870912, 536870913, 1073741823, 1073741824, 1073741825, 2147483647, 2147483648, 2147483649, 4294967295, 4294967296, 4294967297, 8589934591, 8589934592, 8589934593, 17179869183, 17179869184, 17179869185, 34359738367, 34359738368, 34359738369, 68719476735, 68719476736, 68719476737, 137438953471, 137438953472, 137438953473, 274877906943, 274877906944, 274877906945, 549755813887, 549755813888, 549755813889, 1099511627775, 1099511627776, 1099511627777, 2199023255551, 2199023255552, 2199023255553, 4398046511103, 4398046511104, 4398046511105, 8796093022207, 8796093022208, 8796093022209, 17592186044415, 17592186044416, 17592186044417, 35184372088831, 35184372088832, 35184372088833, 70368744177663, 70368744177664, 70368744177665, 140737488355327, 140737488355328, 140737488355329, 281474976710655, 281474976710656, 281474976710657, 562949953421311, 562949953421312, 562949953421313, 1125899906842623, 1125899906842624, 1125899906842625, 2251799813685247, 2251799813685248, 2251799813685249, 4503599627370495, 4503599627370496, 4503599627370497, 9007199254740991, 9007199254740992, 9007199254740993, 18014398509481983, 18014398509481984, 18014398509481985, 36028797018963967, 36028797018963968, 36028797018963969, 72057594037927935, 72057594037927936, 72057594037927937, 144115188075855871, 144115188075855872, 144115188075855873, 288230376151711743, 288230376151711744, 288230376151711745, 576460752303423487, 576460752303423488, 576460752303423489, 1152921504606846975, 1152921504606846976, 1152921504606846977, 2305843009213693951, 2305843009213693952, 2305843009213693953, 4611686018427387903, 4611686018427387904, 4611686018427387905, 9223372036854775807, 9223372036854775808, 9223372036854775809, 18446744073709551615];
        for &i in values.iter() {
            assert_eq!(i, atou64(u64toa(i, &mut buffer)).unwrap());
        }
    }

    #[test]
    fn u64toa_pow10_test() {
        let mut buffer = new_buffer();
        let values: &[u64] = &[0, 1, 5, 9, 10, 11, 15, 99, 100, 101, 105, 999, 1000, 1001, 1005, 9999, 10000, 10001, 10005, 99999, 100000, 100001, 100005, 999999, 1000000, 1000001, 1000005, 9999999, 10000000, 10000001, 10000005, 99999999, 100000000, 100000001, 100000005, 999999999, 1000000000, 1000000001, 1000000005, 9999999999, 10000000000, 10000000001, 10000000005, 99999999999, 100000000000, 100000000001, 100000000005, 999999999999, 1000000000000, 1000000000001, 1000000000005, 9999999999999, 10000000000000, 10000000000001, 10000000000005, 99999999999999, 100000000000000, 100000000000001, 100000000000005, 999999999999999, 1000000000000000, 1000000000000001, 1000000000000005, 9999999999999999, 10000000000000000, 10000000000000001, 10000000000000005, 99999999999999999, 100000000000000000, 100000000000000001, 100000000000000005, 999999999999999999, 1000000000000000000, 1000000000000000001, 1000000000000000005];
        for &i in values.iter() {
            assert_eq!(i, atou64(u64toa(i, &mut buffer)).unwrap());
        }
    }

    #[cfg(has_i128)]
    #[test]
    fn u128toa_pow2_test() {
        let mut buffer = new_buffer();
        let values: &[u128] = &[0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 511, 512, 513, 1023, 1024, 1025, 2047, 2048, 2049, 4095, 4096, 4097, 8191, 8192, 8193, 16383, 16384, 16385, 32767, 32768, 32769, 65535, 65536, 65537, 131071, 131072, 131073, 262143, 262144, 262145, 524287, 524288, 524289, 1048575, 1048576, 1048577, 2097151, 2097152, 2097153, 4194303, 4194304, 4194305, 8388607, 8388608, 8388609, 16777215, 16777216, 16777217, 33554431, 33554432, 33554433, 67108863, 67108864, 67108865, 134217727, 134217728, 134217729, 268435455, 268435456, 268435457, 536870911, 536870912, 536870913, 1073741823, 1073741824, 1073741825, 2147483647, 2147483648, 2147483649, 4294967295, 4294967296, 4294967297, 8589934591, 8589934592, 8589934593, 17179869183, 17179869184, 17179869185, 34359738367, 34359738368, 34359738369, 68719476735, 68719476736, 68719476737, 137438953471, 137438953472, 137438953473, 274877906943, 274877906944, 274877906945, 549755813887, 549755813888, 549755813889, 1099511627775, 1099511627776, 1099511627777, 2199023255551, 2199023255552, 2199023255553, 4398046511103, 4398046511104, 4398046511105, 8796093022207, 8796093022208, 8796093022209, 17592186044415, 17592186044416, 17592186044417, 35184372088831, 35184372088832, 35184372088833, 70368744177663, 70368744177664, 70368744177665, 140737488355327, 140737488355328, 140737488355329, 281474976710655, 281474976710656, 281474976710657, 562949953421311, 562949953421312, 562949953421313, 1125899906842623, 1125899906842624, 1125899906842625, 2251799813685247, 2251799813685248, 2251799813685249, 4503599627370495, 4503599627370496, 4503599627370497, 9007199254740991, 9007199254740992, 9007199254740993, 18014398509481983, 18014398509481984, 18014398509481985, 36028797018963967, 36028797018963968, 36028797018963969, 72057594037927935, 72057594037927936, 72057594037927937, 144115188075855871, 144115188075855872, 144115188075855873, 288230376151711743, 288230376151711744, 288230376151711745, 576460752303423487, 576460752303423488, 576460752303423489, 1152921504606846975, 1152921504606846976, 1152921504606846977, 2305843009213693951, 2305843009213693952, 2305843009213693953, 4611686018427387903, 4611686018427387904, 4611686018427387905, 9223372036854775807, 9223372036854775808, 9223372036854775809, 18446744073709551615, 18446744073709551616, 18446744073709551617, 36893488147419103231, 36893488147419103232, 36893488147419103233, 73786976294838206463, 73786976294838206464, 73786976294838206465, 147573952589676412927, 147573952589676412928, 147573952589676412929, 295147905179352825855, 295147905179352825856, 295147905179352825857, 590295810358705651711, 590295810358705651712, 590295810358705651713, 1180591620717411303423, 1180591620717411303424, 1180591620717411303425, 2361183241434822606847, 2361183241434822606848, 2361183241434822606849, 4722366482869645213695, 4722366482869645213696, 4722366482869645213697, 9444732965739290427391, 9444732965739290427392, 9444732965739290427393, 18889465931478580854783, 18889465931478580854784, 18889465931478580854785, 37778931862957161709567, 37778931862957161709568, 37778931862957161709569, 75557863725914323419135, 75557863725914323419136, 75557863725914323419137, 151115727451828646838271, 151115727451828646838272, 151115727451828646838273, 302231454903657293676543, 302231454903657293676544, 302231454903657293676545, 604462909807314587353087, 604462909807314587353088, 604462909807314587353089, 1208925819614629174706175, 1208925819614629174706176, 1208925819614629174706177, 2417851639229258349412351, 2417851639229258349412352, 2417851639229258349412353, 4835703278458516698824703, 4835703278458516698824704, 4835703278458516698824705, 9671406556917033397649407, 9671406556917033397649408, 9671406556917033397649409, 19342813113834066795298815, 19342813113834066795298816, 19342813113834066795298817, 38685626227668133590597631, 38685626227668133590597632, 38685626227668133590597633, 77371252455336267181195263, 77371252455336267181195264, 77371252455336267181195265, 154742504910672534362390527, 154742504910672534362390528, 154742504910672534362390529, 309485009821345068724781055, 309485009821345068724781056, 309485009821345068724781057, 618970019642690137449562111, 618970019642690137449562112, 618970019642690137449562113, 1237940039285380274899124223, 1237940039285380274899124224, 1237940039285380274899124225, 2475880078570760549798248447, 2475880078570760549798248448, 2475880078570760549798248449, 4951760157141521099596496895, 4951760157141521099596496896, 4951760157141521099596496897, 9903520314283042199192993791, 9903520314283042199192993792, 9903520314283042199192993793, 19807040628566084398385987583, 19807040628566084398385987584, 19807040628566084398385987585, 39614081257132168796771975167, 39614081257132168796771975168, 39614081257132168796771975169, 79228162514264337593543950335, 79228162514264337593543950336, 79228162514264337593543950337, 158456325028528675187087900671, 158456325028528675187087900672, 158456325028528675187087900673, 316912650057057350374175801343, 316912650057057350374175801344, 316912650057057350374175801345, 633825300114114700748351602687, 633825300114114700748351602688, 633825300114114700748351602689, 1267650600228229401496703205375, 1267650600228229401496703205376, 1267650600228229401496703205377, 2535301200456458802993406410751, 2535301200456458802993406410752, 2535301200456458802993406410753, 5070602400912917605986812821503, 5070602400912917605986812821504, 5070602400912917605986812821505, 10141204801825835211973625643007, 10141204801825835211973625643008, 10141204801825835211973625643009, 20282409603651670423947251286015, 20282409603651670423947251286016, 20282409603651670423947251286017, 40564819207303340847894502572031, 40564819207303340847894502572032, 40564819207303340847894502572033, 81129638414606681695789005144063, 81129638414606681695789005144064, 81129638414606681695789005144065, 162259276829213363391578010288127, 162259276829213363391578010288128, 162259276829213363391578010288129, 324518553658426726783156020576255, 324518553658426726783156020576256, 324518553658426726783156020576257, 649037107316853453566312041152511, 649037107316853453566312041152512, 649037107316853453566312041152513, 1298074214633706907132624082305023, 1298074214633706907132624082305024, 1298074214633706907132624082305025, 2596148429267413814265248164610047, 2596148429267413814265248164610048, 2596148429267413814265248164610049, 5192296858534827628530496329220095, 5192296858534827628530496329220096, 5192296858534827628530496329220097, 10384593717069655257060992658440191, 10384593717069655257060992658440192, 10384593717069655257060992658440193, 20769187434139310514121985316880383, 20769187434139310514121985316880384, 20769187434139310514121985316880385, 41538374868278621028243970633760767, 41538374868278621028243970633760768, 41538374868278621028243970633760769, 83076749736557242056487941267521535, 83076749736557242056487941267521536, 83076749736557242056487941267521537, 166153499473114484112975882535043071, 166153499473114484112975882535043072, 166153499473114484112975882535043073, 332306998946228968225951765070086143, 332306998946228968225951765070086144, 332306998946228968225951765070086145, 664613997892457936451903530140172287, 664613997892457936451903530140172288, 664613997892457936451903530140172289, 1329227995784915872903807060280344575, 1329227995784915872903807060280344576, 1329227995784915872903807060280344577, 2658455991569831745807614120560689151, 2658455991569831745807614120560689152, 2658455991569831745807614120560689153, 5316911983139663491615228241121378303, 5316911983139663491615228241121378304, 5316911983139663491615228241121378305, 10633823966279326983230456482242756607, 10633823966279326983230456482242756608, 10633823966279326983230456482242756609, 21267647932558653966460912964485513215, 21267647932558653966460912964485513216, 21267647932558653966460912964485513217, 42535295865117307932921825928971026431, 42535295865117307932921825928971026432, 42535295865117307932921825928971026433, 85070591730234615865843651857942052863, 85070591730234615865843651857942052864, 85070591730234615865843651857942052865, 170141183460469231731687303715884105727, 170141183460469231731687303715884105728, 170141183460469231731687303715884105729, 340282366920938463463374607431768211455];
        for &i in values.iter() {
            assert_eq!(i, atou128(u128toa(i, &mut buffer)).unwrap());
        }
    }

    #[cfg(has_i128)]
    #[test]
    fn u128toa_pow10_test() {
        let mut buffer = new_buffer();
        let values: &[u128] = &[0, 1, 5, 9, 10, 11, 15, 99, 100, 101, 105, 999, 1000, 1001, 1005, 9999, 10000, 10001, 10005, 99999, 100000, 100001, 100005, 999999, 1000000, 1000001, 1000005, 9999999, 10000000, 10000001, 10000005, 99999999, 100000000, 100000001, 100000005, 999999999, 1000000000, 1000000001, 1000000005, 9999999999, 10000000000, 10000000001, 10000000005, 99999999999, 100000000000, 100000000001, 100000000005, 999999999999, 1000000000000, 1000000000001, 1000000000005, 9999999999999, 10000000000000, 10000000000001, 10000000000005, 99999999999999, 100000000000000, 100000000000001, 100000000000005, 999999999999999, 1000000000000000, 1000000000000001, 1000000000000005, 9999999999999999, 10000000000000000, 10000000000000001, 10000000000000005, 99999999999999999, 100000000000000000, 100000000000000001, 100000000000000005, 999999999999999999, 1000000000000000000, 1000000000000000001, 1000000000000000005, 9999999999999999999, 10000000000000000000, 10000000000000000001, 10000000000000000005, 99999999999999999999, 100000000000000000000, 100000000000000000001, 100000000000000000005, 999999999999999999999, 1000000000000000000000, 1000000000000000000001, 1000000000000000000005, 9999999999999999999999, 10000000000000000000000, 10000000000000000000001, 10000000000000000000005, 99999999999999999999999, 100000000000000000000000, 100000000000000000000001, 100000000000000000000005, 999999999999999999999999, 1000000000000000000000000, 1000000000000000000000001, 1000000000000000000000005, 9999999999999999999999999, 10000000000000000000000000, 10000000000000000000000001, 10000000000000000000000005, 99999999999999999999999999, 100000000000000000000000000, 100000000000000000000000001, 100000000000000000000000005, 999999999999999999999999999, 1000000000000000000000000000, 1000000000000000000000000001, 1000000000000000000000000005, 9999999999999999999999999999, 10000000000000000000000000000, 10000000000000000000000000001, 10000000000000000000000000005, 99999999999999999999999999999, 100000000000000000000000000000, 100000000000000000000000000001, 100000000000000000000000000005, 999999999999999999999999999999, 1000000000000000000000000000000, 1000000000000000000000000000001, 1000000000000000000000000000005, 9999999999999999999999999999999, 10000000000000000000000000000000, 10000000000000000000000000000001, 10000000000000000000000000000005, 99999999999999999999999999999999, 100000000000000000000000000000000, 100000000000000000000000000000001, 100000000000000000000000000000005, 999999999999999999999999999999999, 1000000000000000000000000000000000, 1000000000000000000000000000000001, 1000000000000000000000000000000005, 9999999999999999999999999999999999, 10000000000000000000000000000000000, 10000000000000000000000000000000001, 10000000000000000000000000000000005, 99999999999999999999999999999999999, 100000000000000000000000000000000000, 100000000000000000000000000000000001, 100000000000000000000000000000000005, 999999999999999999999999999999999999, 1000000000000000000000000000000000000, 1000000000000000000000000000000000001, 1000000000000000000000000000000000005, 9999999999999999999999999999999999999, 10000000000000000000000000000000000000, 10000000000000000000000000000000000001, 10000000000000000000000000000000000005, 99999999999999999999999999999999999999, 100000000000000000000000000000000000000, 100000000000000000000000000000000000001, 100000000000000000000000000000000000005];
        for &i in values.iter() {
            assert_eq!(i, atou128(u128toa(i, &mut buffer)).unwrap());
        }
    }

    // Quickcheck

    quickcheck! {
        fn u8_quickcheck(i: u8) -> bool {
            let mut buffer = new_buffer();
            i == atou8(u8toa(i, &mut buffer)).unwrap()
        }

        fn u16_quickcheck(i: u16) -> bool {
            let mut buffer = new_buffer();
            i == atou16(u16toa(i, &mut buffer)).unwrap()
        }

        fn u32_quickcheck(i: u32) -> bool {
            let mut buffer = new_buffer();
            i == atou32(u32toa(i, &mut buffer)).unwrap()
        }

        fn u64_quickcheck(i: u64) -> bool {
            let mut buffer = new_buffer();
            i == atou64(u64toa(i, &mut buffer)).unwrap()
        }

        fn usize_quickcheck(i: usize) -> bool {
            let mut buffer = new_buffer();
            i == atousize(usizetoa(i, &mut buffer)).unwrap()
        }

        fn i8_quickcheck(i: i8) -> bool {
            let mut buffer = new_buffer();
            i == atoi8(i8toa(i, &mut buffer)).unwrap()
        }

        fn i16_quickcheck(i: i16) -> bool {
            let mut buffer = new_buffer();
            i == atoi16(i16toa(i, &mut buffer)).unwrap()
        }

        fn i32_quickcheck(i: i32) -> bool {
            let mut buffer = new_buffer();
            i == atoi32(i32toa(i, &mut buffer)).unwrap()
        }

        fn i64_quickcheck(i: i64) -> bool {
            let mut buffer = new_buffer();
            i == atoi64(i64toa(i, &mut buffer)).unwrap()
        }

        fn isize_quickcheck(i: isize) -> bool {
            let mut buffer = new_buffer();
            i == atoisize(isizetoa(i, &mut buffer)).unwrap()
        }
    }

    // Proptest

    #[cfg(feature = "std")]
    proptest! {
        #[test]
        fn u8_proptest(i in u8::min_value()..u8::max_value()) {
            let mut buffer = new_buffer();
            i == atou8(u8toa(i, &mut buffer)).unwrap()
        }

        #[test]
        fn i8_proptest(i in i8::min_value()..i8::max_value()) {
            let mut buffer = new_buffer();
            i == atoi8(i8toa(i, &mut buffer)).unwrap()
        }

        #[test]
        fn u16_proptest(i in u16::min_value()..u16::max_value()) {
            let mut buffer = new_buffer();
            i == atou16(u16toa(i, &mut buffer)).unwrap()
        }

        #[test]
        fn i16_proptest(i in i16::min_value()..i16::max_value()) {
            let mut buffer = new_buffer();
            i == atoi16(i16toa(i, &mut buffer)).unwrap()
        }

        #[test]
        fn u32_proptest(i in u32::min_value()..u32::max_value()) {
            let mut buffer = new_buffer();
            i == atou32(u32toa(i, &mut buffer)).unwrap()
        }

        #[test]
        fn i32_proptest(i in i32::min_value()..i32::max_value()) {
            let mut buffer = new_buffer();
            i == atoi32(i32toa(i, &mut buffer)).unwrap()
        }

        #[test]
        fn u64_proptest(i in u64::min_value()..u64::max_value()) {
            let mut buffer = new_buffer();
            i == atou64(u64toa(i, &mut buffer)).unwrap()
        }

        #[test]
        fn i64_proptest(i in i64::min_value()..i64::max_value()) {
            let mut buffer = new_buffer();
            i == atoi64(i64toa(i, &mut buffer)).unwrap()
        }

        #[test]
        fn u128_proptest(i in u128::min_value()..u128::max_value()) {
            let mut buffer = new_buffer();
            i == atou128(u128toa(i, &mut buffer)).unwrap()
        }

        #[test]
        fn i128_proptest(i in i128::min_value()..i128::max_value()) {
            let mut buffer = new_buffer();
            i == atoi128(i128toa(i, &mut buffer)).unwrap()
        }

        #[test]
        fn usize_proptest(i in usize::min_value()..usize::max_value()) {
            let mut buffer = new_buffer();
            i == atousize(usizetoa(i, &mut buffer)).unwrap()
        }

        #[test]
        fn isize_proptest(i in isize::min_value()..isize::max_value()) {
            let mut buffer = new_buffer();
            i == atoisize(isizetoa(i, &mut buffer)).unwrap()
        }
    }

    // Panic tests

    #[test]
    #[should_panic]
    fn i8toa_buffer_test() {
        let mut buffer = [b'0'; i8::MAX_SIZE_BASE10-1];
        i8toa(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn i16toa_buffer_test() {
        let mut buffer = [b'0'; i16::MAX_SIZE_BASE10-1];
        i16toa(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn i32toa_buffer_test() {
        let mut buffer = [b'0'; i32::MAX_SIZE_BASE10-1];
        i32toa(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn i64toa_buffer_test() {
        let mut buffer = [b'0'; i64::MAX_SIZE_BASE10-1];
        i64toa(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn i128toa_buffer_test() {
        let mut buffer = [b'0'; i128::MAX_SIZE_BASE10-1];
        i128toa(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn isizetoa_buffer_test() {
        let mut buffer = [b'0'; isize::MAX_SIZE_BASE10-1];
        isizetoa(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn u8toa_buffer_test() {
        let mut buffer = [b'0'; u8::MAX_SIZE_BASE10-1];
        i8toa(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn u16toa_buffer_test() {
        let mut buffer = [b'0'; u16::MAX_SIZE_BASE10-1];
        i16toa(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn u32toa_buffer_test() {
        let mut buffer = [b'0'; u32::MAX_SIZE_BASE10-1];
        i32toa(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn u64toa_buffer_test() {
        let mut buffer = [b'0'; u64::MAX_SIZE_BASE10-1];
        i64toa(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn u128toa_buffer_test() {
        let mut buffer = [b'0'; u128::MAX_SIZE_BASE10-1];
        i128toa(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn usizetoa_buffer_test() {
        let mut buffer = [b'0'; usize::MAX_SIZE_BASE10-1];
        usizetoa(12, &mut buffer);
    }
}
