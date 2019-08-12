//! Fast lexical integer-to-string conversion routines for base10.

//  The following algorithms aim to minimize the number of conditional
//  jumps required, by requiring at most 5 linear conditions before
//  jumping to a condition-less set of instructions. This allows high
//  performance formatting for integer sizes, and scales well for
//  both sequential values (primarily low number of digits) and uniform
//  values (primarily high numbers of digits), however, it also works
//  well even with branch misprediction (tested using a linear congruent
//  generator to choose between a sequential or uniform integer).
//
//  The performance is ~2-3x the performance of traditional integer
//  formatters (see, dtolnay/itoa, or the generic algorithm) for 32-bits
//  or less, highlighting the advantage of removing for loops with
//  minimal branches. It also scales well for 64 or more bit integers.

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
//  | u8    | 40,792             | 381,044               | 9.34x             |
//  | u16   | 38,231             | 391,166               | 10.2x             |
//  | u32   | 79,071             | 413,889               | 5.23x             |
//  | u64   | 180,584            | 485,047               | 2.69x             |
//  | i8    | 94,888             | 420,375               | 4.43x             |
//  | i16   | 97,082             | 448,786               | 4.63x             |
//  | i32   | 121,595            | 471,806               | 3.88x             |
//  | i64   | 189,703            | 524,520               | 2.76x             |
//
//  # Raw Benchmarks
//
//  ```text
//  test itoa_i8_itoa                   ... bench:     128,843 ns/iter (+/- 2,855)
//  test itoa_i8_lexical                ... bench:      94,888 ns/iter (+/- 4,079)
//  test itoa_i8_std                    ... bench:     420,375 ns/iter (+/- 20,328)
//  test itoa_i16_itoa                  ... bench:     146,139 ns/iter (+/- 25,783)
//  test itoa_i16_lexical               ... bench:      97,082 ns/iter (+/- 11,157)
//  test itoa_i16_std                   ... bench:     448,786 ns/iter (+/- 15,801)
//  test itoa_i32_itoa                  ... bench:     176,001 ns/iter (+/- 7,240)
//  test itoa_i32_lexical               ... bench:     121,595 ns/iter (+/- 7,035)
//  test itoa_i32_std                   ... bench:     471,806 ns/iter (+/- 23,265)
//  test itoa_i64_itoa                  ... bench:     198,261 ns/iter (+/- 9,307)
//  test itoa_i64_lexical               ... bench:     189,703 ns/iter (+/- 4,475)
//  test itoa_i64_std                   ... bench:     524,520 ns/iter (+/- 21,795)
//  test itoa_u8_heterogeneous_itoa     ... bench:     183,047 ns/iter (+/- 11,117)
//  test itoa_u8_heterogeneous_lexical  ... bench:      83,151 ns/iter (+/- 2,452)
//  test itoa_u8_heterogeneous_std      ... bench:     709,546 ns/iter (+/- 52,002)
//  test itoa_u8_itoa                   ... bench:     105,462 ns/iter (+/- 4,255)
//  test itoa_u8_lexical                ... bench:      40,792 ns/iter (+/- 1,963)
//  test itoa_u8_simple_itoa            ... bench:      69,500 ns/iter (+/- 3,398)
//  test itoa_u8_simple_lexical         ... bench:      23,788 ns/iter (+/- 1,746)
//  test itoa_u8_simple_std             ... bench:     320,189 ns/iter (+/- 9,711)
//  test itoa_u8_std                    ... bench:     381,044 ns/iter (+/- 16,530)
//  test itoa_u16_heterogeneous_itoa    ... bench:     196,402 ns/iter (+/- 6,965)
//  test itoa_u16_heterogeneous_lexical ... bench:      98,869 ns/iter (+/- 5,440)
//  test itoa_u16_heterogeneous_std     ... bench:     773,184 ns/iter (+/- 44,113)
//  test itoa_u16_itoa                  ... bench:      90,331 ns/iter (+/- 5,159)
//  test itoa_u16_lexical               ... bench:      38,231 ns/iter (+/- 1,534)
//  test itoa_u16_simple_itoa           ... bench:      94,923 ns/iter (+/- 6,281)
//  test itoa_u16_simple_lexical        ... bench:      40,726 ns/iter (+/- 1,784)
//  test itoa_u16_simple_std            ... bench:     367,671 ns/iter (+/- 16,110)
//  test itoa_u16_std                   ... bench:     391,166 ns/iter (+/- 16,716)
//  test itoa_u32_heterogeneous_itoa    ... bench:     223,931 ns/iter (+/- 11,154)
//  test itoa_u32_heterogeneous_lexical ... bench:     143,864 ns/iter (+/- 7,003)
//  test itoa_u32_heterogeneous_std     ... bench:     805,283 ns/iter (+/- 43,169)
//  test itoa_u32_itoa                  ... bench:     114,658 ns/iter (+/- 6,751)
//  test itoa_u32_lexical               ... bench:      79,071 ns/iter (+/- 4,162)
//  test itoa_u32_simple_itoa           ... bench:      91,412 ns/iter (+/- 3,034)
//  test itoa_u32_simple_lexical        ... bench:      51,671 ns/iter (+/- 1,778)
//  test itoa_u32_simple_std            ... bench:     370,000 ns/iter (+/- 26,189)
//  test itoa_u32_std                   ... bench:     413,889 ns/iter (+/- 15,907)
//  test itoa_u64_heterogeneous_itoa    ... bench:     311,775 ns/iter (+/- 24,807)
//  test itoa_u64_heterogeneous_lexical ... bench:     258,940 ns/iter (+/- 11,927)
//  test itoa_u64_heterogeneous_std     ... bench:     877,179 ns/iter (+/- 32,833)
//  test itoa_u64_itoa                  ... bench:     206,181 ns/iter (+/- 10,779)
//  test itoa_u64_lexical               ... bench:     180,584 ns/iter (+/- 5,921)
//  test itoa_u64_simple_itoa           ... bench:      91,645 ns/iter (+/- 1,701)
//  test itoa_u64_simple_lexical        ... bench:      72,102 ns/iter (+/- 4,453)
//  test itoa_u64_simple_std            ... bench:     373,958 ns/iter (+/- 22,162)
//  test itoa_u64_std                   ... bench:     485,047 ns/iter (+/- 14,591)
//  ```

// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([40792, 38231, 79071, 180584, 94888, 97082, 121595, 189703]) / 1e3
//  itoa = np.array([105462, 90331, 114658, 206181, 128843, 146139, 176001, 198261]) / 1e3
//  rustcore = np.array([381044, 391166, 413889, 485047, 420375, 448786, 471806, 524520]) / 1e3
//  index = ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
//  df = pd.DataFrame({'lexical': lexical, 'itoa': itoa, 'rustcore': rustcore}, index = index, columns=['lexical', 'itoa', 'rustcore'])
//  ax = df.plot.bar(rot=0, figsize=(16, 8), fontsize=14, color=['#E24A33', '#988ED5', '#348ABD'])
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  ax.legend(loc=2, prop={'size': 14})
//  plt.show()

use util::*;

// Lookup table for optimized base10 itoa.
const TABLE: &[u8] = &DIGIT_TO_BASE10_SQUARED;

// BASE10
// ------

// These calculate N table indexes in series of 4, which produces
// 8 digits each. To decompose larger values into values of less
// than 1e8, we use fast division based off Terje Mathiesen's approach,
// where we use a subtraction and multiply to scale the result.

// INDEX

// Calculate 1 table-index from value.
macro_rules! table_index_1 {
    ($value:ident) => (2 * $value.as_usize());
}

// Calculate 2 table-indexes from value.
macro_rules! table_index_2 {
    ($value:ident) => ({
        let rv0 = $value.as_u32();
        let rv1 = rv0 / 100;
        let ri0 = (rv0 * 2 - rv1 * 200).as_usize();
        let ri1 = table_index_1!(rv1);
        (ri0, ri1)
    });
}

// Calculate 3 table-indexes from value.
macro_rules! table_index_3 {
    ($value:ident) => ({
        let rv0 = $value.as_u32();
        let rv1 = rv0 / 100;
        let ri0 = (rv0 * 2 - rv1 * 200).as_usize();
        let (ri1, ri2) = table_index_2!(rv1);
        (ri0, ri1, ri2)
    });
}

// Calculate 4 table-indexes from value.
macro_rules! table_index_4 {
    ($value:ident) => ({
        let rv0 = $value.as_u32();
        let rv1 = rv0 / 100;
        let ri0 = (rv0 * 2 - rv1 * 200).as_usize();
        let (ri1, ri2, ri3) = table_index_3!(rv1);
        (ri0, ri1, ri2, ri3)
    });
}

// Calculate 5 table-indexes from value.
macro_rules! table_index_5 {
    ($value:ident) => ({
        let t0 = ($value / 100000000).as_u32();
        let v1 = t0;
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let ri4 = table_index_1!(v1);
        (ri0, ri1, ri2, ri3, ri4)
    });
}

// Calculate 6 table-indexes from value.
macro_rules! table_index_6 {
    ($value:ident) => ({
        let t0 = ($value / 100000000).as_u32();
        let v1 = t0;
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5) = table_index_2!(v1);
        (ri0, ri1, ri2, ri3, ri4, ri5)
    });
}

// Calculate 7 table-indexes from value.
macro_rules! table_index_7 {
    ($value:ident) => ({
        let t0 = ($value / 100000000).as_u32();
        let v1 = t0;
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6) = table_index_3!(v1);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6)
    });
}

// Calculate 8 table-indexes from value.
macro_rules! table_index_8 {
    ($value:ident) => ({
        let t0 = ($value / 100000000).as_u32();
        let v1 = t0;
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7)
    });
}

// Calculate 9 table-indexes from value.
macro_rules! table_index_9 {
    ($value:ident) => ({
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v2 = t1;
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let ri8 = table_index_1!(v2);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8)
    });
}

// Calculate 10 table-indexes from value.
macro_rules! table_index_10 {
    ($value:ident) => ({
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v2 = t1;
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let (ri8, ri9) = table_index_2!(v2);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8, ri9)
    });
}

// Calculate 11 table-indexes from value.
#[cfg(has_i128)]
macro_rules! table_index_11 {
    ($value:ident) => ({
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v2 = t1;
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let (ri8, ri9, ri10) = table_index_3!(v2);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8, ri9, ri10)
    });
}

// Calculate 12 table-indexes from value.
#[cfg(has_i128)]
macro_rules! table_index_12 {
    ($value:ident) => ({
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v2 = t1;
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let (ri8, ri9, ri10, ri11) = table_index_4!(v2);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8, ri9, ri10, ri11)
    });
}

// Calculate 13 table-indexes from value.
#[cfg(has_i128)]
macro_rules! table_index_13 {
    ($value:ident) => ({
        let t2 = ($value / 1000000000000000000000000).as_u32();
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v3 = t2;
        let v2 = t1.wrapping_sub(t2.wrapping_mul(100000000));
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let (ri8, ri9, ri10, ri11) = table_index_4!(v2);
        let ri12 = table_index_1!(v3);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8, ri9, ri10, ri11, ri12)
    });
}

// Calculate 14 table-indexes from value.
#[cfg(has_i128)]
macro_rules! table_index_14 {
    ($value:ident) => ({
        let t2 = ($value / 1000000000000000000000000).as_u32();
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v3 = t2;
        let v2 = t1.wrapping_sub(t2.wrapping_mul(100000000));
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let (ri8, ri9, ri10, ri11) = table_index_4!(v2);
        let (ri12, ri13) = table_index_2!(v3);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8, ri9, ri10, ri11, ri12, ri13)
    });
}

// Calculate 15 table-indexes from value.
#[cfg(has_i128)]
macro_rules! table_index_15 {
    ($value:ident) => ({
        let t2 = ($value / 1000000000000000000000000).as_u32();
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v3 = t2;
        let v2 = t1.wrapping_sub(t2.wrapping_mul(100000000));
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let (ri8, ri9, ri10, ri11) = table_index_4!(v2);
        let (ri12, ri13, ri14) = table_index_3!(v3);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8, ri9, ri10, ri11, ri12, ri13, ri14)
    });
}

// Calculate 16 table-indexes from value.
#[cfg(has_i128)]
macro_rules! table_index_16 {
    ($value:ident) => ({
        let t2 = ($value / 1000000000000000000000000).as_u32();
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v3 = t2;
        let v2 = t1.wrapping_sub(t2.wrapping_mul(100000000));
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let (ri8, ri9, ri10, ri11) = table_index_4!(v2);
        let (ri12, ri13, ri14, ri15) = table_index_4!(v3);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8, ri9, ri10, ri11, ri12, ri13, ri14, ri15)
    });
}

// Calculate 17 table-indexes from value.
#[cfg(has_i128)]
macro_rules! table_index_17 {
    ($value:ident) => ({
        let t3 = ($value / 100000000000000000000000000000000).as_u32();
        let t2 = ($value / 1000000000000000000000000).as_u32();
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v4 = t3;
        let v3 = t2.wrapping_sub(t3.wrapping_mul(100000000));
        let v2 = t1.wrapping_sub(t2.wrapping_mul(100000000));
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let (ri8, ri9, ri10, ri11) = table_index_4!(v2);
        let (ri12, ri13, ri14, ri15) = table_index_4!(v3);
        let ri16 = table_index_1!(v4);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8, ri9, ri10, ri11, ri12, ri13, ri14, ri15, ri16)
    });
}

// Calculate 18 table-indexes from value.
#[cfg(has_i128)]
macro_rules! table_index_18 {
    ($value:ident) => ({
        let t3 = ($value / 100000000000000000000000000000000).as_u32();
        let t2 = ($value / 1000000000000000000000000).as_u32();
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v4 = t3;
        let v3 = t2.wrapping_sub(t3.wrapping_mul(100000000));
        let v2 = t1.wrapping_sub(t2.wrapping_mul(100000000));
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let (ri8, ri9, ri10, ri11) = table_index_4!(v2);
        let (ri12, ri13, ri14, ri15) = table_index_4!(v3);
        let (ri16, ri17) = table_index_2!(v4);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8, ri9, ri10, ri11, ri12, ri13, ri14, ri15, ri16, ri17)
    });
}

// Calculate 19 table-indexes from value.
#[cfg(has_i128)]
macro_rules! table_index_19 {
    ($value:ident) => ({
        let t3 = ($value / 100000000000000000000000000000000).as_u32();
        let t2 = ($value / 1000000000000000000000000).as_u32();
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v4 = t3;
        let v3 = t2.wrapping_sub(t3.wrapping_mul(100000000));
        let v2 = t1.wrapping_sub(t2.wrapping_mul(100000000));
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let (ri8, ri9, ri10, ri11) = table_index_4!(v2);
        let (ri12, ri13, ri14, ri15) = table_index_4!(v3);
        let (ri16, ri17, ri18) = table_index_3!(v4);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8, ri9, ri10, ri11, ri12, ri13, ri14, ri15, ri16, ri17, ri18)
    });
}

// Calculate 20 table-indexes from value.
#[cfg(has_i128)]
macro_rules! table_index_20 {
    ($value:ident) => ({
        let t3 = ($value / 100000000000000000000000000000000).as_u32();
        let t2 = ($value / 1000000000000000000000000).as_u32();
        let t1 = ($value / 10000000000000000).as_u32();
        let t0 = ($value / 100000000).as_u32();
        let v4 = t3;
        let v3 = t2.wrapping_sub(t3.wrapping_mul(100000000));
        let v2 = t1.wrapping_sub(t2.wrapping_mul(100000000));
        let v1 = t0.wrapping_sub(t1.wrapping_mul(100000000));
        let v0 = $value.as_u32().wrapping_sub(t0.wrapping_mul(100000000));
        let (ri0, ri1, ri2, ri3) = table_index_4!(v0);
        let (ri4, ri5, ri6, ri7) = table_index_4!(v1);
        let (ri8, ri9, ri10, ri11) = table_index_4!(v2);
        let (ri12, ri13, ri14, ri15) = table_index_4!(v3);
        let (ri16, ri17, ri18, ri19) = table_index_4!(v4);
        (ri0, ri1, ri2, ri3, ri4, ri5, ri6, ri7, ri8, ri9, ri10, ri11, ri12, ri13, ri14, ri15, ri16, ri17, ri18, ri19)
    });
}

// ASSIGN

// These are all implemented recursively in cases of 2 and 1,
// which expands to the efficient, required code.

// Assign 1 digit to the buffer.
macro_rules! assign_1 {
    ($buffer:ident, $r:ident, $l:expr) => (
        unchecked_index_mut!($buffer[$l] = unchecked_index!(TABLE[$r+1]));
    );
}

// Assign 2 digits to the buffer.
macro_rules! assign_2 {
    ($buffer:ident, $r:ident, $l:expr) => (
        unchecked_index_mut!($buffer[$l+1] = unchecked_index!(TABLE[$r+1]));
        unchecked_index_mut!($buffer[$l+0] = unchecked_index!(TABLE[$r+0]));
    );
}

// Assign 3 digits to the buffer.
macro_rules! assign_3 {
    ($buffer:ident, $r0:ident, $r1:ident, $l:expr) => (
        assign_2!($buffer, $r0, $l+1);
        assign_1!($buffer, $r1, $l+0);
    );
}

// Assign 4 digits to the buffer.
macro_rules! assign_4 {
    ($buffer:ident, $r0:ident, $r1:ident, $l:expr) => (
        assign_2!($buffer, $r0, $l+2);
        assign_2!($buffer, $r1, $l+0);
    );
}

// Assign 5 digits to the buffer.
macro_rules! assign_5 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $l:expr) => (
        assign_4!($buffer, $r0, $r1, $l+1);
        assign_1!($buffer, $r2, $l+0);
    );
}

// Assign 6 digits to the buffer.
macro_rules! assign_6 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $l:expr) => (
        assign_4!($buffer, $r0, $r1, $l+2);
        assign_2!($buffer, $r2, $l+0);
    );
}

// Assign 7 digits to the buffer.
macro_rules! assign_7 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $l:expr) => (
        assign_6!($buffer, $r0, $r1, $r2, $l+1);
        assign_1!($buffer, $r3, $l+0);
    );
}

// Assign 8 digits to the buffer.
macro_rules! assign_8 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $l:expr) => (
        assign_6!($buffer, $r0, $r1, $r2, $l+2);
        assign_2!($buffer, $r3, $l+0);
    );
}

// Assign 9 digits to the buffer.
macro_rules! assign_9 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $l:expr) => (
        assign_8!($buffer, $r0, $r1, $r2, $r3, $l+1);
        assign_1!($buffer, $r4, $l+0);
    );
}

// Assign 10 digits to the buffer.
macro_rules! assign_10 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $l:expr) => (
        assign_8!($buffer, $r0, $r1, $r2, $r3, $l+2);
        assign_2!($buffer, $r4, $l+0);
    );
}

// Assign 11 digits to the buffer.
macro_rules! assign_11 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $l:expr) => (
        assign_10!($buffer, $r0, $r1, $r2, $r3, $r4, $l+1);
        assign_1!($buffer, $r5, $l+0);
    );
}

// Assign 12 digits to the buffer.
macro_rules! assign_12 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $l:expr) => (
        assign_10!($buffer, $r0, $r1, $r2, $r3, $r4, $l+2);
        assign_2!($buffer, $r5, $l+0);
    );
}

// Assign 13 digits to the buffer.
macro_rules! assign_13 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $l:expr) => (
        assign_12!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $l+1);
        assign_1!($buffer, $r6, $l+0);
    );
}

// Assign 14 digits to the buffer.
macro_rules! assign_14 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $l:expr) => (
        assign_12!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $l+2);
        assign_2!($buffer, $r6, $l+0);
    );
}

// Assign 15 digits to the buffer.
macro_rules! assign_15 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $l:expr) => (
        assign_14!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $l+1);
        assign_1!($buffer, $r7, $l+0);
    );
}

// Assign 16 digits to the buffer.
macro_rules! assign_16 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $l:expr) => (
        assign_14!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $l+2);
        assign_2!($buffer, $r7, $l+0);
    );
}

// Assign 17 digits to the buffer.
macro_rules! assign_17 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $l:expr) => (
        assign_16!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $l+1);
        assign_1!($buffer, $r8, $l+0);
    );
}

// Assign 18 digits to the buffer.
macro_rules! assign_18 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $l:expr) => (
        assign_16!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $l+2);
        assign_2!($buffer, $r8, $l+0);
    );
}

// Assign 19 digits to the buffer.
macro_rules! assign_19 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $l:expr) => (
        assign_18!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $l+1);
        assign_1!($buffer, $r9, $l+0);
    );
}

// Assign 20 digits to the buffer.
macro_rules! assign_20 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $l:expr) => (
        assign_18!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $l+2);
        assign_2!($buffer, $r9, $l+0);
    );
}

// Assign 21 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_21 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $l:expr) => (
        assign_20!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $l+1);
        assign_1!($buffer, $r10, $l+0);
    );
}

// Assign 22 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_22 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $l:expr) => (
        assign_20!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $l+2);
        assign_2!($buffer, $r10, $l+0);
    );
}

// Assign 23 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_23 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $l:expr) => (
        assign_22!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $l+1);
        assign_1!($buffer, $r11, $l+0);
    );
}

// Assign 24 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_24 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $l:expr) => (
        assign_22!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $l+2);
        assign_2!($buffer, $r11, $l+0);
    );
}

// Assign 25 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_25 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $l:expr) => (
        assign_24!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $l+1);
        assign_1!($buffer, $r12, $l+0);
    );
}

// Assign 26 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_26 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $l:expr) => (
        assign_24!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $l+2);
        assign_2!($buffer, $r12, $l+0);
    );
}

// Assign 27 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_27 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $l:expr) => (
        assign_26!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $l+1);
        assign_1!($buffer, $r13, $l+0);
    );
}

// Assign 28 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_28 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $l:expr) => (
        assign_26!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $l+2);
        assign_2!($buffer, $r13, $l+0);
    );
}

// Assign 29 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_29 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $r14:ident, $l:expr) => (
        assign_28!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $r13, $l+1);
        assign_1!($buffer, $r14, $l+0);
    );
}

// Assign 30 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_30 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $r14:ident, $l:expr) => (
        assign_28!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $r13, $l+2);
        assign_2!($buffer, $r14, $l+0);
    );
}

// Assign 31 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_31 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $r14:ident, $r15:ident, $l:expr) => (
        assign_30!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $r13, $r14, $l+1);
        assign_1!($buffer, $r15, $l+0);
    );
}

// Assign 32 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_32 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $r14:ident, $r15:ident, $l:expr) => (
        assign_30!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $r13, $r14, $l+2);
        assign_2!($buffer, $r15, $l+0);
    );
}

// Assign 33 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_33 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $r14:ident, $r15:ident, $r16:ident, $l:expr) => (
        assign_32!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $r13, $r14, $r15, $l+1);
        assign_1!($buffer, $r16, $l+0);
    );
}

// Assign 34 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_34 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $r14:ident, $r15:ident, $r16:ident, $l:expr) => (
        assign_32!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $r13, $r14, $r15, $l+2);
        assign_2!($buffer, $r16, $l+0);
    );
}

// Assign 35 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_35 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $r14:ident, $r15:ident, $r16:ident, $r17:ident, $l:expr) => (
        assign_34!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $r13, $r14, $r15, $r16, $l+1);
        assign_1!($buffer, $r17, $l+0);
    );
}

// Assign 36 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_36 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $r14:ident, $r15:ident, $r16:ident, $r17:ident, $l:expr) => (
        assign_34!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $r13, $r14, $r15, $r16, $l+2);
        assign_2!($buffer, $r17, $l+0);
    );
}

// Assign 37 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_37 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $r14:ident, $r15:ident, $r16:ident, $r17:ident, $r18:ident, $l:expr) => (
        assign_36!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $r13, $r14, $r15, $r16, $r17, $l+1);
        assign_1!($buffer, $r18, $l+0);
    );
}

// Assign 38 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_38 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $r14:ident, $r15:ident, $r16:ident, $r17:ident, $r18:ident, $l:expr) => (
        assign_36!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $r13, $r14, $r15, $r16, $r17, $l+2);
        assign_2!($buffer, $r18, $l+0);
    );
}

// Assign 39 digits to the buffer.
#[cfg(has_i128)]
macro_rules! assign_39 {
    ($buffer:ident, $r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6:ident, $r7:ident, $r8:ident, $r9:ident, $r10:ident, $r11:ident, $r12:ident, $r13:ident, $r14:ident, $r15:ident, $r16:ident, $r17:ident, $r18:ident, $r19:ident, $l:expr) => (
        assign_38!($buffer, $r0, $r1, $r2, $r3, $r4, $r5, $r6, $r7, $r8, $r9, $r10, $r11, $r12, $r13, $r14, $r15, $r16, $r17, $r18, $l+1);
        assign_1!($buffer, $r19, $l+0);
    );
}

// WRITE

// For the writers, the value **must** have exactly as many base10 digits
// as the function writes, or else there may be undefined behavior.
// This is controlled via the dispatcher, which matches based on a range
// of values.

// Write 1 digit from value.
macro_rules! write_1 {
    ($value:ident, $buffer:ident) => ({
        unchecked_index_mut!($buffer[0] = digit_to_char($value));
        1
    });
}

// Write 2 digits from value.
macro_rules! write_2 {
    ($value:ident, $buffer:ident) => ({
        let r = table_index_1!($value);
        assign_2!($buffer, r, 0);
        2
    });
}

// Write 3 digits from value.
macro_rules! write_3 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1) = table_index_2!($value);
        assign_3!($buffer, r0, r1, 0);
        3
    });
}

// Write 4 digits from value.
macro_rules! write_4 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1) = table_index_2!($value);
        assign_4!($buffer, r0, r1, 0);
        4
    });
}

// Write 5 digits from value.
macro_rules! write_5 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2) = table_index_3!($value);
        assign_5!($buffer, r0, r1, r2, 0);
        5
    });
}

// Write 6 digits from value.
macro_rules! write_6 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2) = table_index_3!($value);
        assign_6!($buffer, r0, r1, r2, 0);
        6
    });
}

// Write 7 digits from value.
macro_rules! write_7 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3) = table_index_4!($value);
        assign_7!($buffer, r0, r1, r2, r3, 0);
        7
    });
}

// Write 8 digits from value.
macro_rules! write_8 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3) = table_index_4!($value);
        assign_8!($buffer, r0, r1, r2, r3, 0);
        8
    });
}

// Write 9 digits from value.
macro_rules! write_9 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4) = table_index_5!($value);
        assign_9!($buffer, r0, r1, r2, r3, r4, 0);
        9
    });
}

// Write 10 digits from value.
macro_rules! write_10 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4) = table_index_5!($value);
        assign_10!($buffer, r0, r1, r2, r3, r4, 0);
        10
    });
}

// Write 11 digits from value.
macro_rules! write_11 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5) = table_index_6!($value);
        assign_11!($buffer, r0, r1, r2, r3, r4, r5, 0);
        11
    });
}

// Write 12 digits from value.
macro_rules! write_12 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5) = table_index_6!($value);
        assign_12!($buffer, r0, r1, r2, r3, r4, r5, 0);
        12
    });
}

// Write 13 digits from value.
macro_rules! write_13 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6) = table_index_7!($value);
        assign_13!($buffer, r0, r1, r2, r3, r4, r5, r6, 0);
        13
    });
}

// Write 14 digits from value.
macro_rules! write_14 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6) = table_index_7!($value);
        assign_14!($buffer, r0, r1, r2, r3, r4, r5, r6, 0);
        14
    });
}

// Write 15 digits from value.
macro_rules! write_15 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7) = table_index_8!($value);
        assign_15!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, 0);
        15
    });
}

// Write 16 digits from value.
macro_rules! write_16 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7) = table_index_8!($value);
        assign_16!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, 0);
        16
    });
}

// Write 17 digits from value.
macro_rules! write_17 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8) = table_index_9!($value);
        assign_17!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, 0);
        17
    });
}

// Write 18 digits from value.
macro_rules! write_18 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8) = table_index_9!($value);
        assign_18!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, 0);
        18
    });
}

// Write 19 digits from value.
macro_rules! write_19 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9) = table_index_10!($value);
        assign_19!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, 0);
        19
    });
}

// Write 20 digits from value.
macro_rules! write_20 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9) = table_index_10!($value);
        assign_20!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, 0);
        20
    });
}

// Write 21 digits from value.
#[cfg(has_i128)]
macro_rules! write_21 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10) = table_index_11!($value);
        assign_21!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, 0);
        21
    });
}

// Write 22 digits from value.
#[cfg(has_i128)]
macro_rules! write_22 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10) = table_index_11!($value);
        assign_22!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, 0);
        22
    });
}

// Write 23 digits from value.
#[cfg(has_i128)]
macro_rules! write_23 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11) = table_index_12!($value);
        assign_23!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, 0);
        23
    });
}

// Write 24 digits from value.
#[cfg(has_i128)]
macro_rules! write_24 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11) = table_index_12!($value);
        assign_24!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, 0);
        24
    });
}

// Write 25 digits from value.
#[cfg(has_i128)]
macro_rules! write_25 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12) = table_index_13!($value);
        assign_25!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, 0);
        25
    });
}

// Write 26 digits from value.
#[cfg(has_i128)]
macro_rules! write_26 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12) = table_index_13!($value);
        assign_26!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, 0);
        26
    });
}

// Write 27 digits from value.
#[cfg(has_i128)]
macro_rules! write_27 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13) = table_index_14!($value);
        assign_27!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, 0);
        27
    });
}

// Write 28 digits from value.
#[cfg(has_i128)]
macro_rules! write_28 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13) = table_index_14!($value);
        assign_28!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, 0);
        28
    });
}

// Write 29 digits from value.
#[cfg(has_i128)]
macro_rules! write_29 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14) = table_index_15!($value);
        assign_29!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, 0);
        29
    });
}

// Write 30 digits from value.
#[cfg(has_i128)]
macro_rules! write_30 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14) = table_index_15!($value);
        assign_30!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, 0);
        30
    });
}

// Write 31 digits from value.
#[cfg(has_i128)]
macro_rules! write_31 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15) = table_index_16!($value);
        assign_31!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, 0);
        31
    });
}

// Write 32 digits from value.
#[cfg(has_i128)]
macro_rules! write_32 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15) = table_index_16!($value);
        assign_32!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, 0);
        32
    });
}

// Write 33 digits from value.
#[cfg(has_i128)]
macro_rules! write_33 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16) = table_index_17!($value);
        assign_33!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, 0);
        33
    });
}

// Write 34 digits from value.
#[cfg(has_i128)]
macro_rules! write_34 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16) = table_index_17!($value);
        assign_34!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, 0);
        34
    });
}

// Write 35 digits from value.
#[cfg(has_i128)]
macro_rules! write_35 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17) = table_index_18!($value);
        assign_35!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, 0);
        35
    });
}

// Write 36 digits from value.
#[cfg(has_i128)]
macro_rules! write_36 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17) = table_index_18!($value);
        assign_36!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, 0);
        36
    });
}

// Write 37 digits from value.
#[cfg(has_i128)]
macro_rules! write_37 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, r18) = table_index_19!($value);
        assign_37!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, r18, 0);
        37
    });
}

// Write 38 digits from value.
#[cfg(has_i128)]
macro_rules! write_38 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, r18) = table_index_19!($value);
        assign_38!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, r18, 0);
        38
    });
}

// Write 39 digits from value.
#[cfg(has_i128)]
macro_rules! write_39 {
    ($value:ident, $buffer:ident) => ({
        let (r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, r18, r19) = table_index_20!($value);
        assign_39!($buffer, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15, r16, r17, r18, r19, 0);
        39
    });
}

// BLOCKS

// Detect digit count from 0-5 and call writer.
macro_rules! write_0_5 {
    ($value:ident, $buffer:ident) => {
        if $value >= 10000 {
            write_5!($value, $buffer)
        } else if $value >= 1000 {
            write_4!($value, $buffer)
        } else if $value >= 100 {
            write_3!($value, $buffer)
        } else if $value >= 10 {
            write_2!($value, $buffer)
        } else {
            write_1!($value, $buffer)
        }
    };
}

// Detect digit count from 5-10 and call writer.
macro_rules! write_5_10 {
    ($value:ident, $buffer:ident) => {
        if $value >= 1000000000 {
            write_10!($value, $buffer)
        } else if $value >= 100000000 {
            write_9!($value, $buffer)
        } else if $value >= 10000000 {
            write_8!($value, $buffer)
        } else if $value >= 1000000 {
            write_7!($value, $buffer)
        } else if $value >= 100000 {
            write_6!($value, $buffer)
        } else {
            write_5!($value, $buffer)
        }
    };
}

// Detect digit count from 10-15 and call writer.
macro_rules! write_10_15 {
    ($value:ident, $buffer:ident) => {
        if $value >= 100000000000000 {
            write_15!($value, $buffer)
        } else if $value >= 10000000000000 {
            write_14!($value, $buffer)
        } else if $value >= 1000000000000 {
            write_13!($value, $buffer)
        } else if $value >= 100000000000 {
            write_12!($value, $buffer)
        } else if $value >= 10000000000 {
            write_11!($value, $buffer)
        } else {
            write_10!($value, $buffer)
        }
    };
}

// Detect digit count from 15-20 and call writer.
macro_rules! write_15_20 {
    ($value:ident, $buffer:ident) => {
        if $value >= 10000000000000000000 {
            write_20!($value, $buffer)
        } else if $value >= 1000000000000000000 {
            write_19!($value, $buffer)
        } else if $value >= 100000000000000000 {
            write_18!($value, $buffer)
        } else if $value >= 10000000000000000 {
            write_17!($value, $buffer)
        } else if $value >= 1000000000000000 {
            write_16!($value, $buffer)
        } else {
            write_15!($value, $buffer)
        }
    };
}

// Detect digit count from 20-25 and call writer.
#[cfg(has_i128)]
macro_rules! write_20_25 {
    ($value:ident, $buffer:ident) => {
        if $value >= 1000000000000000000000000 {
            write_25!($value, $buffer)
        } else if $value >= 100000000000000000000000 {
            write_24!($value, $buffer)
        } else if $value >= 10000000000000000000000 {
            write_23!($value, $buffer)
        } else if $value >= 1000000000000000000000 {
            write_22!($value, $buffer)
        } else if $value >= 100000000000000000000 {
            write_21!($value, $buffer)
        } else {
            write_20!($value, $buffer)
        }
    };
}

// Detect digit count from 25-29 and call writer.
#[cfg(has_i128)]
macro_rules! write_25_29 {
    ($value:ident, $buffer:ident) => {
        if $value >= 10000000000000000000000000000 {
            write_29!($value, $buffer)
        } else if $value >= 1000000000000000000000000000 {
            write_28!($value, $buffer)
        } else if $value >= 100000000000000000000000000 {
            write_27!($value, $buffer)
        } else if $value >= 10000000000000000000000000 {
            write_26!($value, $buffer)
        } else {
            write_25!($value, $buffer)
        }
    };
}

// Detect digit count from 29-34 and call writer.
#[cfg(has_i128)]
macro_rules! write_29_34 {
    ($value:ident, $buffer:ident) => {
        if $value >= 1000000000000000000000000000000000 {
            write_34!($value, $buffer)
        } else if $value >= 100000000000000000000000000000000 {
            write_33!($value, $buffer)
        } else if $value >= 10000000000000000000000000000000 {
            write_32!($value, $buffer)
        } else if $value >= 1000000000000000000000000000000 {
            write_31!($value, $buffer)
        } else if $value >= 100000000000000000000000000000 {
            write_30!($value, $buffer)
        } else {
            write_29!($value, $buffer)
        }
    };
}

// Detect digit count from 34-39 and call writer.
#[cfg(has_i128)]
macro_rules! write_34_39 {
    ($value:ident, $buffer:ident) => {
        if $value >= 100000000000000000000000000000000000000 {
            write_39!($value, $buffer)
        } else if $value >= 10000000000000000000000000000000000000 {
            write_38!($value, $buffer)
        } else if $value >= 1000000000000000000000000000000000000 {
            write_37!($value, $buffer)
        } else if $value >= 100000000000000000000000000000000000 {
            write_36!($value, $buffer)
        } else if $value >= 10000000000000000000000000000000000 {
            write_35!($value, $buffer)
        } else {
            write_34!($value, $buffer)
        }
    };
}

// INTERNAL

// Each flow-path should have no more than 5 comparisons, or
// else we're poorly optimizing our code.
// Use the number of leading zeros to minimize the number
// of jumps we have possible.

// We need to support the `...` syntax, since inclusive ranges
// were stabilized (with `..=` syntax) in 1.26.0.
// Remove `unknown_lints` and `ellipsis_inclusive_range_patterns`
// when this makes stable.

// Internal integer formatter for u8.
perftools_inline!{
#[allow(unused_unsafe)]
fn u8toa(value: u8, buffer: &mut [u8]) -> usize {
    if value >= 100 {
        write_3!(value, buffer)
    } else if value >= 10 {
        write_2!(value, buffer)
    } else {
        write_1!(value, buffer)
    }
}}

// Internal integer formatter for u16.
perftools_inline!{
#[allow(unused_unsafe)]
fn u16toa(value: u16, buffer: &mut [u8]) -> usize {
    write_0_5!(value, buffer)
}}

// Internal integer formatter for u32.
perftools_inline!{
#[allow(unknown_lints, unused_unsafe, ellipsis_inclusive_range_patterns)]
fn u32toa(value: u32, buffer: &mut [u8]) -> usize {
    match value.leading_zeros() {
        // [2^16, 2^32 - 1]
        0 ... 15 => write_5_10!(value, buffer),
        // [0, 2^16 - 1]
        _        => write_0_5! (value, buffer),
    }
}}

// Internal integer formatter for u64.
perftools_inline!{
#[allow(unknown_lints, unused_unsafe, ellipsis_inclusive_range_patterns)]
fn u64toa(value: u64, buffer: &mut [u8]) -> usize {
    match value.leading_zeros() {
        // [2^48, 2^64 - 1]
        0  ... 15 => write_15_20!(value, buffer),
        // [2^32, 2^48 - 1]
        16 ... 31 => write_10_15!(value, buffer),
        // [2^16, 2^32 - 1]
        32 ... 47 => write_5_10! (value, buffer),
        // [0, 2^16 - 1]
        _         => write_0_5!  (value, buffer),
    }
}}

// Internal integer formatter for u128.
perftools_inline!{
#[cfg(has_i128)]
#[allow(unknown_lints, unused_unsafe, ellipsis_inclusive_range_patterns)]
fn u128toa(value: u128, buffer: &mut [u8]) -> usize {
    match value.leading_zeros() {
        // [2^112, 2^128 - 1]
        0  ... 15  => write_34_39!(value, buffer),
        // [2^96, 2^112 - 1]
        16 ... 31  => write_29_34!(value, buffer),
        // [2^80, 2^96 - 1]
        32 ... 47  => write_25_29!(value, buffer),
        // [2^64, 2^80 - 1]
        48 ... 63  => write_20_25!(value, buffer),
        // [2^48, 2^64 - 1]
        64 ... 79  => write_15_20!(value, buffer),
        // [2^32, 2^48 - 1]
        80 ... 95  => write_10_15!(value, buffer),
        // [2^16, 2^32 - 1]
        96 ... 111 => write_5_10! (value, buffer),
        // [0, 2^16 - 1]
        _          => write_0_5!  (value, buffer),
    }
}}

cfg_if! {
if #[cfg(target_pointer_width = "16")] {
    perftools_inline!{
    #[allow(unused_unsafe)]
    fn usizetoa(value: usize, buffer: &mut [u8]) -> usize {
        u16toa(value.as_u16(), buffer)
    }}
} else if #[cfg(target_pointer_width = "32")] {
    perftools_inline!{
    #[allow(unused_unsafe)]
    fn usizetoa(value: usize, buffer: &mut [u8]) -> usize {
        u32toa(value.as_u32(), buffer)
    }}
} else if #[cfg(target_pointer_width = "64")] {
    perftools_inline!{
    #[allow(unused_unsafe)]
    fn usizetoa(value: usize, buffer: &mut [u8]) -> usize {
        u64toa(value.as_u64(), buffer)
    }}
}}  // cfg_if

// BASE10 TRAIT

pub(crate) trait Base10 {
    // Export integer to string.
    fn base10(self, buffer: &mut [u8]) -> usize;
}

// Implement base10 for type.
macro_rules! base10_impl {
    ($t:ty, $cb:ident) => (
        impl Base10 for $t {
            perftools_inline_always!{
            fn base10(self, buffer: &mut [u8]) -> usize {
                $cb(self, buffer)
            }}
        }
    );
}

base10_impl!(u8, u8toa);
base10_impl!(u16, u16toa);
base10_impl!(u32, u32toa);
base10_impl!(u64, u64toa);
base10_impl!(usize, usizetoa);
#[cfg(has_i128)]
base10_impl!(u128, u128toa);
