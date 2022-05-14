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
use lexical_util::num::{AsPrimitive, Float};
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
        // First, move the digits right by 1 after leading digits.
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        let count = digit_count - leading_digits;
        unsafe {
            let buf = &mut index_unchecked_mut!(bytes[leading_digits..digit_count + 1]);
            safe_assert!(buf.len() > count);
            for i in (0..count).rev() {
                index_unchecked_mut!(buf[i + 1] = buf[i]);
            }
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
    let beta = exponent + floor_log2_pow10(-minus_k);

    // Compute xi and zi.
    // SAFETY: safe, since value must be finite and therefore in the correct range.
    let pow5 = unsafe { F::dragonbox_power(-minus_k) };
    let mut xi = F::compute_left_endpoint(&pow5, beta);
    let mut zi = F::compute_right_endpoint(&pow5, beta);

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
    let mut significand = F::compute_round_up(&pow5, beta);

    // When tie occurs, choose one of them according to the rule.
    let bits: i32 = F::MANTISSA_SIZE;
    let lower_threshold: i32 = -floor_log5_pow2_minus_log5_3(bits + 4) - 2 - bits;
    let upper_threshold: i32 = -floor_log5_pow2(bits + 2) - 2 - bits;

    let round_down = RoundMode::Round.prefer_round_down(significand);
    if round_down && exponent >= lower_threshold && exponent <= upper_threshold {
        significand -= 1;
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
    let beta = exponent + floor_log2_pow10(-minus_k);

    // Compute zi and deltai.
    // 10^kappa <= deltai < 10^(kappa + 1)
    let two_fc = mantissa << 1;
    let deltai = F::compute_delta(&pow5, beta);
    // For the case of binary32, the result of integer check is not correct for
    // 29711844 * 2^-82
    // = 6.1442653300000000008655037797566933477355632930994033813476... * 10^-18
    // and 29711844 * 2^-81
    // = 1.2288530660000000001731007559513386695471126586198806762695... * 10^-17,
    // and they are the unique counterexamples. However, since 29711844 is even,
    // this does not cause any problem for the endpoints calculations; it can only
    // cause a problem when we need to perform integer check for the center.
    // Fortunately, with these inputs, that branch is never executed, so we are fine.
    let (zi, is_z_integer) = F::compute_mul((two_fc | 1) << beta, &pow5);

    // Step 2: Try larger divisor; remove trailing zeros if necessary
    let big_divisor = pow32(10, F::KAPPA + 1);
    let small_divisor = pow32(10, F::KAPPA);

    // Using an upper bound on zi, we might be able to optimize the division
    // better than the compiler; we are computing zi / big_divisor here.
    let exp = F::KAPPA + 1;
    let n_max = (1 << (F::MANTISSA_SIZE + 1)) * big_divisor as u64 - 1;
    let mut significand = F::divide_by_pow10(zi, exp, n_max);
    let mut r = (zi - (big_divisor as u64).wrapping_mul(significand)) as u32;

    // Get the interval type.
    // Must be Round since we only use compute_round with a round-nearest direction.
    let interval_type = IntervalType::Symmetric(is_even);

    // Check for short-circuit.
    // We use this, since the `goto` statements in dragonbox are unidiomatic
    // in Rust and lead to unmaintainable code. Using a simple closure is much
    // simpler, however, we do store a boolean in some cases to determine
    // if we need to short-circuit.
    let mut should_short_circuit = true;
    if r < deltai {
        // Exclude the right endpoint if necessary.
        let include_right = interval_type.include_right_endpoint();
        if r == 0 && !include_right && is_z_integer {
            significand -= 1;
            r = big_divisor;
            should_short_circuit = false;
        }
    } else if r > deltai {
        should_short_circuit = false;
    } else {
        // r == deltai; compare fractional parts.
        // Due to the more complex logic in the new dragonbox algorithm,
        // it's much easier logically to store if we should short circuit,
        // the default, and only mark
        let two_fl = two_fc - 1;
        let include_left = interval_type.include_left_endpoint();

        if !include_left || exponent < F::FC_PM_HALF_LOWER || exponent > F::DIV_BY_5_THRESHOLD {
            // If the left endpoint is not included, the condition for
            // success is z^(f) < delta^(f) (odd parity).
            // Otherwise, the inequalities on exponent ensure that
            // x is not an integer, so if z^(f) >= delta^(f) (even parity), we in fact
            // have strict inequality.
            let parity = F::compute_mul_parity(two_fl, &pow5, beta).0;
            if !parity {
                should_short_circuit = false;
            }
        } else {
            let (xi_parity, x_is_integer) = F::compute_mul_parity(two_fl, &pow5, beta);
            if !xi_parity && !x_is_integer {
                should_short_circuit = false
            }
        }
    }

    if should_short_circuit {
        // Short-circuit case.
        let (mant, exp) = F::process_trailing_zeros(significand, minus_k + F::KAPPA as i32 + 1);
        extended_float(mant, exp)
    } else {
        // Step 3: Find the significand with the smaller divisor
        significand *= 10;

        let dist = r - (deltai / 2) + (small_divisor / 2);
        let approx_y_parity = ((dist ^ (small_divisor / 2)) & 1) != 0;

        // Is dist divisible by 10^kappa?
        let (dist, is_dist_div_by_kappa) = F::check_div_pow10(dist);

        // Add dist / 10^kappa to the significand.
        significand += dist as u64;

        if is_dist_div_by_kappa {
            // Check z^(f) >= epsilon^(f).
            // We have either yi == zi - epsiloni or yi == (zi - epsiloni) - 1,
            // where yi == zi - epsiloni if and only if z^(f) >= epsilon^(f).
            // Since there are only 2 possibilities, we only need to care about the
            // parity. Also, zi and r should have the same parity since the divisor is
            // an even number.
            let (yi_parity, is_y_integer) = F::compute_mul_parity(two_fc, &pow5, beta);
            let round_down = RoundMode::Round.prefer_round_down(significand);

            if yi_parity != approx_y_parity || (is_y_integer && round_down) {
                // If z^(f) >= epsilon^(f), we might have a tie
                // when z^(f) == epsilon^(f), or equivalently, when y is an integer.
                // For tie-to-up case, we can just choose the upper one.
                //significand -= 1;
                significand -= 1;
            }
        }

        // Ensure we haven't re-assigned exponent or minus_k, since this
        // is a massive potential security vulnerability.
        debug_assert!(float.exponent() == exponent);
        debug_assert!(minus_k == floor_log10_pow2(exponent) - F::KAPPA as i32);

        extended_float(significand, minus_k + F::KAPPA as i32)
    }
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
    let beta = exponent + floor_log2_pow10(-minus_k);

    // Compute zi and deltai.
    // 10^kappa <= deltai < 10^(kappa + 1)
    let two_fc = mantissa << 1;
    let deltai = F::compute_delta(&pow5, beta);
    let (mut xi, mut is_x_integer) = F::compute_mul(two_fc << beta, &pow5);

    // Deal with the unique exceptional cases
    // 29711844 * 2^-82
    // = 6.1442653300000000008655037797566933477355632930994033813476... * 10^-18
    // and 29711844 * 2^-81
    // = 1.2288530660000000001731007559513386695471126586198806762695... * 10^-17
    // for binary32.
    if F::BITS == 32 && exponent <= -80 {
        is_x_integer = false;
    }

    if !is_x_integer {
        xi += 1;
    }

    // Step 2: Try larger divisor; remove trailing zeros if necessary
    let big_divisor = pow32(10, F::KAPPA + 1);

    // Using an upper bound on xi, we might be able to optimize the division
    // better than the compiler; we are computing xi / big_divisor here.
    let exp = F::KAPPA + 1;
    let n_max = (1 << (F::MANTISSA_SIZE + 1)) * big_divisor as u64 - 1;
    let mut significand = F::divide_by_pow10(xi, exp, n_max);
    let mut r = (xi - (big_divisor as u64).wrapping_mul(significand)) as u32;

    if r != 0 {
        significand += 1;
        r = big_divisor - r;
    }

    // Check for short-circuit.
    // We use this, since the `goto` statements in dragonbox are unidiomatic
    // in Rust and lead to unmaintainable code. Using a simple closure is much
    // simpler, however, we do store a boolean in some cases to determine
    // if we need to short-circuit.
    let mut should_short_circuit = true;
    if r > deltai {
        should_short_circuit = false;
    } else if r == deltai {
        // Compare the fractional parts.
        // This branch is never taken for the exceptional cases
        // 2f_c = 29711482, e = -81
        // (6.1442649164096937243516663440523473127541365101933479309082... * 10^-18)
        // and 2f_c = 29711482, e = -80
        // (1.2288529832819387448703332688104694625508273020386695861816... * 10^-17).
        let (zi_parity, is_z_integer) = F::compute_mul_parity(two_fc + 2, &pow5, beta);
        if zi_parity || is_z_integer {
            should_short_circuit = false;
        }
    }

    if should_short_circuit {
        let (mant, exp) = F::process_trailing_zeros(significand, minus_k + F::KAPPA as i32 + 1);
        extended_float(mant, exp)
    } else {
        // Step 3: Find the significand with the smaller divisor
        significand *= 10;
        significand -= F::div_pow10(r) as u64;

        // Ensure we haven't re-assigned exponent or minus_k, since this
        // is a massive potential security vulnerability.
        debug_assert!(float.exponent() == exponent);
        debug_assert!(minus_k == floor_log10_pow2(exponent) - F::KAPPA as i32);

        extended_float(significand, minus_k + F::KAPPA as i32)
    }
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
    let beta = exponent + floor_log2_pow10(-minus_k);

    // Compute zi and deltai.
    // 10^kappa <= deltai < 10^(kappa + 1)
    let two_fc = mantissa << 1;
    let deltai = F::compute_delta(&pow5, beta - shorter as i32);
    let zi = F::compute_mul(two_fc << beta, &pow5).0;

    // Step 2: Try larger divisor; remove trailing zeros if necessary
    let big_divisor = pow32(10, F::KAPPA + 1);

    // Using an upper bound on zi, we might be able to optimize the division better than
    // the compiler; we are computing zi / big_divisor here.
    let exp = F::KAPPA + 1;
    let n_max = (1 << (F::MANTISSA_SIZE + 1)) * big_divisor as u64 - 1;
    let mut significand = F::divide_by_pow10(zi, exp, n_max);
    let r = (zi - (big_divisor as u64).wrapping_mul(significand)) as u32;

    // Check for short-circuit.
    // We use this, since the `goto` statements in dragonbox are unidiomatic
    // in Rust and lead to unmaintainable code. Using a simple closure is much
    // simpler, however, we do store a boolean in some cases to determine
    // if we need to short-circuit.
    let mut should_short_circuit = true;
    if r > deltai {
        should_short_circuit = false;
    } else if r == deltai {
        // Compare the fractional parts.
        let two_f = two_fc
            - if shorter {
                1
            } else {
                2
            };
        if !F::compute_mul_parity(two_f, &pow5, beta).0 {
            should_short_circuit = false;
        }
    }

    if should_short_circuit {
        let (mant, exp) = F::process_trailing_zeros(significand, minus_k + F::KAPPA as i32 + 1);
        extended_float(mant, exp)
    } else {
        // Step 3: Find the significand with the smaller divisor
        significand *= 10;
        significand -= F::div_pow10(r) as u64;

        // Ensure we haven't re-assigned exponent or minus_k, since this
        // is a massive potential security vulnerability.
        debug_assert!(float.exponent() == exponent);
        debug_assert!(minus_k == floor_log10_pow2(exponent - shorter as i32) - F::KAPPA as i32);

        extended_float(significand, minus_k + F::KAPPA as i32)
    }
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
pub const fn umul192_upper128(x: u64, hi: u64, lo: u64) -> (u64, u64) {
    let mut r = x as u128 * hi as u128;
    r += umul128_upper64(x, lo) as u128;
    ((r >> 64) as u64, r as u64)
}

#[inline(always)]
pub const fn umul192_lower128(x: u64, yhi: u64, ylo: u64) -> (u64, u64) {
    let hi = x.wrapping_mul(yhi);
    let hi_lo = x as u128 * ylo as u128;
    // NOTE: This can wrap exactly to 0, and this is desired.
    (hi.wrapping_add((hi_lo >> 64) as u64), hi_lo as u64)
}

#[inline(always)]
pub const fn umul96_upper64(x: u64, y: u64) -> u64 {
    umul128_upper64(x << 32, y)
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
// Precondition: exp >= 0.
#[inline(always)]
pub const fn divide_by_pow10_32(n: u32, exp: u32) -> u32 {
    // Specialize for 32-bit division by 100.
    // Compiler is supposed to generate the identical code for just writing
    // "n / 100", but for some reason MSVC generates an inefficient code
    // (mul + mov for no apparent reason, instead of single imul),
    // so we does this manually.
    if exp == 2 {
        ((n as u64 * 1374389535) >> 37) as u32
    } else {
        let divisor = pow32(exp as u32, 10);
        n / divisor
    }
}

// Compute floor(n / 10^exp) for small exp.
// Precondition: n <= n_max
#[inline(always)]
pub const fn divide_by_pow10_64(n: u64, exp: u32, n_max: u64) -> u64 {
    // Specialize for 64-bit division by 1000.
    // Ensure that the correctness condition is met.
    if exp == 3 && n_max <= 15534100272597517998 {
        umul128_upper64(n, 2361183241434822607) >> 7
    } else {
        let divisor = pow64(exp as u32, 10);
        n / divisor
    }
}

// ROUNDING
// --------

impl RoundMode {
    /// Determine if we should round down.
    #[inline(always)]
    pub const fn prefer_round_down(&self, significand: u64) -> bool {
        match self {
            RoundMode::Round => significand % 2 != 0,
            RoundMode::Truncate => true,
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

/// Compute the left endpoint from a 64-bit power-of-5.
#[inline(always)]
pub fn compute_left_endpoint_u64<F: DragonboxFloat>(pow5: u64, beta: i32) -> u64 {
    let zero_carry = pow5 >> (F::MANTISSA_SIZE as usize + 2);
    let mantissa_shift = 64 - F::MANTISSA_SIZE as usize - 1;
    (pow5 - zero_carry) >> (mantissa_shift as i32 - beta)
}

#[inline(always)]
pub fn compute_right_endpoint_u64<F: DragonboxFloat>(pow5: u64, beta: i32) -> u64 {
    let zero_carry = pow5 >> (F::MANTISSA_SIZE as usize + 1);
    let mantissa_shift = 64 - F::MANTISSA_SIZE as usize - 1;
    (pow5 + zero_carry) >> (mantissa_shift as i32 - beta)
}

/// Determine if we should round up for the short interval case.
#[inline(always)]
pub fn compute_round_up_u64<F: DragonboxFloat>(pow5: u64, beta: i32) -> u64 {
    let shift = 64 - F::MANTISSA_SIZE - 2;
    ((pow5 >> (shift - beta)) + 1) / 2
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

/// ROR instruction for 32-bit type.
#[inline(always)]
pub const fn rotr32(n: u32, r: u32) -> u32 {
    let r = r & 31;
    (n >> r) | (n << (32 - r))
}

/// ROR instruction for 64-bit type.
#[inline(always)]
pub const fn rotr64(n: u64, r: u64) -> u64 {
    let r = r & 63;
    (n >> r) | (n << (64 - r))
}

/// Magic numbers for division by a power of 10.
/// Replace n by floor(n / 10^N).
/// Returns true if and only if n is divisible by 10^N.
/// Precondition: n <= 10^(N+1)
/// !!It takes an in-out parameter!!
struct Div10Info {
    magic_number: u32,
    shift_amount: i32,
}

const F32_DIV10_INFO: Div10Info = Div10Info {
    magic_number: 6554,
    shift_amount: 16,
};

const F64_DIV10_INFO: Div10Info = Div10Info {
    magic_number: 656,
    shift_amount: 16,
};

macro_rules! check_div_pow10 {
    ($n:ident, $exp:literal, $float:ident, $info:ident) => {{
        // Make sure the computation for max_n does not overflow.
        debug_assert!($exp + 2 < floor_log10_pow2(31));
        debug_assert!($n as u64 <= pow64(10, $exp + 1));

        let n = $n.wrapping_mul($info.magic_number);
        let mask = (1u32 << $info.shift_amount) - 1;
        let r = (n & mask) < $info.magic_number;

        (n >> $info.shift_amount, r)
    }};
}

// These constants are efficient because we can do it in 32-bits.
const MOD_INV_5_U32: u32 = 0xCCCC_CCCD;
const MOD_INV_25_U32: u32 = MOD_INV_5_U32.wrapping_mul(MOD_INV_5_U32);
const MOD_INV_5_U64: u64 = 0xCCCC_CCCC_CCCC_CCCD;
const MOD_INV_25_U64: u64 = MOD_INV_5_U64.wrapping_mul(MOD_INV_5_U64);

macro_rules! div_pow10 {
    ($n:ident, $info:ident) => {{
        $n.wrapping_mul($info.magic_number) >> $info.shift_amount
    }};
}

/// Trait with specialized methods for the Dragonbox algorithm.
pub trait DragonboxFloat: Float {
    /// Constant derived in Section 4.5 of the Dragonbox algorithm.
    const KAPPA: u32;
    /// Ceiling of the maximum number of float decimal digits + 1.
    /// Or, ceil((MANTISSA_SIZE + 1) / log2(10)) + 1.
    const DECIMAL_DIGITS: usize;
    const FC_PM_HALF_LOWER: i32 = -(Self::KAPPA as i32) - floor_log5_pow2(Self::KAPPA as i32);
    const DIV_BY_5_THRESHOLD: i32 = floor_log2_pow10(Self::KAPPA as i32 + 1);

    type Power;

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

    fn compute_mul(u: u64, pow5: &Self::Power) -> (u64, bool);
    fn compute_mul_parity(two_f: u64, pow5: &Self::Power, beta_minus_1: i32) -> (bool, bool);
    fn compute_delta(pow5: &Self::Power, beta_minus_1: i32) -> u32;

    /// Handle trailing zeros, conditional on the float type.
    fn process_trailing_zeros(mantissa: u64, exponent: i32) -> (u64, i32);

    /// Remove trailing zeros from the float.
    fn remove_trailing_zeros(mantissa: u64) -> (u64, i32);

    /// Determine if two_f is divisible by 2^exp.
    #[inline(always)]
    fn divisible_by_pow2(x: u64, exp: u32) -> bool {
        // Preconditions: exp >= 1 && x != 0
        x.trailing_zeros() >= exp
    }

    // Replace n by floor(n / 10^N).
    // Returns true if and only if n is divisible by 10^N.
    // Precondition: n <= 10^(N+1)
    fn check_div_pow10(n: u32) -> (u32, bool);

    // Compute floor(n / 10^N) for small n and exp.
    // Precondition: n <= 10^(N+1)
    fn div_pow10(n: u32) -> u32;

    // Compute floor(n / 10^N) for small N.
    // Precondition: n <= n_max
    fn divide_by_pow10(n: u64, exp: u32, n_max: u64) -> u64;
}

impl DragonboxFloat for f32 {
    const KAPPA: u32 = 1;
    const DECIMAL_DIGITS: usize = 9;

    type Power = u64;

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
    fn compute_mul(u: u64, pow5: &Self::Power) -> (u64, bool) {
        let r = umul96_upper64(u, *pow5);
        (r >> 32, (r as u32) == 0)
    }

    #[inline(always)]
    fn compute_mul_parity(two_f: u64, pow5: &Self::Power, beta: i32) -> (bool, bool) {
        debug_assert!((1..64).contains(&beta));

        let r = umul96_lower64(two_f, *pow5);
        let parity = (r >> (64 - beta)) & 1;
        let is_integer = r >> (32 - beta);
        (parity != 0, is_integer == 0)
    }

    #[inline(always)]
    fn compute_delta(pow5: &Self::Power, beta: i32) -> u32 {
        (*pow5 >> (64 - 1 - beta)) as u32
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
        debug_assert!(mantissa != 0);

        let mut n = mantissa as u32;
        let mut quo: u32;
        let mut s: i32 = 0;
        loop {
            quo = rotr32(n.wrapping_mul(MOD_INV_25_U32), 2);
            if quo <= u32::MAX / 100 {
                n = quo;
                s += 2;
            } else {
                break;
            }
        }

        quo = rotr32(n.wrapping_mul(MOD_INV_5_U32), 1);
        if quo <= u32::MAX / 10 {
            n = quo;
            s |= 1;
        }
        (n as u64, s)
    }

    #[inline(always)]
    fn check_div_pow10(n: u32) -> (u32, bool) {
        check_div_pow10!(n, 1, f32, F32_DIV10_INFO)
    }

    #[inline(always)]
    fn div_pow10(n: u32) -> u32 {
        div_pow10!(n, F32_DIV10_INFO)
    }

    #[inline(always)]
    fn divide_by_pow10(n: u64, exp: u32, _: u64) -> u64 {
        divide_by_pow10_32(n as u32, exp) as u64
    }
}

impl DragonboxFloat for f64 {
    const KAPPA: u32 = 2;
    const DECIMAL_DIGITS: usize = 17;

    type Power = (u64, u64);

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
    fn compute_mul(u: u64, pow5: &Self::Power) -> (u64, bool) {
        let (hi, lo) = umul192_upper128(u, high(pow5), low(pow5));
        (hi, lo == 0)
    }

    #[inline(always)]
    fn compute_mul_parity(two_f: u64, pow5: &Self::Power, beta: i32) -> (bool, bool) {
        debug_assert!((1..64).contains(&beta));

        let (rhi, rlo) = umul192_lower128(two_f, high(pow5), low(pow5));
        let parity = (rhi >> (64 - beta)) & 1;
        let is_integer = (rhi << beta) | (rlo >> (64 - beta));
        (parity != 0, is_integer == 0)
    }

    #[inline(always)]
    fn compute_delta(pow5: &Self::Power, beta: i32) -> u32 {
        (high(pow5) >> (64 - 1 - beta)) as u32
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
        debug_assert!(mantissa != 0);

        // This magic number is ceil(2^90 / 10^8).
        let magic_number = 12379400392853802749u64;
        let nm = mantissa as u128 * magic_number as u128;

        // Is n is divisible by 10^8?
        let high = (nm >> 64) as u64;
        let mask = (1 << (90 - 64)) - 1;
        let low = nm as u64;
        if high & mask == 0 && low < magic_number {
            // If yes, work with the quotient.
            let mut n = (high >> (90 - 64)) as u32;
            let mut s: i32 = 8;
            let mut quo: u32;

            loop {
                quo = rotr32(n.wrapping_mul(MOD_INV_25_U32), 2);
                if quo <= u32::MAX / 100 {
                    n = quo;
                    s += 2;
                } else {
                    break;
                }
            }

            quo = rotr32(n.wrapping_mul(MOD_INV_5_U32), 1);
            if quo <= u32::MAX / 10 {
                n = quo;
                s |= 1;
            }

            (n as u64, s)
        } else {
            // If n is not divisible by 10^8, work with n itself.
            let mut n = mantissa;
            let mut s: i32 = 0;
            let mut quo: u64;

            loop {
                quo = rotr64(n.wrapping_mul(MOD_INV_25_U64), 2);
                if quo <= u64::MAX / 100 {
                    n = quo;
                    s += 2;
                } else {
                    break;
                }
            }

            quo = rotr64(n.wrapping_mul(MOD_INV_5_U64), 1);
            if quo <= u64::MAX / 10 {
                n = quo;
                s |= 1;
            }

            (n, s)
        }
    }

    #[inline(always)]
    fn check_div_pow10(n: u32) -> (u32, bool) {
        check_div_pow10!(n, 2, f64, F64_DIV10_INFO)
    }

    #[inline(always)]
    fn div_pow10(n: u32) -> u32 {
        div_pow10!(n, F64_DIV10_INFO)
    }

    #[inline(always)]
    fn divide_by_pow10(n: u64, exp: u32, n_max: u64) -> u64 {
        divide_by_pow10_64(n, exp, n_max)
    }
}

#[cfg(feature = "f16")]
macro_rules! dragonbox_unimpl {
    ($($t:ident)*) => ($(
        impl DragonboxFloat for $t {
            const KAPPA: u32 = 0;
            const DECIMAL_DIGITS: usize = 0;

            type Power = u64;

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
            fn compute_round_up(_: &Self::Power, _: i32) -> (u64, bool) {
                unimplemented!()
            }

            #[inline(always)]
            fn compute_mul(_: u64, _: &Self::Power) -> (u64, bool) {
                unimplemented!()
            }

            #[inline(always)]
            fn compute_mul_parity(_: u64, _: &Self::Power, _: i32) -> (bool, bool) {
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
            fn check_div_pow10(_: u32) -> (u32, bool) {
                unimplemented!()
            }

            #[inline(always)]
            fn div_pow10(_: u32) -> u32 {
                unimplemented!()
            }
        }
    )*);
}

#[cfg(feature = "f16")]
dragonbox_unimpl! { bf16 f16 }
