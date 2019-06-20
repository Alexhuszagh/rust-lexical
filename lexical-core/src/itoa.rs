//! Fast lexical integer-to-string conversion routines.

//  The following benchmarks were run on an "Intel(R) Core(TM) i7-6560U
//  CPU @ 2.20GHz" CPU, on Fedora 28, Linux kernel version 4.18.16-200
//  (x86-64), using the lexical formatter, `itoa::write()` or `x.to_string()`,
//  avoiding any inefficiencies in Rust string parsing for `format!(...)`
//  or `write!()` macros. The code was compiled with LTO and at an optimization
//  level of 3.
//
//  The benchmarks with `std` were compiled using "rustc 1.29.2 (17a9dc751
//  2018-10-05", and the `no_std` benchmarks were compiled using "rustc
//  1.31.0-nightly (46880f41b 2018-10-15)".
//
//  The benchmark code may be found `benches/itoa.rs`.
//
//  # Benchmarks
//
//  | Type  |  lexical (ns/iter) | to_string (ns/iter)   | Relative Increase |
//  |:-----:|:------------------:|:---------------------:|:-----------------:|
//  | u8    | 118,355            | 580,365               | 4.90x             |
//  | u16   | 130,145            | 559,563               | 4.30x             |
//  | u32   | 168,984            | 572,838               | 3.39x             |
//  | u64   | 307,052            | 706,427               | 2.30x             |
//  | i8    | 149,634            | 788,111               | 5.27x             |
//  | i16   | 169,105            | 810,891               | 4.80x             |
//  | i32   | 218,203            | 868,149               | 3.98x             |
//  | i64   | 323,406            | 923,783               | 2.86x             |
//
//  # Raw Benchmarks
//
//  ```text
//  test itoa_i8_itoa       ... bench:     148,488 ns/iter (+/- 5,138)
//  test itoa_i8_lexical    ... bench:     149,634 ns/iter (+/- 5,080)
//  test itoa_i8_to_string  ... bench:     788,111 ns/iter (+/- 22,323)
//  test itoa_i16_itoa      ... bench:     165,307 ns/iter (+/- 6,178)
//  test itoa_i16_lexical   ... bench:     169,105 ns/iter (+/- 5,531)
//  test itoa_i16_to_string ... bench:     810,891 ns/iter (+/- 31,014)
//  test itoa_i32_itoa      ... bench:     220,269 ns/iter (+/- 11,670)
//  test itoa_i32_lexical   ... bench:     218,203 ns/iter (+/- 7,772)
//  test itoa_i32_to_string ... bench:     868,149 ns/iter (+/- 50,224)
//  test itoa_i64_itoa      ... bench:     275,732 ns/iter (+/- 10,747)
//  test itoa_i64_lexical   ... bench:     323,406 ns/iter (+/- 9,937)
//  test itoa_i64_to_string ... bench:     923,783 ns/iter (+/- 21,573)
//  test itoa_u8_itoa       ... bench:     119,045 ns/iter (+/- 6,135)
//  test itoa_u8_lexical    ... bench:     118,355 ns/iter (+/- 5,028)
//  test itoa_u8_to_string  ... bench:     580,365 ns/iter (+/- 24,363)
//  test itoa_u16_itoa      ... bench:     109,700 ns/iter (+/- 4,134)
//  test itoa_u16_lexical   ... bench:     130,145 ns/iter (+/- 2,553)
//  test itoa_u16_to_string ... bench:     559,563 ns/iter (+/- 18,978)
//  test itoa_u32_itoa      ... bench:     154,892 ns/iter (+/- 5,122)
//  test itoa_u32_lexical   ... bench:     168,984 ns/iter (+/- 5,195)
//  test itoa_u32_to_string ... bench:     572,838 ns/iter (+/- 15,236)
//  test itoa_u64_itoa      ... bench:     284,219 ns/iter (+/- 9,696)
//  test itoa_u64_lexical   ... bench:     307,052 ns/iter (+/- 11,039)
//  test itoa_u64_to_string ... bench:     706,427 ns/iter (+/- 32,680)
//  ```

// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([118355, 130145, 168984, 307052, 149634, 169105, 218203, 323406]) / 1e6
//  itoa = np.array([119045, 109700, 154892, 284219, 148488, 165307, 220269, 275732]) / 1e6
//  rustcore = np.array([580365, 559563, 572838, 706427, 788111, 810891, 868149, 923783]) / 1e6
//  index = ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
//  df = pd.DataFrame({'lexical': lexical, 'itoa': itoa, 'rustcore': rustcore}, index = index, columns=['lexical', 'itoa', 'rustcore'])
//  ax = df.plot.bar(rot=0, figsize=(16, 8), fontsize=14, color=['#E24A33', '#988ED5', '#348ABD'])
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  ax.legend(loc=2, prop={'size': 14})
//  plt.show()

use util::*;

// OPTIMIZED

/// Optimized implementation for radix-N numbers.
///
/// Use a macro to allow for u32 or u64 to be used (u32 is generally faster).
///
/// `value` must be non-negative and mutable.
#[cfg(feature = "table")]
#[inline]
fn optimized<T>(mut value: T, radix: T, table: &[u8], buffer: &mut [u8])
    -> usize
    where T: UnsignedInteger
{
    // Use power-reduction to minimize the number of operations.
    // Idea taken from "3 Optimization Tips for C++".
    let radix2 = radix * radix;
    let radix4 = radix2 * radix2;

    // Decode 4-digits at a time
    let mut iter = buffer.iter_mut().rev();
    while value >= radix4 {
        let rem = value % radix4;
        value /= radix4;
        let r1 = (T::TWO * (rem / radix2)).as_usize();
        let r2 = (T::TWO * (rem % radix2)).as_usize();

        // This is always safe, since the table is 2*radix^2, and
        // r1 and r2 must be in the range [0, 2*radix^2-1), since the maximum
        // value of rem is `radix4-1`, which must have a div and rem
        // in the range [0, radix^2-1).
        *iter.next().unwrap() = index!(table[r2+1]);
        *iter.next().unwrap() = index!(table[r2]);
        *iter.next().unwrap() = index!(table[r1+1]);
        *iter.next().unwrap() = index!(table[r1]);
    }

    // Decode 2 digits at a time.
    while value >= radix2 {
        let rem = (T::TWO * (value % radix2)).as_usize();
        value /= radix2;

        // This is always safe, since the table is 2*radix^2, and
        // rem must be in the range [0, 2*radix^2-1).
        *iter.next().unwrap() = index!(table[rem+1]);
        *iter.next().unwrap() = index!(table[rem]);
    }

    // Decode last 2 digits.
    if value < radix {
        // This is always safe, since value < radix, so it must be < 36.
        // Digit must be <= 36.
        *iter.next().unwrap() = digit_to_char(value);
    } else {
        let rem = (T::TWO * value).as_usize();
        // This is always safe, since the table is 2*radix^2, and the value
        // must <= radix^2, so rem must be in the range [0, 2*radix^2-1).
        *iter.next().unwrap() = index!(table[rem+1]);
        *iter.next().unwrap() = index!(table[rem]);
    }

    iter.count()
}

// NAIVE

/// Naive implementation for radix-N numbers.
///
/// Use a macro to allow for u32 or u64 to be used (u32 is generally faster).
///
/// `value` must be non-negative and mutable.
#[cfg(not(feature = "table"))]
#[inline]
fn naive<T>(mut value: T, radix: T, buffer: &mut [u8])
    -> usize
    where T: UnsignedInteger
{
    // Decode all but last digit, 1 at a time.
    let mut iter = buffer.iter_mut().rev();
    while value >= radix {
        let rem = (value % radix).as_usize();
        value /= radix;

        // This is always safe, since rem must be [0, radix).
        *iter.next().unwrap() = digit_to_char(rem);
    }

    // Decode last digit.
    let rem = (value % radix).as_usize();
    // This is always safe, since rem must be [0, radix).
    *iter.next().unwrap() = digit_to_char(rem);

    iter.count()
}

/// Forward the correct arguments to the implementation.
///
/// Use a macro to allow for u32 or u64 to be used (u32 is generally faster).
///
/// `value` must be non-negative and mutable.
#[inline]
pub(crate) fn forward<T>(value: T, radix: u32, bytes: &mut [u8])
    -> usize
    where T: UnsignedInteger
{
    // Check simple use-cases
    if value == T::ZERO {
        // We know this is safe, because we confirmed the buffer is >= 2
        // in total (since we also handled the sign by here).
        index_mut!(bytes[0] = b'0');
        return 1;
    }

    // Create a temporary buffer, and copy into it.
    // Way faster than reversing a buffer in-place.
    debug_assert_radix!(radix);
    let mut buffer: [u8; BUFFER_SIZE] = explicit_uninitialized();

    let count = {
        #[cfg(not(feature = "table"))] {
            naive(value, as_cast(radix), &mut buffer)
        }

        #[cfg(all(not(feature = "radix"), feature = "table"))] {
            optimized(value, as_cast(radix), &DIGIT_TO_BASE10_SQUARED, &mut buffer)
        }

        #[cfg(all(feature = "radix", feature = "table"))]{
            let table: &[u8] = match radix {
                2   => &DIGIT_TO_BASE2_SQUARED,
                3   => &DIGIT_TO_BASE3_SQUARED,
                4   => &DIGIT_TO_BASE4_SQUARED,
                5   => &DIGIT_TO_BASE5_SQUARED,
                6   => &DIGIT_TO_BASE6_SQUARED,
                7   => &DIGIT_TO_BASE7_SQUARED,
                8   => &DIGIT_TO_BASE8_SQUARED,
                9   => &DIGIT_TO_BASE9_SQUARED,
                10  => &DIGIT_TO_BASE10_SQUARED,
                11  => &DIGIT_TO_BASE11_SQUARED,
                12  => &DIGIT_TO_BASE12_SQUARED,
                13  => &DIGIT_TO_BASE13_SQUARED,
                14  => &DIGIT_TO_BASE14_SQUARED,
                15  => &DIGIT_TO_BASE15_SQUARED,
                16  => &DIGIT_TO_BASE16_SQUARED,
                17  => &DIGIT_TO_BASE17_SQUARED,
                18  => &DIGIT_TO_BASE18_SQUARED,
                19  => &DIGIT_TO_BASE19_SQUARED,
                20  => &DIGIT_TO_BASE20_SQUARED,
                21  => &DIGIT_TO_BASE21_SQUARED,
                22  => &DIGIT_TO_BASE22_SQUARED,
                23  => &DIGIT_TO_BASE23_SQUARED,
                24  => &DIGIT_TO_BASE24_SQUARED,
                25  => &DIGIT_TO_BASE25_SQUARED,
                26  => &DIGIT_TO_BASE26_SQUARED,
                27  => &DIGIT_TO_BASE27_SQUARED,
                28  => &DIGIT_TO_BASE28_SQUARED,
                29  => &DIGIT_TO_BASE29_SQUARED,
                30  => &DIGIT_TO_BASE30_SQUARED,
                31  => &DIGIT_TO_BASE31_SQUARED,
                32  => &DIGIT_TO_BASE32_SQUARED,
                33  => &DIGIT_TO_BASE33_SQUARED,
                34  => &DIGIT_TO_BASE34_SQUARED,
                35  => &DIGIT_TO_BASE35_SQUARED,
                36  => &DIGIT_TO_BASE36_SQUARED,
                _   => unreachable!(),
            };
            optimized(value, as_cast(radix), table, &mut buffer)
        }
    };

    // We know that count <= buffer.len(), so we can safely extract a subslice
    // of buffer. This is because count is generated from `buffer.iter_mut().count()`,
    // after writing a certain number of elements, so it must be <= buffer.len().
    debug_assert!(count <= buffer.len());
    copy_to_dst(bytes, &index!(buffer[count..]))
}

/// Sanitizer for an unsigned number-to-string implementation.
#[inline]
pub(crate) fn unsigned<Value, UWide>(value: Value, radix: u32, bytes: &mut [u8])
    -> usize
    where Value: UnsignedInteger,
          UWide: UnsignedInteger
{
    // Invoke forwarder
    let v: UWide = as_cast(value);
    forward(v, radix, bytes)
}

/// Sanitizer for an signed number-to-string implementation.
#[inline]
pub(crate) fn signed<Value, UWide, IWide>(value: Value, radix: u32, bytes: &mut [u8])
    -> usize
    where Value: SignedInteger,
          UWide: UnsignedInteger,
          IWide: SignedInteger
{
    // Handle negative numbers, use an unsigned type to avoid overflow.
    // Use a wrapping neg to allow overflow.
    // These routines wrap on one condition, where the input number is equal
    // to the minimum possible value of that type (for example, -128 for i8).
    // In this case, and this case only, the value wraps to itself with
    // `x.wrapping_neg()`, so `-128i8.wrapping_neg() == -128i8` in two's
    // complement (the only true integer representation). Conversion of
    // this wrapped value to an unsigned integer of the same size with
    // effectively negates the value, for example, `-128i8 as u8 == 128u8`.
    // Due to type widening, this wrap only occurs for `i64::min_value()`,
    // and since it is converted to `u64`, this algorithm is correct
    // for all numerical input values, since Rust guarantees two's
    // complement representation for signed integers.
    let v: UWide;
    if value < Value::ZERO {
        let wide: IWide = as_cast(value);
        v = as_cast(wide.wrapping_neg());
        // We know this is safe, because we confirmed the buffer is >= 1.
        index_mut!(bytes[0] = b'-');
        forward(v, radix, &mut index_mut!(bytes[1..])) + 1
    } else {
        v = as_cast(value);
        forward(v, radix, bytes)
    }
}

// UNSAFE API

/// Expand the generic unsigned itoa function for specified types.
macro_rules! wrap_unsigned {
    ($name:ident, $t:ty, $uwide:ty) => (
        /// Serialize unsigned integer and return bytes written to.
        #[inline]
        fn $name<'a>(value: $t, radix: u8, bytes: &'a mut [u8])
            -> usize
        {
            unsigned::<$t, $uwide>(value, radix.into(), bytes)
        }
    )
}

wrap_unsigned!(u8toa_impl, u8, u32);
wrap_unsigned!(u16toa_impl, u16, u32);
wrap_unsigned!(u32toa_impl, u32, u32);
wrap_unsigned!(u64toa_impl, u64, u64);
wrap_unsigned!(usizetoa_impl, usize, usize);

#[cfg(has_i128)]
wrap_unsigned!(u128toa_impl, u128, u128);

/// Expand the generic signed itoa function for specified types.
macro_rules! wrap_signed {
    ($name:ident, $t:ty, $uwide:ty, $iwide:ty) => (
        /// Serialize signed integer and return bytes written to.
        #[inline]
        fn $name<'a>(value: $t, radix: u8, bytes: &'a mut [u8])
            -> usize
        {
            signed::<$t, $uwide, $iwide>(value, radix.into(), bytes)
        }
    )
}

wrap_signed!(i8toa_impl, i8, u32, i32);
wrap_signed!(i16toa_impl, i16, u32, i32);
wrap_signed!(i32toa_impl, i32, u32, i32);
wrap_signed!(i64toa_impl, i64, u64, i64);
wrap_signed!(isizetoa_impl, isize, usize, isize);

#[cfg(has_i128)]
wrap_signed!(i128toa_impl, i128, u128, i128);

// LOW-LEVEL API
// -------------

// RANGE API (FFI)
generate_to_range_api!(u8toa_range, u8toa_radix_range, u8, u8toa_impl, MAX_U8_SIZE);
generate_to_range_api!(u16toa_range, u16toa_radix_range, u16, u16toa_impl, MAX_U16_SIZE);
generate_to_range_api!(u32toa_range, u32toa_radix_range, u32, u32toa_impl, MAX_U32_SIZE);
generate_to_range_api!(u64toa_range, u64toa_radix_range, u64, u64toa_impl, MAX_U64_SIZE);
generate_to_range_api!(usizetoa_range, usizetoa_radix_range, usize, usizetoa_impl, MAX_USIZE_SIZE);
generate_to_range_api!(i8toa_range, i8toa_radix_range, i8, i8toa_impl, MAX_I8_SIZE);
generate_to_range_api!(i16toa_range, i16toa_radix_range, i16, i16toa_impl, MAX_I16_SIZE);
generate_to_range_api!(i32toa_range, i32toa_radix_range, i32, i32toa_impl, MAX_I32_SIZE);
generate_to_range_api!(i64toa_range, i64toa_radix_range, i64, i64toa_impl, MAX_I64_SIZE);
generate_to_range_api!(isizetoa_range, isizetoa_radix_range, isize, isizetoa_impl, MAX_ISIZE_SIZE);

#[cfg(has_i128)] generate_to_range_api!(u128toa_range, u128toa_radix_range, u128, u128toa_impl, MAX_U128_SIZE);
#[cfg(has_i128)] generate_to_range_api!(i128toa_range, i128toa_radix_range, i128, i128toa_impl, MAX_I128_SIZE);

// SLICE API
generate_to_slice_api!(u8toa_slice, u8toa_radix_slice, u8, u8toa_impl, MAX_U8_SIZE);
generate_to_slice_api!(u16toa_slice, u16toa_radix_slice, u16, u16toa_impl, MAX_U16_SIZE);
generate_to_slice_api!(u32toa_slice, u32toa_radix_slice, u32, u32toa_impl, MAX_U32_SIZE);
generate_to_slice_api!(u64toa_slice, u64toa_radix_slice, u64, u64toa_impl, MAX_U64_SIZE);
generate_to_slice_api!(usizetoa_slice, usizetoa_radix_slice, usize, usizetoa_impl, MAX_USIZE_SIZE);
generate_to_slice_api!(i8toa_slice, i8toa_radix_slice, i8, i8toa_impl, MAX_I8_SIZE);
generate_to_slice_api!(i16toa_slice, i16toa_radix_slice, i16, i16toa_impl, MAX_I16_SIZE);
generate_to_slice_api!(i32toa_slice, i32toa_radix_slice, i32, i32toa_impl, MAX_I32_SIZE);
generate_to_slice_api!(i64toa_slice, i64toa_radix_slice, i64, i64toa_impl, MAX_I64_SIZE);
generate_to_slice_api!(isizetoa_slice, isizetoa_radix_slice, isize, isizetoa_impl, MAX_ISIZE_SIZE);

#[cfg(has_i128)] generate_to_slice_api!(u128toa_slice, u128toa_radix_slice, u128, u128toa_impl, MAX_U128_SIZE);
#[cfg(has_i128)] generate_to_slice_api!(i128toa_slice, i128toa_radix_slice, i128, i128toa_impl, MAX_I128_SIZE);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use atoi::*;
    use util::test::*;
    use super::*;

    #[test]
    fn u8toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", u8toa_slice(0, &mut buffer));
        assert_eq!(b"1", u8toa_slice(1, &mut buffer));
        assert_eq!(b"127", u8toa_slice(127, &mut buffer));
        assert_eq!(b"128", u8toa_slice(128, &mut buffer));
        assert_eq!(b"255", u8toa_slice(255, &mut buffer));
        assert_eq!(b"255", u8toa_slice(-1i8 as u8, &mut buffer));
    }

    #[test]
    fn i8toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", i8toa_slice(0, &mut buffer));
        assert_eq!(b"1", i8toa_slice(1, &mut buffer));
        assert_eq!(b"127", i8toa_slice(127, &mut buffer));
        assert_eq!(b"-128", i8toa_slice(128u8 as i8, &mut buffer));
        assert_eq!(b"-1", i8toa_slice(255u8 as i8, &mut buffer));
        assert_eq!(b"-1", i8toa_slice(-1, &mut buffer));
    }

    #[test]
    fn u16toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", u16toa_slice(0, &mut buffer));
        assert_eq!(b"1", u16toa_slice(1, &mut buffer));
        assert_eq!(b"32767", u16toa_slice(32767, &mut buffer));
        assert_eq!(b"32768", u16toa_slice(32768, &mut buffer));
        assert_eq!(b"65535", u16toa_slice(65535, &mut buffer));
        assert_eq!(b"65535", u16toa_slice(-1i16 as u16, &mut buffer));
    }

    #[test]
    fn i16toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", i16toa_slice(0, &mut buffer));
        assert_eq!(b"1", i16toa_slice(1, &mut buffer));
        assert_eq!(b"32767", i16toa_slice(32767, &mut buffer));
        assert_eq!(b"-32768", i16toa_slice(32768u16 as i16, &mut buffer));
        assert_eq!(b"-1", i16toa_slice(65535u16 as i16, &mut buffer));
        assert_eq!(b"-1", i16toa_slice(-1, &mut buffer));
    }

    #[test]
    fn u32toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", u32toa_slice(0, &mut buffer));
        assert_eq!(b"1", u32toa_slice(1, &mut buffer));
        assert_eq!(b"2147483647", u32toa_slice(2147483647, &mut buffer));
        assert_eq!(b"2147483648", u32toa_slice(2147483648, &mut buffer));
        assert_eq!(b"4294967295", u32toa_slice(4294967295, &mut buffer));
        assert_eq!(b"4294967295", u32toa_slice(-1i32 as u32, &mut buffer));
    }

    #[test]
    fn i32toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", i32toa_slice(0, &mut buffer));
        assert_eq!(b"1", i32toa_slice(1, &mut buffer));
        assert_eq!(b"2147483647", i32toa_slice(2147483647, &mut buffer));
        assert_eq!(b"-2147483648", i32toa_slice(2147483648u32 as i32, &mut buffer));
        assert_eq!(b"-1", i32toa_slice(4294967295u32 as i32, &mut buffer));
        assert_eq!(b"-1", i32toa_slice(-1, &mut buffer));
    }

    #[test]
    fn u64toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", u64toa_slice(0, &mut buffer));
        assert_eq!(b"1", u64toa_slice(1, &mut buffer));
        assert_eq!(b"9223372036854775807", u64toa_slice(9223372036854775807, &mut buffer));
        assert_eq!(b"9223372036854775808", u64toa_slice(9223372036854775808, &mut buffer));
        assert_eq!(b"18446744073709551615", u64toa_slice(18446744073709551615, &mut buffer));
        assert_eq!(b"18446744073709551615", u64toa_slice(-1i64 as u64, &mut buffer));
    }

    #[test]
    fn i64toa_test() {
        let mut buffer = new_buffer();
        assert_eq!(b"0", i64toa_slice(0, &mut buffer));
        assert_eq!(b"1", i64toa_slice(1, &mut buffer));
        assert_eq!(b"9223372036854775807", i64toa_slice(9223372036854775807, &mut buffer));
        assert_eq!(b"-9223372036854775808", i64toa_slice(9223372036854775808u64 as i64, &mut buffer));
        assert_eq!(b"-1", i64toa_slice(18446744073709551615u64 as i64, &mut buffer));
        assert_eq!(b"-1", i64toa_slice(-1, &mut buffer));
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
            assert_eq!(expected.as_bytes(), i8toa_radix_slice(37, *base, &mut buffer));
        }
    }

    quickcheck! {
        fn u8_quickcheck(i: u8) -> bool {
            let mut buffer = new_buffer();
            i == atou8_slice(u8toa_slice(i, &mut buffer))
        }

        fn u16_quickcheck(i: u16) -> bool {
            let mut buffer = new_buffer();
            i == atou16_slice(u16toa_slice(i, &mut buffer))
        }

        fn u32_quickcheck(i: u32) -> bool {
            let mut buffer = new_buffer();
            i == atou32_slice(u32toa_slice(i, &mut buffer))
        }

        fn u64_quickcheck(i: u64) -> bool {
            let mut buffer = new_buffer();
            i == atou64_slice(u64toa_slice(i, &mut buffer))
        }

        fn usize_quickcheck(i: usize) -> bool {
            let mut buffer = new_buffer();
            i == atousize_slice(usizetoa_slice(i, &mut buffer))
        }

        fn i8_quickcheck(i: i8) -> bool {
            let mut buffer = new_buffer();
            i == atoi8_slice(i8toa_slice(i, &mut buffer))
        }

        fn i16_quickcheck(i: i16) -> bool {
            let mut buffer = new_buffer();
            i == atoi16_slice(i16toa_slice(i, &mut buffer))
        }

        fn i32_quickcheck(i: i32) -> bool {
            let mut buffer = new_buffer();
            i == atoi32_slice(i32toa_slice(i, &mut buffer))
        }

        fn i64_quickcheck(i: i64) -> bool {
            let mut buffer = new_buffer();
            i == atoi64_slice(i64toa_slice(i, &mut buffer))
        }

        fn isize_quickcheck(i: isize) -> bool {
            let mut buffer = new_buffer();
            i == atoisize_slice(isizetoa_slice(i, &mut buffer))
        }
    }

    proptest! {
        #[test]
        fn u8_proptest(i in u8::min_value()..u8::max_value()) {
            let mut buffer = new_buffer();
            i == atou8_slice(u8toa_slice(i, &mut buffer))
        }

        #[test]
        fn i8_proptest(i in i8::min_value()..i8::max_value()) {
            let mut buffer = new_buffer();
            i == atoi8_slice(i8toa_slice(i, &mut buffer))
        }

        #[test]
        fn u16_proptest(i in u16::min_value()..u16::max_value()) {
            let mut buffer = new_buffer();
            i == atou16_slice(u16toa_slice(i, &mut buffer))
        }

        #[test]
        fn i16_proptest(i in i16::min_value()..i16::max_value()) {
            let mut buffer = new_buffer();
            i == atoi16_slice(i16toa_slice(i, &mut buffer))
        }

        #[test]
        fn u32_proptest(i in u32::min_value()..u32::max_value()) {
            let mut buffer = new_buffer();
            i == atou32_slice(u32toa_slice(i, &mut buffer))
        }

        #[test]
        fn i32_proptest(i in i32::min_value()..i32::max_value()) {
            let mut buffer = new_buffer();
            i == atoi32_slice(i32toa_slice(i, &mut buffer))
        }

        #[test]
        fn u64_proptest(i in u64::min_value()..u64::max_value()) {
            let mut buffer = new_buffer();
            i == atou64_slice(u64toa_slice(i, &mut buffer))
        }

        #[test]
        fn i64_proptest(i in i64::min_value()..i64::max_value()) {
            let mut buffer = new_buffer();
            i == atoi64_slice(i64toa_slice(i, &mut buffer))
        }

        #[test]
        fn u128_proptest(i in u128::min_value()..u128::max_value()) {
            let mut buffer = new_buffer();
            i == atou128_slice(u128toa_slice(i, &mut buffer))
        }

        #[test]
        fn i128_proptest(i in i128::min_value()..i128::max_value()) {
            let mut buffer = new_buffer();
            i == atoi128_slice(i128toa_slice(i, &mut buffer))
        }

        #[test]
        fn usize_proptest(i in usize::min_value()..usize::max_value()) {
            let mut buffer = new_buffer();
            i == atousize_slice(usizetoa_slice(i, &mut buffer))
        }

        #[test]
        fn isize_proptest(i in isize::min_value()..isize::max_value()) {
            let mut buffer = new_buffer();
            i == atoisize_slice(isizetoa_slice(i, &mut buffer))
        }
    }

    #[test]
    #[should_panic]
    fn i8toa_buffer_test() {
        let mut buffer = [b'0'; MAX_I8_SIZE-1];
        i8toa_slice(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn i16toa_buffer_test() {
        let mut buffer = [b'0'; MAX_I16_SIZE-1];
        i16toa_slice(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn i32toa_buffer_test() {
        let mut buffer = [b'0'; MAX_I32_SIZE-1];
        i32toa_slice(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn i64toa_buffer_test() {
        let mut buffer = [b'0'; MAX_I64_SIZE-1];
        i64toa_slice(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn i128toa_buffer_test() {
        let mut buffer = [b'0'; MAX_I128_SIZE-1];
        i128toa_slice(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn isizetoa_buffer_test() {
        let mut buffer = [b'0'; MAX_ISIZE_SIZE-1];
        isizetoa_slice(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn u8toa_buffer_test() {
        let mut buffer = [b'0'; MAX_U8_SIZE-1];
        i8toa_slice(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn u16toa_buffer_test() {
        let mut buffer = [b'0'; MAX_U16_SIZE-1];
        i16toa_slice(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn u32toa_buffer_test() {
        let mut buffer = [b'0'; MAX_U32_SIZE-1];
        i32toa_slice(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn u64toa_buffer_test() {
        let mut buffer = [b'0'; MAX_U64_SIZE-1];
        i64toa_slice(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn u128toa_buffer_test() {
        let mut buffer = [b'0'; MAX_U128_SIZE-1];
        i128toa_slice(12, &mut buffer);
    }

    #[test]
    #[should_panic]
    fn usizetoa_buffer_test() {
        let mut buffer = [b'0'; MAX_USIZE_SIZE-1];
        usizetoa_slice(12, &mut buffer);
    }
}
