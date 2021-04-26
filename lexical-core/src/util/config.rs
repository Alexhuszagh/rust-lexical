//! Config settings for lexical-core.

// CONSTANTS

// The f64 buffer is actually a size of 60, but use 64 since it's a
// power of 2.
pub(crate) const I8_FORMATTED_SIZE_DECIMAL: usize = 4;
pub(crate) const I16_FORMATTED_SIZE_DECIMAL: usize = 6;
pub(crate) const I32_FORMATTED_SIZE_DECIMAL: usize = 11;
pub(crate) const I64_FORMATTED_SIZE_DECIMAL: usize = 20;
pub(crate) const U8_FORMATTED_SIZE_DECIMAL: usize = 3;
pub(crate) const U16_FORMATTED_SIZE_DECIMAL: usize = 5;
pub(crate) const U32_FORMATTED_SIZE_DECIMAL: usize = 10;
pub(crate) const U64_FORMATTED_SIZE_DECIMAL: usize = 20;
pub(crate) const F32_FORMATTED_SIZE_DECIMAL: usize = 64;
pub(crate) const F64_FORMATTED_SIZE_DECIMAL: usize = 64;
pub(crate) const I128_FORMATTED_SIZE_DECIMAL: usize = 40;
pub(crate) const U128_FORMATTED_SIZE_DECIMAL: usize = 39;

// Simple, fast optimization.
// Since we're declaring a variable on the stack, and our power-of-two
// alignment dramatically improved atoi performance, do it.
cfg_if! {
if #[cfg(feature = "radix")] {
    // Use 256, actually, since we seem to have memory issues with f64.
    // Clearly not sufficient memory allocated for non-decimal values.
    pub(crate) const I8_FORMATTED_SIZE: usize = 16;
    pub(crate) const I16_FORMATTED_SIZE: usize = 32;
    pub(crate) const I32_FORMATTED_SIZE: usize = 64;
    pub(crate) const I64_FORMATTED_SIZE: usize = 128;
    pub(crate) const U8_FORMATTED_SIZE: usize = 16;
    pub(crate) const U16_FORMATTED_SIZE: usize = 32;
    pub(crate) const U32_FORMATTED_SIZE: usize = 64;
    pub(crate) const U64_FORMATTED_SIZE: usize = 128;
    pub(crate) const F32_FORMATTED_SIZE: usize = 256;
    pub(crate) const F64_FORMATTED_SIZE: usize = 256;
    pub(crate) const I128_FORMATTED_SIZE: usize = 256;
    pub(crate) const U128_FORMATTED_SIZE: usize = 256;
} else {
    // The f64 buffer is actually a size of 60, but use 64 since it's a
    // power of 2.
    pub(crate) const I8_FORMATTED_SIZE: usize = I8_FORMATTED_SIZE_DECIMAL;
    pub(crate) const I16_FORMATTED_SIZE: usize = I16_FORMATTED_SIZE_DECIMAL;
    pub(crate) const I32_FORMATTED_SIZE: usize = I32_FORMATTED_SIZE_DECIMAL;
    pub(crate) const I64_FORMATTED_SIZE: usize = I64_FORMATTED_SIZE_DECIMAL;
    pub(crate) const U8_FORMATTED_SIZE: usize = U8_FORMATTED_SIZE_DECIMAL;
    pub(crate) const U16_FORMATTED_SIZE: usize = U16_FORMATTED_SIZE_DECIMAL;
    pub(crate) const U32_FORMATTED_SIZE: usize = U32_FORMATTED_SIZE_DECIMAL;
    pub(crate) const U64_FORMATTED_SIZE: usize = U64_FORMATTED_SIZE_DECIMAL;
    pub(crate) const F32_FORMATTED_SIZE: usize = F32_FORMATTED_SIZE_DECIMAL;
    pub(crate) const F64_FORMATTED_SIZE: usize = F64_FORMATTED_SIZE_DECIMAL;
    pub(crate) const I128_FORMATTED_SIZE: usize = I128_FORMATTED_SIZE_DECIMAL;
    pub(crate) const U128_FORMATTED_SIZE: usize = U128_FORMATTED_SIZE_DECIMAL;
}} // cfg_if

cfg_if! {
if #[cfg(target_pointer_width = "16")] {
    pub(crate) const ISIZE_FORMATTED_SIZE: usize = I16_FORMATTED_SIZE;
    pub(crate) const ISIZE_FORMATTED_SIZE_DECIMAL: usize = I16_FORMATTED_SIZE_DECIMAL;
    pub(crate) const USIZE_FORMATTED_SIZE: usize = U16_FORMATTED_SIZE;
    pub(crate) const USIZE_FORMATTED_SIZE_DECIMAL: usize = U16_FORMATTED_SIZE_DECIMAL;
} else if #[cfg(target_pointer_width = "32")] {
    pub(crate) const ISIZE_FORMATTED_SIZE: usize = I32_FORMATTED_SIZE;
    pub(crate) const ISIZE_FORMATTED_SIZE_DECIMAL: usize = I32_FORMATTED_SIZE_DECIMAL;
    pub(crate) const USIZE_FORMATTED_SIZE: usize = U32_FORMATTED_SIZE;
    pub(crate) const USIZE_FORMATTED_SIZE_DECIMAL: usize = U32_FORMATTED_SIZE_DECIMAL;
} else if #[cfg(target_pointer_width = "64")] {
    pub(crate) const ISIZE_FORMATTED_SIZE: usize = I64_FORMATTED_SIZE;
    pub(crate) const ISIZE_FORMATTED_SIZE_DECIMAL: usize = I64_FORMATTED_SIZE_DECIMAL;
    pub(crate) const USIZE_FORMATTED_SIZE: usize = U64_FORMATTED_SIZE;
    pub(crate) const USIZE_FORMATTED_SIZE_DECIMAL: usize = U64_FORMATTED_SIZE_DECIMAL;
}}  // cfg_if

/// Maximum number of bytes required to serialize any number to string.
pub const BUFFER_SIZE: usize = F64_FORMATTED_SIZE;

// TEST
// ----

#[cfg(test)]
mod tests {
    use crate::util::*;
    use crate::util::test::*;
    use super::*;

    #[test]
    #[cfg(feature ="radix")]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn exponent_notation_char_test() {
        let default = get_exponent_default_char();
        let backup = get_exponent_backup_char();
        assert_eq!(exponent_notation_char(2), default);
        assert_eq!(exponent_notation_char(8), default);
        assert_eq!(exponent_notation_char(10), default);
        assert_eq!(exponent_notation_char(15), backup);
        assert_eq!(exponent_notation_char(16), backup);
        assert_eq!(exponent_notation_char(32), backup);
    }

    // Only enable when no other threads touch NAN_STRING or INFINITY_STRING.
    #[test]
    #[ignore]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn special_bytes_test() {
        unsafe {
            let mut buffer = new_buffer();
            // Test serializing and deserializing special strings.
            assert!(f32::from_lexical(b"NaN").unwrap().is_nan());
            assert!(f32::from_lexical(b"nan").unwrap().is_nan());
            assert!(f32::from_lexical(b"NAN").unwrap().is_nan());
            assert!(f32::from_lexical(b"inf").unwrap().is_infinite());
            assert!(f32::from_lexical(b"INF").unwrap().is_infinite());
            assert!(f32::from_lexical(b"Infinity").unwrap().is_infinite());
            assert_eq!(f64::NAN.to_lexical(&mut buffer), b"NaN");
            assert_eq!(f64::INFINITY.to_lexical(&mut buffer), b"inf");

            set_nan_string(b"nan");
            set_inf_string(b"Infinity");

            assert!(f32::from_lexical(b"inf").err().unwrap().code == ErrorCode::InvalidDigit);
            assert!(f32::from_lexical(b"Infinity").unwrap().is_infinite());
            assert_eq!(f64::NAN.to_lexical(&mut buffer), b"nan");
            assert_eq!(f64::INFINITY.to_lexical(&mut buffer), b"Infinity");

            set_nan_string(b"NaN");
            set_inf_string(b"inf");
        }
    }

    // Only enable when no other threads touch FLOAT_ROUNDING.
    #[test]
    #[ignore]
    #[cfg(feature = "rounding")]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn special_rounding_test() {
        // Each one of these pairs is halfway, and we can detect the
        // rounding schemes from this.
        unsafe {
            // Nearest, tie-even
            set_float_rounding(RoundingKind::NearestTieEven);
            assert_eq!(f64::from_lexical(b"-9007199254740993").unwrap(), -9007199254740992.0);
            assert_eq!(f64::from_lexical(b"-9007199254740995").unwrap(), -9007199254740996.0);
            assert_eq!(f64::from_lexical(b"9007199254740993").unwrap(), 9007199254740992.0);
            assert_eq!(f64::from_lexical(b"9007199254740995").unwrap(), 9007199254740996.0);

            // Nearest, tie-away-zero
            set_float_rounding(RoundingKind::NearestTieAwayZero);
            assert_eq!(f64::from_lexical(b"-9007199254740993").unwrap(), -9007199254740994.0);
            assert_eq!(f64::from_lexical(b"-9007199254740995").unwrap(), -9007199254740996.0);
            assert_eq!(f64::from_lexical(b"9007199254740993").unwrap(), 9007199254740994.0);
            assert_eq!(f64::from_lexical(b"9007199254740995").unwrap(), 9007199254740996.0);

            // Toward positive infinity
            set_float_rounding(RoundingKind::TowardPositiveInfinity);
            assert_eq!(f64::from_lexical(b"-9007199254740993").unwrap(), -9007199254740992.0);
            assert_eq!(f64::from_lexical(b"-9007199254740995").unwrap(), -9007199254740994.0);
            assert_eq!(f64::from_lexical(b"9007199254740993").unwrap(), 9007199254740994.0);
            assert_eq!(f64::from_lexical(b"9007199254740995").unwrap(), 9007199254740996.0);

            // Toward negative infinity
            set_float_rounding(RoundingKind::TowardNegativeInfinity);
            assert_eq!(f64::from_lexical(b"-9007199254740993").unwrap(), -9007199254740994.0);
            assert_eq!(f64::from_lexical(b"-9007199254740995").unwrap(), -9007199254740996.0);
            assert_eq!(f64::from_lexical(b"9007199254740993").unwrap(), 9007199254740992.0);
            assert_eq!(f64::from_lexical(b"9007199254740995").unwrap(), 9007199254740994.0);

            // Toward zero
            set_float_rounding(RoundingKind::TowardZero);
            assert_eq!(f64::from_lexical(b"-9007199254740993").unwrap(), -9007199254740992.0);
            assert_eq!(f64::from_lexical(b"-9007199254740995").unwrap(), -9007199254740994.0);
            assert_eq!(f64::from_lexical(b"9007199254740993").unwrap(), 9007199254740992.0);
            assert_eq!(f64::from_lexical(b"9007199254740995").unwrap(), 9007199254740994.0);

            // Reset to default
            set_float_rounding(RoundingKind::NearestTieEven);
        }
    }

    // Only enable when no other threads touch FLOAT_ROUNDING.
    #[test]
    #[ignore]
    #[cfg(all(feature = "radix", feature = "rounding"))]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn special_rounding_binary_test() {
        // Each one of these pairs is halfway, and we can detect the
        // rounding schemes from this.
        unsafe {
            // Nearest, tie-even
            set_float_rounding(RoundingKind::NearestTieEven);
            assert_eq!(f64::from_lexical_radix(b"-100000000000000000000000000000000000000000000000000001", 2).unwrap(), -9007199254740992.0);
            assert_eq!(f64::from_lexical_radix(b"-100000000000000000000000000000000000000000000000000011", 2).unwrap(), -9007199254740996.0);
            assert_eq!(f64::from_lexical_radix(b"100000000000000000000000000000000000000000000000000001", 2).unwrap(), 9007199254740992.0);
            assert_eq!(f64::from_lexical_radix(b"100000000000000000000000000000000000000000000000000011", 2).unwrap(), 9007199254740996.0);

            // Nearest, tie-away-zero
            set_float_rounding(RoundingKind::NearestTieAwayZero);
            assert_eq!(f64::from_lexical_radix(b"-100000000000000000000000000000000000000000000000000001", 2).unwrap(), -9007199254740994.0);
            assert_eq!(f64::from_lexical_radix(b"-100000000000000000000000000000000000000000000000000011", 2).unwrap(), -9007199254740996.0);
            assert_eq!(f64::from_lexical_radix(b"100000000000000000000000000000000000000000000000000001", 2).unwrap(), 9007199254740994.0);
            assert_eq!(f64::from_lexical_radix(b"100000000000000000000000000000000000000000000000000011", 2).unwrap(), 9007199254740996.0);

            // Toward positive infinity
            set_float_rounding(RoundingKind::TowardPositiveInfinity);
            assert_eq!(f64::from_lexical_radix(b"-100000000000000000000000000000000000000000000000000001", 2).unwrap(), -9007199254740992.0);
            assert_eq!(f64::from_lexical_radix(b"-100000000000000000000000000000000000000000000000000011", 2).unwrap(), -9007199254740994.0);
            assert_eq!(f64::from_lexical_radix(b"100000000000000000000000000000000000000000000000000001", 2).unwrap(), 9007199254740994.0);
            assert_eq!(f64::from_lexical_radix(b"100000000000000000000000000000000000000000000000000011", 2).unwrap(), 9007199254740996.0);

            // Toward negative infinity
            set_float_rounding(RoundingKind::TowardNegativeInfinity);
            assert_eq!(f64::from_lexical_radix(b"-100000000000000000000000000000000000000000000000000001", 2).unwrap(), -9007199254740994.0);
            assert_eq!(f64::from_lexical_radix(b"-100000000000000000000000000000000000000000000000000011", 2).unwrap(), -9007199254740996.0);
            assert_eq!(f64::from_lexical_radix(b"100000000000000000000000000000000000000000000000000001", 2).unwrap(), 9007199254740992.0);
            assert_eq!(f64::from_lexical_radix(b"100000000000000000000000000000000000000000000000000011", 2).unwrap(), 9007199254740994.0);

            // Toward zero
            set_float_rounding(RoundingKind::TowardZero);
            assert_eq!(f64::from_lexical_radix(b"-100000000000000000000000000000000000000000000000000001", 2).unwrap(), -9007199254740992.0);
            assert_eq!(f64::from_lexical_radix(b"-100000000000000000000000000000000000000000000000000011", 2).unwrap(), -9007199254740994.0);
            assert_eq!(f64::from_lexical_radix(b"100000000000000000000000000000000000000000000000000001", 2).unwrap(), 9007199254740992.0);
            assert_eq!(f64::from_lexical_radix(b"100000000000000000000000000000000000000000000000000011", 2).unwrap(), 9007199254740994.0);

            // Reset to default
            set_float_rounding(RoundingKind::NearestTieEven);
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_exponent_default_char_digit_test() {
        unsafe {
            set_exponent_default_char(b'0')
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_exponent_default_char_period_test() {
        unsafe {
            set_exponent_default_char(b'.')
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_exponent_default_char_add_test() {
        unsafe {
            set_exponent_default_char(b'+')
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_exponent_default_char_sub_test() {
        unsafe {
            set_exponent_default_char(b'-')
        }
    }

    #[test]
    #[should_panic]
    #[cfg(all(feature = "radix"))]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_exponent_backup_char_digit_test() {
        unsafe {
            set_exponent_backup_char(b'0')
        }
    }

    #[test]
    #[should_panic]
    #[cfg(all(feature = "radix"))]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_exponent_backup_char_period_test() {
        unsafe {
            set_exponent_backup_char(b'.')
        }
    }

    #[test]
    #[should_panic]
    #[cfg(all(feature = "radix"))]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_exponent_backup_char_add_test() {
        unsafe {
            set_exponent_backup_char(b'+')
        }
    }

    #[test]
    #[should_panic]
    #[cfg(all(feature = "radix"))]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_exponent_backup_char_sub_test() {
        unsafe {
            set_exponent_backup_char(b'-')
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_nan_string_empty_test() {
        unsafe {
            set_nan_string(b"")
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_nan_string_invalid_test() {
        unsafe {
            set_nan_string(b"i")
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_inf_string_empty_test() {
        unsafe {
            set_inf_string(b"")
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_inf_string_invalid_test() {
        unsafe {
            set_inf_string(b"n")
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_inf_string_long_test() {
        unsafe {
            set_inf_string(b"infinityinfinf")
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_infinity_string_empty_test() {
        unsafe {
            set_infinity_string(b"")
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_infinity_string_invalid_test() {
        unsafe {
            set_infinity_string(b"n")
        }
    }

    #[test]
    #[should_panic]
    #[allow(deprecated)]    // TODO(ahuszagh) Remove in 1.0.
    fn set_infinity_string_short_test() {
        unsafe {
            set_infinity_string(b"i")
        }
    }
}
