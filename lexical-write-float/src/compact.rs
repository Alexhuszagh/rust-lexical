//! Implementation of the Grisu algorithm.
//!
//! These routines are adapted from Andrea Samoljuk's `fpconv` library,
//! which is available [here](https://github.com/night-shift/fpconv).
//!
//! In addition to porting from C to Rust, this also adds format
//! precision control and other features.
//!
//! This code is therefore available under a permissive
//! Boost Software License, as is the original.
//!
//! A few modifications have been made to improve readability,
//! minimize binary size, and add additional features.
//!
//! 1. The exponent is inferred, rather than explicitly store.
//! 2. The mantissas are stored in hex, rather than decimal.
//! 3. Forcing and disabling exponent notation is now supported.
//! 4. Controlling the maximum and minimum number of significant digits is supported.
//! 5. Support for trimming floats (".0") is also included.

#![cfg(feature = "compact")]
#![doc(hidden)]

use crate::float::{ExtendedFloat80, RawFloat};
use crate::options::Options;
use crate::shared;
use crate::table::GRISU_POWERS_OF_TEN;
use core::mem;
use lexical_util::algorithm::rtrim_char_count;
#[cfg(feature = "f16")]
use lexical_util::bf16::bf16;
use lexical_util::digit::digit_to_char_const;
#[cfg(feature = "f16")]
use lexical_util::f16::f16;
use lexical_util::format::NumberFormat;
use lexical_util::num::{AsPrimitive, Float};

/// Compact float-to-string algorithm for decimal strings.
///
/// This is based on "Printing Floating-Point Numbers Quickly and Accurately
/// with Integers", by Florian Loitsch, available online at:
/// <https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf>.
///
/// This assumes the float is:
///     1). Non-special (NaN or Infinite).
///     2). Non-negative.
///
/// # Safety
///
/// Safe as long as the float isn't special (NaN or Infinity), and `bytes`
/// is large enough to hold the significant digits.
pub unsafe fn write_float<F: RawFloat, const FORMAT: u128>(
    float: F,
    bytes: &mut [u8],
    options: &Options,
) -> usize {
    // PRECONDITIONS

    // Assert no special cases remain, no negative numbers,
    // and a valid format.
    let format = NumberFormat::<{ FORMAT }> {};
    assert!(format.is_valid());
    debug_assert!(!float.is_special());
    debug_assert!(float >= F::ZERO);

    // Write our mantissa digits to a temporary buffer.
    let digits: mem::MaybeUninit<[u8; 32]> = mem::MaybeUninit::uninit();
    // SAFETY: safe, since we never read bytes that weren't written.
    let mut digits = unsafe { digits.assume_init() };
    let (digit_count, kappa, carried) = if float == F::ZERO {
        // SAFETY: safe since `digits.len() == 32`.
        unsafe { index_unchecked_mut!(digits[0]) = b'0' };
        (1, 0, false)
    } else {
        // SAFETY: safe since `digits.len()` is large enough to always hold
        // the generated digits, which is always <= 18.
        unsafe {
            let (start, k) = grisu(float, &mut digits);
            let (end, carried) = shared::truncate_and_round_decimal(&mut digits, start, options);
            (end, k + start as i32 - end as i32, carried)
        }
    };

    let sci_exp = kappa + digit_count as i32 - 1 + carried as i32;
    write_float!(
        FORMAT,
        sci_exp,
        options,
        write_float_scientific,
        write_float_positive_exponent,
        write_float_negative_exponent,
        args => bytes, &mut digits, digit_count, sci_exp, options,
    )
}

/// Write float to string in scientific notation.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of digits
/// and the scientific notation's exponent digits.
#[allow(clippy::comparison_chain)]
pub unsafe fn write_float_scientific<const FORMAT: u128>(
    bytes: &mut [u8],
    digits: &mut [u8],
    digit_count: usize,
    sci_exp: i32,
    options: &Options,
) -> usize {
    debug_assert!(rtrim_char_count(&digits[..digit_count], b'0') == 0 || digit_count == 1);
    debug_assert!(digit_count <= 20);

    // Config options
    let format = NumberFormat::<{ FORMAT }> {};
    assert!(format.is_valid());
    let decimal_point = options.decimal_point();

    // Determine the exact number of digits to write.
    let exact_count = shared::min_exact_digits(digit_count, options);

    // Write our significant digits
    let mut cursor: usize;
    unsafe {
        // SAFETY: safe since `digits.len() == 32 && bytes.len() >= 2`.
        index_unchecked_mut!(bytes[0] = digits[0]);
        index_unchecked_mut!(bytes[1]) = decimal_point;

        // SAFETY: safe if bytes is large enough to store all significant digits.
        if !format.no_exponent_without_fraction() && digit_count == 1 && options.trim_floats() {
            // No more digits and need to trim floats.
            cursor = 1;
        } else if digit_count < exact_count {
            // Write our significant digits.
            let src = index_unchecked!(digits[1..digit_count]).as_ptr();
            let dst = &mut index_unchecked_mut!(bytes[2..digit_count + 1]);
            copy_nonoverlapping_unchecked!(dst, src, digit_count - 1);
            cursor = digit_count + 1;

            // Adjust the number of digits written, by appending zeros.
            let zeros = exact_count - digit_count;
            slice_fill_unchecked!(index_unchecked_mut!(bytes[cursor..cursor + zeros]), b'0');
            cursor += zeros;
        } else if digit_count == 1 {
            // Write a single, trailing 0.
            index_unchecked_mut!(bytes[2]) = b'0';
            cursor = 3;
        } else {
            // Write our significant digits.
            let src = index_unchecked!(digits[1..digit_count]).as_ptr();
            let dst = &mut index_unchecked_mut!(bytes[2..digit_count + 1]);
            copy_nonoverlapping_unchecked!(dst, src, digit_count - 1);
            cursor = digit_count + 1;
        }
    }

    // Now, write our scientific notation.
    // SAFETY: safe since bytes must be large enough to store the largest float.
    unsafe { shared::write_exponent::<FORMAT>(bytes, &mut cursor, sci_exp, options.exponent()) };

    cursor
}

/// Write negative float to string without scientific notation.
/// Has a negative exponent (shift right) and no scientific notation.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of
/// significant digits and the leading zeros.
#[allow(clippy::comparison_chain)]
pub unsafe fn write_float_negative_exponent<const FORMAT: u128>(
    bytes: &mut [u8],
    digits: &mut [u8],
    digit_count: usize,
    sci_exp: i32,
    options: &Options,
) -> usize {
    debug_assert!(rtrim_char_count(&digits[..digit_count], b'0') == 0);
    debug_assert!(digit_count <= 20);
    debug_assert!(sci_exp < 0);

    // Config options
    let decimal_point = options.decimal_point();
    let sci_exp = sci_exp.wrapping_neg() as usize;

    // Write our 0 digits. Note that we cannot have carried, since we previously
    // adjusted for carrying and rounding before.
    // SAFETY: safe if `bytes.len() < BUFFER_SIZE - 2`.
    unsafe {
        index_unchecked_mut!(bytes[0]) = b'0';
        index_unchecked_mut!(bytes[1]) = decimal_point;
        let digits = &mut index_unchecked_mut!(bytes[2..sci_exp + 1]);
        slice_fill_unchecked!(digits, b'0');
    }
    let mut cursor = sci_exp + 1;

    // Write out significant digits.
    // SAFETY: safe if the buffer is large enough to hold all the significant digits.
    unsafe {
        let src = digits.as_ptr();
        let dst = &mut index_unchecked_mut!(bytes[cursor..cursor + digit_count]);
        copy_nonoverlapping_unchecked!(dst, src, digit_count);
        cursor += digit_count;
    }

    // Determine the exact number of digits to write.
    let exact_count = shared::min_exact_digits(digit_count, options);

    // Adjust the number of digits written, based on the exact number of digits.
    if digit_count < exact_count {
        let zeros = exact_count - digit_count;
        // SAFETY: safe if bytes is large enough to hold the significant digits.
        unsafe {
            slice_fill_unchecked!(index_unchecked_mut!(bytes[cursor..cursor + zeros]), b'0');
        }
        cursor += zeros;
    }

    cursor
}

/// Write positive float to string without scientific notation.
/// Has a positive exponent (shift left) and no scientific notation.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of
/// significant digits and the (optional) trailing zeros.
pub unsafe fn write_float_positive_exponent<const FORMAT: u128>(
    bytes: &mut [u8],
    digits: &mut [u8],
    mut digit_count: usize,
    sci_exp: i32,
    options: &Options,
) -> usize {
    debug_assert!(rtrim_char_count(&digits[..digit_count], b'0') == 0 || digit_count == 1);
    debug_assert!(digit_count <= 20);
    debug_assert!(sci_exp >= 0);

    // Config options
    let decimal_point = options.decimal_point();

    // Now need to write our significant digits.
    let leading_digits = sci_exp as usize + 1;
    let mut cursor: usize;
    let mut trimmed = false;
    if leading_digits >= digit_count {
        // We have more leading digits than digits we wrote: can write
        // any additional digits, and then just write the remaining ones.
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let src = digits.as_ptr();
            let dst = &mut index_unchecked_mut!(bytes[..digit_count]);
            copy_nonoverlapping_unchecked!(dst, src, digit_count);
            let digits = &mut index_unchecked_mut!(bytes[digit_count..leading_digits]);
            slice_fill_unchecked!(digits, b'0');
        }
        cursor = leading_digits;
        digit_count = leading_digits;
        // Only write decimal point if we're not trimming floats.
        if !options.trim_floats() {
            // SAFETY: safe if `cursor + 2 <= bytes.len()`.
            unsafe { index_unchecked_mut!(bytes[cursor]) = decimal_point };
            cursor += 1;
            unsafe { index_unchecked_mut!(bytes[cursor]) = b'0' };
            cursor += 1;
            digit_count += 1;
        } else {
            trimmed = true;
        }
    } else {
        // We have less leading digits than digits we wrote.

        // Write the digits before the decimal point.
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let src = digits.as_ptr();
            let dst = &mut index_unchecked_mut!(bytes[..leading_digits]);
            copy_nonoverlapping_unchecked!(dst, src, leading_digits);
            index_unchecked_mut!(bytes[leading_digits]) = decimal_point;
        }

        // Write the digits after the decimal point.
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let src = index_unchecked!(digits[leading_digits..digit_count]).as_ptr();
            let dst = &mut index_unchecked_mut!(bytes[leading_digits + 1..digit_count + 1]);
            copy_nonoverlapping_unchecked!(dst, src, digit_count - leading_digits);
        }

        cursor = digit_count + 1;
    }

    // Determine the exact number of digits to write.
    let exact_count = shared::min_exact_digits(digit_count, options);

    // Change the number of digits written, if we need to add more or trim digits.
    if !trimmed && exact_count > digit_count {
        // Check if we need to write more trailing digits.
        let zeros = exact_count - digit_count;
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let digits = &mut index_unchecked_mut!(bytes[cursor..cursor + zeros]);
            slice_fill_unchecked!(digits, b'0');
        }
        cursor += zeros;
    }

    cursor
}

// ALGORITHM
// ---------

/// Round digit to normal approximation.
///
/// # Safety
///
/// Safe as long as `digit_count <= digits.len() && digit_count > 0`.
unsafe fn round_digit(
    digits: &mut [u8],
    digit_count: usize,
    delta: u64,
    mut rem: u64,
    kappa: u64,
    mant: u64,
) {
    debug_assert!((1..=digits.len()).contains(&digit_count));

    while rem < mant
        && delta - rem >= kappa
        && (rem + kappa < mant || mant - rem > rem + kappa - mant)
    {
        // SAFETY: safe if `digit_count > 0`.
        unsafe { index_unchecked_mut!(digits[digit_count - 1]) -= 1 };
        rem += kappa;
    }
}

/// Generate digits from upper and lower range on rounding of number.
///
/// # Safety
///
/// Safe as long as the extended float does not represent a 0.
pub unsafe fn generate_digits(
    fp: &ExtendedFloat80,
    upper: &ExtendedFloat80,
    lower: &ExtendedFloat80,
    digits: &mut [u8],
    mut k: i32,
) -> (usize, i32) {
    debug_assert!(fp.mant != 0);

    let wmant = upper.mant - fp.mant;
    let mut delta = upper.mant - lower.mant;

    let one = ExtendedFloat80 {
        mant: 1 << -upper.exp,
        exp: upper.exp,
    };

    let mut part1 = upper.mant >> -one.exp;
    let mut part2 = upper.mant & (one.mant - 1);

    let mut idx: usize = 0;
    let mut kappa: i32 = 10;
    let mut div = 1000000000;

    while kappa > 0 {
        let digit = part1 / div;
        if digit != 0 || idx != 0 {
            // SAFETY: safe, digits.len() == 32.
            unsafe { index_unchecked_mut!(digits[idx]) = digit_to_char_const(digit as u32, 10) };
            idx += 1;
        }

        part1 -= digit as u64 * div;
        kappa -= 1;

        let tmp = (part1 << -one.exp) + part2;
        if tmp <= delta {
            k += kappa;
            // SAFETY: safe since `idx > 0 && idx < digits.len()`.
            unsafe { round_digit(digits, idx, delta, tmp, div << -one.exp, wmant) };
            return (idx, k);
        }
        div /= 10;
    }

    // 10
    // Guaranteed to be safe, TENS has 20 elements.
    let mut ten = 10;
    loop {
        part2 *= 10;
        delta *= 10;
        kappa -= 1;

        let digit = part2 >> -one.exp;
        if digit != 0 || idx != 0 {
            // SAFETY: safe, digits.len() == 32.
            // In practice, this can't exceed 18, however, we have extra digits
            // **just** in case, since we write technically up to 29 here
            // before we underflow TENS.
            unsafe { index_unchecked_mut!(digits[idx]) = digit_to_char_const(digit as u32, 10) };
            idx += 1;
        }

        part2 &= one.mant - 1;
        if part2 < delta {
            k += kappa;
            // SAFETY: safe since `idx < digits.len() && idx > 0`.
            unsafe { round_digit(digits, idx, delta, part2, one.mant, wmant * ten) };
            return (idx, k);
        }
        ten *= 10;
    }
}

/// Calculate the upper and lower boundaries, then invoke the float formatter.
///
/// # Preconditions
///
/// `float` must not be 0, because this fails with the Grisu algorithm.
///
/// # Safety
///
/// Safe as long as float is not 0.
pub unsafe fn grisu<F: Float>(float: F, digits: &mut [u8]) -> (usize, i32) {
    debug_assert!(float != F::ZERO);

    let mut w = from_float(float);

    let (lower, upper) = normalized_boundaries::<F>(&w);
    normalize(&mut w);
    // SAFETY: safe since upper.exp must be in the valid binary range.
    let (cp, ki) = unsafe { cached_grisu_power(upper.exp) };

    let w = mul(&w, &cp);
    let mut upper = mul(&upper, &cp);
    let mut lower = mul(&lower, &cp);

    lower.mant += 1;
    upper.mant -= 1;

    let k = -ki;

    // SAFETY: safe since generate_digits can only generate 18 digits
    unsafe { generate_digits(&w, &upper, &lower, digits, k) }
}

// EXTENDED FLOAT

/// Create extended float from native float.
pub fn from_float<F: Float>(float: F) -> ExtendedFloat80 {
    ExtendedFloat80 {
        mant: float.mantissa().as_u64(),
        exp: float.exponent(),
    }
}

/// Normalize float-point number.
///
/// Shift the mantissa so the number of leading zeros is 0, or the value
/// itself is 0.
///
/// Get the number of bytes shifted.
pub fn normalize(fp: &mut ExtendedFloat80) {
    // Note:
    // Using the ctlz intrinsic via leading_zeros is way faster (~10x)
    // than shifting 1-bit at a time, via while loop, and also way
    // faster (~2x) than an unrolled loop that checks at 32, 16, 4,
    // 2, and 1 bit.
    //
    // Using a modulus of pow2 (which will get optimized to a bitwise
    // and with 0x3F or faster) is slightly slower than an if/then,
    // however, removing the if/then will likely optimize more branched
    // code as it removes conditional logic.

    // Calculate the number of leading zeros, and then zero-out
    // any overflowing bits, to avoid shl overflow when self.mant == 0.
    if fp.mant != 0 {
        let shift = fp.mant.leading_zeros() as i32;
        fp.mant <<= shift;
        fp.exp -= shift;
    }
}

/// Get normalized boundaries for float.
pub fn normalized_boundaries<F: Float>(fp: &ExtendedFloat80) -> (ExtendedFloat80, ExtendedFloat80) {
    let mut upper = ExtendedFloat80 {
        mant: (fp.mant << 1) + 1,
        exp: fp.exp - 1,
    };
    normalize(&mut upper);

    // Use a boolean hack to get 2 if they're equal, else 1, without
    // any branching.
    let is_hidden = fp.mant == F::HIDDEN_BIT_MASK.as_u64();
    let l_shift: i32 = is_hidden as i32 + 1;

    let mut lower = ExtendedFloat80 {
        mant: (fp.mant << l_shift) - 1,
        exp: fp.exp - l_shift,
    };
    lower.mant <<= lower.exp - upper.exp;
    lower.exp = upper.exp;

    (lower, upper)
}

/// Multiply two normalized extended-precision floats, as if by `a*b`.
///
/// The precision is maximal when the numbers are normalized, however,
/// decent precision will occur as long as both values have high bits
/// set. The result is not normalized.
///
/// Algorithm:
///     1. Non-signed multiplication of mantissas (requires 2x as many bits as input).
///     2. Normalization of the result (not done here).
///     3. Addition of exponents.
pub fn mul(x: &ExtendedFloat80, y: &ExtendedFloat80) -> ExtendedFloat80 {
    // Logic check, values must be decently normalized prior to multiplication.
    debug_assert!(x.mant >> 32 != 0);
    debug_assert!(y.mant >> 32 != 0);

    // Extract high-and-low masks.
    const LOMASK: u64 = u32::MAX as u64;
    let x1 = x.mant >> 32;
    let x0 = x.mant & LOMASK;
    let y1 = y.mant >> 32;
    let y0 = y.mant & LOMASK;

    // Get our products
    let x1_y0 = x1 * y0;
    let x0_y1 = x0 * y1;
    let x0_y0 = x0 * y0;
    let x1_y1 = x1 * y1;

    let mut tmp = (x1_y0 & LOMASK) + (x0_y1 & LOMASK) + (x0_y0 >> 32);
    // round up
    tmp += 1 << (32 - 1);

    ExtendedFloat80 {
        mant: x1_y1 + (x1_y0 >> 32) + (x0_y1 >> 32) + (tmp >> 32),
        exp: x.exp + y.exp + 64,
    }
}

// CACHED POWERS

/// Find cached power of 10 from the exponent.
///
/// # Safety
///
/// Safe as long as exp is within the range [-1140, 1089]
unsafe fn cached_grisu_power(exp: i32) -> (ExtendedFloat80, i32) {
    // Make the bounds 64 + 1 larger, since those will still work,
    // but the exp can be biased within that range.
    debug_assert!(((-1075 - 64 - 1)..=(1024 + 64 + 1)).contains(&exp));

    // FLOATING POINT CONSTANTS
    const ONE_LOG_TEN: f64 = 0.30102999566398114;
    const NPOWERS: i32 = 87;
    const FIRSTPOWER: i32 = -348; // 10 ^ -348
    const STEPPOWERS: i32 = 8;
    const EXPMAX: i32 = -32;
    const EXPMIN: i32 = -60;

    let approx = -((exp + NPOWERS) as f64) * ONE_LOG_TEN;
    let approx = approx as i32;
    let mut idx = ((approx - FIRSTPOWER) / STEPPOWERS) as usize;

    loop {
        // SAFETY: safe as long as the original exponent was in range.
        let mant = unsafe { f64::grisu_power(idx) };
        let decexp = fast_decimal_power(idx);
        let binexp = fast_binary_power(decexp);
        let current = exp + binexp + 64;
        if current < EXPMIN {
            idx += 1;
            continue;
        }

        if current > EXPMAX {
            idx -= 1;
            continue;
        }

        let k = FIRSTPOWER + idx as i32 * STEPPOWERS;
        let power = ExtendedFloat80 {
            mant,
            exp: binexp,
        };
        return (power, k);
    }
}

/// Calculate a base 2 exponent from a decimal exponent.
/// This uses a pre-computed integer approximation for
/// log2(10), where 217706 / 2^16 is accurate for the
/// entire range of non-finite decimal exponents.
fn fast_binary_power(q: i32) -> i32 {
    (q.wrapping_mul(152_170 + 65536) >> 16) - 63
}

/// Calculate the fast decimal power from the index.
fn fast_decimal_power(index: usize) -> i32 {
    index as i32 * 8 - 348
}

// GRISU FLOAT
// -----------

/// Trait with specialized methods for the Grisu algorithm.
pub trait GrisuFloat: Float {
    /// Get the pre-computed Grisu power from the index.
    ///
    /// # Safety
    ///
    /// Safe as long as `index < GRISU_POWERS_OF_TEN.len()`.
    #[inline(always)]
    unsafe fn grisu_power(index: usize) -> u64 {
        debug_assert!(index <= GRISU_POWERS_OF_TEN.len());
        unsafe { index_unchecked!(GRISU_POWERS_OF_TEN[index]) }
    }
}

macro_rules! grisu_impl {
    ($($t:ident)*) => ($(
        impl GrisuFloat for $t {}
    )*);
}

grisu_impl! { f32 f64 }

#[cfg(feature = "f16")]
macro_rules! grisu_unimpl {
    ($($t:ident)*) => ($(
        impl GrisuFloat for $t {
            #[inline(always)]
            unsafe fn grisu_power(_: usize) -> u64 {
                unimplemented!()
            }
        }
    )*);
}

#[cfg(feature = "f16")]
grisu_unimpl! { bf16 f16 }
