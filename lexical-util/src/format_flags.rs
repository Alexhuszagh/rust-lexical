//! Bitmask flags and masks for numeric formats.
//!
//! These bitflags and masks comprise a compressed struct as a 128-bit
//! integer, allowing its use in const generics. This comprises two parts:
//! flags designating which numerical components are valid in a string,
//! and masks to designate the control characters.
//!
//! The flags are designated in the lower 64 bits that modify
//! the syntax of strings that are parsed by lexical.
//!
//! Bits 8-32 are reserved for float component flags, such
//! as for example if base prefixes or postfixes are case-sensitive,
//! if leading zeros in a float are valid, etc.
//!
//! Bits 32-64 are reserved for digit separator flags. These
//! define which locations within a float or integer digit separators
//! are valid, for example, before any digits in the integer component,
//! whether consecutive digit separators are allowed, and more.
//!
//! ```text
//! 0   1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//! |I/R|F/R|E/R|+/M|R/M|e/e|+/E|R/E|e/F|S/S|S/C|N/I|N/F|R/e|e/C|e/P|
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//!
//! 16  17  18  19  20  21  22  23  24  25  26  27  28  29  30  31  32
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//! |e/S|                                                           |
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//!
//! 32  33  34  35  36  37  38  39  40  41 42  43  44  45  46  47   48
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//! |I/I|F/I|E/I|I/L|F/L|E/L|I/T|F/T|E/T|I/C|F/C|E/C|S/D|           |
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//!
//! 48  49  50  51  52  53  54  55  56  57  58  59  60  62  62  63  64
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//! |                                                               |
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//!
//! Where:
//!     Non-Digit Separator Flags:
//!         I/R = Required integer digits.
//!         F/R = Required fraction digits.
//!         E/R = Required exponent digits.
//!         +/M = No mantissa positive sign.
//!         R/M = Required positive sign.
//!         e/e = No exponent notation.
//!         +/E = No exponent positive sign.
//!         R/E = Required exponent sign.
//!         e/F = No exponent without fraction.
//!         S/S = No special (non-finite) values.
//!         S/C = Case-sensitive special (non-finite) values.
//!         N/I = No integer leading zeros.
//!         N/F = No float leading zeros.
//!         R/e = Required exponent characters.
//!         e/C = Case-sensitive exponent character.
//!         e/P = Case-sensitive base prefix.
//!         e/S = Case-sensitive base suffix.
//!
//!     Digit Separator Flags:
//!         I/I = Integer internal digit separator.
//!         F/I = Fraction internal digit separator.
//!         E/I = Exponent internal digit separator.
//!         I/L = Integer leading digit separator.
//!         F/L = Fraction leading digit separator.
//!         E/L = Exponent leading digit separator.
//!         I/T = Integer trailing digit separator.
//!         F/T = Fraction trailing digit separator.
//!         E/T = Exponent trailing digit separator.
//!         I/C = Integer consecutive digit separator.
//!         F/C = Fraction consecutive digit separator.
//!         E/C = Exponent consecutive digit separator.
//!         S/D = Special (non-finite) digit separator.
//! ```
//!
//! The upper 64-bits are designated for control characters and radixes,
//! such as the digit separator character, decimal point, radix characters,
//! and more.
//!
//! ```text
//! 64  65  66  67  68  69  70  71  72  73  74  75  76  77  78  79  80
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//! |     Digit Separator       |   |       Decimal Point       |   |
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//!
//! 80  81  82  83  84  85  86  87  88  89  90  91  92  93  94  95  96
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//! |      Exponent Symbol      |   |        Base Prefix        |   |
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//!
//! 96  97  98  99  100 101 102 103 104 105 106 107 108 109 110 111 112
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//! |        Base Suffix        |   |    Mantissa Radix     |       |
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//!
//! 112 113 114 115 116 117 118 119 120 121 122 123 124 125 126 127 128
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//! |     Exponent Base     |       |    Exponent Radix     |       |
//! +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
//! ```
//!
//!
//! Note:
//! -----
//!
//! In order to limit the format specification and avoid parsing
//! non-numerical data, all number formats require some significant
//! digits. Examples of always invalid numbers include:
//! - ``
//! - `.`
//! - `e`
//! - `e7`
//!
//! Test Cases:
//! -----------
//!
//! The following test-cases are used to define whether a literal or
//! a string float is valid in a given language, and these tests are
//! used to denote features in pre-defined formats. Only a few
//! of these flags may modify the parsing behavior of integers.
//! Integer parsing is assumed to be derived from float parsing,
//! so if consecutive digit separators are valid in the integer
//! component of a float, they are also valid in an integer.
//!
//! ```text
//! 0: '.3'         // Non-required integer.
//! 1: '3.'         // Non-required fraction.
//! 2: '3e'         // Non-required exponent.
//! 3. '+3.0'       // Mantissa positive sign.
//! 4: '3.0e7'      // Exponent notation.
//! 5: '3.0e+7'     // Exponent positive sign.
//! 6. '3e7'        // Exponent notation without fraction.
//! 7: 'NaN'        // Special (non-finite) values.
//! 8: 'NAN'        // Case-sensitive special (non-finite) values.
//! 9: '3_4.01'     // Integer internal digit separator.
//! A: '3.0_1'      // Fraction internal digit separator.
//! B: '3.0e7_1'    // Exponent internal digit separator.
//! C: '_3.01'      // Integer leading digit separator.
//! D: '3._01'      // Fraction leading digit separator.
//! E: '3.0e_71'    // Exponent leading digit separator.
//! F: '3_.01'      // Integer trailing digit separator.
//! G: '3.01_'      // Fraction trailing digit separator.
//! H: '3.0e71_'    // Exponent trailing digit separator.
//! I: '3__4.01'    // Integer consecutive digit separator.
//! J: '3.0__1'     // Fraction consecutive digit separator.
//! K: '3.0e7__1'   // Exponent consecutive digit separator.
//! L: 'In_f'       // Special (non-finite) digit separator.
//! M: '010'        // No integer leading zeros.
//! N: '010.0'      // No float leading zeros.
//! O: '1.0'        // No required exponent notation.
//! P: '3.0E7'      // Case-insensitive exponent character.
//! P: '0x3.0'      // Case-insensitive base prefix.
//! P: '3.0H'       // Case-insensitive base postfix.
//! ```
//!
//! Currently Supported Programming and Data Languages:
//! ---------------------------------------------------
//!
//! 1. Rust
//! 2. Python
//! 3. C++ (98, 03, 11, 14, 17)
//! 4. C (89, 90, 99, 11, 18)
//! 5. Ruby
//! 6. Swift
//! 7. Go
//! 8. Haskell
//! 9. Javascript
//! 10. Perl
//! 11. PHP
//! 12. Java
//! 13. R
//! 14. Kotlin
//! 15. Julia
//! 16. C# (ISO-1, ISO-2, 3, 4, 5, 6, 7)
//! 17. Kawa
//! 18. Gambit-C
//! 19. Guile
//! 20. Clojure
//! 21. Erlang
//! 22. Elm
//! 23. Scala
//! 24. Elixir
//! 25. FORTRAN
//! 26. D
//! 27. Coffeescript
//! 28. Cobol
//! 29. F#
//! 30. Visual Basic
//! 31. OCaml
//! 32. Objective-C
//! 33. ReasonML
//! 34. Octave
//! 35. Matlab
//! 36. Zig
//! 37. SageMath
//! 38. JSON
//! 39. TOML
//! 40. XML
//! 41. SQLite
//! 42. PostgreSQL
//! 43. MySQL
//! 44. MongoDB

#![cfg(feature = "parse")]
#![cfg_attr(rustfmt, rustfmt::skip)]

use static_assertions::const_assert;

// FLAG ASSERTIONS
// ---------------

// Ensure all our bit flags are valid.
macro_rules! check_subsequent_flags {
    ($x:ident, $y:ident) => {
        const_assert!($x << 1 == $y);
    };
}

// NON-DIGIT SEPARATOR FLAGS & MASKS
// ---------------------------------

/// Digits are required before the decimal point.
pub const REQUIRED_INTEGER_DIGITS: u128 =
    0x00000000000000000000000000000001;

/// Digits are required after the decimal point.
/// This check will only occur if the decimal point is present.
pub const REQUIRED_FRACTION_DIGITS: u128 =
    0x00000000000000000000000000000002;

/// Digits are required after the exponent character.
/// This check will only occur if the exponent character is present.
pub const REQUIRED_EXPONENT_DIGITS: u128 =
    0x00000000000000000000000000000004;

/// Digits are required before or after the control characters.
pub const REQUIRED_DIGITS: u128 =
    REQUIRED_INTEGER_DIGITS |
    REQUIRED_FRACTION_DIGITS |
    REQUIRED_EXPONENT_DIGITS;

/// Positive sign before the mantissa is not allowed.
pub const NO_POSITIVE_MANTISSA_SIGN: u128 =
    0x00000000000000000000000000000008;

/// Positive sign before the mantissa is required.
pub const REQUIRED_MANTISSA_SIGN: u128 =
    0x00000000000000000000000000000010;

/// Exponent notation is not allowed.
pub const NO_EXPONENT_NOTATION: u128 =
    0x00000000000000000000000000000020;

/// Positive sign before the exponent is not allowed.
pub const NO_POSITIVE_EXPONENT_SIGN: u128 =
    0x00000000000000000000000000000040;

/// Positive sign before the exponent is required.
pub const REQUIRED_EXPONENT_SIGN: u128 =
    0x00000000000000000000000000000080;

/// Exponent without a fraction component is not allowed.
///
/// This only checks if a decimal point precedes the exponent character.
/// To require fraction digits or exponent digits with this check,
/// please use the appropriate flags.
pub const NO_EXPONENT_WITHOUT_FRACTION: u128 =
    0x00000000000000000000000000000100;

/// Special (non-finite) values are not allowed.
pub const NO_SPECIAL: u128 =
    0x00000000000000000000000000000200;

/// Special (non-finite) values are case-sensitive.
pub const CASE_SENSITIVE_SPECIAL: u128 =
    0x00000000000000000000000000000400;

/// Leading zeros before an integer value are not allowed.
///
/// If the value is a literal, then this distinction applies
/// when the value is treated like an integer literal, typically
/// when there is no decimal point. If the value is parsed,
/// then this distinction applies when the value as parsed
/// as an integer.
///
/// # Warning
///
/// This also does not mean that the value parsed will be correct,
/// for example, in languages like C, this will not auto-
/// deduce that the radix is 8 with leading zeros, for an octal
/// literal.
pub const NO_INTEGER_LEADING_ZEROS: u128 =
    0x00000000000000000000000000000800;

/// Leading zeros before a float value are not allowed.
///
/// If the value is a literal, then this distinction applies
/// when the value is treated like an integer float, typically
/// when there is a decimal point. If the value is parsed,
/// then this distinction applies when the value as parsed
/// as a float.
///
/// # Warning
///
/// This also does not mean that the value parsed will be correct,
/// for example, in languages like C, this will not auto-
/// deduce that the radix is 8 with leading zeros, for an octal
/// literal.
pub const NO_FLOAT_LEADING_ZEROS: u128 =
    0x00000000000000000000000000001000;

/// Exponent notation is required.
///
/// Valid floats must contain an exponent notation character, and if
/// applicable, a sign character and digits afterwards.
pub const REQUIRED_EXPONENT_NOTATION: u128 =
    0x00000000000000000000000000002000;

// Non-digit separator flags.
const_assert!(REQUIRED_INTEGER_DIGITS == 1);
check_subsequent_flags!(REQUIRED_INTEGER_DIGITS, REQUIRED_FRACTION_DIGITS);
check_subsequent_flags!(REQUIRED_FRACTION_DIGITS, REQUIRED_EXPONENT_DIGITS);
check_subsequent_flags!(REQUIRED_EXPONENT_DIGITS, NO_POSITIVE_MANTISSA_SIGN);
check_subsequent_flags!(NO_POSITIVE_MANTISSA_SIGN, REQUIRED_MANTISSA_SIGN);
check_subsequent_flags!(REQUIRED_MANTISSA_SIGN, NO_EXPONENT_NOTATION);
check_subsequent_flags!(NO_EXPONENT_NOTATION, NO_POSITIVE_EXPONENT_SIGN);
check_subsequent_flags!(NO_POSITIVE_EXPONENT_SIGN, REQUIRED_EXPONENT_SIGN);
check_subsequent_flags!(REQUIRED_EXPONENT_SIGN, NO_EXPONENT_WITHOUT_FRACTION);
check_subsequent_flags!(NO_EXPONENT_WITHOUT_FRACTION, NO_SPECIAL);
check_subsequent_flags!(NO_SPECIAL, CASE_SENSITIVE_SPECIAL);
check_subsequent_flags!(NO_SPECIAL, CASE_SENSITIVE_SPECIAL);
check_subsequent_flags!(CASE_SENSITIVE_SPECIAL, NO_INTEGER_LEADING_ZEROS);
check_subsequent_flags!(NO_INTEGER_LEADING_ZEROS, NO_FLOAT_LEADING_ZEROS);
check_subsequent_flags!(NO_FLOAT_LEADING_ZEROS, REQUIRED_EXPONENT_NOTATION);

// DIGIT SEPARATOR FLAGS & MASKS
// -----------------------------

/// Digit separators are allowed between integer digits.
pub const INTEGER_INTERNAL_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000000100000000;

/// Digit separators are allowed between fraction digits.
pub const FRACTION_INTERNAL_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000000200000000;

/// Digit separators are allowed between exponent digits.
pub const EXPONENT_INTERNAL_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000000400000000;

/// A digit separator is allowed before any integer digits.
pub const INTEGER_LEADING_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000000800000000;

/// A digit separator is allowed before any fraction digits.
pub const FRACTION_LEADING_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000001000000000;

/// A digit separator is allowed before any exponent digits.
pub const EXPONENT_LEADING_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000002000000000;

/// A digit separator is allowed after any integer digits.
pub const INTEGER_TRAILING_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000004000000000;

/// A digit separator is allowed after any fraction digits.
pub const FRACTION_TRAILING_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000008000000000;

/// A digit separator is allowed after any exponent digits.
pub const EXPONENT_TRAILING_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000010000000000;

/// Multiple consecutive integer digit separators are allowed.
pub const INTEGER_CONSECUTIVE_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000020000000000;


/// Multiple consecutive fraction digit separators are allowed.
pub const FRACTION_CONSECUTIVE_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000040000000000;

/// Multiple consecutive exponent digit separators are allowed.
pub const EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000080000000000;

/// Digit separators are allowed between digits.
pub const INTERNAL_DIGIT_SEPARATOR: u128 =
    INTEGER_INTERNAL_DIGIT_SEPARATOR |
    FRACTION_INTERNAL_DIGIT_SEPARATOR |
    EXPONENT_INTERNAL_DIGIT_SEPARATOR;

/// A digit separator is allowed before any digits.
pub const LEADING_DIGIT_SEPARATOR: u128 =
    INTEGER_LEADING_DIGIT_SEPARATOR |
    FRACTION_LEADING_DIGIT_SEPARATOR |
    EXPONENT_LEADING_DIGIT_SEPARATOR;

/// A digit separator is allowed after any digits.
pub const TRAILING_DIGIT_SEPARATOR: u128 =
    INTEGER_TRAILING_DIGIT_SEPARATOR |
    FRACTION_TRAILING_DIGIT_SEPARATOR |
    EXPONENT_TRAILING_DIGIT_SEPARATOR;

/// Multiple consecutive digit separators are allowed.
pub const CONSECUTIVE_DIGIT_SEPARATOR: u128 =
    INTEGER_CONSECUTIVE_DIGIT_SEPARATOR |
    FRACTION_CONSECUTIVE_DIGIT_SEPARATOR |
    EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR;

/// Any digit separators are allowed in special (non-finite) values.
pub const SPECIAL_DIGIT_SEPARATOR: u128 =
    0x00000000000000000000100000000000;

// Digit separator flags.
const_assert!(INTEGER_INTERNAL_DIGIT_SEPARATOR == 1 << 32);
check_subsequent_flags!(INTEGER_INTERNAL_DIGIT_SEPARATOR, FRACTION_INTERNAL_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_INTERNAL_DIGIT_SEPARATOR, EXPONENT_INTERNAL_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_INTERNAL_DIGIT_SEPARATOR, INTEGER_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_LEADING_DIGIT_SEPARATOR, FRACTION_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_LEADING_DIGIT_SEPARATOR, EXPONENT_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_LEADING_DIGIT_SEPARATOR, INTEGER_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_TRAILING_DIGIT_SEPARATOR, FRACTION_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_TRAILING_DIGIT_SEPARATOR, EXPONENT_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_TRAILING_DIGIT_SEPARATOR, INTEGER_CONSECUTIVE_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_CONSECUTIVE_DIGIT_SEPARATOR, FRACTION_CONSECUTIVE_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_CONSECUTIVE_DIGIT_SEPARATOR, EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR, SPECIAL_DIGIT_SEPARATOR);

// CONSTROL CHARACTER & RADIX MASKS
// --------------------------------

/// Mask to extract the digit separator character.
pub const DIGIT_SEPARATOR: u128 =
    0x000000000000007F0000000000000000;

/// Shift to convert to and from a digit separator as a `u8`.
pub const DIGIT_SEPARATOR_SHIFT: i32 = 64;

/// Mask to extract the decimal point character.
pub const DECIMAL_POINT: u128 =
    0x0000000000007F000000000000000000;

/// Shift to convert to and from a decimal point as a `u8`.
pub const DECIMAL_POINT_SHIFT: i32 = 72;

/// Mask to extract the exponent character.
pub const EXPONENT: u128 =
    0x00000000007F00000000000000000000;

/// Shift to convert to and from an exponent as a `u8`.
pub const EXPONENT_SHIFT: i32 = 80;

/// Mask to extract the base prefix character.
pub const BASE_PREFIX: u128 =
    0x000000007F0000000000000000000000;

/// Shift to convert to and from a base prefix as a `u8`.
pub const BASE_PREFIX_SHIFT: i32 = 88;

/// Mask to extract the base suffix character.
pub const BASE_SUFFIX: u128 =
    0x0000007F000000000000000000000000;

/// Shift to convert to and from a base suffix as a `u8`.
pub const BASE_SUFFIX_SHIFT: i32 = 96;

/// Mask to extract the mantissa radix: the radix for the significant digits.
pub const MANTISSA_RADIX: u128 =
    0x00003F00000000000000000000000000;

/// Shift to convert to and from a mantissa radix as a `u32`.
pub const MANTISSA_RADIX_SHIFT: i32 = 104;

/// Mask to extract the exponent base: the base the exponent is raised to.
pub const EXPONENT_BASE: u128 =
    0x003F0000000000000000000000000000;

/// Shift to convert to and from an exponent base as a `u32`.
pub const EXPONENT_BASE_SHIFT: i32 = 112;

/// Mask to extract the exponent radix: the radix for the exponent digits.
pub const EXPONENT_RADIX: u128 =
    0x3F000000000000000000000000000000;

/// Shift to convert to and from an exponent radix as a `u32`.
pub const EXPONENT_RADIX_SHIFT: i32 = 120;

// EXTRACTORS
// ----------

/// Extract the digit separator from the format packed struct.
#[inline]
pub const fn digit_separator(format: u128) -> u8 {
    ((format & DIGIT_SEPARATOR) >> DIGIT_SEPARATOR_SHIFT) as u8
}

/// Extract the decimal point character from the format packed struct.
#[inline]
pub const fn decimal_point(format: u128) -> u8 {
    ((format & DECIMAL_POINT) >> DECIMAL_POINT_SHIFT) as u8
}

/// Extract the exponent character from the format packed struct.
#[inline]
pub const fn exponent(format: u128) -> u8 {
    ((format & EXPONENT) >> EXPONENT_SHIFT) as u8
}

/// Extract the base prefix character from the format packed struct.
#[inline]
pub const fn base_prefix(format: u128) -> u8 {
    ((format & BASE_PREFIX) >> BASE_PREFIX_SHIFT) as u8
}

/// Extract the base suffix character from the format packed struct.
#[inline]
pub const fn base_suffix(format: u128) -> u8 {
    ((format & BASE_SUFFIX) >> BASE_SUFFIX_SHIFT) as u8
}

/// Extract the mantissa radix from the format packed struct.
#[inline]
pub const fn mantissa_radix(format: u128) -> u8 {
    ((format & MANTISSA_RADIX) >> MANTISSA_RADIX_SHIFT) as u8
}

/// Extract the exponent base from the format packed struct.
#[inline]
pub const fn exponent_base(format: u128) -> u8 {
    ((format & EXPONENT_BASE) >> EXPONENT_BASE_SHIFT) as u8
}

/// Extract the exponent radix from the format packed struct.
#[inline]
pub const fn exponent_radix(format: u128) -> u8 {
    ((format & EXPONENT_RADIX) >> EXPONENT_RADIX_SHIFT) as u8
}

// VALIDATORS
// ----------

///// Determine if the digit separator is valid.
//#[inline]
//pub const fn is_valid_digit_separator(format: u128) -> bool {
//    let radix = (format & MANTISSA_RADIX).max(format & EXPONENT_RADIX)
//    // TODO(ahuszagh) Need to convert to a digit...
////    match ch {
////        b'0'..=b'9' => false,
////        b'+' | b'-' => false,
////        _ => ch.is_ascii(),
////    }
//}
