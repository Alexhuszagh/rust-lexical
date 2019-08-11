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
