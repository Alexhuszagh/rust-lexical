//! Fast lexical float-to-string conversion routines.
//!
//! The optimized routines are adapted from Andrea Samoljuk's `fpconv` library,
//! which is available [here](https://github.com/night-shift/fpconv).
//!
//! The radix-generic algorithm is adapted from the V8 codebase,
//! and may be found [here](https://github.com/v8/v8).
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
//! test f32_dtoa      ... bench:   1,301,173 ns/iter (+/- 45,283)
//! test f32_lexical   ... bench:   1,221,025 ns/iter (+/- 42,428)
//! test f32_to_string ... bench:   2,711,290 ns/iter (+/- 75,376)
//! test f64_dtoa      ... bench:   1,462,523 ns/iter (+/- 103,974)
//! test f64_lexical   ... bench:   1,248,397 ns/iter (+/- 41,952)
//! test f64_to_string ... bench:   3,558,305 ns/iter (+/- 103,945)
//! ```
//!
//! Raw Benchmarks (`no_std`)
//!
//! ```text
//! test f32_dtoa      ... bench:   1,727,839 ns/iter (+/- 76,330)
//! test f32_lexical   ... bench:   1,461,892 ns/iter (+/- 95,811)
//! test f32_to_string ... bench:   3,687,219 ns/iter (+/- 313,885)
//! test f64_dtoa      ... bench:   1,927,419 ns/iter (+/- 122,000)
//! test f64_lexical   ... bench:   1,505,955 ns/iter (+/- 87,548)
//! test f64_to_string ... bench:   4,774,595 ns/iter (+/- 244,214)
//! ```

// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([1221025, 1248397]) / 1e6
//  to_string = np.array([2711290, 3558305]) / 1e6
//  df = pd.DataFrame({'lexical': lexical, 'to_string': to_string}, index = index)
//  ax = df.plot.bar(rot=0)
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  plt.show()

use sealed::mem;
use sealed::ptr;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::string::String;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::vec::Vec;

use float::{cached_power, FloatType};
use itoa::itoa_forward;
use table::BASEN;
use util::{distance, floor, ln};

// FTOA BASE10
// -----------

// NOTATION CHAR

/// Get the exponent notation character.
///
/// After radix of base15 and higher, 'E' and 'e' are
/// part of the controlled vocabulary.
/// We use a non-standard extension of '^' to signify
/// the exponent in base15 and above.
pub extern "C" fn exponent_notation_char(base: u64)
    -> u8
{
    if base >= 15 { b'^' } else { b'e' }
}

// BUFFER PARAMTERS
// The buffer is actually a size of 60, but use 64 since it's a power of 2.
// Simple, fast optimization.
// Since we're declaring a variable on the stack, and our power-of-two
// alignment dramatically improved atoi performance, do it.
// Use 256, actually, since we seem to have memory issues with 64-bits.
// Clearly not sufficient memory allocated for non-base10 values.
const MAX_FLOAT_SIZE: usize = 256;
const BUFFER_SIZE: usize = MAX_FLOAT_SIZE;

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
    let cp = cached_power(upper.exp, &mut ki);

    w     = w.multiply(&cp);
    upper = upper.multiply(&cp);
    lower = lower.multiply(&cp);

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
    let mut exp = absv!(exp);

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
        let mut offset = ndigits - absv!(k);
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
    ndigits = minv!(ndigits, 18);

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

/// Filter special floating-point numbers.
unsafe extern "C" fn filter_special(fp: f64, dest: *mut u8) -> i32
{
    const EXPONENT_MASK: u64 = FloatType::EXPONENT_MASK;
    const FRACTION_MASK: u64 = FloatType::FRACTION_MASK;

    if fp == 0.0 {
        ptr::copy_nonoverlapping(b"0.0".as_ptr(), dest, 3);
        return 3;
    }

    let bits = fp.to_bits();
    let nan = (bits & EXPONENT_MASK) == EXPONENT_MASK;

    if !nan {
        return 0;
    }

    if bits & FRACTION_MASK != 0 {
        ptr::copy_nonoverlapping(b"NaN".as_ptr(), dest, 3);
        return 3;

    } else {
        ptr::copy_nonoverlapping(b"Infinity".as_ptr(), dest, 8);
        return 8;
    }
}

unsafe extern "C" fn fpconv_dtoa(d: f64, dest: *mut u8) -> i32
{
    let mut digits: [u8; 18] = mem::uninitialized();
    let mut str_len: i32 = 0;
    let spec = filter_special(d, dest.offset(str_len as isize));
    if spec != 0 {
        return str_len + spec;
    }

    let mut k: i32 = 0;
    let ndigits = grisu2(d, digits.as_mut_ptr(), &mut k);
    str_len += emit_digits(digits.as_mut_ptr(), ndigits, dest.offset(str_len as isize), k);

    str_len
}

/// Optimized algorithm for base10 numbers.
unsafe extern "C" fn ftoa_base10(value: f64, first: *mut u8)
    -> *mut u8
{
    let len = fpconv_dtoa(value, first);
    first.offset(len as isize)
}

// FTOA BASEN
// ----------

// V8 RADIX

/// Returns true if the double is a denormal.
#[inline]
#[allow(dead_code)]
fn v8_is_denormal(d: f64) -> bool
{
    (d.to_bits() & FloatType::EXPONENT_MASK) == 0
}

/// Check if the number is special, all non-hidden bits in exponent are set.
/// Denormals are not to be special, hence only Infinity and NaN are special.
#[inline]
#[allow(dead_code)]
fn v8_is_special(d: f64) -> bool
{
    let bits = d.to_bits();
    (bits & FloatType::EXPONENT_MASK) == FloatType::EXPONENT_MASK
}

/// Check if value is NaN.
#[inline]
#[allow(dead_code)]
fn v8_is_nan(d: f64) -> bool
{
    const EXPONENT_MASK: u64 = FloatType::EXPONENT_MASK;
    const FRACTION_MASK: u64 = FloatType::FRACTION_MASK;
    let bits = d.to_bits();
    ((bits & EXPONENT_MASK) == EXPONENT_MASK) && ((bits & FRACTION_MASK) != 0)
}

/// Check if value is infinite.
#[inline]
#[allow(dead_code)]
fn v8_is_infinite(d: f64) -> bool
{
    const EXPONENT_MASK: u64 = FloatType::EXPONENT_MASK;
    const FRACTION_MASK: u64 = FloatType::FRACTION_MASK;
    let bits = d.to_bits();
    ((bits & EXPONENT_MASK) == EXPONENT_MASK) && ((bits & FRACTION_MASK) == 0)
}


/// Get sign from float.
#[inline]
#[allow(dead_code)]
fn v8_sign(d: f64) -> i32
{
    let bits = d.to_bits();
    if (bits & FloatType::SIGN_MASK) == 0 { 1 } else { -1 }
}


/// Get exponent from float.
#[inline]
#[allow(dead_code)]
fn v8_exponent(d: f64) -> i32
{
    const EXPONENT_MASK: u64 = FloatType::EXPONENT_MASK;
    const PHYSICAL_SIGNIFICAND_SIZE: i32 = FloatType::PHYSICAL_SIGNIFICAND_SIZE;

    if v8_is_denormal(d) {
        return FloatType::DENORMAL_EXPONENT;
    }

    let bits = d.to_bits();
    let biased_e = ((bits & EXPONENT_MASK) >> PHYSICAL_SIGNIFICAND_SIZE) as i32;
    biased_e - FloatType::EXPONENT_BIAS
}

/// Get significand from float.
#[inline]
#[allow(dead_code)]
fn v8_significand(d: f64) -> u64
{
    let bits = d.to_bits();
    let s = bits & FloatType::FRACTION_MASK;
    if !v8_is_denormal(d) {
      s + FloatType::HIDDEN_BIT_MASK
    } else {
      s
    }
}

/// Returns the next greater double. Returns +infinity on input +infinity.
#[inline]
#[allow(dead_code)]
fn v8_next_double(d: f64) -> f64
{
    let bits = d.to_bits();
    if bits == FloatType::U64_INFINITY {
        return f64::from_bits(FloatType::U64_INFINITY);
    }
    if v8_sign(d) < 0 && v8_significand(d) == 0 {
      // -0.0
      return 0.0;
    }
    if v8_sign(d) < 0 {
        return f64::from_bits(bits - 1);
    } else {
        return f64::from_bits(bits + 1);
    }
}

/// Floating-point modulus (rust supports this intrinsically).
#[inline]
#[allow(dead_code)]
fn v8_modulo(x: f64, y: f64) -> f64
{
    x % y
}

/// Calculate the naive exponent from a minimal value.
#[inline]
fn naive_exponent(d: f64, base: u64) -> i32
{
    // floor returns the minimal value, which is our
    // desired exponent
    // ln(1.1e-5) -> -4.95 -> -5
    // ln(1.1e5) -> -5.04 -> 5
    (floor(ln(d) / ln(base as f64))) as i32
}

/// Naive algorithm for converting a floating point to a custom radix.
///
/// Adapted from the V8 implementation.
unsafe extern "C" fn ftoa_naive(d: f64, first: *mut u8, base: u64)
    -> *mut u8
{
    // Logic error, base should not be passed dynamically.
    debug_assert!(base >= 2 && base <= 36,"Numerical base must be from 2-36");

    // check for special cases
    let length = filter_special(d, first);
    if length != 0 {
        return first.offset(length as isize);
    }

    // assert no special cases remain
    debug_assert!(!v8_is_special(d));
    debug_assert!(d != 0.0);

    // Store the first digit and up to `BUFFER_SIZE - 20` digits
    // that occur from left-to-right in the decimal representation.
    // For example, for the number 123.45, store the first digit `1`
    // and `2345` as the remaining values. Then, decide on-the-fly
    // if we need scientific or regular formatting.
    //
    //   BUFFER_SIZE
    // - 1      # first digit
    // - 1      # period
    // - 1      # +/- sign
    // - 2      # e and +/- sign
    // - 9      # max exp is 308, in base2 is 9
    // - 1      # null terminator
    // = 15 characters of formatting required
    // Just pad it a bit, we don't want memory corruption.
    const MAX_NONDIGIT_LENGTH: usize = 25;
    const MAX_DIGIT_LENGTH: usize = BUFFER_SIZE - MAX_NONDIGIT_LENGTH;

    // Temporary buffer for the result. We start with the decimal point in the
    // middle and write to the left for the integer part and to the right for the
    // fractional part. 1024 characters for the exponent and 52 for the mantissa
    // either way, with additional space for sign, decimal point and string
    // termination should be sufficient.
    const SIZE: usize = 2200;
    let mut buffer: [u8; SIZE] = mem::uninitialized();
    let buffer = buffer.as_mut_ptr();
    let initial_position: usize = SIZE / 2;
    let mut integer_cursor = initial_position;
    let mut fraction_cursor = initial_position;
    let bf = base as f64;

    // Split the value into an integer part and a fractional part.
    let mut integer = floor(d);
    let mut fraction = d - integer;

    // We only compute fractional digits up to the input double's precision.
    let mut delta = 0.5 * (v8_next_double(d) - d);
    delta = maxv!(v8_next_double(0.0), delta);
    debug_assert!(delta > 0.0);

    if fraction > delta {
        loop {
            // Shift up by one digit.
            fraction *= bf;
            delta *= bf;
            // Write digit.
            let digit = fraction as i32;
            *buffer.add(fraction_cursor) = *BASEN.get_unchecked(digit as usize);
            fraction_cursor += 1;
            // Calculate remainder.
            fraction -= digit as f64;
            // Round to even.
            if fraction > 0.5 || (fraction == 0.5 && (digit & 1) != 0) {
                if fraction + delta > 1.0 {
                    // We need to back trace already written digits in case of carry-over.
                    loop {
                        fraction_cursor -= 1;
                        if fraction_cursor == initial_position-1 {
                            // Carry over to the integer part.
                            integer += 1.0;
                            break;
                        }
                        let c = *buffer.add(fraction_cursor);
                        // Reconstruct digit.
                        let digit: i32;
                        if c <= b'9' {
                            digit = (c - b'0') as i32;
                        } else if c >= b'A' && c <= b'Z' {
                            digit = (c - b'A' + 10) as i32;
                        } else {
                            debug_assert!(c >= b'a' && c <= b'z');
                            digit = (c - b'a' + 10) as i32;
                        }
                        if digit + 1 < base as i32 {
                            let idx = (digit + 1) as usize;
                            *buffer.add(fraction_cursor) = *BASEN.get_unchecked(idx);
                            fraction_cursor += 1;
                            break;
                        }
                    }
                    break;
                }
            }

            if delta >= fraction {
                break;
            }
        }
    }

    // Compute integer digits. Fill unrepresented digits with zero.
    while v8_exponent(integer / bf) > 0 {
        integer /= bf;
        integer_cursor -= 1;
        *buffer.add(integer_cursor) = b'0';
    }

    loop {
        let remainder = v8_modulo(integer, bf);
        integer_cursor -= 1;
        let idx = remainder as usize;
        *buffer.add(integer_cursor) = *BASEN.get_unchecked(idx);
        integer = (integer - remainder) / bf;

        if integer <= 0.0 {
            break;
        }
    };

    if d <= 1e-5 || d >= 1e9 {
        // write scientific notation with negative exponent
        let exponent = naive_exponent(d, base);

        // Non-exponent portion.
        // 1.   Get as many digits as possible, up to `MAX_DIGIT_LENGTH+1`
        //      (since we are ignoring the digit for the first digit),
        //      or the number of written digits
        let start: usize;
        let end: usize;
        if d <= 1e-5 {
            start = ((initial_position as i32) - exponent - 1) as usize;
            end = minv!(fraction_cursor, start + MAX_DIGIT_LENGTH + 1);
        } else {
            start = integer_cursor;
            end = minv!(fraction_cursor, start + MAX_DIGIT_LENGTH + 1);
        }
        let mut buf_first = buffer.add(start);
        let mut buf_last = buf_first.add(end - start);

        // 2.   Remove any trailing 0s in the selected range.
        loop {
            buf_last = buf_last.sub(1);
            if *buf_last != b'0' {
                break;
            }
        }

        // 3.   Write the fraction component
        let mut p = first;
        *p = *buf_first;
        p = p.add(1);
        buf_first = buf_first.add(1);
        *p = b'.';
        p = p.add(1);
        let dist = distance(buf_first, buf_last);
        ptr::copy_nonoverlapping(buf_first, p, dist);
        p = p.add(dist);

        // write the exponent component
        *p = exponent_notation_char(base);
        p = p.add(1);
        return itoa_forward(exponent as u64, p, base);

    } else {
        let mut p;
        // get component lengths
        let integer_length = initial_position - integer_cursor;
        let fraction_length = minv!(fraction_cursor - initial_position, MAX_DIGIT_LENGTH - integer_length);

        // write integer component
        ptr::copy_nonoverlapping(buffer.add(integer_cursor), first, integer_length);
        p = first.add(integer_length);

        // write fraction component
        if fraction_length > 0 {
            // fraction exists, write it
            *p = b'.';
            p = p.add(1);
            ptr::copy_nonoverlapping(buffer.add(initial_position), p, fraction_length);
            p = p.add(fraction_length);
        } else {
            // no fraction, write decimal place
            ptr::copy_nonoverlapping(b".0".as_ptr(), p, 2);
            p = p.add(2);
        }

        return p;
    }
}

#[inline(always)]
unsafe extern "C" fn ftoa_basen(value: f64, first: *mut u8, base: u64)
    -> *mut u8
{
    ftoa_naive(value, first, base)
}

// FTOA

/// Check if the supplied buffer has enough range for the encoded size.
macro_rules! check_digits {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        debug_assert!(distance($first, $last) >= BUFFER_SIZE, "Need a larger buffer.");
    })
}

/// Forward the correct arguments to the implementation.
macro_rules! ftoa_forward {
    ($value:ident, $first:ident, $base:ident) => (match $base {
        10  => ftoa_base10($value, $first),
        _   => ftoa_basen($value, $first, $base),
    })
}

/// Sanitizer for an unsigned number-to-string implementation.
macro_rules! ftoa_unsafe_impl {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        // Sanity checks
        debug_assert!($first <= $last);
        check_digits!($value, $first, $last, $base);

        if $value < 0.0 {
            *$first= b'-';
            $value = -$value;
            $first = $first.add(1);
        }

        ftoa_forward!($value, $first, $base)
    })
}

/// Generate the ftoa wrappers.
macro_rules! ftoa_unsafe {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        // check to use a temporary buffer
        let dist = distance($first, $last);
        if dist == 0 {
            // cannot write null terminator
            $first
        } else if dist < BUFFER_SIZE {
            // use a temporary buffer and write number to buffer
            let mut buffer: [u8; BUFFER_SIZE] = mem::uninitialized();
            let mut f = buffer.as_mut_ptr();
            let l = f.add(BUFFER_SIZE);
            let mut v = $value as f64;
            let b = $base as u64;
            ftoa_unsafe_impl!(v, f, l, b);

            // copy as many bytes as possible except the trailing null byte
            // then, write null-byte so the string is always terminated
            let length = minv!(distance(f, l), dist);
            ptr::copy_nonoverlapping(f, $first, length);
            $first.add(length)
        } else {
            // current buffer has sufficient capacity, use it
            let mut v = $value as f64;
            let b = $base as u64;
            ftoa_unsafe_impl!(v, $first, $last, b)
    }
    })
}

// UNSAFE API

/// Generate the unsafe wrappers.
macro_rules! unsafe_impl {
    ($func:ident, $t:ty) => (
        /// Unsafe, C-like exporter for float numbers.
        ///
        /// # Warning
        ///
        /// Do not call this function directly, unless you **know**
        /// you have a buffer of sufficient size. No size checking is
        /// done in release mode, this function is **highly** dangerous.
        /// Sufficient buffer sizes is denoted by `BUFFER_SIZE`.
        #[inline]
        pub unsafe extern "C" fn $func(
            value: $t,
            mut first: *mut u8,
            last: *mut u8,
            base: u8
        )
            -> *mut u8
        {
            ftoa_unsafe!(value, first, last, base)
        }
    )
}

unsafe_impl!(f32toa_unsafe, f32);
unsafe_impl!(f64toa_unsafe, f64);

// LOW-LEVEL API

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(f32toa_string, f32, f32toa_unsafe, BUFFER_SIZE);

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(f64toa_string, f64, f64toa_unsafe, BUFFER_SIZE);

// TESTS
// -----

#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg(test)]
mod tests {
    use super::*;
    use atof::*;
    use util::*;

    // Test data for roundtrips.
    const F32_DATA : [f32; 31] = [0., 0.1, 1., 1.1, 12., 12.1, 123., 123.1, 1234., 1234.1, 12345., 12345.1, 123456., 123456.1, 1234567., 1234567.1, 12345678., 12345678.1, 123456789., 123456789.1, 123456789.12, 123456789.123, 123456789.1234, 123456789.12345, 1.2345678912345e8, 1.2345e+8, 1.2345e+11, 1.2345e+38, 1.2345e-8, 1.2345e-11, 1.2345e-38];
    const F64_DATA: [f64; 33] = [0., 0.1, 1., 1.1, 12., 12.1, 123., 123.1, 1234., 1234.1, 12345., 12345.1, 123456., 123456.1, 1234567., 1234567.1, 12345678., 12345678.1, 123456789., 123456789.1, 123456789.12, 123456789.123, 123456789.1234, 123456789.12345, 1.2345678912345e8, 1.2345e+8, 1.2345e+11, 1.2345e+38, 1.2345e+308, 1.2345e-8, 1.2345e-11, 1.2345e-38, 1.2345e-299];

    #[test]
    fn f32toa_base2_test() {
        // positive
        assert_eq!("0.0", &f32toa_string(0.0, 2));
        assert_eq!("1.0", &f32toa_string(1.0, 2));
        assert_eq!("10.0", &f32toa_string(2.0, 2));
        assert_eq!("1.1", &f32toa_string(1.5, 2));
        assert_eq!("1.01", &f32toa_string(1.25, 2));
        assert_eq!("1.001111000000110010", &f32toa_string(1.2345678901234567890e0, 2)[..20]);
        assert_eq!("1100.010110000111111", &f32toa_string(1.2345678901234567890e1, 2)[..20]);
        assert_eq!("1111011.011101001111", &f32toa_string(1.2345678901234567890e2, 2)[..20]);
        assert_eq!("10011010010.10010001", &f32toa_string(1.2345678901234567890e3, 2)[..20]);

        // negative
        assert_eq!("-1.001111000000110010", &f32toa_string(-1.2345678901234567890e0, 2)[..21]);
        assert_eq!("-1100.010110000111111", &f32toa_string(-1.2345678901234567890e1, 2)[..21]);
        assert_eq!("-1111011.011101001111", &f32toa_string(-1.2345678901234567890e2, 2)[..21]);
        assert_eq!("-10011010010.10010001", &f32toa_string(-1.2345678901234567890e3, 2)[..21]);

        // special
        assert_eq!("NaN", &f32toa_string(F32_NAN, 2));
        assert_eq!("Infinity", &f32toa_string(F32_INFINITY, 2));
    }

    #[test]
    fn f32toa_base10_test() {
        // positive
        assert_eq!("0.0", &f32toa_string(0.0, 10));
        assert_eq!("1.0", &f32toa_string(1.0, 10));
        assert_eq!("10.0", &f32toa_string(10.0, 10));
        assert_eq!("1.234567", &f32toa_string(1.2345678901234567890e0, 10)[..8]);
        assert_eq!("12.34567", &f32toa_string(1.2345678901234567890e1, 10)[..8]);
        assert_eq!("123.4567", &f32toa_string(1.2345678901234567890e2, 10)[..8]);
        assert_eq!("1234.567", &f32toa_string(1.2345678901234567890e3, 10)[..8]);

        // negative
        assert_eq!("-1.234567", &f32toa_string(-1.2345678901234567890e0, 10)[..9]);
        assert_eq!("-12.34567", &f32toa_string(-1.2345678901234567890e1, 10)[..9]);
        assert_eq!("-123.4567", &f32toa_string(-1.2345678901234567890e2, 10)[..9]);
        assert_eq!("-1234.567", &f32toa_string(-1.2345678901234567890e3, 10)[..9]);

        // special
        assert_eq!("NaN", &f32toa_string(F32_NAN, 10));
        assert_eq!("Infinity", &f32toa_string(F32_INFINITY, 10));
    }

    #[test]
    fn f32toa_base10_roundtrip_test() {
        for f in F32_DATA.iter() {
            let s = f32toa_string(*f, 10);
            assert_float_relative_eq!(atof32_bytes(s.as_bytes(), 10), *f, 1e-6);
        }
    }

    #[test]
    fn f32toa_basen_roundtrip_test() {
        for f in F32_DATA.iter() {
            for radix in 2..37 {
                // The lower accuracy is due to slight rounding errors of
                // ftoa for the Grisu method with non-10 bases.
                let s = f32toa_string(*f, radix);
                assert_float_relative_eq!(atof32_bytes(s.as_bytes(), radix), *f, 2e-5);
            }
        }
    }

    #[test]
    fn f64toa_base2_test() {
        // positive
        assert_eq!("0.0", &f64toa_string(0.0, 2));
        assert_eq!("1.0", &f64toa_string(1.0, 2));
        assert_eq!("10.0", &f64toa_string(2.0, 2));
        assert_eq!("1.00111100000011001010010000101000110001", &f64toa_string(1.2345678901234567890e0, 2)[..40]);
        assert_eq!("1100.01011000011111100110100110010111101", &f64toa_string(1.2345678901234567890e1, 2)[..40]);
        assert_eq!("1111011.01110100111100000001111111101101", &f64toa_string(1.2345678901234567890e2, 2)[..40]);
        assert_eq!("10011010010.1001000101100001001111110100", &f64toa_string(1.2345678901234567890e3, 2)[..40]);

            // negative
        assert_eq!("-1.00111100000011001010010000101000110001", &f64toa_string(-1.2345678901234567890e0, 2)[..41]);
        assert_eq!("-1100.01011000011111100110100110010111101", &f64toa_string(-1.2345678901234567890e1, 2)[..41]);
        assert_eq!("-1111011.01110100111100000001111111101101", &f64toa_string(-1.2345678901234567890e2, 2)[..41]);
        assert_eq!("-10011010010.1001000101100001001111110100", &f64toa_string(-1.2345678901234567890e3, 2)[..41]);

        // special
        assert_eq!("NaN", &f64toa_string(F64_NAN, 2));
        assert_eq!("Infinity", &f64toa_string(F64_INFINITY, 2));
    }

    #[test]
    fn f64toa_base10_test() {
        // positive
        assert_eq!("0.0", &f64toa_string(0.0, 10));
        assert_eq!("1.0", &f64toa_string(1.0, 10));
        assert_eq!("10.0", &f64toa_string(10.0, 10));
        assert_eq!("1.234567", &f64toa_string(1.2345678901234567890e0, 10)[..8]);
        assert_eq!("12.34567", &f64toa_string(1.2345678901234567890e1, 10)[..8]);
        assert_eq!("123.4567", &f64toa_string(1.2345678901234567890e2, 10)[..8]);
        assert_eq!("1234.567", &f64toa_string(1.2345678901234567890e3, 10)[..8]);

        // negative
        assert_eq!("-1.234567", &f64toa_string(-1.2345678901234567890e0, 10)[..9]);
        assert_eq!("-12.34567", &f64toa_string(-1.2345678901234567890e1, 10)[..9]);
        assert_eq!("-123.4567", &f64toa_string(-1.2345678901234567890e2, 10)[..9]);
        assert_eq!("-1234.567", &f64toa_string(-1.2345678901234567890e3, 10)[..9]);

        // special
        assert_eq!("NaN", &f64toa_string(F64_NAN, 10));
        assert_eq!("Infinity", &f64toa_string(F64_INFINITY, 10));
    }

    #[test]
    fn f64toa_base10_roundtrip_test() {
        for f in F64_DATA.iter() {
            let s = f64toa_string(*f, 10);
            assert_float_relative_eq!(atof64_bytes(s.as_bytes(), 10), *f, 1e-12);
        }
    }

    #[test]
    fn f64toa_basen_roundtrip_test() {
        for f in F64_DATA.iter() {
            for radix in 2..37 {
                // The lower accuracy is due to slight rounding errors of
                // ftoa for the Grisu method with non-10 bases.
                let s = f64toa_string(*f, radix);
                assert_float_relative_eq!(atof64_bytes(s.as_bytes(), radix), *f, 3e-5);
            }
        }
    }
}
