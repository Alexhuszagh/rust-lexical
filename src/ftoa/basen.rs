//! Adaptation of the V8 ftoa algorithm with a custom radix.
//!
//! This algorithm is adapted from the V8 codebase,
//! and may be found [here](https://github.com/v8/v8).

use util::*;
use super::util::*;

// FTOA BASEN
// ----------

/// Naive algorithm for converting a floating point to a custom radix.
///
/// `d` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
///
/// Adapted from the V8 implementation.
unsafe extern "C" fn ftoa_naive(d: f64, first: *mut u8, base: u64)
    -> *mut u8
{
    // Logic error, base should not be passed dynamically.
    debug_assert!(base >= 2 && base <= 36,"Numerical base must be from 2-36");

    // Assert no special cases remain, no non-zero values,
    // and no negative numbers.
    debug_assert!(!d.is_special());
    debug_assert!(d != 0.0);
    debug_assert!(d > 0.0);

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
    let mut buffer: [u8; SIZE] = uninitialized!();
    let buffer = buffer.as_mut_ptr();
    let initial_position: usize = SIZE / 2;
    let mut integer_cursor = initial_position;
    let mut fraction_cursor = initial_position;
    let bf = base as f64;

    // Split the value into an integer part and a fractional part.
    let mut integer = floor_f64(d);
    let mut fraction = d - integer;

    // We only compute fractional digits up to the input double's precision.
    let mut delta = 0.5 * (d.next_positive() - d);
    delta = max!(0.0.next_positive(), delta);
    debug_assert!(delta > 0.0);

    if fraction > delta {
        loop {
            // Shift up by one digit.
            fraction *= bf;
            delta *= bf;
            // Write digit.
            let digit = fraction as i32;
            *buffer.add(fraction_cursor) = digit_to_char!(digit);
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
                        // Reconstruct digit.
                        let c = *buffer.add(fraction_cursor);
                        let digit = char_to_digit!(c) as i32;
                        if digit <= base as i32 {
                            let idx = (digit + 1) as usize;
                            *buffer.add(fraction_cursor) = digit_to_char!(idx);
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
    while (integer / bf).exponent() > 0 {
        integer /= bf;
        integer_cursor -= 1;
        *buffer.add(integer_cursor) = b'0';
    }

    loop {
        let remainder = integer % bf;
        integer_cursor -= 1;
        let idx = remainder as usize;
        *buffer.add(integer_cursor) = digit_to_char!(idx);
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
            end = min!(fraction_cursor, start + MAX_DIGIT_LENGTH + 1);
        } else {
            start = integer_cursor;
            end = min!(fraction_cursor, start + MAX_DIGIT_LENGTH + 1);
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
        copy_nonoverlapping!(buf_first, p, dist);
        p = p.add(dist);

        // write the exponent component
        *p = exponent_notation_char(base);
        // Handle negative exponents.
        let mut exp: u32;
        p = p.add(1);
        if exponent < 0 {
            *p = b'-';
            p = p.add(1);
            exp = exponent.wrapping_neg() as u32;
        } else {
            exp = exponent as u32;
        }
        // Forward the exponent writer.
        let base = base as u32;
        return itoa_forward!(exp, p, base);

    } else {
        let mut p;
        // get component lengths
        let integer_length = initial_position - integer_cursor;
        let fraction_length = min!(fraction_cursor - initial_position, MAX_DIGIT_LENGTH - integer_length);

        // write integer component
        copy_nonoverlapping!(buffer.add(integer_cursor), first, integer_length);
        p = first.add(integer_length);

        // write fraction component
        if fraction_length > 0 {
            // fraction exists, write it
            *p = b'.';
            p = p.add(1);
            copy_nonoverlapping!(buffer.add(initial_position), p, fraction_length);
            p = p.add(fraction_length);
        } else {
            // no fraction, write decimal place
            copy_nonoverlapping!(b".0".as_ptr(), p, 2);
            p = p.add(2);
        }

        return p;
    }
}

// F32

/// Forward to double_basen.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
#[inline(always)]
pub(crate) unsafe extern "C" fn float_basen(f: f32, first: *mut u8, base: u64)
    -> *mut u8
{
    double_basen(f as f64, first, base)
}

// F64

/// Optimized algorithm for base10 numbers.
///
/// `d` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
#[inline(always)]
pub(crate) unsafe extern "C" fn double_basen(value: f64, first: *mut u8, base: u64)
    -> *mut u8
{
    ftoa_naive(value, first, base)
}
