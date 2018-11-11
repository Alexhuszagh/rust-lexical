//! Low-level API generator.
//!
//! Uses either the internal "Grisu2", or the external "Grisu3" or "Ryu"
//! algorithms provided by `https://github.com/dtolnay`.

use util::*;
use super::util::*;
use super::basen::{double_basen, float_basen};

// Select the back-end
cfg_if! {
    if #[cfg(feature = "grisu3")] {
        use super::grisu3::{double_base10, float_base10};
    } else if #[cfg(feature = "ryu")] {
        use super::ryu::{double_base10, float_base10};
    }
    else {
        use super::grisu2::{double_base10, float_base10};
    }
}

// MODULES

// Use modules to create consistent naming to avoid concatenating identifiers.

mod float {
    #[inline(always)]
    pub(super) unsafe extern "C" fn base10(f: f32, first: *mut u8) -> *mut u8 {
        super::float_base10(f, first)
    }

    #[inline(always)]
    pub(super) unsafe extern "C" fn basen(f: f32, first: *mut u8, base: u64) -> *mut u8 {
        super::float_basen(f, first, base)
    }
}

mod double {
    #[inline(always)]
    pub(super) unsafe extern "C" fn base10(d: f64, first: *mut u8) -> *mut u8 {
        super::double_base10(d, first)
    }

    #[inline(always)]
    pub(super) unsafe extern "C" fn basen(d: f64, first: *mut u8, base: u64) -> *mut u8 {
        super::double_basen(d, first, base)
    }
}

// FTOA

/// Check if the supplied buffer has enough range for the encoded size.
macro_rules! check_digits {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        debug_assert!(distance($first, $last) >= BUFFER_SIZE, "Need a larger buffer.");
    })
}

/// Forward the correct arguments to the implementation.
macro_rules! ftoa_forward {
    ($value:ident, $first:ident, $base:ident, $mod:ident) => (match $base {
        10  => $mod::base10($value, $first),
        _   => $mod::basen($value, $first, $base),
    })
}

/// Sanitizer for an unsigned number-to-string implementation.
macro_rules! ftoa_unsafe_impl {
    ($value:ident, $first:ident, $last:ident, $base:ident, $mod:ident) => ({
        // Sanity checks
        debug_assert!($first <= $last);
        check_digits!($value, $first, $last, $base);

        // If the sign bit is set, invert it and just set the first
        // value to "-".
        if $value.is_sign_negative() {
            *$first= b'-';
            $value = -$value;
            $first = $first.add(1);
        }

        // Check and emit special values, otherwise, forward to
        // specialized algorithms.
        let spec = $value.emit_special($first);
        if spec != 0 {
            return $first.offset(spec as isize);
        }

        ftoa_forward!($value, $first, $base, $mod)
    })
}

/// Generate the ftoa wrappers.
macro_rules! ftoa_unsafe {
    ($value:ident, $first:ident, $last:ident, $base:ident, $mod:ident) => ({
        // check to use a temporary buffer
        let dist = distance($first, $last);
        let mut value = $value;
        let base = $base as u64;
        if dist == 0 {
            // cannot write null terminator
            $first
        } else if dist < BUFFER_SIZE {
            // use a temporary buffer and write number to buffer
            let mut buffer: [u8; BUFFER_SIZE] = uninitialized!();
            let mut f = buffer.as_mut_ptr();
            let l = f.add(BUFFER_SIZE);
            ftoa_unsafe_impl!(value, f, l, base, $mod);

            // copy as many bytes as possible except the trailing null byte
            // then, write null-byte so the string is always terminated
            let length = min!(distance(f, l), dist);
            copy_nonoverlapping!(f, $first, length);
            $first.add(length)
        } else {
            // current buffer has sufficient capacity, use it
            ftoa_unsafe_impl!(value, $first, $last, base, $mod)
    }
    })
}

// UNSAFE API

/// Generate the unsafe wrappers.
macro_rules! unsafe_impl {
    ($func:ident, $t:ty, $mod:ident) => (
        /// Unsafe, C-like exporter for float numbers.
        ///
        /// # Warning
        ///
        /// Do not call this function directly, unless you **know**
        /// you have a buffer of sufficient size. No size checking is
        /// done in release mode, this function is **highly** dangerous.
        /// Sufficient buffer sizes is denoted by `BUFFER_SIZE`.
        #[inline]
        pub unsafe extern "C" fn $func(
            value: $t,
            mut first: *mut u8,
            last: *mut u8,
            base: u8
        )
            -> *mut u8
        {
            ftoa_unsafe!(value, first, last, base, $mod)
        }
    )
}

unsafe_impl!(f32toa_unsafe, f32, float);
unsafe_impl!(f64toa_unsafe, f64, double);

// LOW-LEVEL API

// Export the high-level wrappers solely if String is available.
cfg_if! {
    if #[cfg(any(feature = "std", feature = "alloc"))] {
        string_impl!(f32toa_string, f32, f32toa_unsafe, BUFFER_SIZE);
        string_impl!(f64toa_string, f64, f64toa_unsafe, BUFFER_SIZE);
    }
}

// TESTS
// -----

#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg(test)]
mod tests {
    use super::*;
    use atof::*;

    // Test data for roundtrips.
    const F32_DATA : [f32; 31] = [0., 0.1, 1., 1.1, 12., 12.1, 123., 123.1, 1234., 1234.1, 12345., 12345.1, 123456., 123456.1, 1234567., 1234567.1, 12345678., 12345678.1, 123456789., 123456789.1, 123456789.12, 123456789.123, 123456789.1234, 123456789.12345, 1.2345678912345e8, 1.2345e+8, 1.2345e+11, 1.2345e+38, 1.2345e-8, 1.2345e-11, 1.2345e-38];
    const F64_DATA: [f64; 33] = [0., 0.1, 1., 1.1, 12., 12.1, 123., 123.1, 1234., 1234.1, 12345., 12345.1, 123456., 123456.1, 1234567., 1234567.1, 12345678., 12345678.1, 123456789., 123456789.1, 123456789.12, 123456789.123, 123456789.1234, 123456789.12345, 1.2345678912345e8, 1.2345e+8, 1.2345e+11, 1.2345e+38, 1.2345e+308, 1.2345e-8, 1.2345e-11, 1.2345e-38, 1.2345e-299];

    #[test]
    fn f32toa_base2_test() {
        // positive
        assert_eq!("0.0", &f32toa_string(0.0, 2));
        assert_eq!("-0.0", &f32toa_string(-0.0, 2));
        assert_eq!("1.0", &f32toa_string(1.0, 2));
        assert_eq!("10.0", &f32toa_string(2.0, 2));
        assert_eq!("1.1", &f32toa_string(1.5, 2));
        assert_eq!("1.01", &f32toa_string(1.25, 2));
        assert_eq!("1.001111000000110010", &f32toa_string(1.2345678901234567890e0, 2)[..20]);
        assert_eq!("1100.010110000111111", &f32toa_string(1.2345678901234567890e1, 2)[..20]);
        assert_eq!("1111011.011101001111", &f32toa_string(1.2345678901234567890e2, 2)[..20]);
        assert_eq!("10011010010.10010001", &f32toa_string(1.2345678901234567890e3, 2)[..20]);

        // negative
        assert_eq!("-1.001111000000110010", &f32toa_string(-1.2345678901234567890e0, 2)[..21]);
        assert_eq!("-1100.010110000111111", &f32toa_string(-1.2345678901234567890e1, 2)[..21]);
        assert_eq!("-1111011.011101001111", &f32toa_string(-1.2345678901234567890e2, 2)[..21]);
        assert_eq!("-10011010010.10010001", &f32toa_string(-1.2345678901234567890e3, 2)[..21]);

        // special
        assert_eq!("NaN", &f32toa_string(F32_NAN, 2));
        assert_eq!("inf", &f32toa_string(F32_INFINITY, 2));

        // bugfixes
        assert_eq!("1.101010000010101111000e-11011", &f32toa_string(0.000000012345, 2));
    }

    #[test]
    fn f32toa_base10_test() {
        // positive
        assert_eq!("0.0", &f32toa_string(0.0, 10));
        assert_eq!("-0.0", &f32toa_string(-0.0, 10));
        assert_eq!("1.0", &f32toa_string(1.0, 10));
        assert_eq!("10.0", &f32toa_string(10.0, 10));
        assert_eq!("1.234567", &f32toa_string(1.2345678901234567890e0, 10)[..8]);
        assert_eq!("12.34567", &f32toa_string(1.2345678901234567890e1, 10)[..8]);
        assert_eq!("123.4567", &f32toa_string(1.2345678901234567890e2, 10)[..8]);
        assert_eq!("1234.567", &f32toa_string(1.2345678901234567890e3, 10)[..8]);

        // negative
        assert_eq!("-1.234567", &f32toa_string(-1.2345678901234567890e0, 10)[..9]);
        assert_eq!("-12.34567", &f32toa_string(-1.2345678901234567890e1, 10)[..9]);
        assert_eq!("-123.4567", &f32toa_string(-1.2345678901234567890e2, 10)[..9]);
        assert_eq!("-1234.567", &f32toa_string(-1.2345678901234567890e3, 10)[..9]);

        // special
        assert_eq!("NaN", &f32toa_string(F32_NAN, 10));
        assert_eq!("inf", &f32toa_string(F32_INFINITY, 10));
    }

    #[test]
    fn f32toa_base10_roundtrip_test() {
        for f in F32_DATA.iter() {
            let s = f32toa_string(*f, 10);
            assert_relative_eq!(atof32_bytes(s.as_bytes(), 10), *f, epsilon=1e-6, max_relative=1e-6);
        }
    }

    #[test]
    fn f32toa_basen_roundtrip_test() {
        for f in F32_DATA.iter() {
            for radix in 2..37 {
                // The lower accuracy is due to slight rounding errors of
                // ftoa for the Grisu method with non-10 bases.
                let s = f32toa_string(*f, radix);
                assert_relative_eq!(atof32_bytes(s.as_bytes(), radix), *f, max_relative=2e-5);
            }
        }
    }

    #[test]
    fn f64toa_base2_test() {
        // positive
        assert_eq!("0.0", &f64toa_string(0.0, 2));
        assert_eq!("-0.0", &f64toa_string(-0.0, 2));
        assert_eq!("1.0", &f64toa_string(1.0, 2));
        assert_eq!("10.0", &f64toa_string(2.0, 2));
        assert_eq!("1.00111100000011001010010000101000110001", &f64toa_string(1.2345678901234567890e0, 2)[..40]);
        assert_eq!("1100.01011000011111100110100110010111101", &f64toa_string(1.2345678901234567890e1, 2)[..40]);
        assert_eq!("1111011.01110100111100000001111111101101", &f64toa_string(1.2345678901234567890e2, 2)[..40]);
        assert_eq!("10011010010.1001000101100001001111110100", &f64toa_string(1.2345678901234567890e3, 2)[..40]);

            // negative
        assert_eq!("-1.00111100000011001010010000101000110001", &f64toa_string(-1.2345678901234567890e0, 2)[..41]);
        assert_eq!("-1100.01011000011111100110100110010111101", &f64toa_string(-1.2345678901234567890e1, 2)[..41]);
        assert_eq!("-1111011.01110100111100000001111111101101", &f64toa_string(-1.2345678901234567890e2, 2)[..41]);
        assert_eq!("-10011010010.1001000101100001001111110100", &f64toa_string(-1.2345678901234567890e3, 2)[..41]);

        // special
        assert_eq!("NaN", &f64toa_string(F64_NAN, 2));
        assert_eq!("inf", &f64toa_string(F64_INFINITY, 2));
    }

    #[test]
    fn f64toa_base10_test() {
        // positive
        assert_eq!("0.0", &f64toa_string(0.0, 10));
        assert_eq!("-0.0", &f64toa_string(-0.0, 10));
        assert_eq!("1.0", &f64toa_string(1.0, 10));
        assert_eq!("10.0", &f64toa_string(10.0, 10));
        assert_eq!("1.234567", &f64toa_string(1.2345678901234567890e0, 10)[..8]);
        assert_eq!("12.34567", &f64toa_string(1.2345678901234567890e1, 10)[..8]);
        assert_eq!("123.4567", &f64toa_string(1.2345678901234567890e2, 10)[..8]);
        assert_eq!("1234.567", &f64toa_string(1.2345678901234567890e3, 10)[..8]);

        // negative
        assert_eq!("-1.234567", &f64toa_string(-1.2345678901234567890e0, 10)[..9]);
        assert_eq!("-12.34567", &f64toa_string(-1.2345678901234567890e1, 10)[..9]);
        assert_eq!("-123.4567", &f64toa_string(-1.2345678901234567890e2, 10)[..9]);
        assert_eq!("-1234.567", &f64toa_string(-1.2345678901234567890e3, 10)[..9]);

        // special
        assert_eq!("NaN", &f64toa_string(F64_NAN, 10));
        assert_eq!("inf", &f64toa_string(F64_INFINITY, 10));
    }

    #[test]
    fn f64toa_base10_roundtrip_test() {
        for f in F64_DATA.iter() {
            let s = f64toa_string(*f, 10);
            assert_relative_eq!(atof64_bytes(s.as_bytes(), 10), *f, epsilon=1e-12, max_relative=1e-12);
        }
    }

    #[test]
    fn f64toa_basen_roundtrip_test() {
        for f in F64_DATA.iter() {
            for radix in 2..37 {
                // The lower accuracy is due to slight rounding errors of
                // ftoa for the Grisu method with non-10 bases.
                let s = f64toa_string(*f, radix);
                assert_relative_eq!(atof64_bytes(s.as_bytes(), radix), *f, max_relative=3e-5);
            }
        }
    }
}
