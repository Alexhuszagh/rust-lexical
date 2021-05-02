//! Low-level API generator.
//!
//! Uses either the internal "Grisu2", or the external "Grisu3" or "Ryu"
//! algorithms provided by `https://github.com/dtolnay`.

//  The following benchmarks were run on an "Intel(R) Core(TM) i7-6560U
//  CPU @ 2.20GHz" CPU, on Fedora 28, Linux kernel version 4.18.16-200
//  (x86-64), using the lexical formatter or `x.parse()`,
//  avoiding any inefficiencies in Rust string parsing. The code was
//  compiled with LTO and at an optimization level of 3.
//
//  The benchmarks with `std` were compiled using "rustc 1.29.2 (17a9dc751
//  2018-10-05", and the `no_std` benchmarks were compiled using "rustc
//  1.31.0-nightly (46880f41b 2018-10-15)".
//
//  The benchmark code may be found `benches/atof.rs`.
//
//  # Benchmarks
//
//  | Type  |  lexical (ns/iter) | libcore (ns/iter)     | Relative Increase |
//  |:-----:|:------------------:|:---------------------:|:-----------------:|
//  | f32   | 465,584            | 1,884,646             | 4.04x             |
//  | f64   | 539,904            | 2,276,839             | 4.22x             |
//
//  # Raw Benchmarks
//
//  ```text
//  test ftoa_f32_dtoa    ... bench:     917,561 ns/iter (+/- 45,458)
//  test ftoa_f32_lexical ... bench:     465,584 ns/iter (+/- 76,158)
//  test ftoa_f32_std     ... bench:   1,884,646 ns/iter (+/- 130,721)
//  test ftoa_f64_dtoa    ... bench:   1,092,687 ns/iter (+/- 125,136)
//  test ftoa_f64_lexical ... bench:     539,904 ns/iter (+/- 29,626)
//  test ftoa_f64_std     ... bench:   2,276,839 ns/iter (+/- 64,515)
//  ```

// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([465584, 539904]) / 1e6
//  rustcore = np.array([1884646, 2276839]) / 1e6
//  dtoa = np.array([917561, 1092687]) / 1e6
//  ryu = np.array([432878, 522515]) / 1e6
//  index = ["f32", "f64"]
//  df = pd.DataFrame({'lexical': lexical, 'rustcore': rustcore, 'dtoa': dtoa, 'ryu': ryu}, index = index, columns=['lexical', 'dtoa', 'ryu', 'rustcore'])
//  ax = df.plot.bar(rot=0, figsize=(16, 8), fontsize=14)
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  ax.legend(loc=2, prop={'size': 14})
//  plt.show()

// NOTICE
//  These internal calls are all ugly, and pass **all** the values
//  as parameters to the function calls because the overhead of
//  adding them to a struct, and passing the struct by reference,
//  was adding a ~15% performance penalty to all calls, likely because
//  the compiler wasn't able to properly inline calls.
//
//  These functions are ugly as a result.

use crate::traits::*;
use crate::util::*;

cfg_if! {
if #[cfg(feature = "radix")] {
    use super::binary::{double_binary, float_binary};
    use super::radix::{double_radix, float_radix};
}}  // cfg_if

// Select the back-end
cfg_if! {
if #[cfg(feature = "grisu3")] {
    use super::grisu3::{double_decimal, float_decimal};
} else if #[cfg(feature = "ryu")] {
    use super::ryu::{double_decimal, float_decimal};
} else {
    use super::grisu2::{double_decimal, float_decimal};
}}  //cfg_if

// TRAITS

/// Trait to define serialization of a float to string.
pub(crate) trait FloatToString: Float {
    /// Export float to decimal string with optimized algorithm.
    fn decimal<'a>(self, bytes: &'a mut [u8], format: NumberFormat) -> usize;

    /// Export float to radix string with slow algorithm.
    #[cfg(feature = "radix")]
    fn radix<'a>(self, radix: u32, bytes: &'a mut [u8], format: NumberFormat) -> usize;
}

impl FloatToString for f32 {
    #[inline]
    fn decimal<'a>(self, bytes: &'a mut [u8], format: NumberFormat) -> usize {
        float_decimal(self, bytes, format)
    }

    #[inline]
    #[cfg(feature = "radix")]
    fn radix<'a>(self, radix: u32, bytes: &'a mut [u8], format: NumberFormat) -> usize {
        if log2(radix) == 0 {
            float_radix(self, radix, bytes, format)
        } else {
            float_binary(self, radix, bytes, format)
        }
    }
}

impl FloatToString for f64 {
    #[inline]
    fn decimal<'a>(self, bytes: &'a mut [u8], format: NumberFormat) -> usize {
        double_decimal(self, bytes, format)
    }

    #[inline]
    #[cfg(feature = "radix")]
    fn radix<'a>(self, radix: u32, bytes: &'a mut [u8], format: NumberFormat) -> usize {
        if log2(radix) == 0 {
            double_radix(self, radix, bytes, format)
        } else {
            double_binary(self, radix, bytes, format)
        }
    }
}

// FTOA

/// Forward the correct arguments the ideal encoder.
#[inline]
fn forward<'a, F: FloatToString>(
    value: F,
    radix: u32,
    bytes: &'a mut [u8],
    format: NumberFormat
)
    -> usize
{
    debug_assert_radix!(radix);

    #[cfg(not(feature = "radix"))] {
        value.decimal(bytes, format)
    }

    #[cfg(feature = "radix")] {
        match radix {
            10 => value.decimal(bytes, format),
            _  => value.radix(radix, bytes, format),
        }
    }
}

/// Convert float-to-string and handle special (positive) floats.
#[inline]
fn filter_special<'a, F: FloatToString>(
    value: F,
    radix: u32,
    bytes: &'a mut [u8],
    format: NumberFormat,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
    trim_floats: bool
)
    -> usize
{
    // Logic errors, disable in release builds.
    debug_assert!(value.is_sign_positive(), "Value cannot be negative.");
    debug_assert_radix!(radix);

    // We already check for 0 in `filter_sign` if value.is_zero().
    if !trim_floats && value.is_zero() {
        // This is safe, because we confirmed the buffer is >= 4
        // in total (since we also handled the sign by here).
        return copy_to_dst(bytes, b"0.0");
    }

    if value.is_nan() {
        // This is safe, because we confirmed the buffer is >= F::FORMATTED_SIZE.
        // We have up to `F::FORMATTED_SIZE - 1` bytes from `nan_string`,
        // and up to 1 byte from the sign.
        copy_to_dst(bytes, nan_string)
    } else if value.is_special() {
        // This is safe, because we confirmed the buffer is >= F::FORMATTED_SIZE.
        // We have up to `F::FORMATTED_SIZE - 1` bytes from `inf_string`,
        // and up to 1 byte from the sign.
        copy_to_dst(bytes, inf_string)
    } else {
        forward(value, radix, bytes, format)
    }
}

/// Handle +/- values.
#[inline]
fn filter_sign<'a, F: FloatToString>(
    value: F,
    radix: u32,
    bytes: &'a mut [u8],
    format: NumberFormat,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
    trim_floats: bool
)
    -> usize
{
    debug_assert_radix!(radix);

    // Export "-0.0" and "0.0" as "0" with trimmed floats.
    if trim_floats && value.is_zero() {
        // We know this is safe, because we confirmed the buffer is >= 1.
        bytes[0] = b'0';
        return 1;
    }

    // If the sign bit is set, invert it and just set the first
    // value to "-".
    if value.is_sign_negative() {
        let value = -value;
        // We know this is safe, because we confirmed the buffer is >= 1.
        bytes[0] = b'-';
        let bytes = &mut bytes[1..];
        filter_special(value, radix, bytes, format, nan_string, inf_string, trim_floats) + 1
    } else {
        filter_special(value, radix, bytes, format, nan_string, inf_string, trim_floats)
    }
}

/// Trim a trailing ".0" from a float.
#[inline]
fn trim<'a>(bytes: &'a mut [u8], trim_floats: bool)
    -> usize
{
    // Trim a trailing ".0" from a float.
    if trim_floats && ends_with_slice(bytes, b".0") {
        bytes.len() - 2
    } else {
        bytes.len()
    }
}

/// Write float to string.
#[inline]
fn from_native<F: FloatToString>(
    value: F,
    radix: u32,
    bytes: &mut [u8],
    format: NumberFormat,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
    trim_floats: bool
)
    -> usize
{
    let len = filter_sign(value, radix, bytes, format, nan_string, inf_string, trim_floats);
    let bytes = &mut bytes[..len];
    trim(bytes, trim_floats)
}

/// Write float to string.
#[inline]
fn ftoa<F: FloatToString>(value: F, radix: u32, bytes: &mut [u8])
    -> usize
{
    from_native(value, radix, bytes, DEFAULT_FORMAT, DEFAULT_NAN_STRING, DEFAULT_INF_STRING, DEFAULT_TRIM_FLOATS)
}

/// Write float to string.
#[inline]
fn ftoa_with_options<F: FloatToString>(value: F, bytes: &mut [u8], options: &WriteFloatOptions)
    -> usize
{
    let format = options.format().unwrap_or(DEFAULT_FORMAT);
    from_native(value, options.radix(), bytes, format, options.nan_string(), options.inf_string(), options.trim_floats())
}

// TO LEXICAL

to_lexical!(ftoa, f32);
to_lexical!(ftoa, f64);

to_lexical_with_options!(ftoa_with_options, f32);
to_lexical_with_options!(ftoa_with_options, f64);

// TESTS
// -----

#[cfg(test)]
mod tests {
    // Shouldn't need to include atof, should be fine with ToLexical in scope.
    use approx::assert_relative_eq;
    use crate::traits::*;
    use crate::util::*;

    cfg_if! {
    if #[cfg(feature = "property_tests")] {
        use quickcheck::quickcheck;
        use proptest::{proptest, prop_assert_eq};
    }}  // cfg_if


    // Test data for roundtrips.
    const F32_DATA : [f32; 31] = [0., 0.1, 1., 1.1, 12., 12.1, 123., 123.1, 1234., 1234.1, 12345., 12345.1, 123456., 123456.1, 1234567., 1234567.1, 12345678., 12345678.1, 123456789., 123456789.1, 123456789.12, 123456789.123, 123456789.1234, 123456789.12345, 1.2345678912345e8, 1.2345e+8, 1.2345e+11, 1.2345e+38, 1.2345e-8, 1.2345e-11, 1.2345e-38];
    const F64_DATA: [f64; 33] = [0., 0.1, 1., 1.1, 12., 12.1, 123., 123.1, 1234., 1234.1, 12345., 12345.1, 123456., 123456.1, 1234567., 1234567.1, 12345678., 12345678.1, 123456789., 123456789.1, 123456789.12, 123456789.123, 123456789.1234, 123456789.12345, 1.2345678912345e8, 1.2345e+8, 1.2345e+11, 1.2345e+38, 1.2345e+308, 1.2345e-8, 1.2345e-11, 1.2345e-38, 1.2345e-299];

    #[test]
    fn special_bytes_test() {
        let options = WriteFloatOptions::decimal();
        let mut buffer = new_buffer();
        assert_eq!(f64::NAN.to_lexical_with_options(&mut buffer, &options), b"NaN");
        assert_eq!(f64::INFINITY.to_lexical_with_options(&mut buffer, &options), b"inf");

        let options = WriteFloatOptions::builder()
            .nan_string(b"nan")
            .inf_string(b"Infinity")
            .build()
            .unwrap();

        assert_eq!(f64::NAN.to_lexical_with_options(&mut buffer, &options), b"nan");
        assert_eq!(f64::INFINITY.to_lexical_with_options(&mut buffer, &options), b"Infinity");
    }

    #[test]
    #[cfg(feature = "radix")]
    fn f32_binary_test() {
        let mut buffer = new_buffer();
        // positive
        let options = WriteFloatOptions::builder()
            .radix(2)
            .trim_floats(true)
            .build()
            .unwrap();
        assert_eq!(as_slice(b"0"), 0.0f32.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"0"), (-0.0f32).to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"1"), 1.0f32.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"10"), 2.0f32.to_lexical_with_options(&mut buffer, &options));

        let options = WriteFloatOptions::binary();
        assert_eq!(as_slice(b"0.0"), 0.0f32.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"-0.0"), (-0.0f32).to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"1.0"), 1.0f32.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"10.0"), 2.0f32.to_lexical_with_options(&mut buffer, &options));

        assert_eq!(as_slice(b"1.1"), 1.5f32.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"1.01"), 1.25f32.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(b"1.001111000000110010", &1.2345678901234567890e0f32.to_lexical_with_options(&mut buffer, &options)[..20]);
        assert_eq!(b"1100.010110000111111", &1.2345678901234567890e1f32.to_lexical_with_options(&mut buffer, &options)[..20]);
        assert_eq!(b"1111011.011101001111", &1.2345678901234567890e2f32.to_lexical_with_options(&mut buffer, &options)[..20]);
        assert_eq!(b"10011010010.10010001", &1.2345678901234567890e3f32.to_lexical_with_options(&mut buffer, &options)[..20]);

        // negative
        assert_eq!(b"-1.001111000000110010", &(-1.2345678901234567890e0f32).to_lexical_with_options(&mut buffer, &options)[..21]);
        assert_eq!(b"-1100.010110000111111", &(-1.2345678901234567890e1f32).to_lexical_with_options(&mut buffer, &options)[..21]);
        assert_eq!(b"-1111011.011101001111", &(-1.2345678901234567890e2f32).to_lexical_with_options(&mut buffer, &options)[..21]);
        assert_eq!(b"-10011010010.10010001", &(-1.2345678901234567890e3f32).to_lexical_with_options(&mut buffer, &options)[..21]);

        // special
        assert_eq!(as_slice(b"NaN"), f32::NAN.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"inf"), f32::INFINITY.to_lexical_with_options(&mut buffer, &options));

        // bugfixes
        assert_eq!(as_slice(b"1.1010100000101011110001^-11011"), 0.000000012345f32.to_lexical_with_options(&mut buffer, &options));
    }

    #[test]
    fn f32_decimal_test() {
        let mut buffer = new_buffer();
        // positive
        let options = WriteFloatOptions::builder()
            .trim_floats(true)
            .build()
            .unwrap();
        assert_eq!(as_slice(b"0"), 0.0f32.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"0"), (-0.0f32).to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"1"), 1.0f32.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"10"), 10.0f32.to_lexical_with_options(&mut buffer, &options));

        assert_eq!(as_slice(b"0.0"), 0.0f32.to_lexical(&mut buffer));
        assert_eq!(as_slice(b"-0.0"), (-0.0f32).to_lexical(&mut buffer));
        assert_eq!(as_slice(b"1.0"), 1.0f32.to_lexical(&mut buffer));
        assert_eq!(as_slice(b"10.0"), 10.0f32.to_lexical(&mut buffer));

        assert_eq!(as_slice(b"1.234567"), &1.2345678901234567890e0f32.to_lexical(&mut buffer)[..8]);
        assert_eq!(as_slice(b"12.34567"), &1.2345678901234567890e1f32.to_lexical(&mut buffer)[..8]);
        assert_eq!(as_slice(b"123.4567"), &1.2345678901234567890e2f32.to_lexical(&mut buffer)[..8]);
        assert_eq!(as_slice(b"1234.567"), &1.2345678901234567890e3f32.to_lexical(&mut buffer)[..8]);

        // negative
        assert_eq!(as_slice(b"-1.234567"), &(-1.2345678901234567890e0f32).to_lexical(&mut buffer)[..9]);
        assert_eq!(as_slice(b"-12.34567"), &(-1.2345678901234567890e1f32).to_lexical(&mut buffer)[..9]);
        assert_eq!(as_slice(b"-123.4567"), &(-1.2345678901234567890e2f32).to_lexical(&mut buffer)[..9]);
        assert_eq!(as_slice(b"-1234.567"), &(-1.2345678901234567890e3f32).to_lexical(&mut buffer)[..9]);

        // special
        assert_eq!(as_slice(b"NaN"), f32::NAN.to_lexical(&mut buffer));
        assert_eq!(as_slice(b"inf"), f32::INFINITY.to_lexical(&mut buffer));
    }

    #[test]
    fn f32_decimal_roundtrip_test() {
        let mut buffer = new_buffer();
        for &f in F32_DATA.iter() {
            let s = f.to_lexical(&mut buffer);
            assert_relative_eq!(f32::from_lexical(s).unwrap(), f, epsilon=1e-6, max_relative=1e-6);
        }
    }

    #[test]
    #[cfg(feature = "radix")]
    fn f32_radix_roundtrip_test() {
        let mut buffer = new_buffer();
        for &f in F32_DATA.iter() {
            for radix in 2..=36 {
                // The lower accuracy is due to slight rounding errors of
                // ftoa for the Grisu method with non-10 bases.
                let writeopts = WriteFloatOptions::builder().radix(radix).build().unwrap();
                let parseopts = ParseFloatOptions::builder().radix(radix).build().unwrap();
                let s = f.to_lexical_with_options(&mut buffer, &writeopts);
                assert_relative_eq!(f32::from_lexical_with_options(s, &parseopts).unwrap(), f, max_relative=2e-5);
            }
        }
    }

    #[test]
    #[cfg(feature = "radix")]
    fn f64_binary_test() {
        let mut buffer = new_buffer();
        // positive
        let options = WriteFloatOptions::builder()
            .radix(2)
            .trim_floats(true)
            .build()
            .unwrap();
        assert_eq!(as_slice(b"0"), 0.0f64.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"0"), (-0.0f64).to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"1"), 1.0f64.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"10"), 2.0f64.to_lexical_with_options(&mut buffer, &options));

        let options = WriteFloatOptions::binary();
        assert_eq!(as_slice(b"0.0"), 0.0f64.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"-0.0"), (-0.0f64).to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"1.0"), 1.0f64.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"10.0"), 2.0f64.to_lexical_with_options(&mut buffer, &options));

        assert_eq!(as_slice(b"1.00111100000011001010010000101000110001"), &1.2345678901234567890e0f64.to_lexical_with_options(&mut buffer, &options)[..40]);
        assert_eq!(as_slice(b"1100.01011000011111100110100110010111101"), &1.2345678901234567890e1f64.to_lexical_with_options(&mut buffer, &options)[..40]);
        assert_eq!(as_slice(b"1111011.01110100111100000001111111101101"), &1.2345678901234567890e2f64.to_lexical_with_options(&mut buffer, &options)[..40]);
        assert_eq!(as_slice(b"10011010010.1001000101100001001111110100"), &1.2345678901234567890e3f64.to_lexical_with_options(&mut buffer, &options)[..40]);

        // negative
        assert_eq!(as_slice(b"-1.00111100000011001010010000101000110001"), &(-1.2345678901234567890e0f64).to_lexical_with_options(&mut buffer, &options)[..41]);
        assert_eq!(as_slice(b"-1100.01011000011111100110100110010111101"), &(-1.2345678901234567890e1f64).to_lexical_with_options(&mut buffer, &options)[..41]);
        assert_eq!(as_slice(b"-1111011.01110100111100000001111111101101"), &(-1.2345678901234567890e2f64).to_lexical_with_options(&mut buffer, &options)[..41]);
        assert_eq!(as_slice(b"-10011010010.1001000101100001001111110100"), &(-1.2345678901234567890e3f64).to_lexical_with_options(&mut buffer, &options)[..41]);

        // special
        assert_eq!(as_slice(b"NaN"), f64::NAN.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"inf"), f64::INFINITY.to_lexical_with_options(&mut buffer, &options));
    }

    #[test]
    fn f64_decimal_test() {
        let mut buffer = new_buffer();
        // positive
        let options = WriteFloatOptions::builder()
            .trim_floats(true)
            .build()
            .unwrap();
        assert_eq!(as_slice(b"0"), 0.0.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"0"), (-0.0).to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"1"), 1.0.to_lexical_with_options(&mut buffer, &options));
        assert_eq!(as_slice(b"10"), 10.0.to_lexical_with_options(&mut buffer, &options));

        assert_eq!(as_slice(b"0.0"), 0.0.to_lexical(&mut buffer));
        assert_eq!(as_slice(b"-0.0"), (-0.0).to_lexical(&mut buffer));
        assert_eq!(as_slice(b"1.0"), 1.0.to_lexical(&mut buffer));
        assert_eq!(as_slice(b"10.0"), 10.0.to_lexical(&mut buffer));

        assert_eq!(as_slice(b"1.234567"), &1.2345678901234567890e0.to_lexical(&mut buffer)[..8]);
        assert_eq!(as_slice(b"12.34567"), &1.2345678901234567890e1.to_lexical(&mut buffer)[..8]);
        assert_eq!(as_slice(b"123.4567"), &1.2345678901234567890e2.to_lexical(&mut buffer)[..8]);
        assert_eq!(as_slice(b"1234.567"), &1.2345678901234567890e3.to_lexical(&mut buffer)[..8]);

        // negative
        assert_eq!(as_slice(b"-1.234567"), &(-1.2345678901234567890e0).to_lexical(&mut buffer)[..9]);
        assert_eq!(as_slice(b"-12.34567"), &(-1.2345678901234567890e1).to_lexical(&mut buffer)[..9]);
        assert_eq!(as_slice(b"-123.4567"), &(-1.2345678901234567890e2).to_lexical(&mut buffer)[..9]);
        assert_eq!(as_slice(b"-1234.567"), &(-1.2345678901234567890e3).to_lexical(&mut buffer)[..9]);

        // special
        assert_eq!(b"NaN".to_vec(), f64::NAN.to_lexical(&mut buffer));
        assert_eq!(b"inf".to_vec(), f64::INFINITY.to_lexical(&mut buffer));
    }

    #[test]
    fn f64_decimal_roundtrip_test() {
        let mut buffer = new_buffer();
        for &f in F64_DATA.iter() {
            let s = f.to_lexical(&mut buffer);
            assert_relative_eq!(f64::from_lexical(s).unwrap(), f, epsilon=1e-12, max_relative=1e-12);
        }
    }

    #[test]
    #[cfg(feature = "radix")]
    fn f64_radix_roundtrip_test() {
        let mut buffer = new_buffer();
        for &f in F64_DATA.iter() {
            for radix in 2..=36 {
                // The lower accuracy is due to slight rounding errors of
                // ftoa for the Grisu method with non-10 bases.
                let writeopts = WriteFloatOptions::builder().radix(radix).build().unwrap();
                let parseopts = ParseFloatOptions::builder().radix(radix).build().unwrap();
                let s = f.to_lexical_with_options(&mut buffer, &writeopts);
                assert_relative_eq!(f64::from_lexical_with_options(s, &parseopts).unwrap(), f, max_relative=3e-5);
            }
        }
    }

    #[cfg(feature = "property_tests")]
    quickcheck! {
        fn f32_quickcheck(f: f32) -> bool {
            let mut buffer = new_buffer();
            let parsed = f32::from_lexical(f.to_lexical(&mut buffer)).unwrap();
            if f.is_nan() {
                parsed.is_nan()
            } else {
                f == parsed
            }
        }

        fn f64_quickcheck(f: f64) -> bool {
            let mut buffer = new_buffer();
            let parsed = f64::from_lexical(f.to_lexical(&mut buffer)).unwrap();
            if f.is_nan() {
                parsed.is_nan()
            } else {
                f == parsed
            }
        }
    }

    #[cfg(feature = "property_tests")]
    proptest! {
        #[test]
        fn f32_proptest(i in f32::MIN..f32::MAX) {
            let mut buffer = new_buffer();
            prop_assert_eq!(i, f32::from_lexical(i.to_lexical(&mut buffer)).unwrap());
        }

        #[test]
        fn f64_proptest(i in f64::MIN..f64::MAX) {
            let mut buffer = new_buffer();
            prop_assert_eq!(i, f64::from_lexical(i.to_lexical(&mut buffer)).unwrap());
        }
    }

    #[test]
    #[should_panic]
    fn f32_buffer_test() {
        let mut buffer = [b'0'; f32::FORMATTED_SIZE_DECIMAL-1];
        1.2345f32.to_lexical(&mut buffer);
    }

    #[test]
    #[should_panic]
    fn f64_buffer_test() {
        let mut buffer = [b'0'; f64::FORMATTED_SIZE_DECIMAL-1];
        1.2345f64.to_lexical(&mut buffer);
    }
}
