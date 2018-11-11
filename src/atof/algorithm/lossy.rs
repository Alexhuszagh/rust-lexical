//! Lossy algorithms for string-to-float conversions.

use ftoa::exponent_notation_char;
use util::*;
use super::overflowing::*;

// POWI

/// Use powi() iteratively.
///
/// * `value`   - Base value.
/// * `op`      - Operation {*, /}.
/// * `base`    - Floating-point base for exponent.
/// * `exp`     - Iteration exponent {+256, -256}.
/// * `count`   - Number of times to iterate.
/// * `rem`     - Remaining exponent after iteration.
macro_rules! stable_powi_impl {
    ($value:ident, $op:tt, $base:ident, $exp:expr, $count:ident, $rem:ident) => ({
        for _ in 0..$count {
            $value = $value $op powi($base, $exp);
        }
        if $rem != 0 {
            $value = $value $op powi($base, $rem)
        }
        $value
    })
}

/// Cached powers to get the desired exponent.
/// Make sure all values are < 1e300.
const POWI_EXPONENTS: [i32; 35] = [
    512, 512, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
    256, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128
];

/// Simplify base to powi to avoid bugs.
macro_rules! base_to_powi {
    ($base:expr) => (unsafe { *POWI_EXPONENTS.get_unchecked($base as usize - 2) })
}

/// Stable powi implementation, with a base value.
///
/// Although valid results will occur with an exponent or value of 0,
/// ideally, you should not pass any value as such to this function.
///
/// Use powi() with an integral exponent, both for speed and
/// stability. Don't go any an exponent of magnitude >1e300, for numerical
/// stability.
macro_rules! stable_powi {
    ($value:ident, $op:tt, $base:ident, $exponent:ident) => ({
        let base = $base as f64;
        let exp = base_to_powi!($base);
        // Choose a multipler of 5 for this since the exp is chosen
        // so at max 2.1 iterations occur to the max exponent.
        // 5 means any input value times the exponent must be insignificant.
        if $exponent > 5*exp {
            // Value is impossibly large, must be infinity.
            F64_INFINITY
        } else if $exponent < -5*exp {
            // Value is impossibly small, must be 0.
            0.0
        } else if $exponent < 0 {
            // negative exponent
            let count = $exponent / -exp;
            let rem = $exponent % exp;
            stable_powi_impl!($value, $op, base, -exp, count, rem)
        } else {
            // positive exponent
            let count = $exponent / exp;
            let rem = $exponent % exp;
            stable_powi_impl!($value, $op, base, exp, count, rem)
        }
    })
}

/// `powi` implementation that is more stable at extremely low powers.
///
/// Equivalent to `value * powi(base, exponent)`
fn stable_powi_multiplier(mut value: f64, base: u64, exponent: i32) -> f64 {
    stable_powi!(value, *, base, exponent)
}

/// `powi` implementation that is more stable at extremely low powers.
///
/// Equivalent to `value / powi(base, exponent)`
fn stable_powi_divisor(mut value: f64, base: u64, exponent: i32) -> f64 {
    stable_powi!(value, /, base, exponent)
}

// FRACTION

// Parse the integer portion.
// Use a float since for large numbers, this may even overflow an
// integer 64.
#[inline(always)]
unsafe extern "C" fn parse_integer(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    let mut integer: f64 = 0.0;
    let p = atoi_pointer!(integer, first, last, base, f64).0;
    (integer, p)
}

// Parse the fraction portion.
// Parse separately from the integer portion, since the small
// values for each may be too small to change the integer components
// representation **immediately**.
// For numeric stability, use this early.
#[inline(always)]
unsafe extern "C" fn parse_fraction(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    // Ensure if there's a decimal, there are trailing values, so
    // invalid floats like "0." lead to an error.
    if distance(first, last) > 1 && *first == b'.' {
        let mut fraction: f64 = 0.;
        let first = first.add(1);
        let mut f = first;
        loop {
            // This would get better numerical precision using Horner's method,
            // but that would require.
            let mut value: u64 = 0;
            let l = min!(last, f.add(12));
            f = atoi_pointer!(value, f, l, base, u64).0;
            let digits = distance(first, f) as i32;

            // Ignore leading 0s, just not we've passed them.
            if value != 0 {
                fraction += stable_powi_divisor(value as f64, base, digits);
            }

            // do/while condition
            if f == last || char_to_digit!(*f) >= base as u8 {
                break;
            }
        }
        (fraction, f)
    } else {
        (0.0, first)
    }
}

// EXPONENT

// Parse the exponential portion, if
// we have an `e[+-]?\d+`.
// We don't care about the pointer after this, so just use `atoi_value`.
#[inline(always)]
unsafe extern "C" fn parse_exponent(first: *const u8, last: *const u8, base: u64)
    -> (i32, *const u8)
{
    let mut p = first;
    let dist = distance(first, last);
    if dist > 1 && (*p).to_ascii_lowercase() == exponent_notation_char(base) {
        p = p.add(1);
        // Use atoi_sign so we can handle overflow differently for +/- numbers.
        // We care whether the value is positive.
        // Use is32::max_value() since it's valid in 2s complement for
        // positive or negative numbers, and will trigger a short-circuit.
        let (exponent, p, overflow, sign) = atoi_sign!(p, last, base, i32);
        let exponent = if overflow { i32::max_value() } else { exponent };
        let exponent = if sign == -1 { -exponent } else { exponent };
        (exponent, p)
    } else {
        (0, p)
    }
}

// PARSE

/// Parse the mantissa and exponent from a string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(crate) unsafe extern "C" fn parse_float(first: *const u8, last: *const u8, base: u64)
    -> (f64, i32, *const u8)
{
    // Parse components
    let (integer, p) = parse_integer(first, last, base);
    let (fraction, p) = parse_fraction(p, last, base);
    let (exponent, p) = parse_exponent(p, last, base);

    (integer + fraction, exponent, p)
}

/// Parse 32-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
#[allow(unused)]
pub(crate) unsafe extern "C" fn atof(first: *const u8, last: *const u8, base: u64)
    -> (f32, *const u8)
{
    let (value, p) = atod(first, last, base);
    (value as f32, p)
}

/// Parse 64-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
#[allow(unused)]
pub(crate) unsafe extern "C" fn atod(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    let (mut value, exponent, p) = parse_float(first, last, base);
    if exponent != 0 && value != 0.0 {
        value = stable_powi_multiplier(value, base, exponent);
    }
    (value, p)
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    // TODO(ahuszagh) Implement
}
