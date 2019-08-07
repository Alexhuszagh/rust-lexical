//! Fast lexical integer-to-string conversion routines.

//  The following benchmarks were run on an "Intel(R) Core(TM) i7-6560U
//  CPU @ 2.20GHz" CPU, on Fedora 28, Linux kernel version 4.18.16-200
//  (x86-64), using the lexical formatter, `itoa::write()` or `x.to_string()`,
//  avoiding any inefficiencies in Rust string parsing for `format!(...)`
//  or `write!()` macros. The code was compiled with LTO and at an optimization
//  level of 3.
//
//  The benchmarks with `std` were compiled using "rustc 1.32.0
// (9fda7c223 2019-01-16".
//
//  The benchmark code may be found `benches/itoa.rs`.
//
//  # Benchmarks
//
//  | Type  |  lexical (ns/iter) | libcore (ns/iter)     | Relative Increase |
//  |:-----:|:------------------:|:---------------------:|:-----------------:|
//  | u8    | 122,329            | 413,025               | 3.38x             |
//  | u16   | 119,888            | 405,945               | 3.41x             |
//  | u32   | 121,150            | 423,174               | 3.49x             |
//  | u64   | 165,609            | 531,862               | 3.21x             |
//  | i8    | 151,478            | 458,374               | 3.03x             |
//  | i16   | 153,211            | 489,010               | 3.19x             |
//  | i32   | 149,433            | 517,710               | 3.46x             |
//  | i64   | 195,575            | 553,387               | 2.83x             |
//
//  # Raw Benchmarks
//
//  ```text
// test itoa_i8_itoa     ... bench:     130,969 ns/iter (+/- 7,420)
// test itoa_i8_lexical  ... bench:     151,478 ns/iter (+/- 7,510)
// test itoa_i8_std      ... bench:     458,374 ns/iter (+/- 26,663)
// test itoa_i16_itoa    ... bench:     143,344 ns/iter (+/- 9,495)
// test itoa_i16_lexical ... bench:     153,211 ns/iter (+/- 7,365)
// test itoa_i16_std     ... bench:     489,010 ns/iter (+/- 25,319)
// test itoa_i32_itoa    ... bench:     176,494 ns/iter (+/- 9,596)
// test itoa_i32_lexical ... bench:     149,433 ns/iter (+/- 5,803)
// test itoa_i32_std     ... bench:     517,710 ns/iter (+/- 38,439)
// test itoa_i64_itoa    ... bench:     205,055 ns/iter (+/- 12,436)
// test itoa_i64_lexical ... bench:     195,575 ns/iter (+/- 8,007)
// test itoa_i64_std     ... bench:     553,387 ns/iter (+/- 26,731)
// test itoa_u8_itoa     ... bench:     112,529 ns/iter (+/- 4,514)
// test itoa_u8_lexical  ... bench:     122,329 ns/iter (+/- 9,902)
// test itoa_u8_std      ... bench:     413,025 ns/iter (+/- 30,262)
// test itoa_u16_itoa    ... bench:      91,936 ns/iter (+/- 5,405)
// test itoa_u16_lexical ... bench:     119,888 ns/iter (+/- 6,089)
// test itoa_u16_std     ... bench:     405,945 ns/iter (+/- 24,104)
// test itoa_u32_itoa    ... bench:     161,679 ns/iter (+/- 6,719)
// test itoa_u32_lexical ... bench:     121,150 ns/iter (+/- 7,580)
// test itoa_u32_std     ... bench:     423,174 ns/iter (+/- 21,801)
// test itoa_u64_itoa    ... bench:     203,847 ns/iter (+/- 18,512)
// test itoa_u64_lexical ... bench:     165,609 ns/iter (+/- 8,620)
// test itoa_u64_std     ... bench:     531,862 ns/iter (+/- 31,223)
//  ```

// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([122329, 119888, 121150, 165609, 151478, 153211, 149433, 195575]) / 1e6
//  itoa = np.array([112529, 91936, 161679, 203847, 130969, 143344, 176494, 205055]) / 1e6
//  rustcore = np.array([413025, 405945, 423174, 531862, 458374, 489010, 517710, 553387]) / 1e6
//  index = ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
//  df = pd.DataFrame({'lexical': lexical, 'itoa': itoa, 'rustcore': rustcore}, index = index, columns=['lexical', 'itoa', 'rustcore'])
//  ax = df.plot.bar(rot=0, figsize=(16, 8), fontsize=14, color=['#E24A33', '#988ED5', '#348ABD'])
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  ax.legend(loc=2, prop={'size': 14})
//  plt.show()

use util::*;

// BASE10
// ------

// Hyper-optimized integer formatters using bit-twiddling tricks for
// base10. These are meant to be correct, however, they use bit-twiddling
// tricks so they may not be very legible.

// Calculate the number of leading 0s.
#[cfg(feature = "table")]
macro_rules! cltz {
    ($value:ident) => {
        $value.leading_zeros().as_usize()
    };
}

// Calculate the offset where the digits were first written.
#[cfg(feature = "table")]
macro_rules! calculate_offset {
    ($value:ident, $digits:ident, $max_digits:ident, $size:expr) => ({
        // Get the log2 of the value to estimate the log10 quickly.
        // log2(0) is undefined, always ensure 1 bit is set.
        let value = $value | 1;
        let log2 = $size - cltz!(value);

        // Estimate log10(value) to calculate number of digits.
        // Put in safe guards so we always have at least 1 digit.
        // Our magic numbers are:
        //  1233 / 2^12 == log10(2)
        // These magic numbers are valid for any value <= 2**18,
        // which encompasses all offsets (<= 40).
        let digits = (log2 * 1233) >> 12;
        let mut offset = $max_digits - digits - 1;
        debug_assert!(offset < $digits.len());
        if digits != 0 && unchecked_index!($digits[offset]) == b'0' {
            offset += 1;
        }

        offset
    });
}

/// Trait for highly-optimized base10 implementations.
#[cfg(feature = "table")]
pub(crate) trait OptimizedItoaBase10 {
    fn optimized_itoa(self, buffer: &mut [u8]) -> usize;
}

/// Trait for highly-optimized base10 implementations.
#[cfg(not(feature = "table"))]
pub(crate) trait OptimizedItoaBase10
{}

/// Implement OptimizedItoaBase10 for type.
macro_rules! optimized_itoa_base10 {
    ($t:ty, $optimized_cb:ident) => {
        #[cfg(feature = "table")]
        impl OptimizedItoaBase10 for $t {
            perftools_inline_always!{
            fn optimized_itoa(self, buffer: &mut [u8]) -> usize {
                $optimized_cb(self, buffer)
            }}
        }

        #[cfg(not(feature = "table"))]
        impl OptimizedItoaBase10 for $t
        {}
    };
}

// U8

// Highly-optimized float formatter for u8.
perftools_inline!{
#[cfg(feature = "table")]
#[allow(unused_unsafe)]
fn optimized_u8(value: u8, digits: &mut [u8])
    -> usize
{
    // Lookup table for base-10 digits.
    const TABLE: &[u8] = &DIGIT_TO_BASE10_SQUARED;
    // Max number of digits in a u8.
    const MAX_DIGITS: usize = 4;
    // Magic numbers for very fast 32-bit division.
    // 1e-2 ~= 1374389535 / 2**37
    const E_2: u64 = 1374389535;
    // Check constantly we have no indexing issues.
    debug_assert!(digits.len() >= MAX_DIGITS);

    let uvalue_0 = value.as_u32();
    let uvalue_1 = ((uvalue_0.as_u64() * E_2) >> 37).as_u32();
    let index_0 = (uvalue_0 * 2 - uvalue_1 * 200).as_usize();
    let index_1 = (uvalue_1 * 2).as_usize();

    unchecked_index_mut!(digits[2] = unchecked_index!(TABLE[index_0]));
    unchecked_index_mut!(digits[3] = unchecked_index!(TABLE[index_0+1]));
    unchecked_index_mut!(digits[0] = unchecked_index!(TABLE[index_1]));
    unchecked_index_mut!(digits[1] = unchecked_index!(TABLE[index_1+1]));

    calculate_offset!(value, digits, MAX_DIGITS, 8)
}}

optimized_itoa_base10! { u8, optimized_u8 }

// U16

// Highly-optimized float formatter for u16.
perftools_inline!{
#[cfg(feature = "table")]
#[allow(unused_unsafe)]
fn optimized_u16(value: u16, digits: &mut [u8])
    -> usize
{
    // Lookup table for base-10 digits.
    const TABLE: &[u8] = &DIGIT_TO_BASE10_SQUARED;
    // Max number of digits in a u16.
    const MAX_DIGITS: usize = 6;
    // Magic numbers for very fast 32-bit division.
    // 1e-2 ~= 1374389535 / 2**37
    const E_2: u64 = 1374389535;
    // 1e-4 ~= 3518437209 / 2**45
    const E_4: u64 = 3518437209;
    // Check constantly we have no indexing issues.
    debug_assert!(digits.len() >= MAX_DIGITS);

    let uvalue_0 = value.as_u32();
    let uvalue_1 = ((uvalue_0.as_u64() * E_2) >> 37).as_u32();
    let uvalue_2 = ((uvalue_0.as_u64() * E_4) >> 45).as_u32();
    let index_0 = (uvalue_0 * 2 - uvalue_1 * 200).as_usize();
    let index_1 = (uvalue_1 * 2 - uvalue_2 * 200).as_usize();
    let index_2 = (uvalue_2 * 2).as_usize();

    unchecked_index_mut!(digits[4] = unchecked_index!(TABLE[index_0]));
    unchecked_index_mut!(digits[5] = unchecked_index!(TABLE[index_0+1]));
    unchecked_index_mut!(digits[2] = unchecked_index!(TABLE[index_1]));
    unchecked_index_mut!(digits[3] = unchecked_index!(TABLE[index_1+1]));
    unchecked_index_mut!(digits[0] = unchecked_index!(TABLE[index_2]));
    unchecked_index_mut!(digits[1] = unchecked_index!(TABLE[index_2+1]));

    calculate_offset!(value, digits, MAX_DIGITS, 16)
}}

optimized_itoa_base10! { u16, optimized_u16 }

// U32

// Highly-optimized float formatter for u32.
perftools_inline!{
#[cfg(feature = "table")]
#[allow(unused_unsafe)]
fn optimized_u32(value: u32, digits: &mut [u8])
    -> usize
{
    // Lookup table for base-10 digits.
    const TABLE: &[u8] = &DIGIT_TO_BASE10_SQUARED;
    // Max number of digits in a 32.
    const MAX_DIGITS: usize = 10;
    // Magic numbers for very fast 32-bit division.
    // 1e-2 ~= 1374389535 / 2**37
    const E_2: u64 = 1374389535;
    // 1e-4 ~= 3518437209 / 2**45
    const E_4: u64 = 3518437209;
    // 1e-5 ~= 1125899907 / 2**50
    const E_6: u64 = 1125899907;
    // Check constantly we have no indexing issues.
    debug_assert!(digits.len() >= MAX_DIGITS);

    // A u32 has 10 digits, therefore, the top2 refers to `value / 1e8`.
    let top2 = (value / 100000000).as_u32();

    // Extremely fast division based on Terje Mathiesen.
    let uvalue_0 = value.as_u32().wrapping_sub(top2.wrapping_mul(100000000));
    let uvalue_1 = ((uvalue_0.as_u64() * E_2) >> 37).as_u32();
    let uvalue_2 = ((uvalue_0.as_u64() * E_4) >> 45).as_u32();
    let uvalue_3 = ((uvalue_0.as_u64() * E_6) >> 50).as_u32();
    let index_0 = (uvalue_0 * 2 - uvalue_1 * 200).as_usize();
    let index_1 = (uvalue_1 * 2 - uvalue_2 * 200).as_usize();
    let index_2 = (uvalue_2 * 2 - uvalue_3 * 200).as_usize();
    let index_3 = (uvalue_3 * 2).as_usize();

    unchecked_index_mut!(digits[8] = unchecked_index!(TABLE[index_0]));
    unchecked_index_mut!(digits[9] = unchecked_index!(TABLE[index_0+1]));
    unchecked_index_mut!(digits[6] = unchecked_index!(TABLE[index_1]));
    unchecked_index_mut!(digits[7] = unchecked_index!(TABLE[index_1+1]));
    unchecked_index_mut!(digits[4] = unchecked_index!(TABLE[index_2]));
    unchecked_index_mut!(digits[5] = unchecked_index!(TABLE[index_2+1]));
    unchecked_index_mut!(digits[2] = unchecked_index!(TABLE[index_3]));
    unchecked_index_mut!(digits[3] = unchecked_index!(TABLE[index_3+1]));

    let uvalue_4 = top2;
    let index_4 = (uvalue_4 * 2).as_usize();

    unchecked_index_mut!(digits[0] = unchecked_index!(TABLE[index_4]));
    unchecked_index_mut!(digits[1] = unchecked_index!(TABLE[index_4+1]));

    calculate_offset!(value, digits, MAX_DIGITS, 32)
}}

optimized_itoa_base10! { u32, optimized_u32 }

// U64

// Highly-optimized float formatter for u64.
// Adapted from:
//     https://gist.github.com/Veedrac/069dd35f896de4df4e14b881a455ca47
perftools_inline!{
#[cfg(feature = "table")]
#[allow(unused_unsafe)]
fn optimized_u64(value: u64, digits: &mut [u8])
    -> usize
{
    // Lookup table for base-10 digits.
    const TABLE: &[u8] = &DIGIT_TO_BASE10_SQUARED;
    // Max number of digits in a u64.
    const MAX_DIGITS: usize = 20;
    // Magic numbers for very fast 32-bit division.
    // 1e-2 ~= 1374389535 / 2**37
    const E_2: u64 = 1374389535;
    // 1e-4 ~= 3518437209 / 2**45
    const E_4: u64 = 3518437209;
    // 1e-5 ~= 1125899907 / 2**50
    const E_6: u64 = 1125899907;
    // Check constantly we have no indexing issues.
    debug_assert!(digits.len() >= MAX_DIGITS);

    // A u64 has 20 digits, therefore, the top12 and top4 refer to
    // `value / 1e8` and `value / 1e16`, respectively.
    let top12 = (value / 100000000).as_u32();
    let top4 = (value / 10000000000000000).as_u32();

    // Extremely fast division based on Terje Mathiesen.
    let uvalue_0 = value.as_u32().wrapping_sub(top12.wrapping_mul(100000000));
    let uvalue_1 = ((uvalue_0.as_u64() * E_2) >> 37).as_u32();
    let uvalue_2 = ((uvalue_0.as_u64() * E_4) >> 45).as_u32();
    let uvalue_3 = ((uvalue_0.as_u64() * E_6) >> 50).as_u32();
    let index_0 = (uvalue_0 * 2 - uvalue_1 * 200).as_usize();
    let index_1 = (uvalue_1 * 2 - uvalue_2 * 200).as_usize();
    let index_2 = (uvalue_2 * 2 - uvalue_3 * 200).as_usize();
    let index_3 = (uvalue_3 * 2).as_usize();

    unchecked_index_mut!(digits[18] = unchecked_index!(TABLE[index_0]));
    unchecked_index_mut!(digits[19] = unchecked_index!(TABLE[index_0+1]));
    unchecked_index_mut!(digits[16] = unchecked_index!(TABLE[index_1]));
    unchecked_index_mut!(digits[17] = unchecked_index!(TABLE[index_1+1]));
    unchecked_index_mut!(digits[14] = unchecked_index!(TABLE[index_2]));
    unchecked_index_mut!(digits[15] = unchecked_index!(TABLE[index_2+1]));
    unchecked_index_mut!(digits[12] = unchecked_index!(TABLE[index_3]));
    unchecked_index_mut!(digits[13] = unchecked_index!(TABLE[index_3+1]));

    let uvalue_4 = top12.wrapping_sub(top4.wrapping_mul(100000000));
    let uvalue_5 = ((uvalue_4.as_u64() * E_2) >> 37).as_u32();
    let uvalue_6 = ((uvalue_4.as_u64() * E_4) >> 45).as_u32();
    let uvalue_7 = ((uvalue_4.as_u64() * E_6) >> 50).as_u32();
    let index_4 = (uvalue_4 * 2 - uvalue_5 * 200).as_usize();
    let index_5 = (uvalue_5 * 2 - uvalue_6 * 200).as_usize();
    let index_6 = (uvalue_6 * 2 - uvalue_7 * 200).as_usize();
    let index_7 = (uvalue_7 * 2).as_usize();

    unchecked_index_mut!(digits[10] = unchecked_index!(TABLE[index_4]));
    unchecked_index_mut!(digits[11] = unchecked_index!(TABLE[index_4+1]));
    unchecked_index_mut!(digits[8] = unchecked_index!(TABLE[index_5]));
    unchecked_index_mut!(digits[9] = unchecked_index!(TABLE[index_5+1]));
    unchecked_index_mut!(digits[6] = unchecked_index!(TABLE[index_6]));
    unchecked_index_mut!(digits[7] = unchecked_index!(TABLE[index_6+1]));
    unchecked_index_mut!(digits[4] = unchecked_index!(TABLE[index_7]));
    unchecked_index_mut!(digits[5] = unchecked_index!(TABLE[index_7+1]));

    let uvalue_8 = top4;
    let uvalue_9 = ((uvalue_8.as_u64() * E_2) >> 37).as_u32();
    let index_8 = (uvalue_8 * 2 - uvalue_9 * 200).as_usize();
    let index_9 = (uvalue_9 * 2).as_usize();

    unchecked_index_mut!(digits[2] = unchecked_index!(TABLE[index_8]));
    unchecked_index_mut!(digits[3] = unchecked_index!(TABLE[index_8+1]));
    unchecked_index_mut!(digits[0] = unchecked_index!(TABLE[index_9]));
    unchecked_index_mut!(digits[1] = unchecked_index!(TABLE[index_9+1]));

    calculate_offset!(value, digits, MAX_DIGITS, 64)
}}

optimized_itoa_base10! { u64, optimized_u64 }

// U128

// Highly-optimized float formatter for u128.
perftools_inline!{
#[cfg(all(has_i128, feature = "table"))]
#[allow(unused_unsafe)]
fn optimized_u128(value: u128, digits: &mut [u8])
    -> usize
{
    // Lookup table for base-10 digits.
    const TABLE: &[u8] = &DIGIT_TO_BASE10_SQUARED;
    // Max number of digits in a u64.
    const MAX_DIGITS: usize = 40;
    // Magic numbers for very fast 32-bit division.
    // 1e-2 ~= 1374389535 / 2**37
    const E_2: u64 = 1374389535;
    // 1e-4 ~= 3518437209 / 2**45
    const E_4: u64 = 3518437209;
    // 1e-5 ~= 1125899907 / 2**50
    const E_6: u64 = 1125899907;
    // Check constantly we have no indexing issues.
    debug_assert!(digits.len() >= MAX_DIGITS);

    let top32 = (value / 100000000).as_u32();
    let top24 = (value / 10000000000000000).as_u32();
    let top16 = (value / 1000000000000000000000000).as_u32();
    let top8 = (value / 100000000000000000000000000000000).as_u32();

    // Extremely fast division based on Terje Mathiesen.
    let uvalue_0 = value.as_u32().wrapping_sub(top32.wrapping_mul(100000000));
    let uvalue_1 = ((uvalue_0.as_u64() * E_2) >> 37).as_u32();
    let uvalue_2 = ((uvalue_0.as_u64() * E_4) >> 45).as_u32();
    let uvalue_3 = ((uvalue_0.as_u64() * E_6) >> 50).as_u32();
    let index_0 = (uvalue_0 * 2 - uvalue_1 * 200).as_usize();
    let index_1 = (uvalue_1 * 2 - uvalue_2 * 200).as_usize();
    let index_2 = (uvalue_2 * 2 - uvalue_3 * 200).as_usize();
    let index_3 = (uvalue_3 * 2).as_usize();

    unchecked_index_mut!(digits[38] = unchecked_index!(TABLE[index_0]));
    unchecked_index_mut!(digits[39] = unchecked_index!(TABLE[index_0+1]));
    unchecked_index_mut!(digits[36] = unchecked_index!(TABLE[index_1]));
    unchecked_index_mut!(digits[37] = unchecked_index!(TABLE[index_1+1]));
    unchecked_index_mut!(digits[34] = unchecked_index!(TABLE[index_2]));
    unchecked_index_mut!(digits[35] = unchecked_index!(TABLE[index_2+1]));
    unchecked_index_mut!(digits[32] = unchecked_index!(TABLE[index_3]));
    unchecked_index_mut!(digits[33] = unchecked_index!(TABLE[index_3+1]));

    let uvalue_4 = top32.wrapping_sub(top24.wrapping_mul(100000000));
    let uvalue_5 = ((uvalue_4.as_u64() * E_2) >> 37).as_u32();
    let uvalue_6 = ((uvalue_4.as_u64() * E_4) >> 45).as_u32();
    let uvalue_7 = ((uvalue_4.as_u64() * E_6) >> 50).as_u32();
    let index_4 = (uvalue_4 * 2 - uvalue_5 * 200).as_usize();
    let index_5 = (uvalue_5 * 2 - uvalue_6 * 200).as_usize();
    let index_6 = (uvalue_6 * 2 - uvalue_7 * 200).as_usize();
    let index_7 = (uvalue_7 * 2).as_usize();

    unchecked_index_mut!(digits[30] = unchecked_index!(TABLE[index_4]));
    unchecked_index_mut!(digits[31] = unchecked_index!(TABLE[index_4+1]));
    unchecked_index_mut!(digits[28] = unchecked_index!(TABLE[index_5]));
    unchecked_index_mut!(digits[29] = unchecked_index!(TABLE[index_5+1]));
    unchecked_index_mut!(digits[26] = unchecked_index!(TABLE[index_6]));
    unchecked_index_mut!(digits[27] = unchecked_index!(TABLE[index_6+1]));
    unchecked_index_mut!(digits[24] = unchecked_index!(TABLE[index_7]));
    unchecked_index_mut!(digits[25] = unchecked_index!(TABLE[index_7+1]));

    let uvalue_8 = top24.wrapping_sub(top16.wrapping_mul(100000000));
    let uvalue_9 = ((uvalue_8.as_u64() * E_2) >> 37).as_u32();
    let uvalue_10 = ((uvalue_8.as_u64() * E_4) >> 45).as_u32();
    let uvalue_11 = ((uvalue_8.as_u64() * E_6) >> 50).as_u32();
    let index_8 = (uvalue_8 * 2 - uvalue_9 * 200).as_usize();
    let index_9 = (uvalue_9 * 2 - uvalue_10 * 200).as_usize();
    let index_10 = (uvalue_10 * 2 - uvalue_11 * 200).as_usize();
    let index_11 = (uvalue_11 * 2).as_usize();

    unchecked_index_mut!(digits[22] = unchecked_index!(TABLE[index_8]));
    unchecked_index_mut!(digits[23] = unchecked_index!(TABLE[index_8+1]));
    unchecked_index_mut!(digits[20] = unchecked_index!(TABLE[index_9]));
    unchecked_index_mut!(digits[21] = unchecked_index!(TABLE[index_9+1]));
    unchecked_index_mut!(digits[18] = unchecked_index!(TABLE[index_10]));
    unchecked_index_mut!(digits[19] = unchecked_index!(TABLE[index_10+1]));
    unchecked_index_mut!(digits[16] = unchecked_index!(TABLE[index_11]));
    unchecked_index_mut!(digits[17] = unchecked_index!(TABLE[index_11+1]));

    let uvalue_12 = top16.wrapping_sub(top8.wrapping_mul(100000000));
    let uvalue_13 = ((uvalue_12.as_u64() * E_2) >> 37).as_u32();
    let uvalue_14 = ((uvalue_12.as_u64() * E_4) >> 45).as_u32();
    let uvalue_15 = ((uvalue_12.as_u64() * E_6) >> 50).as_u32();
    let index_12 = (uvalue_12 * 2 - uvalue_13 * 200).as_usize();
    let index_13 = (uvalue_13 * 2 - uvalue_14 * 200).as_usize();
    let index_14 = (uvalue_14 * 2 - uvalue_15 * 200).as_usize();
    let index_15 = (uvalue_15 * 2).as_usize();

    unchecked_index_mut!(digits[14] = unchecked_index!(TABLE[index_12]));
    unchecked_index_mut!(digits[15] = unchecked_index!(TABLE[index_12+1]));
    unchecked_index_mut!(digits[12] = unchecked_index!(TABLE[index_13]));
    unchecked_index_mut!(digits[13] = unchecked_index!(TABLE[index_13+1]));
    unchecked_index_mut!(digits[10] = unchecked_index!(TABLE[index_14]));
    unchecked_index_mut!(digits[11] = unchecked_index!(TABLE[index_14+1]));
    unchecked_index_mut!(digits[8] = unchecked_index!(TABLE[index_15]));
    unchecked_index_mut!(digits[9] = unchecked_index!(TABLE[index_15+1]));

    let uvalue_16 = top8;
    let uvalue_17 = ((uvalue_16.as_u64() * E_2) >> 37).as_u32();
    let uvalue_18 = ((uvalue_16.as_u64() * E_4) >> 45).as_u32();
    let uvalue_19 = ((uvalue_16.as_u64() * E_6) >> 50).as_u32();
    let index_16 = (uvalue_16 * 2 - uvalue_17 * 200).as_usize();
    let index_17 = (uvalue_17 * 2 - uvalue_18 * 200).as_usize();
    let index_18 = (uvalue_18 * 2 - uvalue_19 * 200).as_usize();
    let index_19 = (uvalue_19 * 2).as_usize();

    unchecked_index_mut!(digits[6] = unchecked_index!(TABLE[index_16]));
    unchecked_index_mut!(digits[7] = unchecked_index!(TABLE[index_16+1]));
    unchecked_index_mut!(digits[4] = unchecked_index!(TABLE[index_17]));
    unchecked_index_mut!(digits[5] = unchecked_index!(TABLE[index_17+1]));
    unchecked_index_mut!(digits[2] = unchecked_index!(TABLE[index_18]));
    unchecked_index_mut!(digits[3] = unchecked_index!(TABLE[index_18+1]));
    unchecked_index_mut!(digits[0] = unchecked_index!(TABLE[index_19]));
    unchecked_index_mut!(digits[1] = unchecked_index!(TABLE[index_19+1]));

    calculate_offset!(value, digits, MAX_DIGITS, 128)
}}

#[cfg(has_i128)]
optimized_itoa_base10! { u128, optimized_u128 }

// USIZE

cfg_if! {
if #[cfg(target_pointer_width = "16")] {
    // Cast usize as u16.
    perftools_inline!{
    #[cfg(feature = "table")]
    fn optimized_usize(value: usize, buffer: &mut [u8])
        -> usize
    {
        optimized_u16(value.as_u16(), buffer)
    }}
} else if #[cfg(target_pointer_width = "32")] {
    // Cast usize as u32.
    perftools_inline!{
    #[cfg(feature = "table")]
    fn optimized_usize(value: usize, buffer: &mut [u8])
        -> usize
    {
        optimized_u32(value.as_u32(), buffer)
    }}
} else if #[cfg(target_pointer_width = "64")] {
    // Cast usize as u64.
    perftools_inline!{
    #[cfg(feature = "table")]
    fn optimized_usize(value: usize, buffer: &mut [u8])
        -> usize
    {
        optimized_u64(value.as_u64(), buffer)
    }}
}}  // cfg_if

optimized_itoa_base10! { usize, optimized_usize }

// OPTIMIZED

// Optimized implementation for radix-N numbers.
//
// Use a macro to allow for u32 or u64 to be used (u32 is generally faster).
//
// `value` must be non-negative and mutable.
perftools_inline!{
#[cfg(all(feature = "radix", feature = "table"))]
fn optimized_generic<T>(mut value: T, radix: T, table: &[u8], buffer: &mut [u8])
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
}}

// NAIVE

// Naive implementation for radix-N numbers.
//
// Use a macro to allow for u32 or u64 to be used (u32 is generally faster).
//
// `value` must be non-negative and mutable.
perftools_inline!{
#[cfg(not(feature = "table"))]
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
}}

// Forward the correct arguments to the implementation.
//
// Use a macro to allow for u32 or u64 to be used (u32 is generally faster).
//
// `value` must be non-negative and mutable.
perftools_inline!{
pub(crate) fn forward<T>(value: T, radix: u32, bytes: &mut [u8])
    -> usize
    where T: UnsignedInteger + OptimizedItoaBase10
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
    // Need to ensure the buffer size is adequate for basen, but small
    // for the optimized base10 formatters.
    debug_assert_radix!(radix);
    let mut buffer: [u8; BUFFER_SIZE] = [b'\0'; BUFFER_SIZE];
    let digits;
    if cfg!(not(feature = "radix")) || radix == 10 {
        digits = &mut buffer[..T::MAX_SIZE_BASE10];
    } else {
        digits = &mut buffer[..T::MAX_SIZE];
    }

    let offset = {
        #[cfg(not(feature = "table"))] {
            naive(value, as_cast(radix), digits)
        }

        #[cfg(all(not(feature = "radix"), feature = "table"))] {
            value.optimized_itoa(digits)
        }

        #[cfg(all(feature = "radix", feature = "table"))] {
            if radix == 10 {
                value.optimized_itoa(digits)
            } else {
                let table: &[u8] = match radix {
                    2   => &DIGIT_TO_BASE2_SQUARED,
                    3   => &DIGIT_TO_BASE3_SQUARED,
                    4   => &DIGIT_TO_BASE4_SQUARED,
                    5   => &DIGIT_TO_BASE5_SQUARED,
                    6   => &DIGIT_TO_BASE6_SQUARED,
                    7   => &DIGIT_TO_BASE7_SQUARED,
                    8   => &DIGIT_TO_BASE8_SQUARED,
                    9   => &DIGIT_TO_BASE9_SQUARED,
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
                optimized_generic(value, as_cast(radix), table, digits)
            }
        }
    };

    debug_assert!(offset <= bytes.len());
    copy_to_dst(bytes, &index!(digits[offset..]))
}}

// Sanitizer for an unsigned number-to-string implementation.
perftools_inline!{
pub(crate) fn unsigned<Value, UWide>(value: Value, radix: u32, bytes: &mut [u8])
    -> usize
    where Value: UnsignedInteger,
          UWide: UnsignedInteger + OptimizedItoaBase10
{
    // Invoke forwarder
    let v: UWide = as_cast(value);
    forward(v, radix, bytes)
}}

// Sanitizer for an signed number-to-string implementation.
perftools_inline!{
pub(crate) fn signed<Value, UWide, IWide>(value: Value, radix: u32, bytes: &mut [u8])
    -> usize
    where Value: SignedInteger,
          UWide: UnsignedInteger + OptimizedItoaBase10,
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
}}

// UNSAFE API

/// Expand the generic unsigned itoa function for specified types.
macro_rules! wrap_unsigned {
    ($name:ident, $t:ty, $uwide:ty) => (
        /// Serialize unsigned integer and return bytes written to.
        perftools_inline!{
        fn $name<'a>(value: $t, radix: u8, bytes: &'a mut [u8])
            -> usize
        {
            unsigned::<$t, $uwide>(value, radix.into(), bytes)
        }}
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
        perftools_inline!{
        fn $name<'a>(value: $t, radix: u8, bytes: &'a mut [u8])
            -> usize
        {
            signed::<$t, $uwide, $iwide>(value, radix.into(), bytes)
        }}
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

    // OPTIMIZED

    #[cfg(feature = "table")]
    macro_rules! check_internal {
        ($value:expr, $expected:expr, $cb:ident, $t:tt) => {
            let mut buffer = new_buffer();
            let digits = &mut buffer[..$t::MAX_SIZE_BASE10];
            let offset = $cb($value, digits);
            let slc = &digits[offset..];
            assert_eq!($expected.len(), slc.len());
            assert_eq!(&$expected[..], slc);
        };
    }

    #[cfg(feature = "table")]
    #[test]
    fn optimized_u8_test() {
        check_internal!(0, b"0", optimized_u8, u8);
        check_internal!(1, b"1", optimized_u8, u8);
        check_internal!(127, b"127", optimized_u8, u8);
        check_internal!(128, b"128", optimized_u8, u8);
        check_internal!(255, b"255", optimized_u8, u8);
    }

    #[cfg(feature = "table")]
    #[test]
    fn optimized_u16_test() {
        check_internal!(0, b"0", optimized_u16, u16);
        check_internal!(1, b"1", optimized_u16, u16);
        check_internal!(32767, b"32767", optimized_u16, u16);
        check_internal!(32768, b"32768", optimized_u16, u16);
        check_internal!(65535, b"65535", optimized_u16, u16);
    }

    #[cfg(feature = "table")]
    #[test]
    fn optimized_u32_test() {
        check_internal!(0, b"0", optimized_u32, u32);
        check_internal!(1, b"1", optimized_u32, u32);
        check_internal!(2147483647, b"2147483647", optimized_u32, u32);
        check_internal!(2147483648, b"2147483648", optimized_u32, u32);
        check_internal!(4294967295, b"4294967295", optimized_u32, u32);
    }

    #[cfg(feature = "table")]
    #[test]
    fn optimized_u64_test() {
        check_internal!(0, b"0", optimized_u64, u64);
        check_internal!(1, b"1", optimized_u64, u64);
        check_internal!(9223372036854775807, b"9223372036854775807", optimized_u64, u64);
        check_internal!(9223372036854775808, b"9223372036854775808", optimized_u64, u64);
        check_internal!(18446744073709551615, b"18446744073709551615", optimized_u64, u64);
    }

    #[cfg(all(has_i128, feature = "table"))]
    #[test]
    fn optimized_u128_test() {
        check_internal!(0, b"0", optimized_u128, u128);
        check_internal!(1, b"1", optimized_u128, u128);
        check_internal!(170141183460469231731687303715884105727, b"170141183460469231731687303715884105727", optimized_u128, u128);
        check_internal!(170141183460469231731687303715884105728, b"170141183460469231731687303715884105728", optimized_u128, u128);
        check_internal!(340282366920938463463374607431768211455, b"340282366920938463463374607431768211455", optimized_u128, u128);
    }

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
