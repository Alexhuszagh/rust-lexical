//! Low-level API generator.
//!
//! Uses either the internal "Grisu2", or the external "Grisu3" or "Ryu"
//! algorithms provided by `https://github.com/dtolnay`.

use lib::{mem, ptr};
use util::*;
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
}}  //cfg_if

// TRAITS

/// Trait to define serialization of a float to string.
pub(crate) trait FloatToString: Float {
    /// Export float to base10 string with optimized algorithm.
    unsafe extern "C" fn base10(self, first: *mut u8) -> *mut u8;

    /// Export float to basen string with slow algorithm.
    unsafe extern "C" fn basen(self, base: u32, first: *mut u8) -> *mut u8;
}

impl FloatToString for f32 {
    #[inline(always)]
    unsafe extern "C" fn base10(self, first: *mut u8) -> *mut u8 {
        float_base10(self, first)
    }

    #[inline(always)]
    unsafe extern "C" fn basen(self, base: u32, first: *mut u8) -> *mut u8 {
        float_basen(self, base, first)
    }
}

impl FloatToString for f64 {
    #[inline(always)]
    unsafe extern "C" fn base10(self, first: *mut u8) -> *mut u8 {
        double_base10(self, first)
    }

    #[inline(always)]
    unsafe extern "C" fn basen(self, base: u32, first: *mut u8) -> *mut u8 {
        double_basen(self, base, first)
    }
}

// FTOA

/// Forward the correct arguments the ideal encoder.
#[inline]
unsafe fn forward<F: FloatToString>(value: F, base: u32, first: *mut u8)
    -> *mut u8
{
    // Logic errors, disable in release builds.
    debug_assert!(base >= 2 && base <= 36, "Numerical base must be from 2-36");

    match base {
        10 => value.base10(first),
        _  => value.basen(base, first),
    }
}

/// Convert float-to-string and handle special (positive) floats.
#[inline]
unsafe fn filter_special<F: FloatToString>(value: F, base: u32, first: *mut u8)
    -> *mut u8
{
    // Logic errors, disable in release builds.
    debug_assert!(value.is_sign_positive(), "Value cannot be negative.");
    debug_assert!(base >= 2 && base <= 36, "Numerical base must be from 2-36");

    if value.is_zero() {
        ptr::copy_nonoverlapping(b"0.0".as_ptr(), first, 3);
        first.add(3)
    } else if value.is_nan() {
        ptr::copy_nonoverlapping(NAN_STRING.as_ptr(), first, NAN_STRING.len());
        first.add(NAN_STRING.len())
    } else if value.is_special() {
        // Must be positive infinity, we've already handled sign
        ptr::copy_nonoverlapping(INFINITY_STRING.as_ptr(), first, INFINITY_STRING.len());
        first.add(INFINITY_STRING.len())
    } else {
        forward(value, base, first)
    }
}

/// Handle +/- values.
#[inline]
unsafe fn filter_sign<F: FloatToString>(mut value: F, base: u32, mut first: *mut u8)
    -> *mut u8
{
    // Logic errors, disable in release builds.
    debug_assert!(base >= 2 && base <= 36, "Numerical base must be from 2-36");

    // If the sign bit is set, invert it and just set the first
    // value to "-".
    if value.is_sign_negative() {
        *first= b'-';
        value = -value;
        first = first.add(1);
    }

    filter_special(value, base, first)
}

/// Handle insufficient buffer sizes.
#[inline]
unsafe fn filter_buffer<F: FloatToString>(value: F, base: u32, first: *mut u8, last: *mut u8)
    -> *mut u8
{
    // Logic errors, disable in release builds.
    debug_assert!(base >= 2 && base <= 36, "Numerical base must be from 2-36");
    debug_assert!(first <= last, "First must be <= last");

    // check to use a temporary buffer
    let dist = distance(first, last);
    if dist == 0 {
        // Cannot write empty range, memory may be invalid.
        first
    } else if dist < BUFFER_SIZE {
        // Use a temporary buffer and write number to buffer
        let mut buffer: [u8; BUFFER_SIZE] = mem::uninitialized();
        let p = buffer.as_mut_ptr();
        filter_sign(value, base, p);

        // Copy as many bytes as possible.
        let length = distance(p, p.add(BUFFER_SIZE)).min(dist);
        ptr::copy_nonoverlapping(p, first, length);
        first.add(length)
    } else {
        // Current buffer has sufficient capacity, use it
        filter_sign(value, base, first)
    }
}

// UNSAFE API

/// Generate the unsafe API wrappers.
///
/// * `name`        Function name.
/// * `f`           Float type.
macro_rules! generate_unsafe_api {
    ($name:ident, $t:ty) => (
        /// Unsafe, C-like exporter for float numbers.
        ///
        /// # Warning
        ///
        /// Do not call this function directly, unless you **know**
        /// you have a buffer of sufficient size. No size checking is
        /// done in release mode, this function is **highly** dangerous.
        /// Sufficient buffer sizes is denoted by `BUFFER_SIZE`.
        #[inline]
        pub unsafe extern "C" fn $name(value: $t, base: u8, first: *mut u8, last: *mut u8) -> *mut u8
        {
            filter_buffer(value, base.into(), first, last)
        }
    )
}

generate_unsafe_api!(f32toa_unsafe, f32);
generate_unsafe_api!(f64toa_unsafe, f64);

// LOW-LEVEL API
// -------------

// WRAP UNSAFE LOCAL

generate_to_bytes_local!(f32toa_local, f32, f32toa_unsafe);
generate_to_bytes_local!(f64toa_local, f64, f64toa_unsafe);

// API

generate_to_bytes_api!(f32toa_bytes, f32, f32toa_local, BUFFER_SIZE);
generate_to_bytes_api!(f64toa_bytes, f64, f64toa_local, BUFFER_SIZE);

// TESTS
// -----

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
        assert_eq!(b"0.0".to_vec(), f32toa_bytes(0.0, 2));
        assert_eq!(b"-0.0".to_vec(), f32toa_bytes(-0.0, 2));
        assert_eq!(b"1.0".to_vec(), f32toa_bytes(1.0, 2));
        assert_eq!(b"10.0".to_vec(), f32toa_bytes(2.0, 2));
        assert_eq!(b"1.1".to_vec(), f32toa_bytes(1.5, 2));
        assert_eq!(b"1.01".to_vec(), f32toa_bytes(1.25, 2));
        assert_eq!(b"1.001111000000110010".to_vec(), f32toa_bytes(1.2345678901234567890e0, 2)[..20].to_vec());
        assert_eq!(b"1100.010110000111111".to_vec(), f32toa_bytes(1.2345678901234567890e1, 2)[..20].to_vec());
        assert_eq!(b"1111011.011101001111".to_vec(), f32toa_bytes(1.2345678901234567890e2, 2)[..20].to_vec());
        assert_eq!(b"10011010010.10010001".to_vec(), f32toa_bytes(1.2345678901234567890e3, 2)[..20].to_vec());

        // negative
        assert_eq!(b"-1.001111000000110010".to_vec(), f32toa_bytes(-1.2345678901234567890e0, 2)[..21].to_vec());
        assert_eq!(b"-1100.010110000111111".to_vec(), f32toa_bytes(-1.2345678901234567890e1, 2)[..21].to_vec());
        assert_eq!(b"-1111011.011101001111".to_vec(), f32toa_bytes(-1.2345678901234567890e2, 2)[..21].to_vec());
        assert_eq!(b"-10011010010.10010001".to_vec(), f32toa_bytes(-1.2345678901234567890e3, 2)[..21].to_vec());

        // special
        assert_eq!(b"NaN".to_vec(), f32toa_bytes(f32::NAN, 2));
        assert_eq!(b"inf".to_vec(), f32toa_bytes(f32::INFINITY, 2));

        // bugfixes
        assert_eq!(b"1.101010000010101111000e-11011".to_vec(), f32toa_bytes(0.000000012345, 2));
    }

    #[test]
    fn f32toa_base10_test() {
        // positive
        assert_eq!(b"0.0".to_vec(), f32toa_bytes(0.0, 10));
        assert_eq!(b"-0.0".to_vec(), f32toa_bytes(-0.0, 10));
        assert_eq!(b"1.0".to_vec(), f32toa_bytes(1.0, 10));
        assert_eq!(b"10.0".to_vec(), f32toa_bytes(10.0, 10));
        assert_eq!(b"1.234567".to_vec(), f32toa_bytes(1.2345678901234567890e0, 10)[..8].to_vec());
        assert_eq!(b"12.34567".to_vec(), f32toa_bytes(1.2345678901234567890e1, 10)[..8].to_vec());
        assert_eq!(b"123.4567".to_vec(), f32toa_bytes(1.2345678901234567890e2, 10)[..8].to_vec());
        assert_eq!(b"1234.567".to_vec(), f32toa_bytes(1.2345678901234567890e3, 10)[..8].to_vec());

        // negative
        assert_eq!(b"-1.234567".to_vec(), f32toa_bytes(-1.2345678901234567890e0, 10)[..9].to_vec());
        assert_eq!(b"-12.34567".to_vec(), f32toa_bytes(-1.2345678901234567890e1, 10)[..9].to_vec());
        assert_eq!(b"-123.4567".to_vec(), f32toa_bytes(-1.2345678901234567890e2, 10)[..9].to_vec());
        assert_eq!(b"-1234.567".to_vec(), f32toa_bytes(-1.2345678901234567890e3, 10)[..9].to_vec());

        // special
        assert_eq!(b"NaN".to_vec(), f32toa_bytes(f32::NAN, 10));
        assert_eq!(b"inf".to_vec(), f32toa_bytes(f32::INFINITY, 10));
    }

    #[test]
    fn f32toa_base10_roundtrip_test() {
        for f in F32_DATA.iter() {
            let s = f32toa_bytes(*f, 10);
            assert_relative_eq!(atof32_bytes(10, s.as_slice()), *f, epsilon=1e-6, max_relative=1e-6);
        }
    }

    #[test]
    fn f32toa_basen_roundtrip_test() {
        for f in F32_DATA.iter() {
            for radix in 2..37 {
                // The lower accuracy is due to slight rounding errors of
                // ftoa for the Grisu method with non-10 bases.
                let s = f32toa_bytes(*f, radix);
                assert_relative_eq!(atof32_bytes(radix, s.as_slice()), *f, max_relative=2e-5);
            }
        }
    }

    #[test]
    fn f64toa_base2_test() {
        // positive
        assert_eq!(b"0.0".to_vec(), f64toa_bytes(0.0, 2));
        assert_eq!(b"-0.0".to_vec(), f64toa_bytes(-0.0, 2));
        assert_eq!(b"1.0".to_vec(), f64toa_bytes(1.0, 2));
        assert_eq!(b"10.0".to_vec(), f64toa_bytes(2.0, 2));
        assert_eq!(b"1.00111100000011001010010000101000110001".to_vec(), f64toa_bytes(1.2345678901234567890e0, 2)[..40].to_vec());
        assert_eq!(b"1100.01011000011111100110100110010111101".to_vec(), f64toa_bytes(1.2345678901234567890e1, 2)[..40].to_vec());
        assert_eq!(b"1111011.01110100111100000001111111101101".to_vec(), f64toa_bytes(1.2345678901234567890e2, 2)[..40].to_vec());
        assert_eq!(b"10011010010.1001000101100001001111110100".to_vec(), f64toa_bytes(1.2345678901234567890e3, 2)[..40].to_vec());

        // negative
        assert_eq!(b"-1.00111100000011001010010000101000110001".to_vec(), f64toa_bytes(-1.2345678901234567890e0, 2)[..41].to_vec());
        assert_eq!(b"-1100.01011000011111100110100110010111101".to_vec(), f64toa_bytes(-1.2345678901234567890e1, 2)[..41].to_vec());
        assert_eq!(b"-1111011.01110100111100000001111111101101".to_vec(), f64toa_bytes(-1.2345678901234567890e2, 2)[..41].to_vec());
        assert_eq!(b"-10011010010.1001000101100001001111110100".to_vec(), f64toa_bytes(-1.2345678901234567890e3, 2)[..41].to_vec());

        // special
        assert_eq!(b"NaN".to_vec(), f64toa_bytes(f64::NAN, 2));
        assert_eq!(b"inf".to_vec(), f64toa_bytes(f64::INFINITY, 2));
    }

    #[test]
    fn f64toa_base10_test() {
        // positive
        assert_eq!(b"0.0".to_vec(), f64toa_bytes(0.0, 10));
        assert_eq!(b"-0.0".to_vec(), f64toa_bytes(-0.0, 10));
        assert_eq!(b"1.0".to_vec(), f64toa_bytes(1.0, 10));
        assert_eq!(b"10.0".to_vec(), f64toa_bytes(10.0, 10));
        assert_eq!(b"1.234567".to_vec(), f64toa_bytes(1.2345678901234567890e0, 10)[..8].to_vec());
        assert_eq!(b"12.34567".to_vec(), f64toa_bytes(1.2345678901234567890e1, 10)[..8].to_vec());
        assert_eq!(b"123.4567".to_vec(), f64toa_bytes(1.2345678901234567890e2, 10)[..8].to_vec());
        assert_eq!(b"1234.567".to_vec(), f64toa_bytes(1.2345678901234567890e3, 10)[..8].to_vec());

        // negative
        assert_eq!(b"-1.234567".to_vec(), f64toa_bytes(-1.2345678901234567890e0, 10)[..9].to_vec());
        assert_eq!(b"-12.34567".to_vec(), f64toa_bytes(-1.2345678901234567890e1, 10)[..9].to_vec());
        assert_eq!(b"-123.4567".to_vec(), f64toa_bytes(-1.2345678901234567890e2, 10)[..9].to_vec());
        assert_eq!(b"-1234.567".to_vec(), f64toa_bytes(-1.2345678901234567890e3, 10)[..9].to_vec());

        // special
        assert_eq!(b"NaN".to_vec(), f64toa_bytes(f64::NAN, 10));
        assert_eq!(b"inf".to_vec(), f64toa_bytes(f64::INFINITY, 10));
    }

    #[test]
    fn f64toa_base10_roundtrip_test() {
        for f in F64_DATA.iter() {
            let s = f64toa_bytes(*f, 10);
            assert_relative_eq!(atof64_bytes(10, s.as_slice()), *f, epsilon=1e-12, max_relative=1e-12);
        }
    }

    #[test]
    fn f64toa_basen_roundtrip_test() {
        for f in F64_DATA.iter() {
            for radix in 2..37 {
                // The lower accuracy is due to slight rounding errors of
                // ftoa for the Grisu method with non-10 bases.
                let s = f64toa_bytes(*f, radix);
                assert_relative_eq!(atof64_bytes(radix, s.as_slice()), *f, max_relative=3e-5);
            }
        }
    }

    quickcheck! {
        fn f32_quickcheck(f: f32) -> bool {
            f == atof32_bytes(10, f32toa_bytes(f, 10).as_slice())
        }

        fn f64_quickcheck(f: f64) -> bool {
            f == atof64_bytes(10, f64toa_bytes(f, 10).as_slice())
        }
    }
}
