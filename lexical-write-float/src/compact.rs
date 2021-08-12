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

use crate::options::{Options, RoundMode};
use crate::shared::{round_up, write_exponent};
use core::mem;
use lexical_util::digit::digit_to_char_const;
use lexical_util::extended_float::ExtendedFloat;
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
pub unsafe fn write_float<F: Float, const FORMAT: u128>(
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
        unsafe { truncate_and_round(&mut digits, ndigits, k, options) }
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

/// Round the number of digits based on the maximum digits.
///
/// # Safety
///
/// Safe as long as `ndigits <= digits.len()`.
pub unsafe fn truncate_and_round(
    digits: &mut [u8],
    ndigits: usize,
    k: i32,
    options: &Options,
) -> (usize, i32) {
    debug_assert!(ndigits <= digits.len());

    let max_digits = if let Some(digits) = options.max_significant_digits() {
        digits.get()
    } else {
        return (ndigits, k);
    };
    if max_digits >= ndigits {
        return (ndigits, k);
    }

    // Need to adjust `k`, since we're shortening the digits in the input.
    let shift = ndigits - max_digits;
    let k = k + shift as i32;
    if options.round_mode() == RoundMode::Truncate {
        // Don't round input, just shorten number of digits emitted.
        return (max_digits, k);
    }

    // We need to round-nearest, tie-even, so we need to handle
    // the truncation **here**. If the representation is above
    // halfway at all, we need to round up, even if 1 digit.

    // Get the last non-truncated digit, and the remaining ones.
    let truncated = unsafe { index_unchecked!(digits[max_digits]) };
    let digits = if truncated < b'5' {
        // Just truncate, going to round-down anyway.
        max_digits
    } else if truncated > b'5' {
        // Round-up always.
        // SAFETY: safe if `ndigits <= digits.len()`, because `max_digits < ndigits`.
        unsafe { round_up(digits, max_digits, 10) }
    } else {
        // Have a near-halfway case, resolve it.
        // SAFETY: safe if `ndigits < digits.len()`.
        let to_round = unsafe { &index_unchecked!(digits[max_digits - 1..ndigits]) };
        let is_odd = unsafe { index_unchecked!(to_round[0]) % 2 == 1 };
        let is_above = unsafe { index_unchecked!(to_round[2..]).iter().any(|&x| x != b'0') };
        if is_odd || is_above {
            // SAFETY: safe if `ndigits <= digits.len()`, because `max_digits < ndigits`.
            unsafe { round_up(digits, max_digits, 10) }
        } else {
            max_digits
        }
    };

    (digits, k)
}

/// Write float to string in scientific notation.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of digits
/// and the scientific notation's exponent digits.
pub unsafe fn write_float_scientific<const FORMAT: u128>(
    bytes: &mut [u8],
    digits: &mut [u8],
    ndigits: usize,
    k: i32,
    options: &Options,
) -> usize {
    debug_assert!(ndigits <= 20);

    // Config options
    let decimal_point = options.decimal_point();

    // Determine the exact number of digits to write.
    let mut exact_count: usize = ndigits;
    if let Some(max_digits) = options.max_significant_digits() {
        exact_count = max_digits.get().min(ndigits);
    }
    if let Some(min_digits) = options.min_significant_digits() {
        exact_count = min_digits.get().max(exact_count);
    }

    // Write our significant digits
    // SAFETY: safe since both digits and bytes must be >= 1 byte.
    let mut cursor: usize;
    unsafe {
        index_unchecked_mut!(bytes[0] = digits[0]);
        index_unchecked_mut!(bytes[1]) = decimal_point;

        if ndigits == 1 && options.trim_floats() {
            cursor = 1;
        } else if ndigits == 1 {
            index_unchecked_mut!(bytes[2]) = b'0';
            cursor = 2;
        } else {
            let src = index_unchecked!(digits[1..ndigits]).as_ptr();
            let dst = &mut index_unchecked_mut!(bytes[2..ndigits + 1]);
            copy_nonoverlapping_unchecked!(dst, src, ndigits - 1);
            cursor = ndigits + 1;
        }
    }

    // Adjust the number of digits written, based on the exact number of digits.
    if ndigits < exact_count {
        let zeros = exact_count - ndigits;
        unsafe {
            slice_fill_unchecked!(&mut index_unchecked_mut!(bytes[cursor..cursor + zeros]), b'0');
        }
    } else if ndigits > exact_count {
        cursor -= ndigits - exact_count;
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
    // SAFETY: must be safe since since `bytes.len() < BUFFER_SIZE - 2`.
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
    let mut exact_count: usize = ndigits;
    if let Some(max_digits) = options.max_significant_digits() {
        exact_count = max_digits.get().min(ndigits);
    }
    if let Some(min_digits) = options.min_significant_digits() {
        exact_count = min_digits.get().max(exact_count);
    }

    // Adjust the number of digits written, based on the exact number of digits.
    if ndigits < exact_count {
        let zeros = exact_count - ndigits;
        unsafe {
            slice_fill_unchecked!(&mut index_unchecked_mut!(bytes[cursor..cursor + zeros]), b'0');
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
    debug_assert!(k + ndigits as i32 - 1 >= 0);

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
    let mut exact_count: usize = ndigits;
    if let Some(max_digits) = options.max_significant_digits() {
        exact_count = max_digits.get().min(ndigits);
    }
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

/// Alias with ~80 bits of precision, 64 for the mantissa and 16 for exponent.
type ExtendedFloat80 = ExtendedFloat<u64>;

/// Create extended float from native float.
pub fn from_float<F: Float>(float: F) -> ExtendedFloat80 {
    ExtendedFloat {
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
    let mut upper = ExtendedFloat {
        mant: (fp.mant << 1) + 1,
        exp: fp.exp - 1,
    };
    normalize(&mut upper);

    // Use a boolean hack to get 2 if they're equal, else 1, without
    // any branching.
    let is_hidden = fp.mant == F::HIDDEN_BIT_MASK.as_u64();
    let l_shift: i32 = is_hidden as i32 + 1;

    let mut lower = ExtendedFloat {
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

    ExtendedFloat {
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
        let mant = unsafe { index_unchecked!(GRISU_POWERS_OF_TEN[idx]) };
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

/// Cached powers of ten as specified by the Grisu algorithm.
///
/// Cached powers of 10^k, calculated as if by:
/// `ceil((alpha-e+63) * ONE_LOG_TEN);`
///
/// The estimation of the exponents can be trivially shown to be true,
/// using the following Python code:
///
/// ```text
/// import math
///
/// def power(x):
///     '''Calculate the binary power from the decimal one.'''
///     return ((x * (152_170 + 65536)) >> 16) - 63
///
/// def power_data(decimal, binary):
///     '''Calculate binary power and get useful data.'''
///
///     binary_calc = power(decimal)
///     return (decimal, binary, binary_calc, binary - binary_calc)
///
/// def run():
///     '''Run our exponent estimation over the entire input.'''
///
///     for index, (mant, b) in enumerate(GRISU_POWERS_OF_TEN):
///         e = index * 8 - 348
///         # Check our decimal exponent approximation is valid.
///         try:
///             f = mant * 2.0**b
///             if f != 0 and math.isfinite(f):
///                 assert math.log10(f) == e
///         except OverflowError:
///             pass
///         print(power_data(e, b))
///
/// GRISU_POWERS_OF_TEN = [
///     (18054884314459144840, -1220),
///     (13451937075301367670, -1193),
///     (10022474136428063862, -1166),
///     (14934650266808366570, -1140),
///     (11127181549972568877, -1113),
///     (16580792590934885855, -1087),
///     (12353653155963782858, -1060),
///     (18408377700990114895, -1034),
///     (13715310171984221708, -1007),
///     (10218702384817765436, -980),
///     (15227053142812498563, -954),
///     (11345038669416679861, -927),
///     (16905424996341287883, -901),
///     (12595523146049147757, -874),
///     (9384396036005875287, -847),
///     (13983839803942852151, -821),
///     (10418772551374772303, -794),
///     (15525180923007089351, -768),
///     (11567161174868858868, -741),
///     (17236413322193710309, -715),
///     (12842128665889583758, -688),
///     (9568131466127621947, -661),
///     (14257626930069360058, -635),
///     (10622759856335341974, -608),
///     (15829145694278690180, -582),
///     (11793632577567316726, -555),
///     (17573882009934360870, -529),
///     (13093562431584567480, -502),
///     (9755464219737475723, -475),
///     (14536774485912137811, -449),
///     (10830740992659433045, -422),
///     (16139061738043178685, -396),
///     (12024538023802026127, -369),
///     (17917957937422433684, -343),
///     (13349918974505688015, -316),
///     (9946464728195732843, -289),
///     (14821387422376473014, -263),
///     (11042794154864902060, -236),
///     (16455045573212060422, -210),
///     (12259964326927110867, -183),
///     (18268770466636286478, -157),
///     (13611294676837538539, -130),
///     (10141204801825835212, -103),
///     (15111572745182864684, -77),
///     (11258999068426240000, -50),
///     (16777216000000000000, -24),
///     (12500000000000000000, 3),
///     (9313225746154785156, 30),
///     (13877787807814456755, 56),
///     (10339757656912845936, 83),
///     (15407439555097886824, 109),
///     (11479437019748901445, 136),
///     (17105694144590052135, 162),
///     (12744735289059618216, 189),
///     (9495567745759798747, 216),
///     (14149498560666738074, 242),
///     (10542197943230523224, 269),
///     (15709099088952724970, 295),
///     (11704190886730495818, 322),
///     (17440603504673385349, 348),
///     (12994262207056124023, 375),
///     (9681479787123295682, 402),
///     (14426529090290212157, 428),
///     (10748601772107342003, 455),
///     (16016664761464807395, 481),
///     (11933345169920330789, 508),
///     (17782069995880619868, 534),
///     (13248674568444952270, 561),
///     (9871031767461413346, 588),
///     (14708983551653345445, 614),
///     (10959046745042015199, 641),
///     (16330252207878254650, 667),
///     (12166986024289022870, 694),
///     (18130221999122236476, 720),
///     (13508068024458167312, 747),
///     (10064294952495520794, 774),
///     (14996968138956309548, 800),
///     (11173611982879273257, 827),
///     (16649979327439178909, 853),
///     (12405201291620119593, 880),
///     (9242595204427927429, 907),
///     (13772540099066387757, 933),
///     (10261342003245940623, 960),
///     (15290591125556738113, 986),
///     (11392378155556871081, 1013),
///     (16975966327722178521, 1039),
///     (12648080533535911531, 1066),
/// ]
///
/// # Expected Output:
/// #   (-348, -1220, -1220, 0)
/// #   (-340, -1193, -1193, 0)
/// #   (-332, -1166, -1166, 0)
/// #   (-324, -1140, -1140, 0)
/// #   (-316, -1113, -1113, 0)
/// #   (-308, -1087, -1087, 0)
/// #   (-300, -1060, -1060, 0)
/// #   (-292, -1034, -1034, 0)
/// #   (-284, -1007, -1007, 0)
/// #   (-276, -980, -980, 0)
/// #   (-268, -954, -954, 0)
/// #   (-260, -927, -927, 0)
/// #   (-252, -901, -901, 0)
/// #   (-244, -874, -874, 0)
/// #   (-236, -847, -847, 0)
/// #   (-228, -821, -821, 0)
/// #   (-220, -794, -794, 0)
/// #   (-212, -768, -768, 0)
/// #   (-204, -741, -741, 0)
/// #   (-196, -715, -715, 0)
/// #   (-188, -688, -688, 0)
/// #   (-180, -661, -661, 0)
/// #   (-172, -635, -635, 0)
/// #   (-164, -608, -608, 0)
/// #   (-156, -582, -582, 0)
/// #   (-148, -555, -555, 0)
/// #   (-140, -529, -529, 0)
/// #   (-132, -502, -502, 0)
/// #   (-124, -475, -475, 0)
/// #   (-116, -449, -449, 0)
/// #   (-108, -422, -422, 0)
/// #   (-100, -396, -396, 0)
/// #   (-92, -369, -369, 0)
/// #   (-84, -343, -343, 0)
/// #   (-76, -316, -316, 0)
/// #   (-68, -289, -289, 0)
/// #   (-60, -263, -263, 0)
/// #   (-52, -236, -236, 0)
/// #   (-44, -210, -210, 0)
/// #   (-36, -183, -183, 0)
/// #   (-28, -157, -157, 0)
/// #   (-20, -130, -130, 0)
/// #   (-12, -103, -103, 0)
/// #   (-4, -77, -77, 0)
/// #   (4, -50, -50, 0)
/// #   (12, -24, -24, 0)
/// #   (20, 3, 3, 0)
/// #   (28, 30, 30, 0)
/// #   (36, 56, 56, 0)
/// #   (44, 83, 83, 0)
/// #   (52, 109, 109, 0)
/// #   (60, 136, 136, 0)
/// #   (68, 162, 162, 0)
/// #   (76, 189, 189, 0)
/// #   (84, 216, 216, 0)
/// #   (92, 242, 242, 0)
/// #   (100, 269, 269, 0)
/// #   (108, 295, 295, 0)
/// #   (116, 322, 322, 0)
/// #   (124, 348, 348, 0)
/// #   (132, 375, 375, 0)
/// #   (140, 402, 402, 0)
/// #   (148, 428, 428, 0)
/// #   (156, 455, 455, 0)
/// #   (164, 481, 481, 0)
/// #   (172, 508, 508, 0)
/// #   (180, 534, 534, 0)
/// #   (188, 561, 561, 0)
/// #   (196, 588, 588, 0)
/// #   (204, 614, 614, 0)
/// #   (212, 641, 641, 0)
/// #   (220, 667, 667, 0)
/// #   (228, 694, 694, 0)
/// #   (236, 720, 720, 0)
/// #   (244, 747, 747, 0)
/// #   (252, 774, 774, 0)
/// #   (260, 800, 800, 0)
/// #   (268, 827, 827, 0)
/// #   (276, 853, 853, 0)
/// #   (284, 880, 880, 0)
/// #   (292, 907, 907, 0)
/// #   (300, 933, 933, 0)
/// #   (308, 960, 960, 0)
/// #   (316, 986, 986, 0)
/// #   (324, 1013, 1013, 0)
/// #   (332, 1039, 1039, 0)
/// #   (340, 1066, 1066, 0)
///
/// if __name__ == '__main__':
///     run()
/// ```
const GRISU_POWERS_OF_TEN: [u64; 87] = [
    0xfa8fd5a0081c0288, // 10^-348
    0xbaaee17fa23ebf76, // 10^-340
    0x8b16fb203055ac76, // 10^-332
    0xcf42894a5dce35ea, // 10^-324
    0x9a6bb0aa55653b2d, // 10^-316
    0xe61acf033d1a45df, // 10^-308
    0xab70fe17c79ac6ca, // 10^-300
    0xff77b1fcbebcdc4f, // 10^-292
    0xbe5691ef416bd60c, // 10^-284
    0x8dd01fad907ffc3c, // 10^-276
    0xd3515c2831559a83, // 10^-268
    0x9d71ac8fada6c9b5, // 10^-260
    0xea9c227723ee8bcb, // 10^-252
    0xaecc49914078536d, // 10^-244
    0x823c12795db6ce57, // 10^-236
    0xc21094364dfb5637, // 10^-228
    0x9096ea6f3848984f, // 10^-220
    0xd77485cb25823ac7, // 10^-212
    0xa086cfcd97bf97f4, // 10^-204
    0xef340a98172aace5, // 10^-196
    0xb23867fb2a35b28e, // 10^-188
    0x84c8d4dfd2c63f3b, // 10^-180
    0xc5dd44271ad3cdba, // 10^-172
    0x936b9fcebb25c996, // 10^-164
    0xdbac6c247d62a584, // 10^-156
    0xa3ab66580d5fdaf6, // 10^-148
    0xf3e2f893dec3f126, // 10^-140
    0xb5b5ada8aaff80b8, // 10^-132
    0x87625f056c7c4a8b, // 10^-124
    0xc9bcff6034c13053, // 10^-116
    0x964e858c91ba2655, // 10^-108
    0xdff9772470297ebd, // 10^-100
    0xa6dfbd9fb8e5b88f, // 10^-92
    0xf8a95fcf88747d94, // 10^-84
    0xb94470938fa89bcf, // 10^-76
    0x8a08f0f8bf0f156b, // 10^-68
    0xcdb02555653131b6, // 10^-60
    0x993fe2c6d07b7fac, // 10^-52
    0xe45c10c42a2b3b06, // 10^-44
    0xaa242499697392d3, // 10^-36
    0xfd87b5f28300ca0e, // 10^-28
    0xbce5086492111aeb, // 10^-20
    0x8cbccc096f5088cc, // 10^-12
    0xd1b71758e219652c, // 10^-4
    0x9c40000000000000, // 10^4
    0xe8d4a51000000000, // 10^12
    0xad78ebc5ac620000, // 10^20
    0x813f3978f8940984, // 10^28
    0xc097ce7bc90715b3, // 10^36
    0x8f7e32ce7bea5c70, // 10^44
    0xd5d238a4abe98068, // 10^52
    0x9f4f2726179a2245, // 10^60
    0xed63a231d4c4fb27, // 10^68
    0xb0de65388cc8ada8, // 10^76
    0x83c7088e1aab65db, // 10^84
    0xc45d1df942711d9a, // 10^92
    0x924d692ca61be758, // 10^100
    0xda01ee641a708dea, // 10^108
    0xa26da3999aef774a, // 10^116
    0xf209787bb47d6b85, // 10^124
    0xb454e4a179dd1877, // 10^132
    0x865b86925b9bc5c2, // 10^140
    0xc83553c5c8965d3d, // 10^148
    0x952ab45cfa97a0b3, // 10^156
    0xde469fbd99a05fe3, // 10^164
    0xa59bc234db398c25, // 10^172
    0xf6c69a72a3989f5c, // 10^180
    0xb7dcbf5354e9bece, // 10^188
    0x88fcf317f22241e2, // 10^196
    0xcc20ce9bd35c78a5, // 10^204
    0x98165af37b2153df, // 10^212
    0xe2a0b5dc971f303a, // 10^220
    0xa8d9d1535ce3b396, // 10^228
    0xfb9b7cd9a4a7443c, // 10^236
    0xbb764c4ca7a44410, // 10^244
    0x8bab8eefb6409c1a, // 10^252
    0xd01fef10a657842c, // 10^260
    0x9b10a4e5e9913129, // 10^268
    0xe7109bfba19c0c9d, // 10^276
    0xac2820d9623bf429, // 10^284
    0x80444b5e7aa7cf85, // 10^292
    0xbf21e44003acdd2d, // 10^300
    0x8e679c2f5e44ff8f, // 10^308
    0xd433179d9c8cb841, // 10^316
    0x9e19db92b4e31ba9, // 10^324
    0xeb96bf6ebadf77d9, // 10^332
    0xaf87023b9bf0ee6b, // 10^340
];
