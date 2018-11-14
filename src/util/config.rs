//! Config settings for lexical.

// GLOBALS

/// Not a Number literal
///
/// To change the expected representation of NaN as a string,
/// change this value during before using lexical.
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut NAN_STRING: &str = "NaN";

/// Infinity literal
///
/// To change the expected representation of Infinity as a string,
/// change this value during before using lexical.
pub static mut INFINITY_STRING: &str = "inf";

/// Default character for scientific notation, used when the radix < 15.
///
/// To change the expected, default character for an exponent,
/// change this value during before using lexical.
pub static mut EXPONENT_DEFAULT_CHAR: u8 = b'e';

/// Backup character for scientific notation, used when the radix >= 15.
///
/// For numerical strings of radix >= 15, 'e' or 'E' is a valid digit,
/// and therefore may no longer be used as a marker for the exponent.
///
/// To change the expected, default character for an exponent,
/// change this value during before using lexical.
pub static mut EXPONENT_BACKUP_CHAR: u8 = b'^';

// TEST
// ----

#[cfg(test)]
mod tests {
    cfg_if! {
        if #[cfg(feature = "std")] {
            use atof::*;
            use ftoa::*;
            use util::*;

            // Only enable when no other threads touch NAN_STRING or INFINITY_STRING.
            #[test]
            #[ignore]
            fn special_string_test() {
                // Test serializing and deserializing special strings.
                assert!(atof32_bytes(b"NaN", 10).is_nan());
                assert!(atof32_bytes(b"inf", 10).is_infinite());
                assert!(!atof32_bytes(b"nan", 10).is_nan());
                assert!(!atof32_bytes(b"Infinity", 10).is_infinite());
                assert_eq!(&f64toa_string(f64::NAN, 10), "NaN");
                assert_eq!(&f64toa_string(f64::INFINITY, 10), "inf");

                unsafe {
                    NAN_STRING = "nan";
                    INFINITY_STRING = "Infinity";
                }

                assert!(!atof32_bytes(b"NaN", 10).is_nan());
                assert!(!atof32_bytes(b"inf", 10).is_infinite());
                assert!(atof32_bytes(b"nan", 10).is_nan());
                assert!(atof32_bytes(b"Infinity", 10).is_infinite());
                assert_eq!(&f64toa_string(f64::NAN, 10), "nan");
                assert_eq!(&f64toa_string(f64::INFINITY, 10), "Infinity");

                unsafe {
                    NAN_STRING = "NaN";
                    INFINITY_STRING = "inf";
                }
            }
        }
    }
}
