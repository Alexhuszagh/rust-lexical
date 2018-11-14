//! Correct algorithms for string-to-float conversions.

use atoi::{atoi_unchecked, atoi_sign};
use util::*;
use ftoa::exponent_notation_char;

#[cfg(any(test, feature = "correct"))]
use table::*;

// TODO(ahuszagh)
//  Base implementation off of:
// Use fast path or bigint as a fallback.
//  https://github.com/gcc-mirror/gcc/blob/master/libgo/go/strconv/atof.go
//  https://github.com/python/cpython/blob/e42b705188271da108de42b55d9344642170aa2b/Python/dtoa.c

// SHARED

/// Parse the exponential portion from a float-string, if we have an `(e|^)[+-]?\d+`.
///
/// On overflow, just return a comically large exponent, since we don't
/// care. It will lead to infinity regardless, and doesn't affect whether
/// the type is representable.
///
/// Returns the exponent and a pointer to the current buffer position.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(super) unsafe extern "C" fn parse_exponent(first: *const u8, last: *const u8, base: u64)
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
        let cb = atoi_unchecked::<i32>;
        let (exponent, p, overflow, sign) = atoi_sign::<i32, _>(base as u8, p, last, cb);
        let exponent = if overflow { i32::max_value() } else { exponent };
        let exponent = if sign == -1 { -exponent } else { exponent };
        (exponent, p)
    } else {
        (0, p)
    }
}

cfg_if! {
if #[cfg(any(test, feature = "correct"))] {
// Fast path for the parse algorithm.
// In this case, the mantissa can be represented by an integer,
// which allows any value to be exactly reconstructed.

use atoi::{atoi_checked, atoi_value};
use float::FloatType;
use super::cached;
use super::double;
use super::float;

// PARSE
// -----

/// Parse the mantissa from a string.
///
/// Returns the mantissa, number of digits since the dot was seen,
/// a pointer to the current buffer position, and if the mantissa was
/// truncated.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(super) unsafe extern "C" fn parse_mantissa(mut first: *const u8, last: *const u8, base: u64)
    -> (u64, i32, *const u8, bool)
{
    // Trim the leading 0s.
    // Need to force this here, since if not, conversion of usize dot to
    // i32 may truncate when mantissa does not, which would lead to faulty
    // results. If we trim the 0s here, we guarantee any time `dot as i32`
    // leads to a truncation, mantissa will overflow.
    first = ltrim_char(first, last, b'0');

    // Parse the integral value.
    // Use the checked parsers so the truncated value is valid even if
    // the entire value is not parsed.
    let cb = atoi_checked::<u64>;
    let (mut mantissa, f, truncated) = atoi_value::<u64, _>(base as u8, first, last, cb);
    if distance(f, last) > 1 && *f == b'.' {
        // Has a decimal, calculate the rest of it.
        let f = f.add(1);
        let tup = match mantissa {
            0 => {
                // Can ignore the leading digits while the mantissa is 0.
                // This allows us to represent extremely small values
                // using the fast route in non-scientific notation.
                // For example, this allows us to use the fast path for
                // both "1e-29" and "0.0000000000000000000000000001",
                // otherwise, only the former would work.
                let f = ltrim_char(f, last, b'0');
                cb(&mut mantissa, base, f, last)
            },
            _ => cb(&mut mantissa, base, f, last),
        };
        let dot = distance(f, tup.0) as i32;
        (mantissa, dot, tup.0, truncated | tup.1)
    } else {
        // No decimal, just return
        (mantissa, 0, f, truncated)
    }
}

/// Calculate the exact exponent without overflow.
///
/// Remove the number of digits that contributed to the mantissa past
/// the dot.
#[inline]
pub(super) extern "C" fn normalize_exponent(exponent: i32, dot: i32)
    -> i32
{
    // TODO(ahuszagh) Need to simplify this...
    match exponent {
         0x7FFFFFFF => i32::max_value(),
        -0x80000000 => i32::min_value(),
        _           => exponent - dot,
    }
}

/// Normalize the mantissa to check if it can use the fast-path.
///
/// Move digits from the mantissa to the exponent when possible.
#[inline]
pub(super) extern "C" fn normalize_mantissa(mut mantissa: u64, base: u64, mut exponent: i32)
    -> (u64, i32)
{
    let base2 = base * base;
    let base4 = base2 * base2;

    // Use power-reduction, we're likely never going to enter most of these
    // loops, but it minimizes the number of expensive operations we need
    // to do.
    while mantissa >= base4 && mantissa % base4 == 0 {
        mantissa /= base4;
        exponent += 4;
    }
    while mantissa >= base2 && mantissa % base2 == 0 {
        mantissa /= base2;
        exponent += 2;
    }
    if mantissa % base == 0 {
        mantissa /= base;
        exponent += 1;
    }
    (mantissa, exponent)
}

/// Parse the mantissa and exponent from a string.
///
/// Returns the mantissa, the exponent, number of digits since the dot
/// was seen, a pointer to the current buffer position, and if mantissa
/// was truncated.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(super) unsafe extern "C" fn parse_float(first: *const u8, last: *const u8, base: u64)
    -> (u64, i32, *const u8, bool)
{
    let (mantissa, dot, p, truncated) = parse_mantissa(first, last, base);
    let (exponent, p) = parse_exponent(p, last, base);
    let exponent = normalize_exponent(exponent, dot);
    let (mantissa, exponent) = normalize_mantissa(mantissa, base, exponent);
    (mantissa, exponent, p, truncated)
}

// EXACT
// -----

/// Macro to convert value to exact value, allowing code reuse.
/// Custom code can be executed inside the code-block to avoid code.
///
/// Returns the resulting float and if the value can be represented exactly.
// TODO(ahuszagh) Convert to function
macro_rules! to_exact {
    ($mantissa:ident, $base:ident, $exponent:ident, $min:ident, $max:ident, $f:tt, $mod:ident) =>
    ({
        // logic error, disable in release builds
        debug_assert!($base >= 2 && $base <= 36, "Numerical base must be from 2-36");

        if $mantissa >> $f::SIGNIFICAND_SIZE != 0 {
            // Would require truncation of the mantissa.
            (0.0, false)
        } else {
            let float = $mantissa as $f;
            if $exponent == 0 {
                // 0 exponent, same as value, exact representation.
                (float,  true)
            } else if $exponent >= $min && $exponent <= $max {
                // Value can be exactly represented, return the value.
                let float = match $base {
                    // TODO(ahuszagh) Make these a method on trait, so I can
                    // call it directly without a macro...
                    2  => $mod::pow2_to_exact(float, 1, $exponent),
                    4  => $mod::pow2_to_exact(float, 2, $exponent),
                    8  => $mod::pow2_to_exact(float, 3, $exponent),
                    16 => $mod::pow2_to_exact(float, 4, $exponent),
                    32 => $mod::pow2_to_exact(float, 5, $exponent),
                    _  => $mod::basen_to_exact(float, $base, $exponent),
                };
                (float, true)
            } else {
                // Cannot be exactly represented, exponent multiplication
                // would require truncation.
                (0.0, false)
            }
        }
    });
}

/// Convert mantissa and exponent to exact f32.
///
/// Return the exact float and if the exact conversion was successful.
#[inline]
pub(super) unsafe fn to_float_exact(mantissa: u64, base: u64, exponent: i32) -> (f32, bool) {
    let (min_exp, max_exp) = f32::exponent_limit(base);
    to_exact!(mantissa, base, exponent, min_exp, max_exp, f32, float)
}

/// Convert mantissa and exponent to exact f64.
///
/// Return the exact float and if the exact conversion was successful.
#[inline]
pub(super) unsafe fn to_double_exact(mantissa: u64, base: u64, exponent: i32) -> (f64, bool) {
    let (min_exp, max_exp) = f64::exponent_limit(base);
    to_exact!(mantissa, base, exponent, min_exp, max_exp, f64, double)
}

// EXTENDED
// --------

// Moderate path for the parse algorithm.
// In this case, the mantissa can be represented by an integer,
// however, the exponent cannot be represented without truncating bytes.

// EXTENDED

/// Count the relative error in the extended-float precision.
struct Errors {
    // Upper bound for the error, in scale * ulp
    count: u32,
}

impl Errors {
    /// Error scale
    const ERROR_SCALE: u32 = 8;

    #[inline(always)]
    fn new(truncated: bool) -> Errors {
        Errors { count: Errors::trunction(truncated) }
    }

    #[inline(always)]
    fn trunction(truncated: bool) -> u32 {
        truncated as u32 * Self::halfscale()
    }

    #[inline(always)]
    fn scale() -> u32 {
        Self::ERROR_SCALE
    }

    #[inline(always)]
    fn halfscale() -> u32 {
        Self::scale() / 2
    }
}


// TODO(ahuszagh)
// We need a way to check if it's a good fit...


/// Multiply the floating-point by the exponent.
///
/// Multiply by pre-calculated powers of the base, modify the extended-
/// float, and return if new value and if the value can be represented
/// accurately.
#[inline]
#[allow(unused)]    // TODO(ahuszagh) Remove
unsafe fn multiply_exponent_extended(mut fp: FloatType, base: u64, exponent: i32, truncated: bool)
    -> (FloatType, bool)
{
    let powers = cached::get_powers(base);
    let exponent = exponent + powers.bias;
    let large_index = exponent / powers.step;
    let small_index = exponent % powers.step;
    if exponent < 0 {
        // Underflow (assign 0)
        (FloatType { frac: 0, exp: 0 }, true)
    } else if large_index as usize >= powers.large.len() {
        // Overflow (assign infinity)
        (FloatType { frac: 1 << 63, exp: 0x7FF }, true)
    } else {
        // Within the valid exponent range, multiply by the large and small
        // exponents and return the resulting value.

        // Track errors to as a factor of unit in last-precision.
        let mut errors = Errors::new(truncated);

        // Multiply by the small power.
        // Check if we can directly multiply by an integer, if not,
        // use extended-precision multiplication.
        match fp.frac.overflowing_mul(powers.get_small_int(small_index as usize)) {
            // Overflow, multiplication unsuccessful, go slow path.
            (_, true)     => {
                fp.normalize();
                fp.imul(powers.get_small(small_index as usize));
                errors.count += Errors::halfscale();
            },
            // No overflow, multiplication successful.
            (frac, false) => {
                fp.frac = frac;
                fp.normalize();
            },
        }

        // Multiply by the large power
        fp.imul(powers.get_large(large_index as usize));
        errors.count += (errors.count > 0) as u32;
        errors.count += Errors::halfscale();

        // Normalize the floating point (and the errors).
        let shift = fp.normalize();
        errors.count <<= shift;

        // TODO(ahuszagh) Need to implement an error checking mechanism
        // https://golang.org/src/strconv/extfloat.go
        //  Line 239

//        let small = powers.small.get_unchecked(small_index as usize);
//        let large = powers.large.get_unchecked(large_index as usize);
//        fp.normalize();
//        let mut fp = fp.mul(large);
//        fp.normalize();
//        let mut fp = fp.mul(small);
//        fp.normalize();
        (fp, true)
    }
}

/// Create a precise f32 using an intermediate extended-precision float.
///
/// Return the float approximation and if the value can be accurately
/// represented with mantissa bits of precision.
#[inline]
pub(super) unsafe fn to_float_extended(mantissa: u64, base: u64, exponent: i32, truncated: bool)
    -> (f32, bool)
{
    let fp = FloatType { frac: mantissa, exp: 0 };
    let (fp, valid) = multiply_exponent_extended(fp, base, exponent, truncated);
    if valid {
        (fp.as_f32(), true)
    } else {
        (0.0, false)
    }
}

/// Create a precise f64 using an intermediate extended-precision float.
///
/// Return the float approximation and if the value can be accurately
/// represented with mantissa bits of precision.
#[inline]
pub(super) unsafe fn to_double_extended(mantissa: u64, base: u64, exponent: i32, truncated: bool)
    -> (f64, bool)
{
    let fp = FloatType { frac: mantissa, exp: 0 };
    let (fp, valid) = multiply_exponent_extended(fp, base, exponent, truncated);
    if valid {
        (fp.as_f64(), true)
    } else {
        (0.0, false)
    }
}

// BIGNUM

// Super slow path...
// TODO(ahuszagh) Implement...

// ATOF/ATOD

/// Parse 32-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
#[cfg(any(test, feature = "correct"))]
#[allow(dead_code)]     // TODO(ahuszagh) Remove
pub(crate) unsafe extern "C" fn atof(first: *const u8, last: *const u8, base: u64)
    -> (f32, *const u8)
{
    let (mantissa, exponent, p, truncated) = parse_float(first, last, base);

    if mantissa == 0 {
        // Literal 0, return early.
        return (0.0, p);
    } else if !truncated {
        // Try fast path
        let (float, valid) = to_float_exact(mantissa, base, exponent);
        if valid {
            return (float, p);
        }
    }

    // Moderate path (use an extended 80-bit representation).
    let (float, valid) = to_float_extended(mantissa, base, exponent, truncated);
    if valid {
        return (float, p);
    }

    // Slow path (use a decimal representation).
    unreachable!()
}

/// Parse 64-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
#[cfg(any(test, feature = "correct"))]
#[allow(dead_code)]     // TODO(ahuszagh) Remove
pub(crate) unsafe extern "C" fn atod(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    let (mantissa, exponent, p, truncated) = parse_float(first, last, base);

    if mantissa == 0 {
        // Literal 0, return early.
        return (0.0, p);
    } else if !truncated {
        // Try fast path
        let (double, valid) = to_double_exact(mantissa, base, exponent);
        if valid {
            return (double, p);
        }
    }

    // Moderate path (use an extended 80-bit representation).
    let (double, valid) = to_double_extended(mantissa, base, exponent,truncated);
    if valid {
        return (double, p);
    }

    // Slow path (use a decimal representation).
    unreachable!()
}

}   // anonymous
}   // cfg_if

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn check_parse_exponent(s: &str, base: u64, tup: (i32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = parse_exponent(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn parse_exponent_test() {
        unsafe {
            // empty
            check_parse_exponent("", 10, (0, 0));

            // lowercase
            check_parse_exponent("e20", 10, (20, 3));
            check_parse_exponent("e+20", 10, (20, 4));
            check_parse_exponent("e-20", 10, (-20, 4));

            // uppercase
            check_parse_exponent("E20", 10, (20, 3));
            check_parse_exponent("E+20", 10, (20, 4));
            check_parse_exponent("E-20", 10, (-20, 4));

            // >= base15
            check_parse_exponent("^20", 15, (30, 3));
            check_parse_exponent("^+20", 15, (30, 4));
            check_parse_exponent("^-20", 15, (-30, 4));

            // overflow
            check_parse_exponent("e10000000000", 10, (i32::max_value(), 12));
            check_parse_exponent("e+10000000000", 10, (i32::max_value(), 13));
            check_parse_exponent("e-10000000000", 10, (-i32::max_value(), 13));

            // trailing
            check_parse_exponent("e20 ", 10, (20, 3));
            check_parse_exponent("e+20 ", 10, (20, 4));
        }
    }

    unsafe fn check_parse_mantissa(s: &str, base: u64, tup: (u64, i32, usize, bool))
    {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, d, p, t) = parse_mantissa(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(d, tup.1);
        assert_eq!(distance(first, p), tup.2);
        assert_eq!(t, tup.3);
    }

    #[test]
    fn parse_mantissa_test() {
        unsafe {
            check_parse_mantissa("1.2345", 10, (12345, 4, 6, false));
            check_parse_mantissa("12.345", 10, (12345, 3, 6, false));
            check_parse_mantissa("12345.6789", 10, (123456789, 4, 10, false));
            check_parse_mantissa("1.2345e10", 10, (12345, 4, 6, false));
            check_parse_mantissa("0.0000000000000000001", 10, (1, 19, 21, false));
            check_parse_mantissa("0.00000000000000000000000000001", 10, (1, 29, 31, false));
            check_parse_mantissa("100000000000000000000", 10, (10000000000000000000, 0, 21, true));
        }
    }

    #[test]
    fn normalize_exponent_test() {
        assert_eq!(normalize_exponent(10, 5), 5);
        assert_eq!(normalize_exponent(0, 5), -5);
        assert_eq!(normalize_exponent(i32::max_value(), 5), i32::max_value());
        assert_eq!(normalize_exponent(i32::min_value(), 5), i32::min_value());
    }

    #[test]
    fn normalize_mantissa_test() {
        assert_eq!(normalize_mantissa(100, 10, 0), (1, 2));
        assert_eq!(normalize_mantissa(101, 10, 0), (101, 0));
        assert_eq!(normalize_mantissa(110, 10, 0), (11, 1));
    }

    unsafe fn check_parse_float(s: &str, base: u64, tup: (u64, i32, usize, bool))
    {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, e, p, t) = parse_float(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(e, tup.1);
        assert_eq!(distance(first, p), tup.2);
        assert_eq!(t, tup.3);
    }

    #[test]
    fn parse_float_test() {
        unsafe {
            check_parse_float("1.2345", 10, (12345, -4, 6, false));
            check_parse_float("12.345", 10, (12345, -3, 6, false));
            check_parse_float("12345.6789", 10, (123456789, -4, 10, false));
            check_parse_float("1.2345e10", 10, (12345, 6, 9, false));
            check_parse_float("100000000000000000000", 10, (1, 19, 21, true));
        }
    }

    #[test]
    fn to_float_exact_test() {
        unsafe {
            // valid
            let mantissa = 1 << (f32::SIGNIFICAND_SIZE - 1);
            for base in 2..37u64 {
                let (min_exp, max_exp) = f32::exponent_limit(base);
                for exp in min_exp..max_exp+1 {
                    let (_, valid) = to_float_exact(mantissa, base, exp);
                    assert!(valid, "should be valid {:?}.", (mantissa, base, exp));
                }
            }

            // invalid mantissa
            let (_, valid) = to_float_exact(1<<f32::SIGNIFICAND_SIZE, 2, 0);
            assert!(!valid, "invalid mantissa");

            // invalid exponents
            for base in 2..37u64 {
                let (min_exp, max_exp) = f32::exponent_limit(base);
                let (_, valid) = to_float_exact(mantissa, base, min_exp-1);
                assert!(!valid, "exponent under min_exp");

                let (_, valid) = to_float_exact(mantissa, base, max_exp+1);
                assert!(!valid, "exponent above max_exp");
            }
        }
    }

    #[test]
    fn to_double_exact_test() {
        unsafe {
            // valid
            let mantissa = 1 << (f64::SIGNIFICAND_SIZE - 1);
            for base in 2..37u64 {
                let (min_exp, max_exp) = f64::exponent_limit(base);
                for exp in min_exp..max_exp+1 {
                    let (_, valid) = to_double_exact(mantissa, base, exp);
                    assert!(valid, "should be valid {:?}.", (mantissa, base, exp));
                }
            }

            // invalid mantissa
            let (_, valid) = to_double_exact(1<<f64::SIGNIFICAND_SIZE, 2, 0);
            assert!(!valid, "invalid mantissa");

            // invalid exponents
            for base in 2..37u64 {
                let (min_exp, max_exp) = f64::exponent_limit(base);
                let (_, valid) = to_double_exact(mantissa, base, min_exp-1);
                assert!(!valid, "exponent under min_exp");

                let (_, valid) = to_double_exact(mantissa, base, max_exp+1);
                assert!(!valid, "exponent above max_exp");
            }
        }
    }

    // TODO(ahuszagh) slow path
    // TODO(ahuszagh) atof, atod
    // Check both known fast and slow paths.
}
