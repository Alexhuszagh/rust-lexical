//! Fast, lossy, basen lexical string-to-float conversion routines.
//!
//! The following benchmarks were run on an "Intel(R) Core(TM) i7-6560U
//! CPU @ 2.20GHz" CPU, on Fedora 28, Linux kernel version 4.18.16-200
//! (x86-64), using the lexical formatter or `x.parse()`,
//! avoiding any inefficiencies in Rust string parsing. The code was
//! compiled with LTO and at an optimization level of 3.
//!
//! The benchmarks with `std` were compiled using "rustc 1.29.2 (17a9dc751
//! 2018-10-05", and the `no_std` benchmarks were compiled using "rustc
//! 1.31.0-nightly (46880f41b 2018-10-15)".
//!
//! The benchmark code may be found `benches/atof.rs`.
//!
//! # Benchmarks
//!
//! | Type  |  lexical (ns/iter) | parse (ns/iter)       | Relative Increase |
//! |:-----:|:------------------:|:---------------------:|:-----------------:|
//! | f32   | 761,670            | 28,650,637            | 37.62x            |
//! | f64   | 1,083,162          | 123,675,824           | 114.18x           |
//!
//! # Raw Benchmarks
//!
//! ```text
//! test f32_lexical ... bench:     761,670 ns/iter (+/- 194,856)
//! test f32_parse   ... bench:  28,650,637 ns/iter (+/- 7,269,036)
//! test f64_lexical ... bench:   1,083,162 ns/iter (+/- 315,101)
//! test f64_parse   ... bench: 123,675,824 ns/iter (+/- 20,924,195)
//! ```
//!
//! Raw Benchmarks (`no_std`)
//!
//! ```text
//! test f32_lexical ... bench:     652,922 ns/iter (+/- 44,491)
//! test f32_parse   ... bench:  24,381,160 ns/iter (+/- 687,175)
//! test f64_lexical ... bench:     835,822 ns/iter (+/- 28,754)
//! test f64_parse   ... bench: 113,449,442 ns/iter (+/- 3,983,104)
//! ```

// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([761670, 1083162]) / 1e6
//  parse = np.array([28650637, 123675824]) / 1e6
//  index = ["f32", "f64"]
//  df = pd.DataFrame({'lexical': lexical, 'parse': parse}, index = index)
//  ax = df.plot.bar(rot=0)
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  plt.show()

use super::algorithm::lossy;

// F32

/// Import float from basen, using a lossy algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
pub(crate) unsafe extern "C" fn float_basen(first: *const u8, last: *const u8, base: u64)
    -> (f32, *const u8)
{
    lossy::atof(first, last, base)
}

/// Import float from base10, using a lossy algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
#[cfg(not(feature = "correct"))]
pub(crate) unsafe extern "C" fn float_base10(first: *const u8, last: *const u8)
    -> (f32, *const u8)
{
    lossy::atof(first, last, 10)
}

// F64

/// Import double from basen, using a lossy algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
pub(crate) unsafe extern "C" fn double_basen(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    lossy::atod(first, last, base)
}

/// Import double from base10, using a lossy algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
#[cfg(not(feature = "correct"))]
pub(crate) unsafe extern "C" fn double_base10(first: *const u8, last: *const u8)
    -> (f64, *const u8)
{
    lossy::atod(first, last, 10)
}
