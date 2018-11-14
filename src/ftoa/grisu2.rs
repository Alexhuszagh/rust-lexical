//! Internal implementation of the Grisu2 algorithm.
//!
//! The optimized routines are adapted from Andrea Samoljuk's `fpconv` library,
//! which is available [here](https://github.com/night-shift/fpconv).
//!
//! The following benchmarks were run on an "Intel(R) Core(TM) i7-6560U
//! CPU @ 2.20GHz" CPU, on Fedora 28, Linux kernel version 4.18.16-200
//! (x86-64), using the lexical formatter, `dtoa::write()` or `x.to_string()`,
//! avoiding any inefficiencies in Rust string parsing for `format!(...)`
//! or `write!()` macros. The code was compiled with LTO and at an optimization
//! level of 3.
//!
//! The benchmarks with `std` were compiled using "rustc 1.29.2 (17a9dc751
//! 2018-10-05", and the `no_std` benchmarks were compiled using "rustc
//! 1.31.0-nightly (46880f41b 2018-10-15)".
//!
//! The benchmark code may be found `benches/ftoa.rs`.
//!
//! # Benchmarks
//!
//! | Type  |  lexical (ns/iter) | to_string (ns/iter)   | Relative Increase |
//! |:-----:|:------------------:|:---------------------:|:-----------------:|
//! | f32   | 1,221,025          | 2,711,290             | 2.22x             |
//! | f64   | 1,248,397          | 3,558,305             | 2.85x             |
//!
//! # Raw Benchmarks
//!
//! ```text
//! test f32_dtoa      ... bench:   1,174,070 ns/iter (+/- 442,501)
//! test f32_lexical   ... bench:   1,433,234 ns/iter (+/- 633,261)
//! test f32_ryu       ... bench:     669,828 ns/iter (+/- 192,291)
//! test f32_to_string ... bench:   3,341,733 ns/iter (+/- 1,346,744)
//! test f64_dtoa      ... bench:   1,302,522 ns/iter (+/- 364,655)
//! test f64_lexical   ... bench:   1,375,384 ns/iter (+/- 596,860)
//! test f64_ryu       ... bench:   1,015,171 ns/iter (+/- 187,552)
//! test f64_to_string ... bench:   3,900,299 ns/iter (+/- 521,956)
//! ```
//!
//! Raw Benchmarks (`no_std`)
//!
//! ```text
//! test f32_dtoa      ... bench:   1,174,070 ns/iter (+/- 442,501)
//! test f32_lexical   ... bench:   1,433,234 ns/iter (+/- 633,261)
//! test f32_ryu       ... bench:     669,828 ns/iter (+/- 192,291)
//! test f32_to_string ... bench:   3,341,733 ns/iter (+/- 1,346,744)
//! test f64_dtoa      ... bench:   1,302,522 ns/iter (+/- 364,655)
//! test f64_lexical   ... bench:   1,375,384 ns/iter (+/- 596,860)
//! test f64_ryu       ... bench:   1,015,171 ns/iter (+/- 187,552)
//! test f64_to_string ... bench:   3,900,299 ns/iter (+/- 521,956)
//! ```

// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([1221025, 1248397]) / 1e6
//  to_string = np.array([2711290, 3558305]) / 1e6
//  index = ["f32", "f64"]
//  df = pd.DataFrame({'lexical': lexical, 'to_string': to_string}, index = index)
//  ax = df.plot.bar(rot=0)
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  plt.show()

use float::FloatType;
use lib::{mem, ptr};
use util::*;
use super::util::*;

// CACHED POWERS

/// Find cached power of 10 from the exponent.
#[inline]
pub(crate) unsafe extern "C" fn cached_grisu_power(exp: i32, k: *mut i32)
    -> &'static FloatType
{
    // FLOATING POINT CONSTANTS
    const ONE_LOG_TEN: f64 = 0.30102999566398114;
    const NPOWERS: i32 = 87;
    const FIRSTPOWER: i32 = -348;       // 10 ^ -348
    const STEPPOWERS: i32 = 8;
    const EXPMAX: i32 = -32;
    const EXPMIN: i32 = -60;

    let approx = -((exp + NPOWERS) as f64) * ONE_LOG_TEN;
    let approx = approx as i32;
    let mut idx = ((approx - FIRSTPOWER) / STEPPOWERS) as usize;

    loop {
        let power = GRISU_POWERS_OF_TEN.get_unchecked(idx);
        let current = exp + power.exp + 64;
        if current < EXPMIN {
            idx += 1;
            continue;
        }

        if current > EXPMAX {
            idx -= 1;
            continue;
        }

        *k = FIRSTPOWER + (idx as i32) * STEPPOWERS;
        return power;
    }
}



/// Cached powers of ten as specified by the Grisu algorithm.
///
/// Cached powers of 10^k, calculated as if by:
/// `ceil((alpha-e+63) * ONE_LOG_TEN);`
const GRISU_POWERS_OF_TEN: [FloatType; 87] = [
    FloatType { frac: 18054884314459144840, exp: -1220 },
    FloatType { frac: 13451937075301367670, exp: -1193 },
    FloatType { frac: 10022474136428063862, exp: -1166 },
    FloatType { frac: 14934650266808366570, exp: -1140 },
    FloatType { frac: 11127181549972568877, exp: -1113 },
    FloatType { frac: 16580792590934885855, exp: -1087 },
    FloatType { frac: 12353653155963782858, exp: -1060 },
    FloatType { frac: 18408377700990114895, exp: -1034 },
    FloatType { frac: 13715310171984221708, exp: -1007 },
    FloatType { frac: 10218702384817765436, exp: -980 },
    FloatType { frac: 15227053142812498563, exp: -954 },
    FloatType { frac: 11345038669416679861, exp: -927 },
    FloatType { frac: 16905424996341287883, exp: -901 },
    FloatType { frac: 12595523146049147757, exp: -874 },
    FloatType { frac: 9384396036005875287, exp: -847 },
    FloatType { frac: 13983839803942852151, exp: -821 },
    FloatType { frac: 10418772551374772303, exp: -794 },
    FloatType { frac: 15525180923007089351, exp: -768 },
    FloatType { frac: 11567161174868858868, exp: -741 },
    FloatType { frac: 17236413322193710309, exp: -715 },
    FloatType { frac: 12842128665889583758, exp: -688 },
    FloatType { frac: 9568131466127621947, exp: -661 },
    FloatType { frac: 14257626930069360058, exp: -635 },
    FloatType { frac: 10622759856335341974, exp: -608 },
    FloatType { frac: 15829145694278690180, exp: -582 },
    FloatType { frac: 11793632577567316726, exp: -555 },
    FloatType { frac: 17573882009934360870, exp: -529 },
    FloatType { frac: 13093562431584567480, exp: -502 },
    FloatType { frac: 9755464219737475723, exp: -475 },
    FloatType { frac: 14536774485912137811, exp: -449 },
    FloatType { frac: 10830740992659433045, exp: -422 },
    FloatType { frac: 16139061738043178685, exp: -396 },
    FloatType { frac: 12024538023802026127, exp: -369 },
    FloatType { frac: 17917957937422433684, exp: -343 },
    FloatType { frac: 13349918974505688015, exp: -316 },
    FloatType { frac: 9946464728195732843, exp: -289 },
    FloatType { frac: 14821387422376473014, exp: -263 },
    FloatType { frac: 11042794154864902060, exp: -236 },
    FloatType { frac: 16455045573212060422, exp: -210 },
    FloatType { frac: 12259964326927110867, exp: -183 },
    FloatType { frac: 18268770466636286478, exp: -157 },
    FloatType { frac: 13611294676837538539, exp: -130 },
    FloatType { frac: 10141204801825835212, exp: -103 },
    FloatType { frac: 15111572745182864684, exp: -77 },
    FloatType { frac: 11258999068426240000, exp: -50 },
    FloatType { frac: 16777216000000000000, exp: -24 },
    FloatType { frac: 12500000000000000000, exp:  3 },
    FloatType { frac: 9313225746154785156, exp:  30 },
    FloatType { frac: 13877787807814456755, exp: 56 },
    FloatType { frac: 10339757656912845936, exp: 83 },
    FloatType { frac: 15407439555097886824, exp: 109 },
    FloatType { frac: 11479437019748901445, exp: 136 },
    FloatType { frac: 17105694144590052135, exp: 162 },
    FloatType { frac: 12744735289059618216, exp: 189 },
    FloatType { frac: 9495567745759798747, exp: 216 },
    FloatType { frac: 14149498560666738074, exp: 242 },
    FloatType { frac: 10542197943230523224, exp: 269 },
    FloatType { frac: 15709099088952724970, exp: 295 },
    FloatType { frac: 11704190886730495818, exp: 322 },
    FloatType { frac: 17440603504673385349, exp: 348 },
    FloatType { frac: 12994262207056124023, exp: 375 },
    FloatType { frac: 9681479787123295682, exp: 402 },
    FloatType { frac: 14426529090290212157, exp: 428 },
    FloatType { frac: 10748601772107342003, exp: 455 },
    FloatType { frac: 16016664761464807395, exp: 481 },
    FloatType { frac: 11933345169920330789, exp: 508 },
    FloatType { frac: 17782069995880619868, exp: 534 },
    FloatType { frac: 13248674568444952270, exp: 561 },
    FloatType { frac: 9871031767461413346, exp: 588 },
    FloatType { frac: 14708983551653345445, exp: 614 },
    FloatType { frac: 10959046745042015199, exp: 641 },
    FloatType { frac: 16330252207878254650, exp: 667 },
    FloatType { frac: 12166986024289022870, exp: 694 },
    FloatType { frac: 18130221999122236476, exp: 720 },
    FloatType { frac: 13508068024458167312, exp: 747 },
    FloatType { frac: 10064294952495520794, exp: 774 },
    FloatType { frac: 14996968138956309548, exp: 800 },
    FloatType { frac: 11173611982879273257, exp: 827 },
    FloatType { frac: 16649979327439178909, exp: 853 },
    FloatType { frac: 12405201291620119593, exp: 880 },
    FloatType { frac: 9242595204427927429, exp: 907 },
    FloatType { frac: 13772540099066387757, exp: 933 },
    FloatType { frac: 10261342003245940623, exp: 960 },
    FloatType { frac: 15290591125556738113, exp: 986 },
    FloatType { frac: 11392378155556871081, exp: 1013 },
    FloatType { frac: 16975966327722178521, exp: 1039 },
    FloatType { frac: 12648080533535911531, exp: 1066 }
];

// FTOA BASE10

// LOOKUPS
const TENS: [u64; 20] = [
    10000000000000000000, 1000000000000000000, 100000000000000000,
    10000000000000000, 1000000000000000, 100000000000000,
    10000000000000, 1000000000000, 100000000000,
    10000000000, 1000000000, 100000000,
    10000000, 1000000, 100000,
    10000, 1000, 100,
    10, 1
];

// FPCONV GRISU

/// Round digit to sane approximation.
unsafe extern "C"
fn round_digit(digits: *mut u8, ndigits: isize, delta: u64, mut rem: u64, kappa: u64, frac: u64)
{
    while rem < frac && delta - rem >= kappa &&
           (rem + kappa < frac || frac - rem > rem + kappa - frac) {

        *digits.offset(ndigits - 1) -= 1;
        rem += kappa;
    }
}

/// Generate digits from upper and lower range on rounding of number.
unsafe extern "C"
fn generate_digits(fp: &FloatType, upper: &FloatType, lower: &FloatType, digits: *mut u8, k: *mut i32)
    -> i32
{
    let wfrac = upper.frac - fp.frac;
    let mut delta = upper.frac - lower.frac;

    let one = FloatType {
        frac: 1 << -upper.exp,
        exp: upper.exp,
    };

    let mut part1 = upper.frac >> -one.exp;
    let mut part2 = upper.frac & (one.frac - 1);

    let mut idx: isize = 0;
    let mut kappa: i32 = 10;
    // 1000000000
    let mut divp: *const u64 = TENS.as_ptr().add(10);
    while kappa > 0 {
        // Remember not to continue! This loop has an increment condition.
        let div = *divp;
        let digit = part1 / div;
        if digit != 0 || idx != 0 {
            *digits.offset(idx) = (digit as u8) + b'0';
            idx += 1;
        }

        part1 -= (digit as u64) * div;
        kappa -= 1;

        let tmp = (part1 <<-one.exp) + part2;
        if tmp <= delta {
            *k += kappa;
            round_digit(digits, idx, delta, tmp, div << -one.exp, wfrac);
            return idx as i32;
        }

        // Increment condition, DO NOT ADD continue.
        divp = divp.add(1);
    }

    /* 10 */
    let mut unit: *const u64 = TENS.as_ptr().add(18);

    loop {
        part2 *= 10;
        delta *= 10;
        kappa -= 1;

        let digit = part2 >> -one.exp;
        if digit != 0 || idx != 0 {
            *digits.offset(idx) = (digit as u8) + b'0';
            idx += 1;
        }

        part2 &= one.frac - 1;
        if part2 < delta {
            *k += kappa;
            round_digit(digits, idx, delta, part2, one.frac, wfrac * *unit);
            return idx as i32;
        }

        unit = unit.sub(1);
    }
}

/// Core Grisu2 algorithm for the float formatter.
unsafe extern "C" fn grisu2(d: f64, digits: *mut u8, k: *mut i32) -> i32
{
    let mut w = FloatType::from_f64(d);

    let (mut lower, mut upper) = w.normalized_boundaries();
    w.normalize();

    let mut ki: i32 = mem::uninitialized();
    let cp = cached_grisu_power(upper.exp, &mut ki);

    w     = w.mul(&cp);
    upper = upper.mul(&cp);
    lower = lower.mul(&cp);

    lower.frac += 1;
    upper.frac -= 1;

    *k = -ki;

    return generate_digits(&w, &upper, &lower, digits, k);
}

/// Write the produced digits to string.
///
/// Adds formatting for exponents, and other types of information.
unsafe extern "C" fn emit_digits(digits: *mut u8, mut ndigits: i32, dest: *mut u8, k: i32)
    -> i32
{
    let exp = k + ndigits - 1;
    let mut exp = exp.abs();

    // write plain integer (with ".0" suffix).
    if k >= 0 && exp < (ndigits + 7) {
        let idx = ndigits as usize;
        let count = k as usize;
        ptr::copy_nonoverlapping(digits, dest, idx);
        ptr::write_bytes(dest.add(idx), b'0', count);
        ptr::copy_nonoverlapping(b".0".as_ptr(), dest.add(idx + count), 2);

        return ndigits + k + 2;
    }

    // write decimal w/o scientific notation
    if k < 0 && (k > -7 || exp < 4) {
        let mut offset = ndigits - k.abs();
        // fp < 1.0 -> write leading zero
        if offset <= 0 {
            offset = -offset;
            *dest = b'0';
            *dest.add(1) = b'.';
            ptr::write_bytes(dest.add(2), b'0', offset as usize);
            let dst = dest.add(offset as usize + 2);
            ptr::copy_nonoverlapping(digits, dst, ndigits as usize);

            return ndigits + 2 + offset;

        } else {
            // fp > 1.0
            ptr::copy_nonoverlapping(digits, dest, offset as usize);
            *dest.offset(offset as isize) = b'.';
            let dst = dest.offset(offset as isize + 1);
            let src = digits.offset(offset as isize);
            let count = (ndigits - offset) as usize;
            ptr::copy_nonoverlapping(src, dst, count);

            return ndigits + 1;
        }
    }

    // write decimal w/ scientific notation
    ndigits = ndigits.min(18);

    let mut idx: isize = 0;
    *dest.offset(idx) = *digits;
    idx += 1;

    if ndigits > 1 {
        *dest.offset(idx) = b'.';
        idx += 1;
        let dst = dest.offset(idx);
        let src = digits.add(1);
        let count = (ndigits - 1) as usize;
        ptr::copy_nonoverlapping(src, dst, count);
        idx += (ndigits - 1) as isize;
    }

    *dest.offset(idx) = exponent_notation_char(10);
    idx += 1;

    let sign: u8 = match k + ndigits - 1 < 0 {
        true    => b'-',
        false   => b'+',
    };
    *dest.offset(idx) = sign;
    idx += 1;

    let mut cent: i32 = 0;
    if exp > 99 {
        cent = exp / 100;
        *dest.offset(idx) = (cent as u8) + b'0';
        idx += 1;
        exp -= cent * 100;
    }
    if exp > 9 {
        let dec = exp / 10;
        *dest.offset(idx) = (dec as u8) + b'0';
        idx += 1;
        exp -= dec * 10;
    } else if cent != 0 {
        *dest.offset(idx) = b'0';
        idx += 1;
    }

    let shift: u8 = (exp % 10) as u8;
    *dest.offset(idx) = shift + b'0';
    idx += 1;

    idx as i32
}

unsafe extern "C" fn fpconv_dtoa(d: f64, dest: *mut u8) -> i32
{
    let mut digits: [u8; 18] = mem::uninitialized();
    let mut k: i32 = 0;
    let ndigits = grisu2(d, digits.as_mut_ptr(), &mut k);
    emit_digits(digits.as_mut_ptr(), ndigits, dest, k)
}

// F32

/// Forward to double_base10.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
#[inline(always)]
pub(crate) unsafe extern "C" fn float_base10(f: f32, first: *mut u8)
    -> *mut u8
{
    double_base10(f as f64, first)
}

// F64

/// Optimized algorithm for base10 numbers.
///
/// `d` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
#[inline(always)]
pub(crate) unsafe extern "C" fn double_base10(d: f64, first: *mut u8)
    -> *mut u8
{
    let len = fpconv_dtoa(d, first);
    first.offset(len as isize)
}
