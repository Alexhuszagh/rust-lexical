//! Low-level API generator.
//!
//! Uses either the lossy or the correct algorithm.

use util::*;

// Select the back-end
cfg_if! {
    if #[cfg(feature = "correct")] {
        use super::correct::*;
    } else {
        use super::lossy::*;
    }
}

// MODULES

// Use modules to create consistent naming to avoid concatenating identifiers.

mod float {
    #[inline(always)]
    pub(super) unsafe extern "C" fn base10(first: *const u8, last: *const u8) -> (f32, *const u8) {
        super::float_base10(first, last)
    }

    #[inline(always)]
    pub(super) unsafe extern "C" fn basen(first: *const u8, last: *const u8, base: u64) -> (f32, *const u8) {
        super::float_basen(first, last, base)
    }
}

mod double {
    #[inline(always)]
    pub(super) unsafe extern "C" fn base10(first: *const u8, last: *const u8) -> (f64, *const u8) {
        super::double_base10(first, last)
    }

    #[inline(always)]
    pub(super) unsafe extern "C" fn basen(first: *const u8, last: *const u8, base: u64) -> (f64, *const u8) {
        super::double_basen(first, last, base)
    }
}

// SPECIAL

#[inline(always)]
pub(crate) unsafe extern "C" fn is_nan(first: *const u8, length: usize)
    -> bool
{
    starts_with(first, length, NAN_STRING.as_ptr(), NAN_STRING.len())
}

#[inline(always)]
pub(crate) unsafe extern "C" fn is_infinity(first: *const u8, length: usize)
    -> bool
{
    starts_with(first, length, INFINITY_STRING.as_ptr(), INFINITY_STRING.len())
}

#[inline(always)]
pub(crate) unsafe extern "C" fn is_zero(first: *const u8, length: usize)
    -> bool
{
    length == 3 && equal_to(first, "0.0".as_ptr(), 3)
}

// ATOF

/// Implied atof for non-special (no NaN or Infinity) numbers.
///
/// Allows a custom quad type (if enabled) to be passed for higher-precision
/// calculations.
macro_rules! atof_foward {
    ($first:expr, $last:expr, $base:expr, $mod:ident) => (match $base{
        10  => $mod::base10($first, $last),
        _   => $mod::basen($first, $last, $base),
    })
}

/// Convert string to float (must be called within an unsafe block).
macro_rules! atof_value {
    ($first:expr, $last:expr, $base:expr, $mod:ident, $f:tt) => ({
        // special case checks
        let length = distance($first, $last);
        if is_nan($first, length) {
            ($f::NAN, $first.add(NAN_STRING.len()))
        } else if is_infinity($first, length) {
            ($f::INFINITY, $first.add(INFINITY_STRING.len()))
        } else if is_zero($first, length) {
            (0.0, $first.add(3))
        } else {
            atof_foward!($first, $last, $base, $mod)
        }
    })
}

/// Sanitizer for string to float (must be called within an unsafe block).
macro_rules! atof {
    ($first:expr, $last:expr, $base:expr, $mod:ident, $f:tt) => ({
        if $first == $last {
            (0.0, nullptr!())
        } else if *$first == b'-' {
            let (value, p) = atof_value!($first.add(1), $last, $base, $mod, $f);
            (-value, p)
        } else if *$first == b'+' {
            atof_value!($first.add(1), $last, $base, $mod, $f)
        } else {
            atof_value!($first, $last, $base, $mod, $f)
        }
    })
}

// UNSAFE API

/// Generate the unsafe public wrappers.
///
/// * `func`        Function name.
/// * `sig`         Significand step for exponent.
/// * `f`           Float type.
/// * `nan`         NaN literal.
/// * `inf`         Infinity literal.
macro_rules! unsafe_impl {
    ($func:ident, $mod:ident, $f:tt) => (
        /// Unsafe, C-like importer for signed numbers.
        #[inline]
        pub unsafe extern "C" fn $func(
            first: *const u8,
            last: *const u8,
            base: u8
        )
            -> ($f, *const u8, bool)
        {
            let (value, p) = atof!(first, last, base as u64, $mod, $f);
            (value as $f, p, false)
        }
    )
}

unsafe_impl!(atof32_unsafe, float, f32);
unsafe_impl!(atof64_unsafe, double, f64);

// LOW-LEVEL API

bytes_impl!(atof32_bytes, f32, atof32_unsafe);
bytes_impl!(atof64_bytes, f64, atof64_unsafe);
try_bytes_impl!(try_atof32_bytes, f32, atof32_unsafe);
try_bytes_impl!(try_atof64_bytes, f64, atof64_unsafe);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use error::invalid_digit;
    use super::*;

    #[test]
    fn atof32_base10_test() {
        // integer test
        assert_relative_eq!(0.0, atof32_bytes(b"0", 10), epsilon=1e-20);
        assert_relative_eq!(1.0, atof32_bytes(b"1", 10), epsilon=1e-20);
        assert_relative_eq!(12.0, atof32_bytes(b"12", 10), epsilon=1e-20);
        assert_relative_eq!(123.0, atof32_bytes(b"123", 10), epsilon=1e-20);
        assert_relative_eq!(1234.0, atof32_bytes(b"1234", 10), epsilon=1e-20);
        assert_relative_eq!(12345.0, atof32_bytes(b"12345", 10), epsilon=1e-20);
        assert_relative_eq!(123456.0, atof32_bytes(b"123456", 10), epsilon=1e-20);
        assert_relative_eq!(1234567.0, atof32_bytes(b"1234567", 10), epsilon=1e-20);
        assert_relative_eq!(12345678.0, atof32_bytes(b"12345678", 10), epsilon=1e-20);

        // decimal test
        assert_relative_eq!(123.1, atof32_bytes(b"123.1", 10), epsilon=1e-20);
        assert_relative_eq!(123.12, atof32_bytes(b"123.12", 10), epsilon=1e-20);
        assert_relative_eq!(123.123, atof32_bytes(b"123.123", 10), epsilon=1e-20);
        assert_relative_eq!(123.1234, atof32_bytes(b"123.1234", 10), epsilon=1e-20);
        assert_relative_eq!(123.12345, atof32_bytes(b"123.12345", 10), epsilon=1e-20);

        // rounding test
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789", 10), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789.1", 10), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789.12", 10), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789.123", 10), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789.1234", 10), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789.12345", 10), epsilon=1e-20);

        // exponent test
        assert_relative_eq!(123456789.12345, atof32_bytes(b"1.2345678912345e8", 10), epsilon=1e-20);
        assert_relative_eq!(123450000.0, atof32_bytes(b"1.2345e+8", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof32_bytes(b"1.2345e+11", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof32_bytes(b"123450000000", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof32_bytes(b"1.2345e+38", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof32_bytes(b"123450000000000000000000000000000000000", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-8, atof32_bytes(b"1.2345e-8", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-8, atof32_bytes(b"0.000000012345", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof32_bytes(b"1.2345e-38", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof32_bytes(b"0.000000000000000000000000000000000000012345", 10), epsilon=1e-20);

        #[cfg(feature = "std")]
        assert!(atof32_bytes(b"NaN", 10).is_nan());
        assert!(atof32_bytes(b"inf", 10).is_infinite());
        assert!(atof32_bytes(b"+inf", 10).is_infinite());
        assert!(atof32_bytes(b"-inf", 10).is_infinite());
    }

    #[test]
    fn atof32_basen_test() {
        assert_relative_eq!(1234.0, atof32_bytes(b"YA", 36));
    }

    #[test]
    fn atof64_base10_test() {
        // integer test
        assert_relative_eq!(0.0, atof64_bytes(b"0", 10), epsilon=1e-20);
        assert_relative_eq!(1.0, atof64_bytes(b"1", 10), epsilon=1e-20);
        assert_relative_eq!(12.0, atof64_bytes(b"12", 10), epsilon=1e-20);
        assert_relative_eq!(123.0, atof64_bytes(b"123", 10), epsilon=1e-20);
        assert_relative_eq!(1234.0, atof64_bytes(b"1234", 10), epsilon=1e-20);
        assert_relative_eq!(12345.0, atof64_bytes(b"12345", 10), epsilon=1e-20);
        assert_relative_eq!(123456.0, atof64_bytes(b"123456", 10), epsilon=1e-20);
        assert_relative_eq!(1234567.0, atof64_bytes(b"1234567", 10), epsilon=1e-20);
        assert_relative_eq!(12345678.0, atof64_bytes(b"12345678", 10), epsilon=1e-20);

        // decimal test
        assert_relative_eq!(123456789.0, atof64_bytes(b"123456789", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.1, atof64_bytes(b"123456789.1", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12, atof64_bytes(b"123456789.12", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.123, atof64_bytes(b"123456789.123", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.1234, atof64_bytes(b"123456789.1234", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345, atof64_bytes(b"123456789.12345", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.123456, atof64_bytes(b"123456789.123456", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.1234567, atof64_bytes(b"123456789.1234567", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345678, atof64_bytes(b"123456789.12345678", 10), epsilon=1e-20);

        // rounding test
        assert_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.123456789", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.1234567890", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.123456789012", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.1234567890123", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.12345678901234", 10), epsilon=1e-20);

        // exponent test
        assert_relative_eq!(123456789.12345, atof64_bytes(b"1.2345678912345e8", 10), epsilon=1e-20);
        assert_relative_eq!(123450000.0, atof64_bytes(b"1.2345e+8", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof64_bytes(b"123450000000", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof64_bytes(b"1.2345e+11", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof64_bytes(b"1.2345e+38", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof64_bytes(b"123450000000000000000000000000000000000", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+308, atof64_bytes(b"1.2345e+308", 10), max_relative=1e-12);
        assert_relative_eq!(1.2345e+308, atof64_bytes(b"123450000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 10), max_relative=1e-12);
        assert_relative_eq!(0.000000012345, atof64_bytes(b"1.2345e-8", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-8, atof64_bytes(b"0.000000012345", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof64_bytes(b"1.2345e-38", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof64_bytes(b"0.000000000000000000000000000000000000012345", 10), epsilon=1e-20);

        // denormalized (try extremely low values)
        assert_relative_eq!(1.2345e-308, atof64_bytes(b"1.2345e-308", 10), epsilon=1e-20);
        assert_eq!(5e-322, atof64_bytes(b"5e-322", 10));
        assert_eq!(5e-323, atof64_bytes(b"5e-323", 10));
        assert_eq!(5e-324, atof64_bytes(b"5e-324", 10));
        // due to issues in how the data is parsed, manually extracting
        // non-exponents of 1.<e-299 is prone to error
        // test the limit of our ability
        // We tend to get relative errors of 1e-16, even at super low values.
        assert_relative_eq!(1.2345e-299, atof64_bytes(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", 10), epsilon=1e-314);

        // Keep pushing from -300 to -324
        assert_relative_eq!(1.2345e-300, atof64_bytes(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", 10), epsilon=1e-315);
        assert_relative_eq!(1.2345e-310, atof64_bytes(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", 10), epsilon=5e-324);
        assert_relative_eq!(1.2345e-320, atof64_bytes(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", 10), epsilon=5e-324);
        assert_relative_eq!(1.2345e-321, atof64_bytes(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", 10), epsilon=5e-324);
        assert_relative_eq!(1.24e-322, atof64_bytes(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000124", 10), epsilon=5e-324);
        assert_eq!(1e-323, atof64_bytes(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001", 10));
        assert_eq!(5e-324, atof64_bytes(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005", 10));

        #[cfg(feature = "std")]
        assert!(atof64_bytes(b"NaN", 10).is_nan());
        assert!(atof64_bytes(b"inf", 10).is_infinite());
        assert!(atof64_bytes(b"+inf", 10).is_infinite());
        assert!(atof64_bytes(b"-inf", 10).is_infinite());
    }

    #[test]
    #[should_panic]
    fn limit_test() {
        assert_relative_eq!(1.2345e-320, 0.0, epsilon=5e-324);
    }

    #[test]
    fn atof64_basen_test() {
        assert_relative_eq!(1234.0, atof64_bytes(b"YA", 36));
    }

    #[test]
    fn try_atof32_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atof32_bytes(b"", 10));
        assert_eq!(Ok(0.0), try_atof32_bytes(b"0.0", 10));
        assert_eq!(Err(invalid_digit(1)), try_atof32_bytes(b"1a", 10));
    }

    #[test]
    fn try_atof64_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atof64_bytes(b"", 10));
        assert_eq!(Ok(0.0), try_atof64_bytes(b"0.0", 10));
        assert_eq!(Err(invalid_digit(1)), try_atof64_bytes(b"1a", 10));
    }
}
