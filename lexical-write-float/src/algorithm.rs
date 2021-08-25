//! Implementation of the Dragonbox algorithm.
//!
//! This is modified from the Rust port of Dragonbox, available
//! [here](https://github.com/dtolnay/dragonbox). It also uses a direct
//! port of Dragonbox, available [here](https://github.com/jk-jeon/dragonbox/).
//!
//! This is therefore under an Apache 2.0/Boost Software dual-license.
//!
//! We use a u64 for the significant digits, even for a 32-bit integer,
//! however, we use the proper bitshifts, etc. for the float in question,
//! rather than clobbering the result to f64, as Rust's port does.

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

use crate::float::{ExtendedFloat80, RawFloat};
use crate::options::{Options, RoundMode};
use crate::shared::{truncate_and_round_decimal, write_exponent};
use crate::table::*;
use lexical_util::format::NumberFormat;
use lexical_util::num::{AsPrimitive, Float, Integer};
use lexical_write_integer::decimal::DigitCount;
use lexical_write_integer::table::DIGIT_TO_BASE10_SQUARED;

// TODO(ahuszagh) Implement...
// TODO(ahuszagh) Need to handle rounding and stuff...
//      And trailing zeros...

/// Optimized float-to-string algorithm for decimal strings.
///
/// # Safety
///
/// Safe as long as the float isn't special (NaN or Infinity), and `bytes`
/// is large enough to hold the significant digits.
#[inline]
pub unsafe fn write_float<F: RawFloat, const FORMAT: u128>(
    float: F,
    bytes: &mut [u8],
    options: &Options,
) -> usize {
    debug_assert!(float.is_sign_positive());
    debug_assert!(!float.is_special());

    let fp = to_decimal(float, options);
    let digit_count = F::digit_count(fp.mant);
    let sci_exp = fp.exp + digit_count as i32 - 1;

    // Note that for performance reasons, we write the significant digits
    // later into the algorithms, since we can determine the right path
    // and write the significant digits without using an intermediate buffer
    // in most cases.

    write_float!(
        FORMAT,
        sci_exp,
        options,
        write_float_scientific,
        write_float_positive_exponent,
        write_float_negative_exponent,
        generic => F,
        args => bytes, fp, options,
    )
}

/// Write float to string in scientific notation.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of digits
/// and the scientific notation's exponent digits.
pub unsafe fn write_float_scientific<F: DragonboxFloat, const FORMAT: u128>(
    bytes: &mut [u8],
    fp: ExtendedFloat80,
    options: &Options,
) -> usize {
    // Config options.
    let format = NumberFormat::<{ FORMAT }> {};
    assert!(format.is_valid());
    let decimal_point = options.decimal_point();

    // Write the significant digits, and round them.
    // SAFETY: safe, if we have enough bytes to write the significant digits.
    let mut digits = unsafe { &mut index_unchecked_mut!(bytes[1..]) };
    let digit_count = unsafe { F::write_digits(digits, fp.mant) };

    // Truncate and round the significant digits.
    // SAFETY: safe since `digit_count < digits.len()`.
    let (digit_count, exp) =
        unsafe { truncate_and_round_decimal(&mut digits, digit_count, fp.exp, options) };

    // Determine the exact number of digits to write.
    let mut exact_count: usize = digit_count;
    if let Some(min_digits) = options.min_significant_digits() {
        exact_count = min_digits.get().max(exact_count);
    }

    // Adjust to scientific notation.
    // SAFETY: safe if the above steps were safe, since `bytes.len() >= 2`.
    let mut cursor: usize;
    unsafe {
        index_unchecked_mut!(bytes[0] = bytes[1]);
        index_unchecked_mut!(bytes[1]) = decimal_point;

        if digit_count == 1 && options.trim_floats() {
            cursor = 1;
        } else if digit_count == 1 {
            index_unchecked_mut!(bytes[2]) = b'0';
            cursor = 3;
        } else if digit_count < exact_count {
            // Adjust the number of digits written, by appending zeros.
            cursor = digit_count + 1;
            let zeros = exact_count - digit_count;
            unsafe {
                slice_fill_unchecked!(index_unchecked_mut!(bytes[cursor..cursor + zeros]), b'0');
            }
        } else {
            cursor = digit_count + 1;
        }
    }

    // Now, write our scientific notation.
    // SAFETY: safe since bytes must be large enough to store all digits.
    let sci_exp = exp + digit_count as i32 - 1;
    unsafe { write_exponent::<FORMAT>(bytes, &mut cursor, sci_exp, options.exponent()) };

    cursor
}

/// Write negative float to string without scientific notation.
/// Has a negative exponent (shift right) and no scientific notation.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of
/// significant digits and the leading zeros.
#[allow(unused)] // TODO(ahuszagh) Restore...
pub unsafe fn write_float_negative_exponent<F: DragonboxFloat, const FORMAT: u128>(
    bytes: &mut [u8],
    fp: ExtendedFloat80,
    options: &Options,
) -> usize {
    // TODO(ahuszagh) Here...
    //    // Config options.
    //    let decimal_point = options.decimal_point();
    //    let sci_exp = (fp.exp + digit_count as i32 - 1).wrapping_neg() as usize;
    //
    //    // Write our 0 digits.
    //    // SAFETY: must be safe since since `bytes.len() < BUFFER_SIZE - 2`.
    //    unsafe {
    //        index_unchecked_mut!(bytes[0]) = b'0';
    //        index_unchecked_mut!(bytes[1]) = decimal_point;
    //        let digits = &mut index_unchecked_mut!(bytes[2..sci_exp + 1]);
    //        slice_fill_unchecked!(digits, b'0');
    //    }
    //    let mut cursor = sci_exp + 1;

    // TODO(ahuszagh)
    //    // Write out significant digits.
    //    // SAFETY: safe if bytes is large enough to hold all the significant digits.
    //    let digits = unsafe { &mut index_unchecked_mut!(bytes[cursor..]) };
    //    cursor += F::write_digits(digits, fp.mant, digit_count, options);

    todo!();
}

/// Write positive float to string without scientific notation.
/// Has a positive exponent (shift left) and no scientific notation.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of
/// significant digits and the (optional) trailing zeros.
#[allow(unused)] // TODO(ahuszagh) Restore...
pub unsafe fn write_float_positive_exponent<F: DragonboxFloat, const FORMAT: u128>(
    bytes: &mut [u8],
    fp: ExtendedFloat80,
    options: &Options,
) -> usize {
    //    // Config options.
    //    let decimal_point = options.decimal_point();
    //    let sci_exp = (fp.exp + digit_count as i32 - 1) as usize;
    //    let leading_digits = sci_exp + 1;

    // TODO(ahuszagh)
    //    // Write out all the digits.
    //    // SAFETY: safe if bytes is large enough to hold all the significant digits.
    //    let mut cursor = F::write_digits(bytes, fp.mant, digit_count, options);
    //    if leading_digits >= cursor {
    //        // We have more leading digits than digits we wrote: can write
    //        // any additional digits, and then just write the remaining ones.
    //        // SAFETY: safe if the buffer is large enough to hold the significant digits.
    //    } else {
    //        // We have less leading digits than digits we wrote.
    //        // Shift all digits, and write the decimal point.
    //    }

    todo!();
}

// ALGORITHM
// ---------

/// Get an extended representation of the decimal float.
///
/// The returned float has a decimal exponent, and the significant digits
/// returned to the nearest mantissa. For example, `1.5f32` will return
/// `ExtendedFloat80 { mant: 15, exp: -1 }`, although trailing zeros
/// might not be removed.
#[inline]
pub fn to_decimal<F: RawFloat>(float: F, options: &Options) -> ExtendedFloat80 {
    let bits = float.to_bits();
    let mantissa_bits = bits & F::MANTISSA_MASK;

    if bits.as_u64() == 0 {
        return extended_float(0, 0);
    }

    // Shorter interval case; proceed like Schubfach. One might think this
    // condition is wrong, since when exponent_bits == 1 and two_fc == 0,
    // the interval is actually regular. However, it turns out that this
    // seemingly wrong condition is actually fine, because the end result is
    // anyway the same.
    //
    // [binary32]
    // floor( (fc-1/2) * 2^e ) = 1.175'494'28... * 10^-38
    // floor( (fc-1/4) * 2^e ) = 1.175'494'31... * 10^-38
    // floor(    fc    * 2^e ) = 1.175'494'35... * 10^-38
    // floor( (fc+1/2) * 2^e ) = 1.175'494'42... * 10^-38
    //
    // Hence, shorter_interval_case will return 1.175'494'4 * 10^-38.
    // 1.175'494'3 * 10^-38 is also a correct shortest representation that
    // will be rejected if we assume shorter interval, but 1.175'494'4 *
    // 10^-38 is closer to the true value so it doesn't matter.
    //
    // [binary64]
    // floor( (fc-1/2) * 2^e ) = 2.225'073'858'507'201'13... * 10^-308
    // floor( (fc-1/4) * 2^e ) = 2.225'073'858'507'201'25... * 10^-308
    // floor(    fc    * 2^e ) = 2.225'073'858'507'201'38... * 10^-308
    // floor( (fc+1/2) * 2^e ) = 2.225'073'858'507'201'63... * 10^-308
    //
    // Hence, shorter_interval_case will return 2.225'073'858'507'201'4 * 10^-308.
    // This is indeed of the shortest length, and it is the unique one
    // closest to the true value among valid representations of the same
    // length.

    // NOTE: unlike Dragonbox, we don't need to check exponent_bits,
    // since if both the exponent bits and mantissa bits are 0, we have
    // a literal 0.
    match options.round_mode() {
        RoundMode::Round => {
            if mantissa_bits.as_u64() == 0 {
                compute_round_short(float, options)
            } else {
                compute_round(float, options)
            }
        },
        RoundMode::Truncate => compute_truncate(float, options),
    }
}

/// Simpler case, where we have **only** the hidden bit set.
pub fn compute_round_short<F: RawFloat>(float: F, options: &Options) -> ExtendedFloat80 {
    debug_assert!(options.round_mode() == RoundMode::Round);

    // Compute k and beta.
    let exponent = float.exponent();
    let minus_k = floor_log10_pow2_minus_log10_4_over_3(exponent);
    let beta_minus_1 = exponent + floor_log2_pow10(-minus_k);

    // Compute xi and zi.
    // SAFETY: safe, since value must be finite and therefore in the correct range.
    let pow5 = unsafe { F::dragonbox_power(-minus_k) };
    let mut xi = F::compute_left_endpoint(&pow5, beta_minus_1);
    let mut zi = F::compute_right_endpoint(&pow5, beta_minus_1);

    // Get the interval type.
    // Must be Round since we only use compute_round with a round-nearest direction.
    let interval_type = IntervalType::Closed;

    // If we don't accept the right endpoint and if the right endpoint is an
    // integer, decrease it.
    if !interval_type.include_right_endpoint() && is_right_endpoint(exponent) {
        zi -= 1;
    }

    // If the left endpoint is not an integer, increase it.
    if !interval_type.include_left_endpoint() && is_left_endpoint(exponent) {
        xi += 1;
    }

    // Try bigger divisor.
    let significand = zi / 10;

    // If succeed, remove trailing zeros if necessary and return.
    if significand * 10 >= xi {
        let (mant, exp) = F::process_trailing_zeros(significand, minus_k + 1);
        return extended_float(mant, exp);
    }

    // Otherwise, compute the round-up of y.
    let mut significand = F::compute_round_up(&pow5, beta_minus_1);
    let exponent = minus_k;

    // When tie occurs, choose one of them according to the rule.
    let bits: i32 = F::MANTISSA_SIZE;
    let lower_threshold: i32 = -floor_log5_pow2_minus_log5_3(bits + 4) - 2 - bits;
    let upper_threshold: i32 = -floor_log5_pow2(bits + 2) - 2 - bits;

    if exponent >= lower_threshold && exponent <= upper_threshold {
        significand = options.round_mode().break_rounding_tie(significand);
    } else if significand < xi {
        significand += 1;
    }

    extended_float(significand, exponent)
}

/// The main algorithm assumes the input is a normal/subnormal finite number.
#[allow(clippy::comparison_chain)]
pub fn compute_round<F: RawFloat>(float: F, options: &Options) -> ExtendedFloat80 {
    debug_assert!(options.round_mode() == RoundMode::Round);

    let mantissa = float.mantissa().as_u64();
    let exponent = float.exponent();
    let is_even = mantissa % 2 == 0;

    // Step 1: Schubfach multiplier calculation
    // Compute k and beta.
    let minus_k = floor_log10_pow2(exponent) - F::KAPPA as i32;
    // SAFETY: safe, since value must be finite and therefore in the correct range.
    let pow5 = unsafe { F::dragonbox_power(-minus_k) };
    let beta_minus_1 = exponent + floor_log2_pow10(-minus_k);

    // Compute zi and deltai.
    // 10^kappa <= deltai < 10^(kappa + 1)
    let two_fc = mantissa << 1;
    let deltai = F::compute_delta(&pow5, beta_minus_1);
    let two_fr = two_fc | 1;
    let zi = F::compute_mul(two_fr << beta_minus_1, &pow5);

    // Step 2: Try larger divisor; remove trailing zeros if necessary
    let big_divisor = pow32(10, F::KAPPA + 1);
    let small_divisor = pow32(10, F::KAPPA);

    // Using an upper bound on zi, we might be able to optimize the division
    // better than the compiler; we are computing zi / big_divisor here.
    let exp = F::KAPPA + 1;
    let max_pow2 = F::MANTISSA_SIZE + F::KAPPA as i32 + 2;
    let max_pow5 = F::KAPPA as i32 + 1;
    let mut significand = divide_by_pow10(zi, exp, max_pow2, max_pow5);
    let mut r = (zi - big_divisor as u64 * significand) as u32;

    // Get the interval type.
    // Must be Round since we only use compute_round with a round-nearest direction.
    let interval_type = IntervalType::Symmetric(is_even);

    // Short-circuit case.
    let short_circuit = || {
        let (mant, exp) = F::process_trailing_zeros(significand, minus_k + F::KAPPA as i32 + 1);
        extended_float(mant, exp)
    };

    // Check for short-circuit.
    if r < deltai {
        // Exclude the right endpoint if necessary.
        let include_right = interval_type.include_right_endpoint();
        if r == 0 && !include_right && F::is_product_fc_pm_half(two_fr, exponent, minus_k) {
            significand -= 1;
            r = big_divisor;
        } else {
            return short_circuit();
        }
    } else if r == deltai {
        // r == deltai; compare fractional parts.
        // Check conditions in the order different from the paper to take
        // advantage of short-circuiting.
        let two_fl = two_fc - 1;
        let include_left = interval_type.include_left_endpoint();
        let is_prod = F::is_product_fc_pm_half(two_fl, exponent, minus_k);
        let is_mul_parity = F::compute_mul_parity(two_fl, &pow5, beta_minus_1);
        if !((include_left && is_prod) || is_mul_parity) {
            return short_circuit();
        }
    }

    // Step 3: Find the significand with the smaller divisor
    significand *= 10;
    let exponent = minus_k + F::KAPPA as i32;

    let dist = r - (deltai / 2) + (small_divisor / 2);
    let approx_y_parity = ((dist ^ (small_divisor / 2)) & 1) != 0;

    // Is dist divisible by 10^kappa?
    let (dist, is_dist_div_by_kappa) = F::check_div_pow10(dist);

    // Add dist / 10^kappa to the significand.
    significand += dist as u64;

    if is_dist_div_by_kappa {
        // Check z^(f) >= epsilon^(f)
        // We have either yi == zi - epsiloni or yi == (zi - epsiloni) - 1,
        // where yi == zi - epsiloni if and only if z^(f) >= epsilon^(f)
        // Since there are only 2 possibilities, we only need to care about the parity.
        // Also, zi and r should have the same parity since the divisor
        // is an even number.
        if F::compute_mul_parity(two_fc, &pow5, beta_minus_1) != approx_y_parity {
            significand -= 1;
        } else {
            // If z^(f) >= epsilon^(f), we might have a tie
            // when z^(f) == epsilon^(f), or equivalently, when y is an integer.
            // For tie-to-up case, we can just choose the upper one.
            if F::is_product_fc(two_fc, exponent, minus_k) {
                significand = options.round_mode().break_rounding_tie(significand);
            }
        }
    }

    extended_float(significand, exponent)
}

/// The main algorithm when truncating digits.
#[allow(clippy::comparison_chain)]
pub fn compute_truncate<F: RawFloat>(float: F, options: &Options) -> ExtendedFloat80 {
    debug_assert!(options.round_mode() == RoundMode::Truncate);

    let mantissa = float.mantissa().as_u64();
    let exponent = float.exponent();

    // Step 1: Schubfach multiplier calculation
    // Compute k and beta.
    let minus_k = floor_log10_pow2(exponent) - F::KAPPA as i32;
    // SAFETY: safe, since value must be finite and therefore in the correct range.
    let pow5 = unsafe { F::dragonbox_power(-minus_k) };
    let beta_minus_1 = exponent + floor_log2_pow10(-minus_k);

    // Compute zi and deltai.
    // 10^kappa <= deltai < 10^(kappa + 1)
    let two_fc = mantissa << 1;
    let deltai = F::compute_delta(&pow5, beta_minus_1);
    let mut xi = F::compute_mul(two_fc << beta_minus_1, &pow5);

    if !F::is_product_fc(two_fc, exponent, minus_k) {
        xi += 1;
    }

    // Step 2: Try larger divisor; remove trailing zeros if necessary
    let big_divisor = pow32(10, F::KAPPA + 1);

    // Using an upper bound on xi, we might be able to optimize the division
    // better than the compiler; we are computing xi / big_divisor here.
    let exp = F::KAPPA + 1;
    let max_pow2 = F::MANTISSA_SIZE + F::KAPPA as i32 + 2;
    let max_pow5 = F::KAPPA as i32 + 1;
    let mut significand = divide_by_pow10(xi, exp, max_pow2, max_pow5);
    let mut r = (xi - big_divisor as u64 * significand) as u32;

    if r != 0 {
        significand += 1;
        r = big_divisor - r;
    }

    // Short-circuit case.
    let short_circuit = || {
        let (mant, exp) = F::process_trailing_zeros(significand, minus_k + F::KAPPA as i32 + 1);
        extended_float(mant, exp)
    };

    // Check for short-circuit.
    if r < deltai {
        return short_circuit();
    } else if r == deltai {
        // Compare the fractional parts.
        let is_prod = F::is_product_fc(two_fc + 2, exponent, minus_k);
        let is_mul_parity = F::compute_mul_parity(two_fc + 2, &pow5, beta_minus_1);
        if !(is_mul_parity || is_prod) {
            return short_circuit();
        }
    }

    // Step 3: Find the significand with the smaller divisor
    significand *= 10;
    significand -= F::small_div_pow10(r) as u64;
    let exponent = minus_k + F::KAPPA as i32;

    extended_float(significand, exponent)
}

// DIGITS
// ------

/// Write 2 digits to buffer.
macro_rules! write_digits {
    ($bytes:ident, $index:ident, $r:ident) => {{
        $index -= 1;
        unsafe { index_unchecked_mut!($bytes[$index] = DIGIT_TO_BASE10_SQUARED[$r + 1]) };
        $index -= 1;
        unsafe { index_unchecked_mut!($bytes[$index] = DIGIT_TO_BASE10_SQUARED[$r]) };
    }};

    (@1 $bytes:ident, $index:ident, $r:ident) => {{
        $index -= 1;
        unsafe { index_unchecked_mut!($bytes[$index] = DIGIT_TO_BASE10_SQUARED[$r]) };
    }};
}

/// Write 1 digit to buffer.
macro_rules! write_digit {
    ($bytes:ident, $index:ident, $r:ident) => {{
        $index -= 1;
        unsafe { index_unchecked_mut!($bytes[$index]) = b'0' + $r as u8 };
    }};
}

/// Shift index and digit count.
macro_rules! shift_digits {
    ($index:ident, $digit_count:ident, $count:literal) => {{
        $index -= $count;
        $digit_count -= $count;
    }};
}

/// Write the significant digits, when the significant digits can fit in a
/// 32-bit integer. Returns the number of digits written. This assumes any
/// trailing zeros have been removed.
///
/// # Safety
///
/// Safe if `bytes.len() >= 10`.
#[inline]
pub unsafe fn write_digits_u32(bytes: &mut [u8], mantissa: u32, digit_count: usize) -> usize {
    debug_assert!(bytes.len() >= 10);

    let mut index = digit_count;
    let mut s32 = mantissa;
    // Need at least more than 5, since we will always write at least 1 trailing value.
    while index >= 5 {
        let c = s32 % 10000;
        s32 /= 10000;
        // c1 = s32 / 100; c2 = s32 % 100;
        let (c1, c2) = fast_div(c, 100, 14, 5);
        let r1 = c1 as usize * 2;
        let r2 = c2 as usize * 2;

        // SAFETY: This is always safe, since the `table.len() == 200`.
        write_digits!(bytes, index, r2);
        write_digits!(bytes, index, r1);
    }

    if index >= 3 {
        // c1 = s32 / 100; c2 = s32 % 100;
        let (c1, c2) = fast_div(s32, 100, 14, 5);
        s32 = c1;
        let r2 = c2 as usize * 2;

        // SAFETY: This is always safe, since the `table.len() == 200`.
        write_digits!(bytes, index, r2);
    }
    if index > 1 {
        debug_assert!(index == 2);
        // d1 = s32 / 10; d2 = s32 % 10;
        let (d1, d2) = fast_div(s32, 10, 7, 3);

        // SAFETY: this is always safe, since `index == 2`.
        write_digit!(bytes, index, d2);
        write_digit!(bytes, index, d1);
    } else {
        debug_assert!(index == 1);

        // SAFETY: this is always safe, since `index == 1`.
        write_digit!(bytes, index, s32);
    }

    digit_count
}

/// Write the significant digits, when the significant digits cannot fit in a
/// 32-bit integer. Returns the number of digits written. Note that this
/// might not be the same as the number of digits in the mantissa, since
/// trailing zeros will be removed.
///
/// # Safety
///
/// Safe if `bytes.len() >= 20`.
#[inline]
#[allow(clippy::branches_sharing_code)]
pub unsafe fn write_digits_u64(bytes: &mut [u8], mantissa: u64, mut digit_count: usize) -> usize {
    debug_assert!(bytes.len() >= 20);

    let mut index = digit_count;
    let mut s32: u32;
    let mut trailing_zeros = false;
    let mut tz: u8;

    // Write the upper 32 bits.
    if mantissa >> 32 != 0 {
        // Since significand is at most 10^17, the quotient is at most 10^9, so
        // it fits inside 32-bit integer.
        s32 = (mantissa / 100000000) as u32;
        let mut r = mantissa.wrapping_sub((s32 as u64).wrapping_mul(100000000)) as u32;

        if r != 0 {
            let c = r % 10000;
            r /= 10000;
            // c1 = s32 / 100; c2 = s32 % 100;
            let (c1, c2) = fast_div(r, 100, 14, 5);
            // c3 = c / 100; c4 = c % 100;
            let (c3, c4) = fast_div(c, 100, 14, 5);
            // SAFETY: safe, since `c4 < 100`.
            tz = unsafe { TRAILING_ZEROS[c4 as usize] };
            let r1 = c1 as usize * 2;
            let r2 = c2 as usize * 2;
            let r3 = c3 as usize * 2;
            let r4 = c4 as usize * 2;

            if tz == 0 {
                // SAFETY: This is always safe, since the `table.len() == 200`.
                write_digits!(bytes, index, r4);
                write_digits!(bytes, index, r3);
                write_digits!(bytes, index, r2);
                write_digits!(bytes, index, r1);
            } else if tz == 1 {
                shift_digits!(index, digit_count, 1);
                // SAFETY: This is always safe, since the `table.len() == 200`.
                write_digits!(@1 bytes, index, r4);
                write_digits!(bytes, index, r3);
                write_digits!(bytes, index, r2);
                write_digits!(bytes, index, r1);
            } else {
                shift_digits!(index, digit_count, 2);
                // SAFETY: safe, since `c3 < 100`.
                tz = unsafe { TRAILING_ZEROS[c3 as usize] };
                if tz == 0 {
                    // SAFETY: This is always safe, since the `table.len() == 200`.
                    write_digits!(bytes, index, r3);
                    write_digits!(bytes, index, r2);
                    write_digits!(bytes, index, r1);
                } else if tz == 1 {
                    shift_digits!(index, digit_count, 1);
                    // SAFETY: This is always safe, since the `table.len() == 200`.
                    write_digits!(@1 bytes, index, r3);
                    write_digits!(bytes, index, r2);
                    write_digits!(bytes, index, r1);
                } else {
                    shift_digits!(index, digit_count, 2);
                    // SAFETY: safe, since `c2 < 100`.
                    tz = unsafe { TRAILING_ZEROS[c2 as usize] };
                    if tz == 0 {
                        // SAFETY: This is always safe, since the `table.len() == 200`.
                        write_digits!(bytes, index, r2);
                        write_digits!(bytes, index, r1);
                    } else if tz == 1 {
                        shift_digits!(index, digit_count, 1);
                        write_digits!(@1 bytes, index, r2);
                        write_digits!(bytes, index, r1);
                    } else {
                        shift_digits!(index, digit_count, 2);
                        // SAFETY: safe, since `c1 < 100`.
                        tz = unsafe { TRAILING_ZEROS[c1 as usize] };
                        if tz == 0 {
                            // SAFETY: This is always safe, since the `table.len() == 200`.
                            write_digits!(bytes, index, r1);
                        } else {
                            // We assumed r != 0, so c1 cannot be zero in this case.
                            debug_assert!(tz == 1);
                            shift_digits!(index, digit_count, 1);
                            write_digits!(@1 bytes, index, r1);
                        }
                    }
                }
            }
        } else {
            // r == 0
            shift_digits!(index, digit_count, 8);
            trailing_zeros = true;
        }
    } else {
        // mantissa >> 32 == 0
        s32 = mantissa as u32;
        trailing_zeros = true;
    }

    // Write the lower 32 bits.
    // Need at least more than 5, since we will always write at least 1 trailing value.
    while index >= 5 {
        let c = s32 % 10000;
        s32 /= 10000;
        // c1 = s32 / 100; c2 = s32 % 100;
        let (c1, c2) = fast_div(c, 100, 14, 5);
        let r1 = c1 as usize * 2;
        let r2 = c2 as usize * 2;

        if trailing_zeros {
            // SAFETY: safe, since `c2 < 100`.
            tz = unsafe { TRAILING_ZEROS[c2 as usize] };
            if tz == 0 {
                // SAFETY: This is always safe, since the `table.len() == 200`.
                write_digits!(bytes, index, r2);
                write_digits!(bytes, index, r1);
                trailing_zeros = false;
            } else if tz == 1 {
                // SAFETY: This is always safe, since the `table.len() == 200`.
                shift_digits!(index, digit_count, 1);
                write_digits!(@1 bytes, index, r2);
                write_digits!(bytes, index, r1);
                trailing_zeros = false;
            } else {
                shift_digits!(index, digit_count, 2);
                // SAFETY: safe, since `c1 < 100`.
                tz = unsafe { TRAILING_ZEROS[c1 as usize] };
                if tz == 0 {
                    write_digits!(bytes, index, r1);
                    trailing_zeros = false;
                } else if tz == 1 {
                    shift_digits!(index, digit_count, 1);
                    write_digits!(@1 bytes, index, r1);
                    trailing_zeros = false;
                } else {
                    shift_digits!(index, digit_count, 2);
                }
            }
        } else {
            // SAFETY: This is always safe, since the `table.len() == 200`.
            write_digits!(bytes, index, r2);
            write_digits!(bytes, index, r1);
        }
    }

    if index >= 3 {
        // c1 = s32 / 100; c2 = s32 % 100;
        let (c1, c2) = fast_div(s32, 100, 14, 5);
        s32 = c1;
        let r2 = c2 as usize * 2;

        if trailing_zeros {
            // SAFETY: safe, since `c1 < 100`.
            tz = unsafe { TRAILING_ZEROS[c2 as usize] };
            if tz == 0 {
                write_digits!(bytes, index, r2);
                trailing_zeros = false;
            } else if tz == 1 {
                shift_digits!(index, digit_count, 1);
                write_digits!(@1 bytes, index, r2);
                trailing_zeros = false;
            } else {
                shift_digits!(index, digit_count, 2);
            }
        } else {
            // SAFETY: This is always safe, since the `table.len() == 200`.
            write_digits!(bytes, index, r2);
        }
    }

    if index > 1 {
        debug_assert!(index == 2);
        // d1 = s32 / 10; d2 = s32 % 10;
        let (d1, d2) = fast_div(s32, 10, 7, 3);

        // SAFETY: this is always safe, since `index == 2`.
        if trailing_zeros && d2 == 0 {
            shift_digits!(index, digit_count, 1);
            write_digit!(bytes, index, d1);
        } else {
            write_digit!(bytes, index, d2);
            write_digit!(bytes, index, d1);
        }
    } else {
        debug_assert!(index == 1);

        // SAFETY: this is always safe, since `index == 1`.
        write_digit!(bytes, index, s32);
    }

    digit_count
}

// EXTENDED
// --------

/// Create extended float from significant digits and exponent.
#[inline(always)]
pub const fn extended_float(mant: u64, exp: i32) -> ExtendedFloat80 {
    ExtendedFloat80 {
        mant,
        exp,
    }
}

// COMPUTE
// -------

#[inline(always)]
pub const fn floor_log2(mut n: u64) -> i32 {
    let mut count = -1;
    while n != 0 {
        count += 1;
        n >>= 1;
    }
    count
}

#[inline(always)]
pub const fn is_endpoint(exponent: i32, lower: i32, upper: i32) -> bool {
    exponent >= lower && exponent <= upper
}

#[inline(always)]
pub const fn is_right_endpoint(exponent: i32) -> bool {
    const LOWER_THRESHOLD: i32 = 0;
    const FACTORS: u32 = count_factors(5, (1 << (f64::MANTISSA_SIZE + 1)) + 1) + 1;
    const UPPER_THRESHOLD: i32 = 2 + floor_log2(pow64(10, FACTORS) / 3);
    is_endpoint(exponent, LOWER_THRESHOLD, UPPER_THRESHOLD)
}

#[inline(always)]
pub const fn is_left_endpoint(exponent: i32) -> bool {
    const LOWER_THRESHOLD: i32 = 2;
    const FACTORS: u32 = count_factors(5, (1 << (f64::MANTISSA_SIZE + 2)) - 1) + 1;
    const UPPER_THRESHOLD: i32 = 2 + floor_log2(pow64(10, FACTORS) / 3);
    is_endpoint(exponent, LOWER_THRESHOLD, UPPER_THRESHOLD)
}

// MUL
// ---

#[inline(always)]
pub const fn umul128_upper64(x: u64, y: u64) -> u64 {
    let p = x as u128 * y as u128;
    (p >> 64) as u64
}

#[inline(always)]
pub const fn umul192_upper64(x: u64, hi: u64, lo: u64) -> u64 {
    let mut g0 = x as u128 * hi as u128;
    g0 += umul128_upper64(x, lo) as u128;
    (g0 >> 64) as u64
}

#[inline(always)]
pub const fn umul192_middle64(x: u64, hi: u64, lo: u64) -> u64 {
    let g01 = x.wrapping_mul(hi);
    let g10 = umul128_upper64(x, lo);
    g01.wrapping_add(g10)
}

#[inline(always)]
pub const fn umul96_upper32(x: u64, y: u64) -> u64 {
    umul128_upper64(x, y)
}

#[inline(always)]
pub const fn umul96_lower64(x: u64, y: u64) -> u64 {
    x.wrapping_mul(y)
}

// LOG
// ---

// These are much more efficient log routines than the ones
// provided by dragonbox, since they use only a mul and shr.

/// Calculate `x * log5(2)` quickly.
/// Generated by `etc/log.py`.
/// Only needs to be valid for values from `[-1492, 1492]`
#[inline(always)]
pub const fn floor_log5_pow2(q: i32) -> i32 {
    q.wrapping_mul(225799) >> 19
}

/// Calculate `x * log10(2)` quickly.
/// Generated by `etc/log.py`.
/// Only needs to be valid for values from `[-1700, 1700]`
#[inline(always)]
pub const fn floor_log10_pow2(q: i32) -> i32 {
    q.wrapping_mul(315653) >> 20
}

/// Calculate `x * log2(10)` quickly.
/// Generated by `etc/log.py`.
/// Only needs to be valid for values from `[-1233, 1233]`
#[inline(always)]
pub const fn floor_log2_pow10(q: i32) -> i32 {
    q.wrapping_mul(1741647) >> 19
}

/// Calculate `x * log5(2) - log5(3)` quickly.
/// Generated by `etc/log.py`.
/// Only needs to be valid for values from `[-2427, 2427]`
#[inline(always)]
pub const fn floor_log5_pow2_minus_log5_3(q: i32) -> i32 {
    q.wrapping_mul(451597).wrapping_sub(715764) >> 20
}

/// Calculate `(x * log10(2) - log10(4)) / 3` quickly.
/// Generated by `etc/log.py`.
/// Only needs to be valid for values from `[-1700, 1700]`
#[inline(always)]
pub const fn floor_log10_pow2_minus_log10_4_over_3(q: i32) -> i32 {
    // NOTE: these values aren't actually exact:
    //      They're off sfor -295 and 97, so any automated way of computing
    //      them will also be off.
    q.wrapping_mul(1262611).wrapping_sub(524031) >> 22
}

// POW
// ---

/// const fn to calculate `radix^exp`.
#[inline(always)]
pub const fn pow32(radix: u32, mut exp: u32) -> u32 {
    let mut p = 1;
    while exp > 0 {
        p *= radix;
        exp -= 1;
    }
    p
}

/// const fn to calculate `radix^exp`.
#[inline(always)]
pub const fn pow64(radix: u32, mut exp: u32) -> u64 {
    let mut p = 1;
    while exp > 0 {
        p *= radix as u64;
        exp -= 1;
    }
    p
}

/// Counter the number of powers of radix are in `n`.
#[inline(always)]
pub const fn count_factors(radix: usize, mut n: usize) -> u32 {
    let mut c = 0;
    while n % radix == 0 {
        n /= radix;
        c += 1;
    }
    c
}

// DIV
// ---

// Compute floor(n / 10^exp) for small exp.
// Precondition: n <= 2^a * 5^b (a = max_pow2, b = max_pow5)
#[inline(always)]
pub const fn divide_by_pow10(n: u64, exp: u32, max_pow2: i32, max_pow5: i32) -> u64 {
    // Specialize for 64-bit division by 1000.
    // Ensure that the correctness condition is met.
    let pow2 = max_pow2 + (floor_log2_pow10(exp as i32 + max_pow5) - (exp as i32 + max_pow5));
    if exp == 3 && pow2 < 70 {
        umul128_upper64(n, 0x8312_6e97_8d4f_df3c) >> 9
    } else {
        n / pow64(10, exp)
    }
}

/// Calculate the modular inverse for the type.
macro_rules! mod_inverse {
    ($t:ident, $a:ident) => {{
        // By Euler's theorem, a^phi(2^n) == 1 (mod 2^n),
        // where phi(2^n) = 2^(n-1), so the modular inverse of a is
        // a^(2^(n-1) - 1) = a^(1 + 2 + 2^2 + ... + 2^(n-2)).
        let mut mod_inverse: $t = 1;
        let mut i = 1;
        while i < <$t as Integer>::BITS {
            mod_inverse = mod_inverse.wrapping_mul(mod_inverse).wrapping_mul($a);
            i += 1;
        }
        mod_inverse
    }};
}

#[inline(always)]
pub const fn mod32_inverse(a: u32) -> u32 {
    mod_inverse!(u32, a)
}

#[inline(always)]
pub const fn mod64_inverse(a: u64) -> u64 {
    mod_inverse!(u64, a)
}

pub struct Div32Table<const N: usize> {
    mod_inv: [u32; N],
    max_quotients: [u32; N],
}

pub struct Div64Table<const N: usize> {
    mod_inv: [u64; N],
    max_quotients: [u64; N],
}

/// Generate a division table as a const fn.
macro_rules! div_table {
    ($t:ident, $table:ident, $modular_inverse:ident, $a:ident) => {{
        let mod_inverse = $modular_inverse($a);
        let mut mod_inv = [0; N];
        let mut max_quotients = [0; N];
        let mut pow_of_mod_inverse: $t = 1;
        let mut pow_of_a = 1;
        let mut i = 0;

        while i < N {
            mod_inv[i] = pow_of_mod_inverse;
            max_quotients[i] = $t::MAX / pow_of_a;

            pow_of_mod_inverse = pow_of_mod_inverse.wrapping_mul(mod_inverse);
            pow_of_a *= $a;
            i += 1;
        }

        $table {
            mod_inv,
            max_quotients,
        }
    }};
}

/// Generate a pre-computed table of u32 constants for division.
#[inline(always)]
pub const fn div32_table<const N: usize>(a: u32) -> Div32Table<N> {
    div_table!(u32, Div32Table, mod32_inverse, a)
}

/// Generate a pre-computed table of u64 constants for division.
#[inline(always)]
pub const fn div64_table<const N: usize>(a: u64) -> Div64Table<N> {
    div_table!(u64, Div64Table, mod64_inverse, a)
}

/// Granlund-Montgomery style fast division
#[inline]
pub const fn fast_div(
    n: u32,
    divisor: u32,
    max_precision: u32,
    additional_precision: u32,
) -> (u32, u32) {
    // max_precision ∊ (0, 32] && `n < 2^MAX_PRECISION`.

    let left_end = (((1 << (max_precision + additional_precision)) + divisor - 1) / divisor) as u32;
    let quotient = (n * left_end) >> (max_precision + additional_precision);
    let remainder = n - divisor * quotient;

    (quotient, remainder)
}

// ROUNDING
// --------

impl RoundMode {
    /// Zero out the lowest bit.
    #[inline(always)]
    pub const fn break_rounding_tie(&self, significand: u64) -> u64 {
        match self {
            RoundMode::Round => significand & !1u64,
            RoundMode::Truncate => significand - 1u64,
        }
    }
}

// INTERVAL TYPE
// -------------

/// Interval types for rounding modes to compute endpoints.
#[non_exhaustive]
pub enum IntervalType {
    Symmetric(bool),
    Closed,
    RightClosedLeftOpen,
}

impl IntervalType {
    /// Determine if the interval type is symmetric.
    #[inline(always)]
    pub fn is_symmetric(&self) -> bool {
        match self {
            Self::Symmetric(_) => true,
            Self::Closed => true,
            Self::RightClosedLeftOpen => false,
        }
    }

    /// Determine if we include the left endpoint.
    #[inline(always)]
    pub fn include_left_endpoint(&self) -> bool {
        match self {
            Self::Symmetric(closed) => *closed,
            Self::Closed => true,
            Self::RightClosedLeftOpen => false,
        }
    }

    /// Determine if we include the right endpoint.
    #[inline(always)]
    pub fn include_right_endpoint(&self) -> bool {
        match self {
            Self::Symmetric(closed) => *closed,
            Self::Closed => true,
            Self::RightClosedLeftOpen => true,
        }
    }
}

// ENDPOINTS
// ---------

/// Compute the left endpoint from a 64-bit power-of-5..
#[inline(always)]
pub fn compute_left_endpoint_u64<F: DragonboxFloat>(pow5: u64, beta_minus_1: i32) -> u64 {
    let zero_carry = pow5 >> (F::MANTISSA_SIZE as usize + 2);
    let mantissa_shift = 64 - F::MANTISSA_SIZE as usize - 1;
    (pow5 - zero_carry) >> (mantissa_shift as i32 - beta_minus_1)
}

#[inline(always)]
pub fn compute_right_endpoint_u64<F: DragonboxFloat>(pow5: u64, beta_minus_1: i32) -> u64 {
    let zero_carry = pow5 >> (F::MANTISSA_SIZE as usize + 1);
    let mantissa_shift = 64 - F::MANTISSA_SIZE as usize - 1;
    (pow5 + zero_carry) >> (mantissa_shift as i32 - beta_minus_1)
}

/// Determine if we should round up for the short interval case.
#[inline(always)]
pub fn compute_round_up_u64<F: DragonboxFloat>(pow5: u64, beta_minus_1: i32) -> u64 {
    let shift = 64 - f64::MANTISSA_SIZE - 2;
    ((pow5 >> (shift - beta_minus_1)) + 1) / 2
}

// TABLE
// -----

/// THe number of trailing zeros, pre-calculated for all values from 0-100.
const TRAILING_ZEROS: [u8; 100] = [
    2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0,
];

// DRAGONBOX FLOAT
// ---------------

/// Get the high bits from the power-of-5.
#[inline(always)]
pub const fn high(pow5: &(u64, u64)) -> u64 {
    pow5.0
}

/// Get the low bits from the power-of-5.
#[inline(always)]
pub const fn low(pow5: &(u64, u64)) -> u64 {
    pow5.1
}

/// Calculate the maximum possible power for the mantissa.
#[inline(always)]
pub fn max_power<F: DragonboxFloat>() -> i32 {
    let max = F::Unsigned::MAX.as_u64();
    let max_mantissa = max / pow64(10, F::KAPPA + 2);
    let mut k = 0i32;
    let mut p = 1u64;
    while p < max_mantissa {
        p *= 10;
        k += 1;
    }
    k
}

/// Check and calculate quotient for value by 10^N.
macro_rules! div10 {
    (@4 $table:ident, $n:ident, $quo:ident, $s:ident $(, $mul:ident)?) => {{
        // Is n divisible by 10^4?
        if $n & 0xf == 0 {
            $quo = ($n >> 4).wrapping_mul($table.mod_inv[4]);
            if ($quo <= $table.max_quotients[4]) {
                $n = $quo;
                $($mul = 10000;)?
                $s |= 0x4;
            }
        }
    }};

    (@2 $table:ident, $n:ident, $quo:ident, $s:ident $(, $mul:ident)?) => {{
        // Is n divisible by 10^2?
        if $n & 0x3 == 0 {
            $quo = ($n >> 2).wrapping_mul($table.mod_inv[2]);
            if ($quo <= $table.max_quotients[2]) {
                $n = $quo;
                $($mul = if $s == 4 { 100 } else { 1000000 };)?
                $s |= 0x2;
            }
        }
    }};

    (@1 $table:ident, $n:ident, $quo:ident, $s:ident $(, $mul:ident)?) => {{
        // Is n divisible by 10^2?
        if $n & 0x1 == 0 {
            $quo = ($n >> 1).wrapping_mul($table.mod_inv[1]);
            if ($quo <= $table.max_quotients[1]) {
                $n = $quo;
                $($mul = ($mul >> 1) * $table.mod_inv[1];)?
                $s |= 0x1;
            }
        }
    }};
}

/// Determine if `x` is divisible by `5^exp`.
macro_rules! divisible_by_pow5 {
    (Self:: $table:ident, $x:ident, $exp:ident) => {{
        // SAFETY: safe if `exp < TABLE_SIZE`.
        let mod_inv = unsafe { *Self::$table.mod_inv.get_unchecked($exp as usize) };
        let max_quo = unsafe { *Self::$table.max_quotients.get_unchecked($exp as usize) };
        $x.wrapping_mul(mod_inv as u64) <= max_quo as u64
    }};
}

/// Magic numbers for division by a power of 10.
struct Div10Info {
    magic_number: u32,
    bits_for_comparison: i32,
    threshold: u32,
    shift_amount: i32,
}

impl Div10Info {
    #[inline(always)]
    pub const fn comparison_mask(&self) -> u32 {
        (1u32 << self.bits_for_comparison) - 1
    }
}

const F32_DIV10_INFO: Div10Info = Div10Info {
    magic_number: 0xcccd,
    bits_for_comparison: 16,
    threshold: 0x3333,
    shift_amount: 19,
};

const F64_DIV10_INFO: Div10Info = Div10Info {
    magic_number: 0x147c29,
    bits_for_comparison: 12,
    threshold: 0xa3,
    shift_amount: 27,
};

macro_rules! check_div_pow10 {
    ($n:ident, $float:ident, $info:ident) => {{
        let mut res = $n * $info.magic_number;

        // The lowest N bits of (n & comparison_mask) must be zero, and
        // (n >> N) & comparison_mask must be at most threshold.
        let shr = $float::KAPPA;
        let shl = $info.bits_for_comparison as u32 - $float::KAPPA;
        let c = ((res >> shr) | (res << shl)) & $info.comparison_mask();

        res >>= $info.shift_amount;
        (res, c <= $info.threshold)
    }};
}

/// Magic numbers for division by a small power of 10.
struct SmallDiv10Info {
    magic_number: u32,
    shift_amount: i32,
}

const SMALL_F32_DIV10_INFO: SmallDiv10Info = SmallDiv10Info {
    magic_number: 0xcccd,
    shift_amount: 19,
};

const SMALL_F64_DIV10_INFO: SmallDiv10Info = SmallDiv10Info {
    magic_number: 0xa3d8,
    shift_amount: 22,
};

macro_rules! small_div_pow10 {
    ($n:ident, $info:ident) => {{
        ($n * $info.magic_number) >> $info.shift_amount
    }};
}

/// Trait with specialized methods for the Dragonbox algorithm.
pub trait DragonboxFloat: Float {
    /// Constant derived in Section 4.5 of the Dragonbox algorithm.
    const KAPPA: u32;
    /// Ceiling of the maximum number of float decimal digits + 1.
    /// Or, ceil((MANTISSA_SIZE + 1) / log2(10)) + 1.
    const DECIMAL_DIGITS: usize;

    const MAX_POW5_FACTOR: i32 = floor_log5_pow2(Self::MANTISSA_SIZE + 2);
    const TABLE_SIZE: usize = Self::MAX_POW5_FACTOR as usize + 1;
    const DIV5_THRESHOLD: i32 = floor_log2_pow10(Self::MAX_POW5_FACTOR + Self::KAPPA as i32 + 1);
    const DIV5_TABLE: Self::Table;

    type Power;
    type Table;

    /// Quick calculation for the number of significant digits in the float.
    fn digit_count(mantissa: u64) -> usize;

    /// Write the significant digits to a buffer.
    /// Does not handle rounding or truncated digits.
    ///
    /// # Safety
    ///
    /// Safe if `bytes` is large enough to hold a decimal string for mantissa.
    unsafe fn write_digits(bytes: &mut [u8], mantissa: u64) -> usize;

    /// Get the pre-computed Dragonbox power from the exponent.
    ///
    /// # Safety
    ///
    /// Safe as long as the exponent is within the valid power-of-5 range.
    unsafe fn dragonbox_power(exponent: i32) -> Self::Power;

    /// Compute the left endpoint for the shorter interval case.
    fn compute_left_endpoint(pow5: &Self::Power, beta_minus_1: i32) -> u64;

    /// Compute the right endpoint for the shorter interval case.
    fn compute_right_endpoint(pow5: &Self::Power, beta_minus_1: i32) -> u64;

    /// Handle rounding-up for the short interval case.
    fn compute_round_up(pow5: &Self::Power, beta_minus_1: i32) -> u64;

    fn compute_mul(u: u64, pow5: &Self::Power) -> u64;
    fn compute_mul_parity(two_f: u64, pow5: &Self::Power, beta_minus_1: i32) -> bool;
    fn compute_delta(pow5: &Self::Power, beta_minus_1: i32) -> u32;

    /// Handle trailing zeros, conditional on the float type.
    fn process_trailing_zeros(mantissa: u64, exponent: i32) -> (u64, i32);

    /// Remove trailing zeros from the float.
    fn remove_trailing_zeros(mantissa: u64) -> (u64, i32);

    /// Determine if two_f is divisible by 5^exp.
    ///
    /// # Safety
    ///
    /// Safe if `exp < TABLE_SIZE`.
    unsafe fn divisible_by_pow5(x: u64, exp: u32) -> bool;

    /// Determine if two_f is divisible by 2^exp.
    #[inline(always)]
    fn divisible_by_pow2(x: u64, exp: u32) -> bool {
        // Preconditions: exp >= 1 && x != 0
        x.trailing_zeros() >= exp
    }

    #[inline(always)]
    fn is_product_fc_pm_half(two_f: u64, exponent: i32, minus_k: i32) -> bool {
        let lower_threshold = -(Self::KAPPA as i32) - floor_log5_pow2(Self::KAPPA as i32);
        let upper_threshold = floor_log2_pow10(Self::KAPPA as i32 + 1);

        if exponent < lower_threshold {
            // Case I: f = fc +- 1/2
            false
        } else if exponent <= upper_threshold {
            // For k >= 0
            true
        } else if exponent > Self::DIV5_THRESHOLD {
            // For k < 0
            false
        } else {
            // SAFETY: safe since `minus_k <= MAX_POW5_FACTOR + 1`.
            unsafe { Self::divisible_by_pow5(two_f, minus_k as u32) }
        }
    }

    #[inline(always)]
    fn is_product_fc(two_f: u64, exponent: i32, minus_k: i32) -> bool {
        let lower_threshold = -(Self::KAPPA as i32) - 1 - floor_log5_pow2(Self::KAPPA as i32 + 1);
        let upper_threshold = floor_log2_pow10(Self::KAPPA as i32 + 1);

        // Case II: f = fc + 1
        // Case III: f = fc
        // Exponent for 5 is negative
        if exponent > Self::DIV5_THRESHOLD {
            false
        } else if exponent > upper_threshold {
            // SAFETY: safe since `minus_k <= MAX_POW5_FACTOR + 1`.
            unsafe { Self::divisible_by_pow5(two_f, minus_k as u32) }
        } else if exponent >= lower_threshold {
            // Both exponents are nonnegative
            true
        } else {
            // Exponent for 2 is negative
            Self::divisible_by_pow2(two_f, (minus_k - exponent + 1) as u32)
        }
    }

    // Replace n by floor(n / 10^N).
    // Returns true if and only if n is divisible by 10^N.
    // Precondition: n <= 10^(N+1)
    fn check_div_pow10(n: u32) -> (u32, bool);

    // Compute floor(n / 10^N) for small n and exp.
    // Precondition: n <= 10^(N+1)
    fn small_div_pow10(n: u32) -> u32;
}

impl DragonboxFloat for f32 {
    const KAPPA: u32 = 1;
    const DECIMAL_DIGITS: usize = 9;
    const DIV5_TABLE: Self::Table = div32_table::<{ Self::TABLE_SIZE }>(5);

    type Power = u64;
    type Table = Div32Table<{ Self::TABLE_SIZE }>;

    #[inline(always)]
    fn digit_count(mantissa: u64) -> usize {
        (mantissa as u32).digit_count()
    }

    #[inline(always)]
    unsafe fn write_digits(bytes: &mut [u8], mantissa: u64) -> usize {
        let digit_count = Self::digit_count(mantissa);
        unsafe { write_digits_u32(bytes, mantissa as u32, digit_count) }
    }

    #[inline(always)]
    unsafe fn dragonbox_power(exponent: i32) -> Self::Power {
        debug_assert!((SMALLEST_F32_POW5..=LARGEST_F32_POW5).contains(&exponent));
        let index = (exponent - SMALLEST_F32_POW5) as usize;
        // SAFETY: safe if the exponent is in the correct range.
        unsafe { index_unchecked!(DRAGONBOX32_POWERS_OF_FIVE[index]) }
    }

    #[inline(always)]
    fn compute_left_endpoint(pow5: &Self::Power, beta_minus_1: i32) -> u64 {
        compute_left_endpoint_u64::<Self>(*pow5, beta_minus_1)
    }

    #[inline(always)]
    fn compute_right_endpoint(pow5: &Self::Power, beta_minus_1: i32) -> u64 {
        compute_right_endpoint_u64::<Self>(*pow5, beta_minus_1)
    }

    #[inline(always)]
    fn compute_round_up(pow5: &Self::Power, beta_minus_1: i32) -> u64 {
        compute_round_up_u64::<Self>(*pow5, beta_minus_1)
    }

    #[inline(always)]
    fn compute_mul(u: u64, pow5: &Self::Power) -> u64 {
        umul96_upper32(u, *pow5)
    }

    #[inline(always)]
    fn compute_mul_parity(two_f: u64, pow5: &Self::Power, beta_minus_1: i32) -> bool {
        // beta_minus_1 ∊ [1, 64]
        ((umul96_lower64(two_f, *pow5) >> (64 - beta_minus_1)) & 1) != 0
    }

    #[inline(always)]
    fn compute_delta(pow5: &Self::Power, beta_minus_1: i32) -> u32 {
        (*pow5 >> (64 - 1 - beta_minus_1)) as u32
    }

    #[inline(always)]
    fn process_trailing_zeros(mantissa: u64, exponent: i32) -> (u64, i32) {
        // Policy is to remove the trailing zeros.
        let (mantissa, trailing) = Self::remove_trailing_zeros(mantissa);
        (mantissa, exponent + trailing)
    }

    #[inline(always)]
    fn remove_trailing_zeros(mantissa: u64) -> (u64, i32) {
        debug_assert!(mantissa <= u32::MAX as u64);
        debug_assert!(max_power::<Self>() == 7);

        // Efficient because we can do it in 32-bits.
        let mut n = mantissa as u32;
        let table = div32_table::<{ Self::DECIMAL_DIGITS }>(5);

        // Perform a binary search
        let mut quo: u32;
        let mut s: i32 = 0;
        div10!(@4 table, n, quo, s);
        div10!(@2 table, n, quo, s);
        div10!(@1 table, n, quo, s);

        (n as u64, s)
    }

    #[inline(always)]
    unsafe fn divisible_by_pow5(x: u64, exp: u32) -> bool {
        divisible_by_pow5!(Self::DIV5_TABLE, x, exp)
    }

    #[inline(always)]
    fn check_div_pow10(n: u32) -> (u32, bool) {
        check_div_pow10!(n, f32, F32_DIV10_INFO)
    }

    #[inline(always)]
    fn small_div_pow10(n: u32) -> u32 {
        small_div_pow10!(n, SMALL_F32_DIV10_INFO)
    }
}

impl DragonboxFloat for f64 {
    const KAPPA: u32 = 2;
    const DECIMAL_DIGITS: usize = 17;
    const DIV5_TABLE: Self::Table = div64_table::<{ Self::TABLE_SIZE }>(5);

    type Power = (u64, u64);
    type Table = Div64Table<{ Self::TABLE_SIZE }>;

    #[inline(always)]
    fn digit_count(mantissa: u64) -> usize {
        mantissa.digit_count()
    }

    #[inline(always)]
    unsafe fn write_digits(bytes: &mut [u8], mantissa: u64) -> usize {
        let digit_count = Self::digit_count(mantissa);
        unsafe { write_digits_u64(bytes, mantissa, digit_count) }
    }

    #[inline(always)]
    unsafe fn dragonbox_power(exponent: i32) -> Self::Power {
        debug_assert!((SMALLEST_F64_POW5..=LARGEST_F64_POW5).contains(&exponent));
        let index = (exponent - SMALLEST_F64_POW5) as usize;
        // SAFETY: safe if the exponent is in the correct range.
        unsafe { index_unchecked!(DRAGONBOX64_POWERS_OF_FIVE[index]) }
    }

    #[inline(always)]
    fn compute_left_endpoint(pow5: &Self::Power, beta_minus_1: i32) -> u64 {
        compute_left_endpoint_u64::<Self>(high(pow5), beta_minus_1)
    }

    #[inline(always)]
    fn compute_right_endpoint(pow5: &Self::Power, beta_minus_1: i32) -> u64 {
        compute_right_endpoint_u64::<Self>(high(pow5), beta_minus_1)
    }

    #[inline(always)]
    fn compute_round_up(pow5: &Self::Power, beta_minus_1: i32) -> u64 {
        compute_round_up_u64::<Self>(high(pow5), beta_minus_1)
    }

    #[inline(always)]
    fn compute_mul(u: u64, pow5: &Self::Power) -> u64 {
        umul192_upper64(u, high(pow5), low(pow5))
    }

    #[inline(always)]
    fn compute_mul_parity(two_f: u64, pow5: &Self::Power, beta_minus_1: i32) -> bool {
        // beta_minus_1 ∊ [1, 64]
        ((umul192_middle64(two_f, high(pow5), low(pow5)) >> (64 - beta_minus_1)) & 1) != 0
    }

    #[inline(always)]
    fn compute_delta(pow5: &Self::Power, beta_minus_1: i32) -> u32 {
        (high(pow5) >> (64 - 1 - beta_minus_1)) as u32
    }

    #[inline(always)]
    fn process_trailing_zeros(mantissa: u64, exponent: i32) -> (u64, i32) {
        // Policy is to ignore the trailing zeros.
        (mantissa, exponent)
    }

    #[inline(always)]
    fn remove_trailing_zeros(mantissa: u64) -> (u64, i32) {
        debug_assert!(max_power::<Self>() == 16);

        // Divide by 10^8 and reduce to 32-bits.
        // Since ret_value.significand <= (2^64 - 1) / 1000 < 10^17,
        // both of the quotient and the r should fit in 32-bits.
        let mut n = mantissa;
        let table = div32_table::<{ f32::DECIMAL_DIGITS }>(5);

        // If the number is divisible by 10^8, work with the quotient.
        let quo_pow10_8 = divide_by_pow10(n, 8, 54, 0) as u32;
        let mut rem = n.wrapping_sub(100000000.wrapping_mul(quo_pow10_8 as u64)) as u32;

        if rem == 0 {
            let mut n32 = quo_pow10_8;
            let mut quo32: u32;

            // Is n divisible by 10^8?
            // This branch is extremely unlikely.
            // I suspect it is impossible to get into this branch.
            if n32 & 0xff == 0 {
                quo32 = (n32 >> 8) * table.mod_inv[8];
                if quo32 <= table.max_quotients[8] {
                    n = quo32 as u64;
                    return (n, 16);
                }
            }

            // Otherwise, perform a binary search.
            let mut s: i32 = 8;

            div10!(@4 table, n32, quo32, s);
            div10!(@2 table, n32, quo32, s);
            div10!(@1 table, n32, quo32, s);

            (n32 as u64, s)
        } else {
            // If the number is not divisible by 10^8, work with the remainder.
            let mut quo32: u32;
            let mut mul: u32 = 100000000;
            let mut s: i32 = 0;

            div10!(@4 table, rem, quo32, s, mul);
            div10!(@2 table, rem, quo32, s, mul);
            div10!(@1 table, rem, quo32, s, mul);

            let n = rem as u64 + quo_pow10_8 as u64 * mul as u64;
            (n, s)
        }
    }

    #[inline(always)]
    unsafe fn divisible_by_pow5(x: u64, exp: u32) -> bool {
        divisible_by_pow5!(Self::DIV5_TABLE, x, exp)
    }

    #[inline(always)]
    fn check_div_pow10(n: u32) -> (u32, bool) {
        check_div_pow10!(n, f64, F64_DIV10_INFO)
    }

    #[inline(always)]
    fn small_div_pow10(n: u32) -> u32 {
        small_div_pow10!(n, SMALL_F64_DIV10_INFO)
    }
}
