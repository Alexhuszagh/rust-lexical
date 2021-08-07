//! Implementation of the Grisu algorithm.
//!
//! These routines are adapted from Andrea Samoljuk's `fpconv` library,
//! which is available [here](https://github.com/night-shift/fpconv).
//!
//! In addition to porting from C to Rust, this also adds format
//! precision control and other features.
// TODO(ahuszagh) Document inferring exponents, **and more**.

#![cfg(feature = "compact")]
#![allow(unused)] // TODO(ahuszagh) Remove...

use crate::options::{Options, RoundMode};
use core::mem;
use lexical_util::digit::digit_to_char_const;
use lexical_util::extended_float::ExtendedFloat;
use lexical_util::format::NumberFormat;
use lexical_util::num::{AsPrimitive, Float};
use lexical_write_integer::write::WriteInteger;

// TODO(ahuszagh) Document...
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

    // Write our mantissa digits to a temporary buffer.
    let mut digits: mem::MaybeUninit<[u8; 32]> = mem::MaybeUninit::uninit();
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
        unsafe { round_and_truncate(&mut digits, ndigits, k, options) }
    };

    // See if we should write the number in exponent notation.
    let min_exp = options.negative_exponent_break().map_or(-5, |x| x.get());
    let max_exp = options.positive_exponent_break().map_or(9, |x| x.get());
    let exp = k + ndigits as i32 - 1;
    if !format.no_exponent_notation()
        && (format.required_exponent_notation() || exp < min_exp || exp > max_exp)
    {
        // Write digits in scientific notation.
        // SAFETY: safe as long as bytes is large enough to hold all the digits.
        unsafe { write_float_scientific::<FORMAT>(bytes, &mut digits, ndigits, k, options) }
    } else if exp >= 0 {
        // Write positive exponent without scientific notation.
        // SAFETY: safe as long as bytes is large enough to hold all the digits.
        unsafe { write_float_positive_exponent::<FORMAT>(bytes, &mut digits, ndigits, k, options) }
    } else {
        // Write negative exponent without scientific notation.
        // SAFETY: safe as long as bytes is large enough to hold all the digits.
        unsafe { write_float_negative_exponent::<FORMAT>(bytes, &mut digits, ndigits, k, options) }
    }
}

/// Round digit to normal approximation.
unsafe fn round_digit(
    digits: &mut [u8],
    ndigits: usize,
    delta: u64,
    mut rem: u64,
    kappa: u64,
    mant: u64,
) {
    while rem < mant
        && delta - rem >= kappa
        && (rem + kappa < mant || mant - rem > rem + kappa - mant)
    {
        unsafe { index_unchecked_mut!(digits[ndigits - 1]) -= 1 };
        rem += kappa;
    }
}

/// Generate digits from upper and lower range on rounding of number.
pub unsafe fn generate_digits(
    fp: &ExtendedFloat80,
    upper: &ExtendedFloat80,
    lower: &ExtendedFloat80,
    digits: &mut [u8],
    mut k: i32,
) -> (usize, i32) {
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
// TODO(ahuszagh) Add preconditions: doesn't work on 0.
pub unsafe fn grisu<F: Float>(float: F, digits: &mut [u8]) -> (usize, i32) {
    debug_assert!(float != F::ZERO);

    let mut w = from_float(float);

    let (lower, upper) = normalized_boundaries::<F>(&w);
    normalize(&mut w);
    let (cp, ki) = cached_grisu_power(upper.exp);

    let w = mul(&w, &cp);
    let mut upper = mul(&upper, &cp);
    let mut lower = mul(&lower, &cp);

    lower.mant += 1;
    upper.mant -= 1;

    let k = -ki;

    // SAFETY: safe since generate_digits can only generate 18 digits
    unsafe { generate_digits(&w, &upper, &lower, digits, k) }
}

/// Round-up the last digit.
pub unsafe fn round_up(digits: &mut [u8], ndigits: usize) -> usize {
    let mut index = ndigits;
    while index != 0 {
        // SAFETY: safe since `index > 0 && index < digits.len()`.
        let digit = unsafe { index_unchecked!(digits[index - 1]) };
        if digit < b'9' {
            // SAFETY: safe since `index > 0 && index < digits.len()`.
            unsafe { index_unchecked_mut!(digits[index - 1]) = digit + 1 };
            return index;
        }
        // Don't have to assign b'0' otherwise, since we're just carrying
        // to the next digit.
        index -= 1;
    }

    // Means all digits were b'9': we need to round up.
    // TODO(ahuszagh) I think I need k here... But not sure...
    // SAFETY: safe since `digits.len() > 1`.
    unsafe { index_unchecked_mut!(digits[0]) = b'1' };

    1
}

/// Round the number of digits based on the maximum digits.
pub unsafe fn round_and_truncate(
    digits: &mut [u8],
    ndigits: usize,
    k: i32,
    options: &Options,
) -> (usize, i32) {
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
    let count = ndigits - max_digits;
    let truncated = unsafe { index_unchecked!(digits[max_digits]) };
    let digits = if truncated < b'5' {
        // Just truncate, going to round-down anyway.
        max_digits
    } else if truncated > b'5' {
        // Round-up always.
        // SAFETY: safe since `max_digits < digits.len()`.
        unsafe { round_up(digits, max_digits) }
    } else {
        // Have a near-halfway case, resolve it.
        let to_round = unsafe { &index_unchecked!(digits[max_digits - 1..ndigits]) };
        let is_odd = unsafe { index_unchecked!(to_round[0]) % 2 == 1 };
        let is_above = unsafe { index_unchecked!(to_round[2..]).iter().any(|&x| x != b'0') };
        if is_odd || is_above {
            unsafe { round_up(digits, max_digits) }
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
    // TODO(ahuszagh) Add additional checks...

    // Config options
    let format = NumberFormat::<{ FORMAT }> {};
    let decimal_point = format.decimal_point();
    let exponent_character = format.exponent();

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
    unsafe { index_unchecked_mut!(bytes[cursor]) = exponent_character };
    cursor += 1;

    // We've handled the zero case: write the sign for the exponent.
    let exp = k + ndigits as i32 - 1;
    let positive_exp: u32;
    if exp < 0 {
        // SAFETY: safe if bytes is large enough to hold the output
        unsafe { index_unchecked_mut!(bytes[cursor]) = b'-' };
        cursor += 1;
        positive_exp = exp.wrapping_neg() as u32;
    } else if cfg!(feature = "format") && format.required_exponent_sign() {
        // SAFETY: safe if bytes is large enough to hold the output
        unsafe { index_unchecked_mut!(bytes[cursor]) = b'+' };
        cursor += 1;
        positive_exp = exp as u32;
    } else {
        positive_exp = exp as u32;
    }

    // Write our exponent digits.
    // SAFETY: safe since bytes must be large enough to store all digits.
    cursor += unsafe {
        positive_exp.write_exponent::<u32, FORMAT>(&mut index_unchecked_mut!(bytes[cursor..]))
    };

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
    mut ndigits: usize,
    k: i32,
    options: &Options,
) -> usize {
    // TODO(ahuszagh) Add additional checks...

    // Config options
    let format = NumberFormat::<{ FORMAT }> {};
    let decimal_point = format.decimal_point();

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
    // TODO(ahuszagh) Add additional checks...

    // Config options
    let format = NumberFormat::<{ FORMAT }> {};
    let decimal_point = format.decimal_point();

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
fn cached_grisu_power(exp: i32) -> (&'static ExtendedFloat80, i32) {
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
        // Use `arr.get(idx)`, which explicitly provides a reference,
        // instead of `arr[idx]`, which provides a value.
        // We have a bug in versions <= 1.27.0 where it creates
        // a local copy, which we then get the reference to and return.
        // This allows use-after-free, without any warning, so we're
        // using unidiomatic code to avoid any issue.

        // TODO(ahuszagh) This isn't working... Also, can be unsafe...
        let power = GRISU_POWERS_OF_TEN.get(idx).unwrap();
        let current = exp + power.exp + 64;
        if current < EXPMIN {
            idx += 1;
            continue;
        }

        if current > EXPMAX {
            idx -= 1;
            continue;
        }

        let k = FIRSTPOWER + idx as i32 * STEPPOWERS;
        return (power, k);
    }
}

// TODO(ahuszagh) Use this...
//  Need to go the reverse way.
//  DOCUMENT THE CHANGES
//#[inline]
//fn power(q: i32) -> i32 {
//    (q.wrapping_mul(152_170 + 65536) >> 16) + 63
//}
// TODO(ahuszagh) This doesn't work **exactly**, but it shouldn't be **too** hard
//  to get it right... Main issue is the format is
//      9.223372036854776e-50
//      9.223372036854776e-42
//  So the actual exponent isn't necessarily accurate: can over or underflow.

/// Calculate a base 2 exponent from a decimal exponent.
/// This uses a pre-computed integer approximation for
/// log2(10), where 217706 / 2^16 is accurate for the
/// entire range of non-finite decimal exponents.

/// Cached powers of ten as specified by the Grisu algorithm.
///
/// Cached powers of 10^k, calculated as if by:
/// `ceil((alpha-e+63) * ONE_LOG_TEN);`
// TODO(ahuszagh) A few things...
//  1). Can infer the exponent (EASY).
//  2). Store in hex
//  3). Store as a static array of u64...
const GRISU_POWERS_OF_TEN: [ExtendedFloat80; 87] = [
    ExtendedFloat80 {
        mant: 18054884314459144840,
        exp: -1220,
    },
    ExtendedFloat80 {
        mant: 13451937075301367670,
        exp: -1193,
    },
    ExtendedFloat80 {
        mant: 10022474136428063862,
        exp: -1166,
    },
    ExtendedFloat80 {
        mant: 14934650266808366570,
        exp: -1140,
    },
    ExtendedFloat80 {
        mant: 11127181549972568877,
        exp: -1113,
    },
    ExtendedFloat80 {
        mant: 16580792590934885855,
        exp: -1087,
    },
    ExtendedFloat80 {
        mant: 12353653155963782858,
        exp: -1060,
    },
    ExtendedFloat80 {
        mant: 18408377700990114895,
        exp: -1034,
    },
    ExtendedFloat80 {
        mant: 13715310171984221708,
        exp: -1007,
    },
    ExtendedFloat80 {
        mant: 10218702384817765436,
        exp: -980,
    },
    ExtendedFloat80 {
        mant: 15227053142812498563,
        exp: -954,
    },
    ExtendedFloat80 {
        mant: 11345038669416679861,
        exp: -927,
    },
    ExtendedFloat80 {
        mant: 16905424996341287883,
        exp: -901,
    },
    ExtendedFloat80 {
        mant: 12595523146049147757,
        exp: -874,
    },
    ExtendedFloat80 {
        mant: 9384396036005875287,
        exp: -847,
    },
    ExtendedFloat80 {
        mant: 13983839803942852151,
        exp: -821,
    },
    ExtendedFloat80 {
        mant: 10418772551374772303,
        exp: -794,
    },
    ExtendedFloat80 {
        mant: 15525180923007089351,
        exp: -768,
    },
    ExtendedFloat80 {
        mant: 11567161174868858868,
        exp: -741,
    },
    ExtendedFloat80 {
        mant: 17236413322193710309,
        exp: -715,
    },
    ExtendedFloat80 {
        mant: 12842128665889583758,
        exp: -688,
    },
    ExtendedFloat80 {
        mant: 9568131466127621947,
        exp: -661,
    },
    ExtendedFloat80 {
        mant: 14257626930069360058,
        exp: -635,
    },
    ExtendedFloat80 {
        mant: 10622759856335341974,
        exp: -608,
    },
    ExtendedFloat80 {
        mant: 15829145694278690180,
        exp: -582,
    },
    ExtendedFloat80 {
        mant: 11793632577567316726,
        exp: -555,
    },
    ExtendedFloat80 {
        mant: 17573882009934360870,
        exp: -529,
    },
    ExtendedFloat80 {
        mant: 13093562431584567480,
        exp: -502,
    },
    ExtendedFloat80 {
        mant: 9755464219737475723,
        exp: -475,
    },
    ExtendedFloat80 {
        mant: 14536774485912137811,
        exp: -449,
    },
    ExtendedFloat80 {
        mant: 10830740992659433045,
        exp: -422,
    },
    ExtendedFloat80 {
        mant: 16139061738043178685,
        exp: -396, // -80
    },
    ExtendedFloat80 {
        mant: 12024538023802026127,
        exp: -369, // -72
    },
    ExtendedFloat80 {
        mant: 17917957937422433684,
        exp: -343, // -66
    },
    ExtendedFloat80 {
        mant: 13349918974505688015,
        exp: -316, // -58
    },
    ExtendedFloat80 {
        mant: 9946464728195732843,
        exp: -289, // -50
    },
    ExtendedFloat80 {
        mant: 14821387422376473014,
        exp: -263, // -42
    },
    ExtendedFloat80 {
        mant: 11042794154864902060,
        exp: -236,
    },
    ExtendedFloat80 {
        mant: 16455045573212060422,
        exp: -210,
    },
    ExtendedFloat80 {
        mant: 12259964326927110867,
        exp: -183,
    },
    ExtendedFloat80 {
        mant: 18268770466636286478,
        exp: -157,
    },
    ExtendedFloat80 {
        mant: 13611294676837538539,
        exp: -130,
    },
    ExtendedFloat80 {
        mant: 10141204801825835212,
        exp: -103,
    },
    ExtendedFloat80 {
        mant: 15111572745182864684,
        exp: -77,
    },
    ExtendedFloat80 {
        mant: 11258999068426240000,
        exp: -50,
    },
    ExtendedFloat80 {
        mant: 16777216000000000000,
        exp: -24,
    },
    ExtendedFloat80 {
        mant: 12500000000000000000,
        exp: 3,
    },
    ExtendedFloat80 {
        mant: 9313225746154785156,
        exp: 30,
    },
    ExtendedFloat80 {
        mant: 13877787807814456755,
        exp: 56,
    },
    ExtendedFloat80 {
        mant: 10339757656912845936,
        exp: 83,
    },
    ExtendedFloat80 {
        mant: 15407439555097886824,
        exp: 109,
    },
    ExtendedFloat80 {
        mant: 11479437019748901445,
        exp: 136,
    },
    ExtendedFloat80 {
        mant: 17105694144590052135,
        exp: 162,
    },
    ExtendedFloat80 {
        mant: 12744735289059618216,
        exp: 189,
    },
    ExtendedFloat80 {
        mant: 9495567745759798747,
        exp: 216,
    },
    ExtendedFloat80 {
        mant: 14149498560666738074,
        exp: 242,
    },
    ExtendedFloat80 {
        mant: 10542197943230523224,
        exp: 269,
    },
    ExtendedFloat80 {
        mant: 15709099088952724970,
        exp: 295,
    },
    ExtendedFloat80 {
        mant: 11704190886730495818,
        exp: 322,
    },
    ExtendedFloat80 {
        mant: 17440603504673385349,
        exp: 348,
    },
    ExtendedFloat80 {
        mant: 12994262207056124023,
        exp: 375,
    },
    ExtendedFloat80 {
        mant: 9681479787123295682,
        exp: 402,
    },
    ExtendedFloat80 {
        mant: 14426529090290212157,
        exp: 428,
    },
    ExtendedFloat80 {
        mant: 10748601772107342003,
        exp: 455,
    },
    ExtendedFloat80 {
        mant: 16016664761464807395,
        exp: 481,
    },
    ExtendedFloat80 {
        mant: 11933345169920330789,
        exp: 508,
    },
    ExtendedFloat80 {
        mant: 17782069995880619868,
        exp: 534,
    },
    ExtendedFloat80 {
        mant: 13248674568444952270,
        exp: 561,
    },
    ExtendedFloat80 {
        mant: 9871031767461413346,
        exp: 588,
    },
    ExtendedFloat80 {
        mant: 14708983551653345445,
        exp: 614,
    },
    ExtendedFloat80 {
        mant: 10959046745042015199,
        exp: 641,
    },
    ExtendedFloat80 {
        mant: 16330252207878254650,
        exp: 667,
    },
    ExtendedFloat80 {
        mant: 12166986024289022870,
        exp: 694,
    },
    ExtendedFloat80 {
        mant: 18130221999122236476,
        exp: 720,
    },
    ExtendedFloat80 {
        mant: 13508068024458167312,
        exp: 747,
    },
    ExtendedFloat80 {
        mant: 10064294952495520794,
        exp: 774,
    },
    ExtendedFloat80 {
        mant: 14996968138956309548,
        exp: 800,
    },
    ExtendedFloat80 {
        mant: 11173611982879273257,
        exp: 827,
    },
    ExtendedFloat80 {
        mant: 16649979327439178909,
        exp: 853,
    },
    ExtendedFloat80 {
        mant: 12405201291620119593,
        exp: 880,
    },
    ExtendedFloat80 {
        mant: 9242595204427927429,
        exp: 907,
    },
    ExtendedFloat80 {
        mant: 13772540099066387757,
        exp: 933,
    },
    ExtendedFloat80 {
        mant: 10261342003245940623,
        exp: 960,
    },
    ExtendedFloat80 {
        mant: 15290591125556738113,
        exp: 986,
    },
    ExtendedFloat80 {
        mant: 11392378155556871081,
        exp: 1013,
    },
    ExtendedFloat80 {
        mant: 16975966327722178521,
        exp: 1039,
    },
    ExtendedFloat80 {
        mant: 12648080533535911531,
        exp: 1066,
    },
];
