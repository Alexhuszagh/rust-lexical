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
use crate::shared::{debug_assert_digits, truncate_and_round_decimal, write_exponent};
use crate::table::GRISU_POWERS_OF_TEN;
use core::mem;
use lexical_util::digit::digit_to_char_const;
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
    let (ndigits, k) = if float == F::ZERO {
        // SAFETY: safe since `digits.len() == 32`.
        unsafe { index_unchecked_mut!(digits[0]) = b'0' };
        (1, 0)
    } else {
        // SAFETY: safe since `digits.len()` is large enough to always hold enough digits.
        let (ndigits, k) = unsafe { grisu(float, &mut digits) };
        // SAFETY: safe since `ndigits < digits.len()`.
        unsafe { truncate_and_round_decimal(&mut digits, ndigits, k, options) }
    };

    // See if we should write the number in exponent notation.
    let exp = k + ndigits as i32 - 1;
    write_float!(
        FORMAT,
        exp,
        options,
        write_float_scientific,
        write_float_positive_exponent,
        write_float_negative_exponent,
        args => bytes, &mut digits, ndigits, k, options,
    )
}

/// Round digit to normal approximation.
///
/// # Safety
///
/// Safe as long as `ndigits <= digits.len() && ndigits >= 0`.
unsafe fn round_digit(
    digits: &mut [u8],
    ndigits: usize,
    delta: u64,
    mut rem: u64,
    kappa: u64,
    mant: u64,
) {
    debug_assert!((1..=digits.len()).contains(&ndigits));

    while rem < mant
        && delta - rem >= kappa
        && (rem + kappa < mant || mant - rem > rem + kappa - mant)
    {
        unsafe { index_unchecked_mut!(digits[ndigits - 1]) -= 1 };
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

    const TENS: [u64; 20] = [
        10000000000000000000,
        1000000000000000000,
        100000000000000000,
        10000000000000000,
        1000000000000000,
        100000000000000,
        10000000000000,
        1000000000000,
        100000000000,
        10000000000,
        1000000000,
        100000000,
        10000000,
        1000000,
        100000,
        10000,
        1000,
        100,
        10,
        1,
    ];

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
    let mut index = 10;

    while kappa > 0 {
        // SAFETY: safe, TENS.len() == 20.
        let div = unsafe { index_unchecked!(TENS[index]) };
        let digit = part1 / div;
        if digit != 0 || idx != 0 {
            // SAFETY: safe, digits.len() == 32.
            unsafe { index_unchecked_mut!(digits[idx]) = digit_to_char_const(digit as u32, 10) };
            idx += 1;
        }

        part1 -= digit as u64 * div;
        kappa -= 1;
        index += 1;

        let tmp = (part1 << -one.exp) + part2;
        if tmp <= delta {
            k += kappa;
            // SAFETY: safe since `idx > 0 && idx < digits.len()`.
            unsafe { round_digit(digits, idx, delta, tmp, div << -one.exp, wmant) };
            return (idx, k);
        }
    }

    // 10
    // Guaranteed to be safe, TENS has 20 elements.
    let mut index = 18;
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
        // SAFETY: safe, TENS.len() == 20, and `index >= 0 && index <= 18`.
        let ten = unsafe { index_unchecked!(TENS[index]) };
        index -= 1;
        if part2 < delta {
            k += kappa;
            unsafe { round_digit(digits, idx, delta, part2, one.mant, wmant * ten) };
            return (idx, k);
        }
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
    ndigits: usize,
    k: i32,
    options: &Options,
) -> usize {
    debug_assert!(ndigits <= 20);

    // Config options
    let format = NumberFormat::<{ FORMAT }> {};
    assert!(format.is_valid());
    let decimal_point = options.decimal_point();

    // Determine the exact number of digits to write.
    debug_assert_digits(ndigits, options);
    let mut exact_count: usize = ndigits;
    if let Some(min_digits) = options.min_significant_digits() {
        exact_count = min_digits.get().max(exact_count);
    }

    // Write our significant digits
    // SAFETY: safe since both digits and bytes must be >= 1 byte.
    let mut cursor: usize;
    unsafe {
        index_unchecked_mut!(bytes[0] = digits[0]);
        index_unchecked_mut!(bytes[1]) = decimal_point;

        if !format.no_exponent_without_fraction() && ndigits == 1 && options.trim_floats() {
            cursor = 1;
        } else if ndigits == 1 {
            index_unchecked_mut!(bytes[2]) = b'0';
            cursor = 3;
        } else {
            let src = index_unchecked!(digits[1..ndigits]).as_ptr();
            let dst = &mut index_unchecked_mut!(bytes[2..ndigits + 1]);
            copy_nonoverlapping_unchecked!(dst, src, ndigits - 1);
            cursor = ndigits + 1;
        }
    }

    // Adjust the number of digits written, based on the exact number of digits.
    // Cursor is 1 if we trimmed floats, in which case skip this.
    debug_assert!(ndigits <= exact_count);
    if cursor != 1 && ndigits < exact_count {
        let zeros = exact_count - ndigits;
        // SAFETY: safe if bytes is large enough to hold the significant digits.
        unsafe {
            slice_fill_unchecked!(index_unchecked_mut!(bytes[cursor..cursor + zeros]), b'0');
        }
    }

    // Now, write our scientific notation.
    // SAFETY: safe since bytes must be large enough to store all digits.
    let exp = k + ndigits as i32 - 1;
    unsafe { write_exponent::<FORMAT>(bytes, &mut cursor, exp, options.exponent()) };

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
    ndigits: usize,
    k: i32,
    options: &Options,
) -> usize {
    debug_assert!(ndigits <= 20);
    debug_assert!(k + ndigits as i32 - 1 < 0);

    // Config options
    let decimal_point = options.decimal_point();

    let exp = k + ndigits as i32 - 1;
    let exp = exp.wrapping_neg() as usize;

    // Write our 0 digits.
    // SAFETY: safe if `bytes.len() < BUFFER_SIZE - 2`.
    unsafe {
        index_unchecked_mut!(bytes[0]) = b'0';
        index_unchecked_mut!(bytes[1]) = decimal_point;
        let digits = &mut index_unchecked_mut!(bytes[2..exp + 1]);
        slice_fill_unchecked!(digits, b'0');
    }
    let mut cursor = exp + 1;

    // Write out significant digits.
    unsafe {
        let src = digits.as_ptr();
        let dst = &mut index_unchecked_mut!(bytes[cursor..cursor + ndigits]);
        copy_nonoverlapping_unchecked!(dst, src, ndigits);
        cursor += ndigits;
    }

    // Determine the exact number of digits to write.
    debug_assert_digits(ndigits, options);
    let mut exact_count: usize = ndigits;
    if let Some(min_digits) = options.min_significant_digits() {
        exact_count = min_digits.get().max(exact_count);
    }

    // Adjust the number of digits written, based on the exact number of digits.
    if ndigits < exact_count {
        let zeros = exact_count - ndigits;
        unsafe {
            slice_fill_unchecked!(index_unchecked_mut!(bytes[cursor..cursor + zeros]), b'0');
        }
    } else if ndigits > exact_count {
        cursor -= ndigits - exact_count;
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
    mut ndigits: usize,
    k: i32,
    options: &Options,
) -> usize {
    debug_assert!(ndigits <= 20);
    debug_assert!(k + ndigits as i32 > 0);

    // Config options
    let decimal_point = options.decimal_point();

    // Now need to write our significant digits.
    let exp = (k + ndigits as i32 - 1) as usize;
    let leading_digits = exp + 1;
    let mut cursor: usize;
    if leading_digits >= ndigits {
        // We have more leading digits than digits we wrote: can write
        // any additional digits, and then just write the remaining ones.
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let src = digits.as_ptr();
            let dst = &mut index_unchecked_mut!(bytes[..ndigits]);
            copy_nonoverlapping_unchecked!(dst, src, ndigits);
            let digits = &mut index_unchecked_mut!(bytes[ndigits..leading_digits]);
            slice_fill_unchecked!(digits, b'0');
        }
        cursor = leading_digits;
        unsafe { index_unchecked_mut!(bytes[cursor]) = decimal_point };
        cursor += 1;
        unsafe { index_unchecked_mut!(bytes[cursor]) = b'0' };
        cursor += 1;
        ndigits += 1;
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
            let src = index_unchecked!(digits[leading_digits..ndigits]).as_ptr();
            let dst = &mut index_unchecked_mut!(bytes[leading_digits + 1..ndigits + 1]);
            copy_nonoverlapping_unchecked!(dst, src, ndigits - leading_digits);
        }

        cursor = ndigits + 1;
    }

    // Determine the exact number of digits to write.
    // Note: we might have written an extra digit for leading digits.
    debug_assert_digits(ndigits - 1, options);
    let mut exact_count: usize = ndigits;
    if let Some(min_digits) = options.min_significant_digits() {
        exact_count = min_digits.get().max(exact_count);
    }

    // Change the number of digits written, if we need to add more or trim digits.
    if options.trim_floats() && exact_count == ndigits {
        // SAFETY: safe, cursor must be at least 3.
        if unsafe { index_unchecked!(bytes[cursor - 2]) == decimal_point } {
            // Need to trim floats from trailing zeros, and we have only a decimal
            cursor -= 2;
        }
    } else if exact_count > ndigits {
        // Check if we need to write more trailing digits.
        let zeros = exact_count - ndigits;
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let digits = &mut index_unchecked_mut!(bytes[cursor..cursor + zeros]);
            slice_fill_unchecked!(digits, b'0');
        }
        cursor += zeros;
    }

    cursor
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
        exp: x.exp + y.exp + u64::BITS as i32,
    }
}

// CACHED POWERS

/// Find cached power of 10 from the exponent.
///
/// # Safety
///
/// Safe as long as exp is within the range [-1075, ]
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
        unsafe { index_unchecked!(GRISU_POWERS_OF_TEN[index]) }
    }
}

macro_rules! grisu_impl {
    ($($t:ident)*) => ($(
        impl GrisuFloat for $t {}
    )*);
}

grisu_impl! { f32 f64 }
