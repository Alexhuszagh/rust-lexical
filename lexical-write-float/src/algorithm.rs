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
//!
//! Each one of the algorithms described here has the main implementation,
//! according to the reference Dragonbox paper, as well as an alias for
//! our own purposes. The existing algorithms include:
//!
//! 1. compute_nearest_normal
//! 2. compute_nearest_shorter
//! 3. compute_left_closed_directed
//! 4. compute_right_closed_directed
//!
//! `compute_nearest_normal` and `compute_nearest_shorter` are used for
//! round-nearest, tie-even and `compute_right_closed_directed` is used
//! for round-to-zero (see below for details).

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

use crate::float::{ExtendedFloat80, RawFloat};
use crate::options::{Options, RoundMode};
use crate::shared;
use crate::table::*;
#[cfg(feature = "f16")]
use lexical_util::bf16::bf16;
#[cfg(feature = "f16")]
use lexical_util::f16::f16;
use lexical_util::format::{NumberFormat, STANDARD};
use lexical_util::num::{AsPrimitive, Float, Integer};
use lexical_write_integer::decimal::DigitCount;
use lexical_write_integer::write::WriteInteger;

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
    debug_assert!(!float.is_special());
    debug_assert!(float >= F::ZERO);

    let fp = to_decimal(float);
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
        args => bytes, fp, sci_exp, options,
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
    sci_exp: i32,
    options: &Options,
) -> usize {
    // Config options.
    debug_assert_eq!(count_factors(10, fp.mant), 0);
    let format = NumberFormat::<{ FORMAT }> {};
    assert!(format.is_valid());
    let decimal_point = options.decimal_point();

    // Write the significant digits. Write at index 1, so we can shift 1
    // for the decimal point without intermediate buffers.
    // SAFETY: safe, if we have enough bytes to write the significant digits.
    let digits = unsafe { &mut index_unchecked_mut!(bytes[1..]) };
    let digit_count = unsafe { F::write_digits(digits, fp.mant) };

    // Truncate and round the significant digits.
    // SAFETY: safe since `digit_count < digits.len()`.
    let (digit_count, carried) =
        unsafe { shared::truncate_and_round_decimal(digits, digit_count, options) };
    let sci_exp = sci_exp + carried as i32;

    // Determine the exact number of digits to write.
    let exact_count = shared::min_exact_digits(digit_count, options);

    // Write any trailing digits.
    // SAFETY: safe, if we have enough bytes to write the significant digits.
    let mut cursor: usize;
    unsafe {
        index_unchecked_mut!(bytes[0] = bytes[1]);
        index_unchecked_mut!(bytes[1]) = decimal_point;

        if !format.no_exponent_without_fraction() && digit_count == 1 && options.trim_floats() {
            cursor = 1;
        } else if digit_count < exact_count {
            // Adjust the number of digits written, by appending zeros.
            cursor = digit_count + 1;
            let zeros = exact_count - digit_count;
            unsafe {
                slice_fill_unchecked!(index_unchecked_mut!(bytes[cursor..cursor + zeros]), b'0');
            }
            cursor += zeros;
        } else if digit_count == 1 {
            index_unchecked_mut!(bytes[2]) = b'0';
            cursor = 3;
        } else {
            cursor = digit_count + 1;
        }
    }

    // Now, write our scientific notation.
    // SAFETY: safe since bytes must be large enough to store all digits.
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
pub unsafe fn write_float_negative_exponent<F: DragonboxFloat, const FORMAT: u128>(
    bytes: &mut [u8],
    fp: ExtendedFloat80,
    sci_exp: i32,
    options: &Options,
) -> usize {
    debug_assert!(sci_exp < 0);
    debug_assert_eq!(count_factors(10, fp.mant), 0);

    // Config options.
    let decimal_point = options.decimal_point();
    let sci_exp = sci_exp.wrapping_neg() as usize;

    // Write our 0 digits.
    let mut cursor = sci_exp + 1;
    debug_assert!(cursor >= 2);
    // SAFETY: safe, if we have enough bytes to write the significant digits.
    unsafe {
        // We write 0 digits even over the decimal point, since we might have
        // to round carry over. This is rare, but it could happen, and would
        // require a shift after. The good news is: if we have a shift, we
        // only need to move 1 digit.
        let digits = &mut index_unchecked_mut!(bytes[..cursor]);
        slice_fill_unchecked!(digits, b'0');
    }

    // Write out our significant digits.
    // SAFETY: safe, if we have enough bytes to write the significant digits.
    let digits = unsafe { &mut index_unchecked_mut!(bytes[cursor..]) };
    let digit_count = unsafe { F::write_digits(digits, fp.mant) };

    // Truncate and round the significant digits.
    // SAFETY: safe since `cursor > 0 && cursor < digits.len()`.
    debug_assert!(cursor > 0);
    let (digit_count, carried) =
        unsafe { shared::truncate_and_round_decimal(digits, digit_count, options) };

    // Handle any trailing digits.
    let mut trimmed = false;
    if carried && cursor == 2 {
        // Rounded-up, and carried to the first byte, so instead of having
        // 0.9999, we have 1.0.
        // SAFETY: safe if `bytes.len() >= 3`.
        unsafe {
            index_unchecked_mut!(bytes[0]) = b'1';
            if options.trim_floats() {
                cursor = 1;
                trimmed = true;
            } else {
                index_unchecked_mut!(bytes[1]) = decimal_point;
                index_unchecked_mut!(bytes[2]) = b'0';
                cursor = 3;
            }
        }
    } else if carried {
        // Carried, so we need to remove 1 zero before our digits.
        // SAFETY: safe if `bytes.len() > cursor && cursor > 0`.
        unsafe {
            index_unchecked_mut!(bytes[1]) = decimal_point;
            index_unchecked_mut!(bytes[cursor - 1] = bytes[cursor]);
        }
    } else {
        // SAFETY: safe if `bytes.len() >= 2`.
        unsafe { index_unchecked_mut!(bytes[1]) = decimal_point };
        cursor += digit_count;
    }

    // Determine the exact number of digits to write.
    let exact_count = shared::min_exact_digits(digit_count, options);

    // Write any trailing digits.
    // Cursor is 1 if we trimmed floats, in which case skip this.
    if !trimmed && digit_count < exact_count {
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
pub unsafe fn write_float_positive_exponent<F: DragonboxFloat, const FORMAT: u128>(
    bytes: &mut [u8],
    fp: ExtendedFloat80,
    sci_exp: i32,
    options: &Options,
) -> usize {
    // Config options.
    debug_assert!(sci_exp >= 0);
    debug_assert_eq!(count_factors(10, fp.mant), 0);
    let decimal_point = options.decimal_point();

    // Write out our significant digits.
    // Let's be optimistic and try to write without needing to move digits.
    // This only works if the if the resulting leading digits, or `sci_exp + 1`,
    // is greater than the written digits. If not, we have to move digits after
    // and then adjust the decimal point. However, with truncating and remove
    // trailing zeros, we **don't** know the exact digit count **yet**.
    // SAFETY: safe, if we have enough bytes to write the significant digits.
    let digit_count = unsafe { F::write_digits(bytes, fp.mant) };
    let (mut digit_count, carried) =
        unsafe { shared::truncate_and_round_decimal(bytes, digit_count, options) };

    // Now, check if we have shift digits.
    let leading_digits = sci_exp as usize + 1 + carried as usize;
    let mut cursor: usize;
    let mut trimmed = false;
    if leading_digits >= digit_count {
        // Great: we have more leading digits than we wrote, can write trailing zeros
        // and an optional decimal point.
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let digits = &mut index_unchecked_mut!(bytes[digit_count..leading_digits]);
            slice_fill_unchecked!(digits, b'0');
        }
        cursor = leading_digits;
        digit_count = leading_digits;
        // Only write decimal point if we're not trimming floats.
        if !options.trim_floats() {
            // SAFETY: safe if `bytes.len() >= cursor + 2`.
            unsafe { index_unchecked_mut!(bytes[cursor]) = decimal_point };
            cursor += 1;
            unsafe { index_unchecked_mut!(bytes[cursor]) = b'0' };
            cursor += 1;
            digit_count += 1;
        } else {
            trimmed = true;
        }
    } else {
        // Need to shift digits internally, and write the decimal point.
        // First, move the digits by 1 after leading digits.
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        let count = digit_count - leading_digits;
        unsafe {
            let src = index_unchecked!(bytes[leading_digits..digit_count]).as_ptr();
            let dst = &mut index_unchecked_mut!(bytes[leading_digits + 1..digit_count + 1]);
            copy_unchecked!(dst, src, count);
        }

        // Now, write the decimal point.
        // SAFETY: safe if the above step was safe, since `leading_digits < digit_count`.
        unsafe { index_unchecked_mut!(bytes[leading_digits]) = decimal_point };
        cursor = digit_count + 1;
    }

    // Determine the exact number of digits to write.
    // Don't worry if we carried: we cannot write **MORE** digits if we've
    // already previously truncated the input.
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

/// Get an extended representation of the decimal float.
///
/// The returned float has a decimal exponent, and the significant digits
/// returned to the nearest mantissa. For example, `1.5f32` will return
/// `ExtendedFloat80 { mant: 15, exp: -1 }`, although trailing zeros
/// might not be removed.
///
/// This algorithm **only** fails when `float == 0.0`, and we want to
/// short-circuit anyway.
#[inline]
pub fn to_decimal<F: RawFloat>(float: F) -> ExtendedFloat80 {
    let bits = float.to_bits();
    let mantissa_bits = bits & F::MANTISSA_MASK;

    if (bits & !F::SIGN_MASK).as_u64() == 0 {
        return extended_float(0, 0);
    }

    // Shorter interval case; proceed like Schubfach.
    // One might think this condition is wrong, since when exponent_bits == 1
    // and two_fc == 0, the interval is actullay regular. However, it turns out
    // that this seemingly wrong condition is actually fine, because the end
    // result is anyway the same.
    //
    // [binary32]
    // (fc-1/2) * 2^e = 1.175'494'28... * 10^-38
    // (fc-1/4) * 2^e = 1.175'494'31... * 10^-38
    //    fc    * 2^e = 1.175'494'35... * 10^-38
    // (fc+1/2) * 2^e = 1.175'494'42... * 10^-38
    //
    // Hence, shorter_interval_case will return 1.175'494'4 * 10^-38.
    // 1.175'494'3 * 10^-38 is also a correct shortest representation that will
    // be rejected if we assume shorter interval, but 1.175'494'4 * 10^-38 is
    // closer to the true value so it doesn't matter.
    //
    // [binary64]
    // (fc-1/2) * 2^e = 2.225'073'858'507'201'13... * 10^-308
    // (fc-1/4) * 2^e = 2.225'073'858'507'201'25... * 10^-308
    //    fc    * 2^e = 2.225'073'858'507'201'38... * 10^-308
    // (fc+1/2) * 2^e = 2.225'073'858'507'201'63... * 10^-308
    //
    // Hence, shorter_interval_case will return 2.225'073'858'507'201'4 *
    // 10^-308. This is indeed of the shortest length, and it is the unique one
    // closest to the true value among valid representations of the same length.

    // Toward zero case:
    //
    // What we need is a compute-nearest, but with truncated digits in the
    // truncated case. Note that we don't need the left-closed direct
    // rounding case of I = [w,w+), or right-closed directed rounding
    // case of I = (w−,w], since these produce the shortest intervals for
    // a **float parser** assuming the rounding of the float-parser.
    // The left-directed case assumes the float parser will round-down,
    // while the right-directed case assumed the float parser will round-up.
    //
    // A few examples of this behavior is described here:
    //    **compute_nearest_normal**
    //
    //    - `1.23456 => (123456, -5)` for binary32.
    //    - `1.23456 => (123456, -5)` for binary64.
    //    - `13.9999999999999982236431606 => (13999999999999998, -15)` for binary64.
    //
    //     **compute_left_closed_directed**
    //
    //    - `1.23456 => (12345601, -7)` for binary32.
    //    - `1.23456 => (12345600000000002, -16)` for binary64.
    //    - `13.9999999999999982236431606 => (13999999999999999, -15)` for binary64.
    //
    //     **compute_right_closed_directed**
    //
    //    - `1.23456 => (123456, -5)` for binary32.
    //    - `1.23456 => (123456, -5)` for binary64.
    //    - `13.9999999999999982236431606 => (13999999999999982, -15)` for binary64.

    if mantissa_bits.as_u64() == 0 {
        compute_round_short(float)
    } else {
        compute_round(float)
    }
}

/// Compute for a simple case when rounding nearest, tie-even.
#[inline(always)]
pub fn compute_round_short<F: RawFloat>(float: F) -> ExtendedFloat80 {
    compute_nearest_shorter(float)
}

/// Compute for a non-simple case when rounding nearest, tie-even.
#[inline(always)]
pub fn compute_round<F: RawFloat>(float: F) -> ExtendedFloat80 {
    compute_nearest_normal(float)
}

/// Compute the interval I = [m−w,m+w] if even, otherwise, (m−w,m+w).
/// This is the simple case for a finite number where only the hidden bit is set.
pub fn compute_nearest_shorter<F: RawFloat>(float: F) -> ExtendedFloat80 {
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
    if !interval_type.include_right_endpoint() && is_right_endpoint::<F>(exponent) {
        zi -= 1;
    }

    // If the left endpoint is not an integer, increase it.
    if !(interval_type.include_left_endpoint() && is_left_endpoint::<F>(exponent)) {
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

    // When tie occurs, choose one of them according to the rule.
    let bits: i32 = F::MANTISSA_SIZE;
    let lower_threshold: i32 = -floor_log5_pow2_minus_log5_3(bits + 4) - 2 - bits;
    let upper_threshold: i32 = -floor_log5_pow2(bits + 2) - 2 - bits;

    if exponent >= lower_threshold && exponent <= upper_threshold {
        significand = RoundMode::Round.break_rounding_tie(significand);
    } else if significand < xi {
        significand += 1;
    }

    // Ensure we haven't re-assigned exponent or minus_k, since this
    // is a massive potential security vulnerability.
    debug_assert!(float.exponent() == exponent);
    debug_assert!(minus_k == floor_log10_pow2_minus_log10_4_over_3(exponent));

    extended_float(significand, minus_k)
}

/// Compute the interval I = [m−w,m+w] if even, otherwise, (m−w,m+w).
/// This is the normal case for a finite number with non-zero significant digits.
#[allow(clippy::comparison_chain)]
pub fn compute_nearest_normal<F: RawFloat>(float: F) -> ExtendedFloat80 {
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
        if (include_left && is_prod) || is_mul_parity {
            return short_circuit();
        }
    }

    // Step 3: Find the significand with the smaller divisor
    significand *= 10;

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
                significand = RoundMode::Round.break_rounding_tie(significand);
            }
        }
    }

    // Ensure we haven't re-assigned exponent or minus_k, since this
    // is a massive potential security vulnerability.
    debug_assert!(float.exponent() == exponent);
    debug_assert!(minus_k == floor_log10_pow2(exponent) - F::KAPPA as i32);

    extended_float(significand, minus_k + F::KAPPA as i32)
}

/// Compute the interval I = [w,w+).
#[allow(clippy::comparison_chain)]
pub fn compute_left_closed_directed<F: RawFloat>(float: F) -> ExtendedFloat80 {
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

    // Ensure we haven't re-assigned exponent or minus_k, since this
    // is a massive potential security vulnerability.
    debug_assert!(float.exponent() == exponent);
    debug_assert!(minus_k == floor_log10_pow2(exponent) - F::KAPPA as i32);

    extended_float(significand, minus_k + F::KAPPA as i32)
}

/// Compute the interval I = (w−,w]..
#[allow(clippy::comparison_chain, clippy::if_same_then_else)]
pub fn compute_right_closed_directed<F: RawFloat>(float: F, shorter: bool) -> ExtendedFloat80 {
    let mantissa = float.mantissa().as_u64();
    let exponent = float.exponent();

    // Step 1: Schubfach multiplier calculation
    // Compute k and beta.
    let minus_k = floor_log10_pow2(exponent - shorter as i32) - F::KAPPA as i32;
    // SAFETY: safe, since value must be finite and therefore in the correct range.
    let pow5 = unsafe { F::dragonbox_power(-minus_k) };
    let beta_minus_1 = exponent + floor_log2_pow10(-minus_k);

    // Compute zi and deltai.
    // 10^kappa <= deltai < 10^(kappa + 1)
    let two_fc = mantissa << 1;
    let deltai = F::compute_delta(&pow5, beta_minus_1 - shorter as i32);
    let zi = F::compute_mul(two_fc << beta_minus_1, &pow5);

    // Step 2: Try larger divisor; remove trailing zeros if necessary
    let big_divisor = pow32(10, F::KAPPA + 1);

    // Using an upper bound on zi, we might be able to optimize the division
    // better than the compiler; we are computing zi / big_divisor here.
    let exp = F::KAPPA + 1;
    let max_pow2 = F::MANTISSA_SIZE + F::KAPPA as i32 + 2;
    let max_pow5 = F::KAPPA as i32 + 1;
    let mut significand = divide_by_pow10(zi, exp, max_pow2, max_pow5);
    let r = (zi - big_divisor as u64 * significand) as u32;

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
        let two_f = two_fc
            - if shorter {
                1
            } else {
                2
            };
        if F::compute_mul_parity(two_f, &pow5, beta_minus_1) {
            return short_circuit();
        }
    }

    // Step 3: Find the significand with the smaller divisor
    significand *= 10;
    significand -= F::small_div_pow10(r) as u64;

    // Ensure we haven't re-assigned exponent or minus_k, since this
    // is a massive potential security vulnerability.
    debug_assert!(float.exponent() == exponent);
    debug_assert!(minus_k == floor_log10_pow2(exponent - shorter as i32) - F::KAPPA as i32);

    extended_float(significand, minus_k + F::KAPPA as i32)
}

// DIGITS
// ------

//  NOTE:
//      Dragonbox has a heavily-branched, dubiously optimized algorithm using
//      fast division, that leads to no practical performance benefits in my
//      benchmarks, and the division algorithm is at best ~3% faster. It also
//      tries to avoid writing digits extensively, but requires division operations
//      for each step regardless, which means the **actual** overhead of said
//      branching likely exceeds any benefits. The code is also impossible to
//      maintain, and in my benchmarks is slower (by a small amount) for
//      a 32-bit mantissa, and a **lot** slower for a 64-bit mantissa,
//      where we need to trim trailing zeros.

/// Write the significant digits, when the significant digits can fit in a
/// 32-bit integer. Returns the number of digits written. This assumes any
/// trailing zeros have been removed.
///
/// # Safety
///
/// Safe if `bytes.len() >= 10`, since `u32::MAX` is at most 10 digits.
#[inline]
pub unsafe fn write_digits_u32(bytes: &mut [u8], mantissa: u32) -> usize {
    debug_assert!(bytes.len() >= 10);
    unsafe { mantissa.write_mantissa::<u32, { STANDARD }>(bytes) }
}

/// Write the significant digits, when the significant digits cannot fit in a
/// 32-bit integer. Returns the number of digits written. Note that this
/// might not be the same as the number of digits in the mantissa, since
/// trailing zeros will be removed.
///
/// # Safety
///
/// Safe if `bytes.len() >= 20`, since `u64::MAX` is at most 20 digits.
#[inline]
#[allow(clippy::branches_sharing_code)]
pub unsafe fn write_digits_u64(bytes: &mut [u8], mantissa: u64) -> usize {
    debug_assert!(bytes.len() >= 20);
    unsafe { mantissa.write_mantissa::<u64, { STANDARD }>(bytes) }
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
pub fn is_right_endpoint<F: Float>(exponent: i32) -> bool {
    let lower_threshold = 0;
    let factors = count_factors(5, (1u64 << (F::MANTISSA_SIZE + 1)) + 1) + 1;
    let upper_threshold = 2 + floor_log2(pow64(10, factors) / 3);
    is_endpoint(exponent, lower_threshold, upper_threshold)
}

#[inline(always)]
pub fn is_left_endpoint<F: Float>(exponent: i32) -> bool {
    let lower_threshold = 2;
    let factors = count_factors(5, (1u64 << (F::MANTISSA_SIZE + 2)) - 1) + 1;
    let upper_threshold = 2 + floor_log2(pow64(10, factors) / 3);
    is_endpoint(exponent, lower_threshold, upper_threshold)
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
    umul128_upper64(x, y) as u32 as _
}

#[inline(always)]
pub const fn umul96_lower64(x: u64, y: u64) -> u64 {
    x.wrapping_mul(y)
}

// LOG
// ---

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

/// Calculate `(x * log10(2) - log10(4 / 3))` quickly.
/// Generated by `etc/log.py`.
/// Only needs to be valid for values from `[-1700, 1700]`
#[inline(always)]
pub const fn floor_log10_pow2_minus_log10_4_over_3(q: i32) -> i32 {
    // NOTE: these values aren't actually exact:
    //      They're off for -295 and 97, so any automated way of computing
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
pub const fn count_factors(radix: u32, mut n: u64) -> u32 {
    let mut c = 0;
    while n != 0 && n % radix as u64 == 0 {
        n /= radix as u64;
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
    Asymmetric(bool),
    Closed,
    Open,
    LeftClosedRightOpen,
    RightClosedLeftOpen,
}

impl IntervalType {
    /// Determine if the interval type is symmetric.
    #[inline(always)]
    pub fn is_symmetric(&self) -> bool {
        match self {
            Self::Symmetric(_) => true,
            Self::Asymmetric(_) => false,
            Self::Closed => true,
            Self::Open => true,
            Self::LeftClosedRightOpen => false,
            Self::RightClosedLeftOpen => false,
        }
    }

    /// Determine if we include the left endpoint.
    #[inline(always)]
    pub fn include_left_endpoint(&self) -> bool {
        match self {
            Self::Symmetric(closed) => *closed,
            Self::Asymmetric(left_closed) => *left_closed,
            Self::Closed => true,
            Self::Open => false,
            Self::LeftClosedRightOpen => true,
            Self::RightClosedLeftOpen => false,
        }
    }

    /// Determine if we include the right endpoint.
    #[inline(always)]
    pub fn include_right_endpoint(&self) -> bool {
        match self {
            Self::Symmetric(closed) => *closed,
            Self::Asymmetric(left_closed) => !*left_closed,
            Self::Closed => true,
            Self::Open => false,
            Self::LeftClosedRightOpen => false,
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
    let shift = 64 - F::MANTISSA_SIZE - 2;
    ((pow5 >> (shift - beta_minus_1)) + 1) / 2
}

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
    //  NOTE:
    //      Dragonbox's reference implementation uses
    //      `max_mantissa = max / pow64(10, F::KAPPA + 1)`, but then does
    //      `p < max_mantissa / 10`, which produces the same result in all
    //      cases.
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
                $( $mul = ($mul >> 1).wrapping_mul($table.mod_inv[1]); )?
                $s |= 0x1;
            }
        }
    }};
}

/// Determine if `x` is divisible by `5^exp`.
///
/// # Safety
///
/// Safe if `exp < table.mod_inv.len()`
macro_rules! divisible_by_pow5 {
    (Self:: $table:ident, $x:ident, $exp:ident) => {{
        debug_assert!(($exp as usize) < Self::$table.mod_inv.len());
        let mod_inv = &Self::$table.mod_inv;
        let max_quotients = &Self::$table.max_quotients;
        // SAFETY: safe if `exp < TABLE_SIZE`.
        let mod_inv = unsafe { index_unchecked!(mod_inv[$exp as usize]) };
        let max_quo = unsafe { index_unchecked!(max_quotients[$exp as usize]) };
        $x.wrapping_mul(mod_inv) <= max_quo
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
            // SAFETY: safe since `minus_k < MAX_POW5_FACTOR + 1`.
            debug_assert!(minus_k < Self::MAX_POW5_FACTOR + 1);
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
            // SAFETY: safe since `minus_k < MAX_POW5_FACTOR + 1`.
            debug_assert!(minus_k < Self::MAX_POW5_FACTOR + 1);
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
        debug_assert!(mantissa <= u32::MAX as u64);
        (mantissa as u32).digit_count()
    }

    #[inline(always)]
    unsafe fn write_digits(bytes: &mut [u8], mantissa: u64) -> usize {
        debug_assert!(mantissa <= u32::MAX as u64);
        // SAFETY: safe is `bytes.len() >= 10`.
        unsafe { write_digits_u32(bytes, mantissa as u32) }
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
        debug_assert!(x <= u32::MAX as u64);
        let x = x as u32;
        // SAFETY: safe if `exp < Self::DIV5_TABLE.mod_inv.len()`.
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
        // SAFETY: safe if `bytes.len() >= 20`.
        unsafe { write_digits_u64(bytes, mantissa) }
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
        // Policy is to remove the trailing zeros.
        // This differs from dragonbox proper, but leads to faster benchmarks.
        let (mantissa, trailing) = Self::remove_trailing_zeros(mantissa);
        (mantissa, exponent + trailing)
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
            if n32 & 0xff == 0 {
                quo32 = (n32 >> 8).wrapping_mul(table.mod_inv[8]);
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
        // SAFETY: safe if `exp < Self::DIV5_TABLE.mod_inv.len()`.
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

#[cfg(feature = "f16")]
macro_rules! dragonbox_unimpl {
    ($($t:ident)*) => ($(
        impl DragonboxFloat for $t {
            const KAPPA: u32 = 0;
            const DECIMAL_DIGITS: usize = 0;
            const DIV5_TABLE: Self::Table = 0;

            type Power = u64;
            type Table = u8;

            #[inline(always)]
            fn digit_count(_: u64) -> usize {
                unimplemented!()
            }

            #[inline(always)]
            unsafe fn write_digits(_: &mut [u8], _: u64) -> usize {
                unimplemented!()
            }

            #[inline(always)]
            unsafe fn dragonbox_power(_: i32) -> Self::Power {
                unimplemented!()
            }

            #[inline(always)]
            fn compute_left_endpoint(_: &Self::Power, _: i32) -> u64 {
                unimplemented!()
            }

            #[inline(always)]
            fn compute_right_endpoint(_: &Self::Power, _: i32) -> u64 {
                unimplemented!()
            }

            #[inline(always)]
            fn compute_round_up(_: &Self::Power, _: i32) -> u64 {
                unimplemented!()
            }

            #[inline(always)]
            fn compute_mul(_: u64, _: &Self::Power) -> u64 {
                unimplemented!()
            }

            #[inline(always)]
            fn compute_mul_parity(_: u64, _: &Self::Power, _: i32) -> bool {
                unimplemented!()
            }

            #[inline(always)]
            fn compute_delta(_: &Self::Power, _: i32) -> u32 {
                unimplemented!()
            }

            #[inline(always)]
            fn process_trailing_zeros(_: u64, _: i32) -> (u64, i32) {
                unimplemented!()
            }

            #[inline(always)]
            fn remove_trailing_zeros(_: u64) -> (u64, i32) {
                unimplemented!()
            }

            #[inline(always)]
            unsafe fn divisible_by_pow5(_: u64, _: u32) -> bool {
                unimplemented!()
            }

            #[inline(always)]
            fn check_div_pow10(_: u32) -> (u32, bool) {
                unimplemented!()
            }

            #[inline(always)]
            fn small_div_pow10(_: u32) -> u32 {
                unimplemented!()
            }
        }
    )*);
}

#[cfg(feature = "f16")]
dragonbox_unimpl! { bf16 f16 }
