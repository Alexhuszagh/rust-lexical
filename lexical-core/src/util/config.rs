//! Config settings for lexical.

// GLOBALS

/// Not a Number literal
///
/// To change the expected representation of NaN as a string,
/// change this value during before using lexical.
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut NAN_STRING: &str = "NaN";

/// Short infinity literal
///
/// To change the expected representation of Infinity as a string,
/// change this value during before using lexical.
pub static mut INF_STRING: &str = "inf";

/// Long infinity literal
///
/// To change the expected backup representation of Infinity as a string,
/// change this value during before using lexical.
pub static mut INFINITY_STRING: &str = "infinity";

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

// CONSTANTS

// Simple, fast optimization.
// Since we're declaring a variable on the stack, and our power-of-two
// alignment dramatically improved atoi performance, do it.
cfg_if! {
if #[cfg(feature = "radix")] {
    // Use 256, actually, since we seem to have memory issues with f64.
    // Clearly not sufficient memory allocated for non-base10 values.

    /// The minimum buffer size required to serialize i8 and u8.
    pub const MAX_INT8_SIZE: usize = 16;

    /// The minimum buffer size required to serialize i16 and u16.
    pub const MAX_INT16_SIZE: usize = 32;

    /// The minimum buffer size required to serialize i32 and u32.
    pub const MAX_INT32_SIZE: usize = 64;

    /// The minimum buffer size required to serialize i64 and u64.
    pub const MAX_INT64_SIZE: usize = 128;

    /// The minimum buffer size required to serialize i128 and u128.
    pub const MAX_INT128_SIZE: usize = 256;

    /// The minimum buffer size required to serialize f32 and f64.
    pub const MAX_FLOAT_SIZE: usize = 256;
} else {
    // The f64 buffer is actually a size of 60, but use 64 since it's a
    // power of 2.

    /// The minimum buffer size required to serialize i8 and u8.
    pub const MAX_INT8_SIZE: usize = 3;

    /// The minimum buffer size required to serialize i16 and u16.
    pub const MAX_INT16_SIZE: usize = 5;

    /// The minimum buffer size required to serialize i32 and u32.
    pub const MAX_INT32_SIZE: usize = 10;

    /// The minimum buffer size required to serialize i64 and u64.
    pub const MAX_INT64_SIZE: usize = 20;

    /// The minimum buffer size required to serialize i128 and u128.
    pub const MAX_INT128_SIZE: usize = 39;

    /// The minimum buffer size required to serialize f32 and f64.
    pub const MAX_FLOAT_SIZE: usize = 64;
}} // cfg_if

cfg_if! {
if #[cfg(target_pointer_width = "16")] {
    /// The minimum buffer size required to serialize isize and usize.
    pub const MAX_INTSIZE_SIZE: usize = MAX_INT16_SIZE;
} else if #[cfg(target_pointer_width = "32")] {
    /// The minimum buffer size required to serialize isize and usize.
    pub const MAX_INTSIZE_SIZE: usize = MAX_INT32_SIZE;
} else if #[cfg(target_pointer_width = "64")] {
    /// The minimum buffer size required to serialize isize and usize.
    pub const MAX_INTSIZE_SIZE: usize = MAX_INT64_SIZE;
}}  // cfg_if

pub const BUFFER_SIZE: usize = MAX_FLOAT_SIZE;

// FUNCTIONS

/// Get the exponent notation character.
pub(crate) extern "C" fn exponent_notation_char(radix: u32)
    -> u8
{
    unsafe {
        if radix >= 15 { EXPONENT_BACKUP_CHAR } else { EXPONENT_DEFAULT_CHAR }
    }
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use atof::*;
    use ftoa::*;
    use util::*;
    use util::test::*;
    use super::*;

    #[test]
    fn exponent_notation_char_test() {
        unsafe {
            assert_eq!(exponent_notation_char(2), EXPONENT_DEFAULT_CHAR);
            assert_eq!(exponent_notation_char(8), EXPONENT_DEFAULT_CHAR);
            assert_eq!(exponent_notation_char(10), EXPONENT_DEFAULT_CHAR);
            assert_eq!(exponent_notation_char(15), EXPONENT_BACKUP_CHAR);
            assert_eq!(exponent_notation_char(16), EXPONENT_BACKUP_CHAR);
            assert_eq!(exponent_notation_char(32), EXPONENT_BACKUP_CHAR);
        }
    }

    // Only enable when no other threads touch NAN_STRING or INFINITY_STRING.
    #[test]
    #[ignore]
    fn special_bytes_test() {
        let mut buffer = new_buffer();
        // Test serializing and deserializing special strings.
        assert!(try_atof32_slice(10, b"NaN").value.is_nan());
        assert!(try_atof32_slice(10, b"nan").value.is_nan());
        assert!(try_atof32_slice(10, b"NAN").value.is_nan());
        assert!(try_atof32_slice(10, b"inf").value.is_infinite());
        assert!(try_atof32_slice(10, b"INF").value.is_infinite());
        assert!(try_atof32_slice(10, b"Infinity").value.is_infinite());
        assert_eq!(f64toa_slice(f64::NAN, 10, &mut buffer), b"NaN");
        assert_eq!(f64toa_slice(f64::INFINITY, 10, &mut buffer), b"inf");

        unsafe {
            NAN_STRING = "nan";
            INF_STRING = "Infinity";
        }

        assert!(try_atof32_slice(10, b"inf").error.code == ErrorCode::InvalidDigit);
        assert!(try_atof32_slice(10, b"Infinity").value.is_infinite());
        assert_eq!(f64toa_slice(f64::NAN, 10, &mut buffer), b"nan");
        assert_eq!(f64toa_slice(f64::INFINITY, 10, &mut buffer), b"Infinity");

        unsafe {
            NAN_STRING = "NaN";
            INF_STRING = "inf";
        }
    }
}
