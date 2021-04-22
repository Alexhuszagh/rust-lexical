//! Implementation of `format` with the feature enabled.

#![cfg(feature = "format")]

use bitflags::bitflags;

use super::super::config;
use super::flags;
use super::traits::*;

// NUMBER FORMAT

bitflags! {
    /// Bitflags for a number format.
    ///
    /// This is used to derive the high-level bitflags. The default
    /// representation has no digit separators, no required integer or
    /// fraction digits, required exponent digits, a b'.' character
    /// for the decimal point, a b'e' character for the exponent,
    /// a b'^' character for the exponent backup, and does not
    /// use the incorrect or lossy parser.
    ///
    /// Bit Flags Layout
    /// ----------------
    ///
    /// The bitflags has the lower bits designated for flags that modify
    /// the parsing behavior of lexical, with 7 bits each set for the
    /// decimal point, exponent, backup exponent, and digit separator,
    /// allowing any valid ASCII character as punctuation. The first
    /// 12-bits are reserved for non-digit separator flags, bits 12-18
    /// for the radix (if the radix feature is enabled), bits 18-25 for
    /// the exponent character, bits 25-32 for the exponent backup
    /// character, bits 32-48 are reserved for digit separator flags,
    /// bit 48 for the incorrect (fastest) float parser, bit 49 for the
    /// lossy (intermediate) float parser, bits 50-57 for the decimal
    /// point character, and the last 7 bits for the digit separator
    /// character.
    ///
    /// ```text
    /// 0   1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |I/R|F/R|E/R|+/M|R/M|e/e|+/E|R/E|e/F|S/S|S/C|   |      R/D      |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 16  17  18  19  20  21  22  23  24  25  26  27  28  29  30  31  32
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |  R/D  |         Exponent          |     Exponent Backup       |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 32  33  34  35  36  37  38  39  40  41 42  43  44  45  46  47   48
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |I/I|F/I|E/I|I/L|F/L|E/L|I/T|F/T|E/T|I/C|F/C|E/C|S/D|           |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 48  49  50  51  52  53  54  55  56  57  58  59  60  62  62  63  64
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |L/I|L/L|      Decimal Point        |     Digit Separator       |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// Where:
    ///     I/R = Required integer digits.
    ///     F/R = Required fraction digits.
    ///     E/R = Required exponent digits.
    ///     +/M = No mantissa positive sign.
    ///     R/M = Required positive sign.
    ///     e/e = No exponent notation.
    ///     +/E = No exponent positive sign.
    ///     R/E = Required exponent sign.
    ///     e/F = No exponent without fraction.
    ///     S/S = No special (non-finite) values.
    ///     S/C = Case-sensitive special (non-finite) values.
    ///     I/I = Integer internal digit separator.
    ///     F/I = Fraction internal digit separator.
    ///     E/I = Exponent internal digit separator.
    ///     I/L = Integer leading digit separator.
    ///     F/L = Fraction leading digit separator.
    ///     E/L = Exponent leading digit separator.
    ///     I/T = Integer trailing digit separator.
    ///     F/T = Fraction trailing digit separator.
    ///     E/T = Exponent trailing digit separator.
    ///     I/C = Integer consecutive digit separator.
    ///     F/C = Fraction consecutive digit separator.
    ///     E/C = Exponent consecutive digit separator.
    ///     S/D = Special (non-finite) digit separator.
    ///     R/D = Radix (as a 6-bit integer).
    ///     L/I = Incorrect algorithm (everything done with native floats).
    ///     L/L = Lossy algorithm (using the fast and moderate paths).
    /// ```
    ///
    /// Note:
    /// -----
    ///
    /// In order to limit the format specification and avoid parsing
    /// non-numerical data, all number formats require some significant
    /// digits. Examples of always invalid numbers include:
    /// - ``
    /// - `.`
    /// - `e`
    /// - `e7`
    ///
    /// Test Cases:
    /// -----------
    ///
    /// The following test-cases are used to define whether a literal or
    /// a string float is valid in a given language, and these tests are
    /// used to denote features in pre-defined formats. Only a few
    /// of these flags may modify the parsing behavior of integers.
    /// Integer parsing is assumed to be derived from float parsing,
    /// so if consecutive digit separators are valid in the integer
    /// component of a float, they are also valid in an integer.
    ///
    /// ```text
    /// 0: '.3'         // Non-required integer.
    /// 1: '3.'         // Non-required fraction.
    /// 2: '3e'         // Non-required exponent.
    /// 3. '+3.0'       // Mantissa positive sign.
    /// 4: '3.0e7'      // Exponent notation.
    /// 5: '3.0e+7'     // Exponent positive sign.
    /// 6. '3e7'        // Exponent notation without fraction.
    /// 7: 'NaN'        // Special (non-finite) values.
    /// 8: 'NAN'        // Case-sensitive special (non-finite) values.
    /// 9: '3_4.01'     // Integer internal digit separator.
    /// A: '3.0_1'      // Fraction internal digit separator.
    /// B: '3.0e7_1'    // Exponent internal digit separator.
    /// C: '_3.01'      // Integer leading digit separator.
    /// D: '3._01'      // Fraction leading digit separator.
    /// E: '3.0e_71'    // Exponent leading digit separator.
    /// F: '3_.01'      // Integer trailing digit separator.
    /// G: '3.01_'      // Fraction trailing digit separator.
    /// H: '3.0e71_'    // Exponent trailing digit separator.
    /// I: '3__4.01'    // Integer consecutive digit separator.
    /// J: '3.0__1'     // Fraction consecutive digit separator.
    /// K: '3.0e7__1'   // Exponent consecutive digit separator.
    /// L: 'In_f'       // Special (non-finite) digit separator.
    /// M: '010'        // No integer leading zeros.
    /// N: '010.0'      // No float leading zeros.
    /// ```
    ///
    /// Currently Supported Programming and Data Languages:
    /// ---------------------------------------------------
    ///
    /// 1. Rust
    /// 2. Python
    /// 3. C++ (98, 03, 11, 14, 17)
    /// 4. C (89, 90, 99, 11, 18)
    /// 5. Ruby
    /// 6. Swift
    /// 7. Go
    /// 8. Haskell
    /// 9. Javascript
    /// 10. Perl
    /// 11. PHP
    /// 12. Java
    /// 13. R
    /// 14. Kotlin
    /// 15. Julia
    /// 16. C# (ISO-1, ISO-2, 3, 4, 5, 6, 7)
    /// 17. Kawa
    /// 18. Gambit-C
    /// 19. Guile
    /// 20. Clojure
    /// 21. Erlang
    /// 22. Elm
    /// 23. Scala
    /// 24. Elixir
    /// 25. FORTRAN
    /// 26. D
    /// 27. Coffeescript
    /// 28. Cobol
    /// 29. F#
    /// 30. Visual Basic
    /// 31. OCaml
    /// 32. Objective-C
    /// 33. ReasonML
    /// 34. Octave
    /// 35. Matlab
    /// 36. Zig
    /// 37. SageMath
    /// 38. JSON
    /// 39. TOML
    /// 40. XML
    /// 41. SQLite
    /// 42. PostgreSQL
    /// 43. MySQL
    /// 44. MongoDB
    #[derive(Default)]
    pub struct NumberFormat: u64 {
        // MASKS & FLAGS

        /// Mask to extract the flag bits.
        #[doc(hidden)]
        const FLAG_MASK                             = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::REQUIRED_MANTISSA_SIGN.bits
            | Self::NO_EXPONENT_NOTATION.bits
            | Self::NO_POSITIVE_EXPONENT_SIGN.bits
            | Self::REQUIRED_EXPONENT_SIGN.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_SPECIAL.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
            | Self::SPECIAL_DIGIT_SEPARATOR.bits
        );

        /// Mask to extract the flag bits controlling interface parsing.
        ///
        /// This mask controls all the flags handled by the interface,
        /// omitting those that are handled prior. This limits the
        /// number of match paths required to determine the correct
        /// interface.
        const INTERFACE_FLAG_MASK                   = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_NOTATION.bits
            | Self::NO_POSITIVE_EXPONENT_SIGN.bits
            | Self::REQUIRED_EXPONENT_SIGN.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        /// Mask to extract digit separator flags.
        #[doc(hidden)]
        const DIGIT_SEPARATOR_FLAG_MASK             = (
            Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
            | Self::SPECIAL_DIGIT_SEPARATOR.bits
        );

        /// Mask to extract integer digit separator flags.
        #[doc(hidden)]
        const INTEGER_DIGIT_SEPARATOR_FLAG_MASK     = (
            Self::INTEGER_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::INTEGER_LEADING_DIGIT_SEPARATOR.bits
            | Self::INTEGER_TRAILING_DIGIT_SEPARATOR.bits
            | Self::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        /// Mask to extract fraction digit separator flags.
        #[doc(hidden)]
        const FRACTION_DIGIT_SEPARATOR_FLAG_MASK     = (
            Self::FRACTION_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::FRACTION_TRAILING_DIGIT_SEPARATOR.bits
            | Self::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        /// Mask to extract exponent digit separator flags.
        #[doc(hidden)]
        const EXPONENT_DIGIT_SEPARATOR_FLAG_MASK     = (
            Self::EXPONENT_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_LEADING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_TRAILING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        /// Mask to extract exponent flags.
        #[doc(hidden)]
        const EXPONENT_FLAG_MASK                    = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_POSITIVE_EXPONENT_SIGN.bits
            | Self::REQUIRED_EXPONENT_SIGN.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::EXPONENT_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_LEADING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_TRAILING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // NON-DIGIT SEPARATOR FLAGS & MASKS
        // See `flags` for documentation.

        #[doc(hidden)]
        const REQUIRED_INTEGER_DIGITS               = flags::REQUIRED_INTEGER_DIGITS;

        #[doc(hidden)]
        const REQUIRED_FRACTION_DIGITS              = flags::REQUIRED_FRACTION_DIGITS;

        #[doc(hidden)]
        const REQUIRED_EXPONENT_DIGITS              = flags::REQUIRED_EXPONENT_DIGITS;

        #[doc(hidden)]
        const REQUIRED_DIGITS                       = flags::REQUIRED_DIGITS;

        #[doc(hidden)]
        const NO_POSITIVE_MANTISSA_SIGN             = flags::NO_POSITIVE_MANTISSA_SIGN;

        #[doc(hidden)]
        const REQUIRED_MANTISSA_SIGN                = flags::REQUIRED_MANTISSA_SIGN;

        #[doc(hidden)]
        const NO_EXPONENT_NOTATION                  = flags::NO_EXPONENT_NOTATION;

        #[doc(hidden)]
        const NO_POSITIVE_EXPONENT_SIGN             = flags::NO_POSITIVE_EXPONENT_SIGN;

        #[doc(hidden)]
        const REQUIRED_EXPONENT_SIGN                = flags::REQUIRED_EXPONENT_SIGN;

        #[doc(hidden)]
        const NO_EXPONENT_WITHOUT_FRACTION          = flags::NO_EXPONENT_WITHOUT_FRACTION;

        #[doc(hidden)]
        const NO_SPECIAL                            = flags::NO_SPECIAL;

        #[doc(hidden)]
        const CASE_SENSITIVE_SPECIAL                = flags::CASE_SENSITIVE_SPECIAL;

        #[doc(hidden)]
        const NO_INTEGER_LEADING_ZEROS              = flags::NO_INTEGER_LEADING_ZEROS;

        #[doc(hidden)]
        const NO_FLOAT_LEADING_ZEROS                = flags::NO_FLOAT_LEADING_ZEROS;

        // DIGIT SEPARATOR FLAGS & MASKS
        // See `flags` for documentation.

        #[doc(hidden)]
        const INTEGER_INTERNAL_DIGIT_SEPARATOR      = flags::INTEGER_INTERNAL_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const INTEGER_LEADING_DIGIT_SEPARATOR       = flags::INTEGER_LEADING_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const INTEGER_TRAILING_DIGIT_SEPARATOR      = flags::INTEGER_TRAILING_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const INTEGER_CONSECUTIVE_DIGIT_SEPARATOR   = flags::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const FRACTION_INTERNAL_DIGIT_SEPARATOR     = flags::FRACTION_INTERNAL_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const FRACTION_LEADING_DIGIT_SEPARATOR      = flags::FRACTION_LEADING_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const FRACTION_TRAILING_DIGIT_SEPARATOR     = flags::FRACTION_TRAILING_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const FRACTION_CONSECUTIVE_DIGIT_SEPARATOR  = flags::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const EXPONENT_INTERNAL_DIGIT_SEPARATOR     = flags::EXPONENT_INTERNAL_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const EXPONENT_LEADING_DIGIT_SEPARATOR      = flags::EXPONENT_LEADING_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const EXPONENT_TRAILING_DIGIT_SEPARATOR     = flags::EXPONENT_TRAILING_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR  = flags::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const INTERNAL_DIGIT_SEPARATOR              = flags::INTERNAL_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const LEADING_DIGIT_SEPARATOR               = flags::LEADING_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const TRAILING_DIGIT_SEPARATOR              = flags::TRAILING_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const CONSECUTIVE_DIGIT_SEPARATOR           = flags::CONSECUTIVE_DIGIT_SEPARATOR;

        #[doc(hidden)]
        const SPECIAL_DIGIT_SEPARATOR               = flags::SPECIAL_DIGIT_SEPARATOR;

        // CONVERSION PRECISION FLAGS & MASKS
        // See `flags` for documentation.

        #[doc(hidden)]
        const INCORRECT                             = flags::INCORRECT;

        #[doc(hidden)]
        const LOSSY                                 = flags::LOSSY;

        // PRE-DEFINED
        //
        // Sample Format Shorthand:
        // ------------------------
        //
        // The format shorthand lists the test cases, and if applicable,
        // the digit separator character. For example, the shorthand
        // `[134-_]` specifies it passes tests 1, 3, and 4, and uses
        // `'_'` as a digit-separator character. Meanwhile, `[0]` means it
        // passes test 0, and has no digit separator.

        // RUST LITERAL [4569ABFGHIJKMN-_]
        /// Float format for a Rust literal floating-point number.
        const RUST_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // RUST STRING [0134567MN]
        /// Float format to parse a Rust float from string.
        const RUST_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // RUST STRING STRICT [01345678MN]
        /// `RUST_STRING`, but enforces strict equality for special values.
        const RUST_STRING_STRICT = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        /// Float format for a Python literal floating-point number.
        const PYTHON_LITERAL = Self::PYTHON3_LITERAL.bits;

        /// Float format to parse a Python float from string.
        const PYTHON_STRING = Self::PYTHON3_STRING.bits;

        // PYTHON3 LITERAL [013456N]
        /// Float format for a Python3 literal floating-point number.
        const PYTHON3_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
        );

        // PYTHON3 STRING [0134567MN]
        /// Float format to parse a Python3 float from string.
        const PYTHON3_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // PYTHON2 LITERAL [013456MN]
        /// Float format for a Python2 literal floating-point number.
        const PYTHON2_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // PYTHON2 STRING [0134567MN]
        /// Float format to parse a Python2 float from string.
        const PYTHON2_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        /// Float format for a C++ literal floating-point number.
        const CXX_LITERAL = Self::CXX17_LITERAL.bits;

        /// Float format to parse a C++ float from string.
        const CXX_STRING = Self::CXX17_STRING.bits;

        // C++17 LITERAL [01345689ABMN-']
        /// Float format for a C++17 literal floating-point number.
        const CXX17_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'\'')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // C++17 STRING [013456MN]
        const CXX17_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++14 LITERAL [01345689ABMN-']
        /// Float format for a C++14 literal floating-point number.
        const CXX14_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'\'')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // C++14 STRING [013456MN]
        /// Float format to parse a C++14 float from string.
        const CXX14_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++11 LITERAL [0134568MN]
        /// Float format for a C++11 literal floating-point number.
        const CXX11_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C++11 STRING [013456MN]
        /// Float format to parse a C++11 float from string.
        const CXX11_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++03 LITERAL [0134567MN]
        /// Float format for a C++03 literal floating-point number.
        const CXX03_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C++03 STRING [013456MN]
        /// Float format to parse a C++03 float from string.
        const CXX03_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++98 LITERAL [0134567MN]
        /// Float format for a C++98 literal floating-point number.
        const CXX98_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C++98 STRING [013456MN]
        /// Float format to parse a C++98 float from string.
        const CXX98_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        /// Float format for a C literal floating-point number.
        const C_LITERAL = Self::C18_LITERAL.bits;

        /// Float format to parse a C float from string.
        const C_STRING = Self::C18_STRING.bits;

        // C18 LITERAL [0134568MN]
        /// Float format for a C18 literal floating-point number.
        const C18_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C18 STRING [013456MN]
        /// Float format to parse a C18 float from string.
        const C18_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C11 LITERAL [0134568MN]
        /// Float format for a C11 literal floating-point number.
        const C11_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C11 STRING [013456MN]
        /// Float format to parse a C11 float from string.
        const C11_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C99 LITERAL [0134568MN]
        /// Float format for a C99 literal floating-point number.
        const C99_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C99 STRING [013456MN]
        /// Float format to parse a C99 float from string.
        const C99_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C90 LITERAL [0134567MN]
        /// Float format for a C90 literal floating-point number.
        const C90_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C90 STRING [013456MN]
        /// Float format to parse a C90 float from string.
        const C90_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C89 LITERAL [0134567MN]
        /// Float format for a C89 literal floating-point number.
        const C89_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C89 STRING [013456MN]
        /// Float format to parse a C89 float from string.
        const C89_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // RUBY LITERAL [345689AM-_]
        /// Float format for a Ruby literal floating-point number.
        const RUBY_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // RUBY STRING [01234569ABMN-_]
        /// Float format to parse a Ruby float from string.
        // Note: Amazingly, Ruby 1.8+ do not allow parsing special values.
        const RUBY_STRING = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // SWIFT LITERAL [34569ABFGHIJKMN-_]
        /// Float format for a Swift literal floating-point number.
        const SWIFT_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // SWIFT STRING [13456MN]
        /// Float format to parse a Swift float from string.
        const SWIFT_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
        );

        // GO LITERAL [0134567MN]
        /// Float format for a Golang literal floating-point number.
        const GO_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GO STRING [013456MN]
        /// Float format to parse a Golang float from string.
        const GO_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
        );

        // HASKELL LITERAL [456MN]
        /// Float format for a Haskell literal floating-point number.
        const HASKELL_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::NO_SPECIAL.bits
        );

        // HASKELL STRING [45678MN]
        /// Float format to parse a Haskell float from string.
        const HASKELL_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // JAVASCRIPT LITERAL [01345678M]
        /// Float format for a Javascript literal floating-point number.
        const JAVASCRIPT_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // JAVASCRIPT STRING [012345678MN]
        /// Float format to parse a Javascript float from string.
        const JAVASCRIPT_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // PERL LITERAL [0134569ABDEFGHIJKMN-_]
        /// Float format for a Perl literal floating-point number.
        const PERL_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // PERL STRING [01234567MN]
        /// Float format to parse a Perl float from string.
        const PERL_STRING = Self::PERMISSIVE.bits;

        // PHP LITERAL [01345678MN]
        /// Float format for a PHP literal floating-point number.
        const PHP_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // PHP STRING [0123456MN]
        /// Float format to parse a PHP float from string.
        const PHP_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::NO_SPECIAL.bits
        );

        // JAVA LITERAL [0134569ABIJKMN-_]
        /// Float format for a Java literal floating-point number.
        const JAVA_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // JAVA STRING [01345678MN]
        /// Float format to parse a Java float from string.
        const JAVA_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // R LITERAL [01345678MN]
        /// Float format for a R literal floating-point number.
        const R_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // R STRING [01234567MN]
        /// Float format to parse a R float from string.
        const R_STRING = Self::PERMISSIVE.bits;

        // KOTLIN LITERAL [0134569ABIJKN-_]
        /// Float format for a Kotlin literal floating-point number.
        const KOTLIN_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // KOTLIN STRING [0134568MN]
        /// Float format to parse a Kotlin float from string.
        const KOTLIN_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // JULIA LITERAL [01345689AMN-_]
        /// Float format for a Julia literal floating-point number.
        const JULIA_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTEGER_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_INTERNAL_DIGIT_SEPARATOR.bits
        );

        // JULIA STRING [01345678MN]
        /// Float format to parse a Julia float from string.
        const JULIA_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        /// Float format for a C# literal floating-point number.
        const CSHARP_LITERAL = Self::CSHARP7_LITERAL.bits;

        /// Float format to parse a C# float from string.
        const CSHARP_STRING = Self::CSHARP7_STRING.bits;

        // CSHARP7 LITERAL [034569ABIJKMN-_]
        /// Float format for a C#7 literal floating-point number.
        const CSHARP7_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // CSHARP7 STRING [0134568MN]
        /// Float format to parse a C#7 float from string.
        const CSHARP7_STRING = (
            flags::radix_to_flags(10)
            | flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP6 LITERAL [03456MN]
        /// Float format for a C#6 literal floating-point number.
        const CSHARP6_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP6 STRING [0134568MN]
        /// Float format to parse a C#6 float from string.
        const CSHARP6_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP5 LITERAL [03456MN]
        /// Float format for a C#5 literal floating-point number.
        const CSHARP5_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP5 STRING [0134568MN]
        /// Float format to parse a C#5 float from string.
        const CSHARP5_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP4 LITERAL [03456MN]
        /// Float format for a C#4 literal floating-point number.
        const CSHARP4_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP4 STRING [0134568MN]
        /// Float format to parse a C#4 float from string.
        const CSHARP4_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP3 LITERAL [03456MN]
        /// Float format for a C#3 literal floating-point number.
        const CSHARP3_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP3 STRING [0134568MN]
        /// Float format to parse a C#3 float from string.
        const CSHARP3_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP2 LITERAL [03456MN]
        /// Float format for a C#2 literal floating-point number.
        const CSHARP2_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP2 STRING [0134568MN]
        /// Float format to parse a C#2 float from string.
        const CSHARP2_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP1 LITERAL [03456MN]
        /// Float format for a C#1 literal floating-point number.
        const CSHARP1_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP1 STRING [0134568MN]
        /// Float format to parse a C#1 float from string.
        const CSHARP1_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // KAWA LITERAL [013456MN]
        /// Float format for a Kawa literal floating-point number.
        const KAWA_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // KAWA STRING [013456MN]
        /// Float format to parse a Kawa float from string.
        const KAWA_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GAMBITC LITERAL [013456MN]
        /// Float format for a Gambit-C literal floating-point number.
        const GAMBITC_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GAMBITC STRING [013456MN]
        /// Float format to parse a Gambit-C float from string.
        const GAMBITC_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GUILE LITERAL [013456MN]
        /// Float format for a Guile literal floating-point number.
        const GUILE_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GUILE STRING [013456MN]
        /// Float format to parse a Guile float from string.
        const GUILE_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CLOJURE LITERAL [13456MN]
        /// Float format for a Clojure literal floating-point number.
        const CLOJURE_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CLOJURE STRING [01345678MN]
        /// Float format to parse a Clojure float from string.
        const CLOJURE_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // ERLANG LITERAL [34578MN]
        /// Float format for an Erlang literal floating-point number.
        const ERLANG_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // ERLANG STRING [345MN]
        /// Float format to parse an Erlang float from string.
        const ERLANG_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_SPECIAL.bits
        );

        // ELM LITERAL [456]
        /// Float format for an Elm literal floating-point number.
        const ELM_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // ELM STRING [01345678MN]
        /// Float format to parse an Elm float from string.
        // Note: There is no valid representation of NaN, just Infinity.
        const ELM_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // SCALA LITERAL [3456]
        /// Float format for a Scala literal floating-point number.
        const SCALA_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // SCALA STRING [01345678MN]
        /// Float format to parse a Scala float from string.
        const SCALA_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // ELIXIR LITERAL [3459ABMN-_]
        /// Float format for an Elixir literal floating-point number.
        const ELIXIR_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // ELIXIR STRING [345MN]
        /// Float format to parse an Elixir float from string.
        const ELIXIR_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_SPECIAL.bits
        );

        // FORTRAN LITERAL [013456MN]
        /// Float format for a FORTRAN literal floating-point number.
        const FORTRAN_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // FORTRAN STRING [0134567MN]
        /// Float format to parse a FORTRAN float from string.
        const FORTRAN_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // D LITERAL [0134569ABFGHIJKN-_]
        /// Float format for a D literal floating-point number.
        const D_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // D STRING [01345679AFGMN-_]
        /// Float format to parse a D float from string.
        const D_STRING = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTEGER_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::INTEGER_TRAILING_DIGIT_SEPARATOR.bits
            | Self::FRACTION_TRAILING_DIGIT_SEPARATOR.bits
        );

        // COFFEESCRIPT LITERAL [01345678]
        /// Float format for a Coffeescript literal floating-point number.
        const COFFEESCRIPT_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // COFFEESCRIPT STRING [012345678MN]
        /// Float format to parse a Coffeescript float from string.
        const COFFEESCRIPT_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // COBOL LITERAL [0345MN]
        /// Float format for a Cobol literal floating-point number.
        const COBOL_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_SPECIAL.bits
        );

        // COBOL STRING [012356MN]
        /// Float format to parse a Cobol float from string.
        const COBOL_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_SIGN.bits
            | Self::NO_SPECIAL.bits
        );

        // FSHARP LITERAL [13456789ABIJKMN-_]
        /// Float format for a F# literal floating-point number.
        const FSHARP_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // FSHARP STRING [013456789ABCDEFGHIJKLMN-_]
        /// Float format to parse a F# float from string.
        const FSHARP_STRING = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
            | Self::SPECIAL_DIGIT_SEPARATOR.bits
        );

        // VB LITERAL [03456MN]
        /// Float format for a Visual Basic literal floating-point number.
        const VB_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // VB STRING [01345678MN]
        /// Float format to parse a Visual Basic float from string.
        // Note: To my knowledge, Visual Basic cannot parse infinity.
        const VB_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // OCAML LITERAL [1456789ABDFGHIJKMN-_]
        /// Float format for an OCaml literal floating-point number.
        const OCAML_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // OCAML STRING [01345679ABCDEFGHIJKLMN-_]
        /// Float format to parse an OCaml float from string.
        const OCAML_STRING = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
            | Self::SPECIAL_DIGIT_SEPARATOR.bits
        );

        // OBJECTIVEC LITERAL [013456MN]
        /// Float format for an Objective-C literal floating-point number.
        const OBJECTIVEC_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // OBJECTIVEC STRING [013456MN]
        /// Float format to parse an Objective-C float from string.
        const OBJECTIVEC_STRING = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // REASONML LITERAL [13456789ABDFGHIJKMN-_]
        /// Float format for a ReasonML literal floating-point number.
        const REASONML_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // REASONML STRING [01345679ABCDEFGHIJKLMN-_]
        /// Float format to parse a ReasonML float from string.
        const REASONML_STRING = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
            | Self::SPECIAL_DIGIT_SEPARATOR.bits
        );

        // OCTAVE LITERAL [013456789ABDFGHIJKMN-_]
        /// Float format for an Octave literal floating-point number.
        // Note: Octave accepts both NaN and nan, Inf and inf.
        const OCTAVE_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // OCTAVE STRING [01345679ABCDEFGHIJKMN-,]
        /// Float format to parse an Octave float from string.
        const OCTAVE_STRING = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b',')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // MATLAB LITERAL [013456789ABDFGHIJKMN-_]
        /// Float format for an Matlab literal floating-point number.
        // Note: Matlab accepts both NaN and nan, Inf and inf.
        const MATLAB_LITERAL = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // MATLAB STRING [01345679ABCDEFGHIJKMN-,]
        /// Float format to parse an Matlab float from string.
        const MATLAB_STRING = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b',')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // ZIG LITERAL [1456MN]
        /// Float format for a Zig literal floating-point number.
        const ZIG_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::NO_SPECIAL.bits
        );

        // ZIG STRING [01234567MN]
        /// Float format to parse a Zig float from string.
        const ZIG_STRING = Self::PERMISSIVE.bits;

        // SAGE LITERAL [012345678MN]
        /// Float format for a Sage literal floating-point number.
        // Note: Both Infinity and infinity are accepted.
        const SAGE_LITERAL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // SAGE STRING [01345679ABMN-_]
        /// Float format to parse a Sage float from string.
        const SAGE_STRING = (
            flags::radix_to_flags(10)
            | flags::digit_separator_to_flags(b'_')
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // JSON [456]
        /// Float format for a JSON literal floating-point number.
        const JSON = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // TOML [34569AB]
        /// Float format for a TOML literal floating-point number.
        const TOML = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // YAML (defined in-terms of JSON schema).
        /// Float format for a YAML literal floating-point number.
        const YAML = Self::JSON.bits;

        // XML [01234578MN]
        /// Float format for a XML literal floating-point number.
        const XML = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // SQLITE [013456MN]
        /// Float format for a SQLite literal floating-point number.
        const SQLITE = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // POSTGRESQL [013456MN]
        /// Float format for a PostgreSQL literal floating-point number.
        const POSTGRESQL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // MYSQL [013456MN]
        /// Float format for a MySQL literal floating-point number.
        const MYSQL = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // MONGODB [01345678M]
        /// Float format for a MongoDB literal floating-point number.
        const MONGODB = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // HIDDEN DEFAULTS

        /// Float format when no flags are set.
        #[doc(hidden)]
        const PERMISSIVE = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
        );

        /// Permissive interface float format flags.
        #[doc(hidden)]
        const PERMISSIVE_INTERFACE = Self::PERMISSIVE.bits & Self::INTERFACE_FLAG_MASK.bits;

        /// Standard float format.
        #[doc(hidden)]
        const STANDARD = Self::RUST_STRING.bits;

        /// Standard interface float format flags.
        #[doc(hidden)]
        const STANDARD_INTERFACE = Self::STANDARD.bits & Self::INTERFACE_FLAG_MASK.bits;

        /// Float format when all digit separator flags are set.
        #[doc(hidden)]
        const IGNORE = (
            flags::radix_to_flags(10)
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::DIGIT_SEPARATOR_FLAG_MASK.bits
        );

        /// Ignore interface float format flags.
        #[doc(hidden)]
        const IGNORE_INTERFACE = Self::IGNORE.bits & Self::INTERFACE_FLAG_MASK.bits;
    }
}

impl Format for NumberFormat {
    #[inline]
    fn flags(self) -> Self {
        return self & Self::FLAG_MASK
    }

    #[inline]
    fn interface_flags(self) -> Self {
        return self & Self::INTERFACE_FLAG_MASK
    }

    #[inline]
    #[cfg(feature = "radix")]
    fn radix(self) -> u8 {
        flags::radix_from_flags(self.bits)
    }

    #[inline]
    fn digit_separator(self) -> u8 {
        flags::digit_separator_from_flags(self.bits)
    }

    #[inline]
    fn decimal_point(self) -> u8 {
        flags::decimal_point_from_flags(self.bits)
    }

    #[inline]
    fn exponent(self) -> u8 {
        flags::exponent_from_flags(self.bits)
    }

    #[cfg(feature = "radix")]
    #[inline]
    fn exponent_backup(self) -> u8 {
        flags::exponent_backup_from_flags(self.bits)
    }

    #[inline]
    fn required_integer_digits(self) -> bool {
        self.intersects(Self::REQUIRED_INTEGER_DIGITS)
    }

    #[inline]
    fn required_fraction_digits(self) -> bool {
        self.intersects(Self::REQUIRED_FRACTION_DIGITS)
    }

    #[inline]
    fn required_exponent_digits(self) -> bool {
        self.intersects(Self::REQUIRED_EXPONENT_DIGITS)
    }

    #[inline]
    fn required_digits(self) -> bool {
        self.intersects(Self::REQUIRED_DIGITS)
    }

    #[inline]
    fn no_positive_mantissa_sign(self) -> bool {
        self.intersects(Self::NO_POSITIVE_MANTISSA_SIGN)
    }

    #[inline]
    fn required_mantissa_sign(self) -> bool {
        self.intersects(Self::REQUIRED_MANTISSA_SIGN)
    }

    #[inline]
    fn no_exponent_notation(self) -> bool {
        self.intersects(Self::NO_EXPONENT_NOTATION)
    }

    #[inline]
    fn no_positive_exponent_sign(self) -> bool {
        self.intersects(Self::NO_POSITIVE_EXPONENT_SIGN)
    }

    #[inline]
    fn required_exponent_sign(self) -> bool {
        self.intersects(Self::REQUIRED_EXPONENT_SIGN)
    }

    #[inline]
    fn no_exponent_without_fraction(self) -> bool {
        self.intersects(Self::NO_EXPONENT_WITHOUT_FRACTION)
    }

    #[inline]
    fn no_special(self) -> bool {
        self.intersects(Self::NO_SPECIAL)
    }

    #[inline]
    fn case_sensitive_special(self) -> bool {
        self.intersects(Self::CASE_SENSITIVE_SPECIAL)
    }

    #[inline]
    fn no_integer_leading_zeros(self) -> bool {
        self.intersects(Self::NO_INTEGER_LEADING_ZEROS)
    }

    #[inline]
    fn no_float_leading_zeros(self) -> bool {
        self.intersects(Self::NO_FLOAT_LEADING_ZEROS)
    }

    #[inline]
    fn integer_internal_digit_separator(self) -> bool {
        self.intersects(Self::INTEGER_INTERNAL_DIGIT_SEPARATOR)
    }

    #[inline]
    fn fraction_internal_digit_separator(self) -> bool {
        self.intersects(Self::FRACTION_INTERNAL_DIGIT_SEPARATOR)
    }

    #[inline]
    fn exponent_internal_digit_separator(self) -> bool {
        self.intersects(Self::EXPONENT_INTERNAL_DIGIT_SEPARATOR)
    }

    #[inline]
    fn internal_digit_separator(self) -> bool {
        self.intersects(Self::INTERNAL_DIGIT_SEPARATOR)
    }

    #[inline]
    fn integer_leading_digit_separator(self) -> bool {
        self.intersects(Self::INTEGER_LEADING_DIGIT_SEPARATOR)
    }

    #[inline]
    fn fraction_leading_digit_separator(self) -> bool {
        self.intersects(Self::FRACTION_LEADING_DIGIT_SEPARATOR)
    }

    #[inline]
    fn exponent_leading_digit_separator(self) -> bool {
        self.intersects(Self::EXPONENT_LEADING_DIGIT_SEPARATOR)
    }

    #[inline]
    fn leading_digit_separator(self) -> bool {
        self.intersects(Self::LEADING_DIGIT_SEPARATOR)
    }

    #[inline]
    fn integer_trailing_digit_separator(self) -> bool {
        self.intersects(Self::INTEGER_TRAILING_DIGIT_SEPARATOR)
    }

    #[inline]
    fn fraction_trailing_digit_separator(self) -> bool {
        self.intersects(Self::FRACTION_TRAILING_DIGIT_SEPARATOR)
    }

    #[inline]
    fn exponent_trailing_digit_separator(self) -> bool {
        self.intersects(Self::EXPONENT_TRAILING_DIGIT_SEPARATOR)
    }

    #[inline]
    fn trailing_digit_separator(self) -> bool {
        self.intersects(Self::TRAILING_DIGIT_SEPARATOR)
    }

    #[inline]
    fn integer_consecutive_digit_separator(self) -> bool {
        self.intersects(Self::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR)
    }

    #[inline]
    fn fraction_consecutive_digit_separator(self) -> bool {
        self.intersects(Self::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR)
    }

    #[inline]
    fn exponent_consecutive_digit_separator(self) -> bool {
        self.intersects(Self::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR)
    }

    #[inline]
    fn consecutive_digit_separator(self) -> bool {
        self.intersects(Self::CONSECUTIVE_DIGIT_SEPARATOR)
    }

    #[inline]
    fn special_digit_separator(self) -> bool {
        self.intersects(Self::SPECIAL_DIGIT_SEPARATOR)
    }

    #[inline]
    fn incorrect(self) -> bool {
        self.intersects(Self::INCORRECT)
    }

    #[inline]
    fn lossy(self) -> bool {
        self.intersects(Self::LOSSY)
    }

    #[inline]
    fn compile(
        radix: u8,
        digit_separator: u8,
        decimal_point: u8,
        exponent: u8,
        exponent_backup: u8,
        required_integer_digits: bool,
        required_fraction_digits: bool,
        required_exponent_digits: bool,
        no_positive_mantissa_sign: bool,
        required_mantissa_sign: bool,
        no_exponent_notation: bool,
        no_positive_exponent_sign: bool,
        required_exponent_sign: bool,
        no_exponent_without_fraction: bool,
        no_special: bool,
        case_sensitive_special: bool,
        no_integer_leading_zeros: bool,
        no_float_leading_zeros: bool,
        integer_internal_digit_separator: bool,
        fraction_internal_digit_separator: bool,
        exponent_internal_digit_separator: bool,
        integer_leading_digit_separator: bool,
        fraction_leading_digit_separator: bool,
        exponent_leading_digit_separator: bool,
        integer_trailing_digit_separator: bool,
        fraction_trailing_digit_separator: bool,
        exponent_trailing_digit_separator: bool,
        integer_consecutive_digit_separator: bool,
        fraction_consecutive_digit_separator: bool,
        exponent_consecutive_digit_separator: bool,
        special_digit_separator: bool,
        incorrect: bool,
        lossy: bool
    ) -> Option<Self> {
        let builder = NumberFormatBuilder {
            radix,
            digit_separator,
            decimal_point,
            exponent,
            exponent_backup,
            required_integer_digits,
            required_fraction_digits,
            required_exponent_digits,
            no_positive_mantissa_sign,
            required_mantissa_sign,
            no_exponent_notation,
            no_positive_exponent_sign,
            required_exponent_sign,
            no_exponent_without_fraction,
            no_special,
            case_sensitive_special,
            no_integer_leading_zeros,
            no_float_leading_zeros,
            integer_internal_digit_separator,
            fraction_internal_digit_separator,
            exponent_internal_digit_separator,
            integer_leading_digit_separator,
            fraction_leading_digit_separator,
            exponent_leading_digit_separator,
            integer_trailing_digit_separator,
            fraction_trailing_digit_separator,
            exponent_trailing_digit_separator,
            integer_consecutive_digit_separator,
            fraction_consecutive_digit_separator,
            exponent_consecutive_digit_separator,
            special_digit_separator,
            incorrect,
            lossy
        };
        builder.build()
    }

    #[inline]
    fn permissive() -> Option<Self> {
        Some(Self::PERMISSIVE)
    }

    #[inline]
    fn standard() -> Option<Self> {
        Some(Self::STANDARD)
    }

    #[inline]
    fn ignore(digit_separator: u8) -> Option<Self> {
        let decimal_point = b'.';
        let exponent = b'e';
        let exponent_backup = b'^';
        if !flags::is_valid_digit_separator(digit_separator) {
            None
        } else if !flags::is_valid_punctuation(digit_separator, decimal_point, exponent, exponent_backup)  {
            None
        } else {
            let mut format = Self::IGNORE;
            format.bits |= flags::digit_separator_to_flags(digit_separator);
            Some(format)
        }
    }

    #[cfg(test)]
    #[inline]
    fn from_separator(digit_separator: u8) -> Self {
        let mut format = Self::PERMISSIVE;
        format.bits |= flags::digit_separator_to_flags(digit_separator);
        format
    }
}

// NUMBER FORMAT BUILDER

/// Build float format value from specifications.
///
/// * `radix`                                   - Radix for number encoding or decoding.
/// * `digit_separator`                         - Character to separate digits.
/// * `decimal_point`                           - Character to designate the decimal point.
/// * `exponent`                                - Character to designate the exponent.
/// * `exponent_backup`                         - Backup character to designate the exponent for radix >= 0xE.
/// * `required_integer_digits`                 - If digits are required before the decimal point.
/// * `required_fraction_digits`                - If digits are required after the decimal point.
/// * `required_exponent_digits`                - If digits are required after the exponent character.
/// * `no_positive_mantissa_sign`               - If positive sign before the mantissa is not allowed.
/// * `required_mantissa_sign`                  - If positive sign before the mantissa is required.
/// * `no_exponent_notation`                    - If exponent notation is not allowed.
/// * `no_positive_exponent_sign`               - If positive sign before the exponent is not allowed.
/// * `required_exponent_sign`                  - If sign before the exponent is required.
/// * `no_exponent_without_fraction`            - If exponent without fraction is not allowed.
/// * `no_special`                              - If special (non-finite) values are not allowed.
/// * `case_sensitive_special`                  - If special (non-finite) values are case-sensitive.
/// * `no_integer_leading_zeros`                - If leading zeros before an integer are not allowed.
/// * `no_float_leading_zeros`                  - If leading zeros before a float are not allowed.
/// * `integer_internal_digit_separator`        - If digit separators are allowed between integer digits.
/// * `fraction_internal_digit_separator`       - If digit separators are allowed between fraction digits.
/// * `exponent_internal_digit_separator`       - If digit separators are allowed between exponent digits.
/// * `integer_leading_digit_separator`         - If a digit separator is allowed before any integer digits.
/// * `fraction_leading_digit_separator`        - If a digit separator is allowed before any fraction digits.
/// * `exponent_leading_digit_separator`        - If a digit separator is allowed before any exponent digits.
/// * `integer_trailing_digit_separator`        - If a digit separator is allowed after any integer digits.
/// * `fraction_trailing_digit_separator`       - If a digit separator is allowed after any fraction digits.
/// * `exponent_trailing_digit_separator`       - If a digit separator is allowed after any exponent digits.
/// * `integer_consecutive_digit_separator`     - If multiple consecutive integer digit separators are allowed.
/// * `fraction_consecutive_digit_separator`    - If multiple consecutive fraction digit separators are allowed.
/// * `special_digit_separator`                 - If any digit separators are allowed in special (non-finite) values.
/// * `incorrect`                               - Use incorrect, but fast conversion routines.
/// * `lossy`                                   - Use lossy, but moderately fast, conversion routines.
///
/// Returns the format on calling build if it was able to compile the format,
/// otherwise, returns None.
#[allow(dead_code)]     // radix & exponent_backup are never used without radix
#[derive(Debug, Clone)]
pub struct NumberFormatBuilder {
    radix: u8,
    digit_separator: u8,
    decimal_point: u8,
    exponent: u8,
    exponent_backup: u8,
    required_integer_digits: bool,
    required_fraction_digits: bool,
    required_exponent_digits: bool,
    no_positive_mantissa_sign: bool,
    required_mantissa_sign: bool,
    no_exponent_notation: bool,
    no_positive_exponent_sign: bool,
    required_exponent_sign: bool,
    no_exponent_without_fraction: bool,
    no_special: bool,
    case_sensitive_special: bool,
    no_integer_leading_zeros: bool,
    no_float_leading_zeros: bool,
    integer_internal_digit_separator: bool,
    fraction_internal_digit_separator: bool,
    exponent_internal_digit_separator: bool,
    integer_leading_digit_separator: bool,
    fraction_leading_digit_separator: bool,
    exponent_leading_digit_separator: bool,
    integer_trailing_digit_separator: bool,
    fraction_trailing_digit_separator: bool,
    exponent_trailing_digit_separator: bool,
    integer_consecutive_digit_separator: bool,
    fraction_consecutive_digit_separator: bool,
    exponent_consecutive_digit_separator: bool,
    special_digit_separator: bool,
    incorrect: bool,
    lossy: bool
}

impl NumberFormatBuilder {
    /// Create new NumberFormatBuilder with default arguments.
    #[inline(always)]
    #[allow(deprecated)]    // Remove when we deprecate these methods.
    fn new() -> Self {
        #[cfg(feature = "radix")]
        let exponent_backup = config::get_exponent_backup_char();
        #[cfg(not(feature = "radix"))]
        let exponent_backup = b'^';

        Self {
            radix: 10,
            digit_separator: b'\x00',
            decimal_point: b'.',
            exponent: config::get_exponent_default_char(),
            exponent_backup,
            required_integer_digits: false,
            required_fraction_digits: false,
            required_exponent_digits: false,
            no_positive_mantissa_sign: false,
            required_mantissa_sign: false,
            no_exponent_notation: false,
            no_positive_exponent_sign: false,
            required_exponent_sign: false,
            no_exponent_without_fraction: false,
            no_special: false,
            case_sensitive_special: false,
            no_integer_leading_zeros: false,
            no_float_leading_zeros: false,
            integer_internal_digit_separator: false,
            fraction_internal_digit_separator: false,
            exponent_internal_digit_separator: false,
            integer_leading_digit_separator: false,
            fraction_leading_digit_separator: false,
            exponent_leading_digit_separator: false,
            integer_trailing_digit_separator: false,
            fraction_trailing_digit_separator: false,
            exponent_trailing_digit_separator: false,
            integer_consecutive_digit_separator: false,
            fraction_consecutive_digit_separator: false,
            exponent_consecutive_digit_separator: false,
            special_digit_separator: false,
            incorrect: false,
            lossy: false
        }
    }

    #[cfg(feature = "radix")]
    #[inline(always)]
    pub fn radix(&mut self, radix: u8) -> &mut Self {
        self.radix = radix;
        self
    }

    #[inline(always)]
    pub fn digit_separator(&mut self, digit_separator: u8) -> &mut Self {
        self.digit_separator = digit_separator;
        self
    }

    #[inline(always)]
    pub fn decimal_point(&mut self, decimal_point: u8) -> &mut Self {
        self.decimal_point = decimal_point;
        self
    }

    #[inline(always)]
    pub fn exponent(&mut self, exponent: u8) -> &mut Self {
        self.exponent = exponent;
        self
    }

    #[cfg(feature = "radix")]
    #[inline(always)]
    pub fn exponent_backup(&mut self, exponent_backup: u8) -> &mut Self {
        self.exponent_backup = exponent_backup;
        self
    }

    #[inline(always)]
    pub fn required_integer_digits(&mut self, required_integer_digits: bool) -> &mut Self {
        self.required_integer_digits = required_integer_digits;
        self
    }

    #[inline(always)]
    pub fn required_fraction_digits(&mut self, required_fraction_digits: bool) -> &mut Self {
        self.required_fraction_digits = required_fraction_digits;
        self
    }

    #[inline(always)]
    pub fn required_exponent_digits(&mut self, required_exponent_digits: bool) -> &mut Self {
        self.required_exponent_digits = required_exponent_digits;
        self
    }

    #[inline(always)]
    pub fn no_positive_mantissa_sign(&mut self, no_positive_mantissa_sign: bool) -> &mut Self {
        self.no_positive_mantissa_sign = no_positive_mantissa_sign;
        self
    }

    #[inline(always)]
    pub fn required_mantissa_sign(&mut self, required_mantissa_sign: bool) -> &mut Self {
        self.required_mantissa_sign = required_mantissa_sign;
        self
    }

    #[inline(always)]
    pub fn no_exponent_notation(&mut self, no_exponent_notation: bool) -> &mut Self {
        self.no_exponent_notation = no_exponent_notation;
        self
    }

    #[inline(always)]
    pub fn no_positive_exponent_sign(&mut self, no_positive_exponent_sign: bool) -> &mut Self {
        self.no_positive_exponent_sign = no_positive_exponent_sign;
        self
    }

    #[inline(always)]
    pub fn required_exponent_sign(&mut self, required_exponent_sign: bool) -> &mut Self {
        self.required_exponent_sign = required_exponent_sign;
        self
    }

    #[inline(always)]
    pub fn no_exponent_without_fraction(&mut self, no_exponent_without_fraction: bool) -> &mut Self {
        self.no_exponent_without_fraction = no_exponent_without_fraction;
        self
    }

    #[inline(always)]
    pub fn no_special(&mut self, no_special: bool) -> &mut Self {
        self.no_special = no_special;
        self
    }

    #[inline(always)]
    pub fn case_sensitive_special(&mut self, case_sensitive_special: bool) -> &mut Self {
        self.case_sensitive_special = case_sensitive_special;
        self
    }

    #[inline(always)]
    pub fn no_integer_leading_zeros(&mut self, no_integer_leading_zeros: bool) -> &mut Self {
        self.no_integer_leading_zeros = no_integer_leading_zeros;
        self
    }

    #[inline(always)]
    pub fn no_float_leading_zeros(&mut self, no_float_leading_zeros: bool) -> &mut Self {
        self.no_float_leading_zeros = no_float_leading_zeros;
        self
    }

    #[inline(always)]
    pub fn integer_internal_digit_separator(&mut self, integer_internal_digit_separator: bool) -> &mut Self {
        self.integer_internal_digit_separator = integer_internal_digit_separator;
        self
    }

    #[inline(always)]
    pub fn fraction_internal_digit_separator(&mut self, fraction_internal_digit_separator: bool) -> &mut Self {
        self.fraction_internal_digit_separator = fraction_internal_digit_separator;
        self
    }

    #[inline(always)]
    pub fn exponent_internal_digit_separator(&mut self, exponent_internal_digit_separator: bool) -> &mut Self {
        self.exponent_internal_digit_separator = exponent_internal_digit_separator;
        self
    }

    #[inline(always)]
    pub fn integer_leading_digit_separator(&mut self, integer_leading_digit_separator: bool) -> &mut Self {
        self.integer_leading_digit_separator = integer_leading_digit_separator;
        self
    }

    #[inline(always)]
    pub fn fraction_leading_digit_separator(&mut self, fraction_leading_digit_separator: bool) -> &mut Self {
        self.fraction_leading_digit_separator = fraction_leading_digit_separator;
        self
    }

    #[inline(always)]
    pub fn exponent_leading_digit_separator(&mut self, exponent_leading_digit_separator: bool) -> &mut Self {
        self.exponent_leading_digit_separator = exponent_leading_digit_separator;
        self
    }

    #[inline(always)]
    pub fn integer_trailing_digit_separator(&mut self, integer_trailing_digit_separator: bool) -> &mut Self {
        self.integer_trailing_digit_separator = integer_trailing_digit_separator;
        self
    }

    #[inline(always)]
    pub fn fraction_trailing_digit_separator(&mut self, fraction_trailing_digit_separator: bool) -> &mut Self {
        self.fraction_trailing_digit_separator = fraction_trailing_digit_separator;
        self
    }

    #[inline(always)]
    pub fn exponent_trailing_digit_separator(&mut self, exponent_trailing_digit_separator: bool) -> &mut Self {
        self.exponent_trailing_digit_separator = exponent_trailing_digit_separator;
        self
    }

    #[inline(always)]
    pub fn integer_consecutive_digit_separator(&mut self, integer_consecutive_digit_separator: bool) -> &mut Self {
        self.integer_consecutive_digit_separator = integer_consecutive_digit_separator;
        self
    }

    #[inline(always)]
    pub fn fraction_consecutive_digit_separator(&mut self, fraction_consecutive_digit_separator: bool) -> &mut Self {
        self.fraction_consecutive_digit_separator = fraction_consecutive_digit_separator;
        self
    }

    #[inline(always)]
    pub fn exponent_consecutive_digit_separator(&mut self, exponent_consecutive_digit_separator: bool) -> &mut Self {
        self.exponent_consecutive_digit_separator = exponent_consecutive_digit_separator;
        self
    }

    #[inline(always)]
    pub fn special_digit_separator(&mut self, special_digit_separator: bool) -> &mut Self {
        self.special_digit_separator = special_digit_separator;
        self
    }

    #[inline(always)]
    pub fn incorrect(&mut self, incorrect: bool) -> &mut Self {
        self.incorrect = incorrect;
        self
    }

    #[inline(always)]
    pub fn lossy(&mut self, lossy: bool) -> &mut Self {
        self.lossy = lossy;
        self
    }
}

impl Default for NumberFormatBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for NumberFormatBuilder {
    type Buildable = NumberFormat;

    #[inline]
    fn build(self) -> Option<Self::Buildable> {
        let mut format = Self::Buildable::default();
        // Generic flags.
        add_flag!(format, self.required_integer_digits, REQUIRED_INTEGER_DIGITS);
        add_flag!(format, self.required_fraction_digits, REQUIRED_FRACTION_DIGITS);
        add_flag!(format, self.required_exponent_digits, REQUIRED_EXPONENT_DIGITS);
        add_flag!(format, self.no_positive_mantissa_sign, NO_POSITIVE_MANTISSA_SIGN);
        add_flag!(format, self.required_mantissa_sign, REQUIRED_MANTISSA_SIGN);
        add_flag!(format, self.no_exponent_notation, NO_EXPONENT_NOTATION);
        add_flag!(format, self.no_positive_exponent_sign, NO_POSITIVE_EXPONENT_SIGN);
        add_flag!(format, self.required_exponent_sign, REQUIRED_EXPONENT_SIGN);
        add_flag!(format, self.no_exponent_without_fraction, NO_EXPONENT_WITHOUT_FRACTION);
        add_flag!(format, self.no_special, NO_SPECIAL);
        add_flag!(format, self.case_sensitive_special, CASE_SENSITIVE_SPECIAL);
        add_flag!(format, self.no_integer_leading_zeros, NO_INTEGER_LEADING_ZEROS);
        add_flag!(format, self.no_float_leading_zeros, NO_FLOAT_LEADING_ZEROS);

        // Digit separator flags.
        add_flag!(format, self.integer_internal_digit_separator, INTEGER_INTERNAL_DIGIT_SEPARATOR);
        add_flag!(format, self.fraction_internal_digit_separator, FRACTION_INTERNAL_DIGIT_SEPARATOR);
        add_flag!(format, self.exponent_internal_digit_separator, EXPONENT_INTERNAL_DIGIT_SEPARATOR);
        add_flag!(format, self.integer_leading_digit_separator, INTEGER_LEADING_DIGIT_SEPARATOR);
        add_flag!(format, self.fraction_leading_digit_separator, FRACTION_LEADING_DIGIT_SEPARATOR);
        add_flag!(format, self.exponent_leading_digit_separator, EXPONENT_LEADING_DIGIT_SEPARATOR);
        add_flag!(format, self.integer_trailing_digit_separator, INTEGER_TRAILING_DIGIT_SEPARATOR);
        add_flag!(format, self.fraction_trailing_digit_separator, FRACTION_TRAILING_DIGIT_SEPARATOR);
        add_flag!(format, self.exponent_trailing_digit_separator, EXPONENT_TRAILING_DIGIT_SEPARATOR);
        add_flag!(format, self.integer_consecutive_digit_separator, INTEGER_CONSECUTIVE_DIGIT_SEPARATOR);
        add_flag!(format, self.fraction_consecutive_digit_separator, FRACTION_CONSECUTIVE_DIGIT_SEPARATOR);
        add_flag!(format, self.exponent_consecutive_digit_separator, EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR);
        add_flag!(format, self.special_digit_separator, SPECIAL_DIGIT_SEPARATOR);

        // Add conversion precision flags.
        add_flag!(format, self.incorrect, INCORRECT);
        add_flag!(format, self.lossy, LOSSY);

        // Add punctuation characters.
        if format.intersects(NumberFormat::DIGIT_SEPARATOR_FLAG_MASK) {
            format.bits |= flags::digit_separator_to_flags(self.digit_separator);
        }
        format.bits |= flags::decimal_point_to_flags(self.decimal_point);
        format.bits |= flags::exponent_to_flags(self.exponent);
        format.bits |= flags::exponent_backup_to_flags(self.exponent_backup);

        // Add radix
        format.bits |= flags::radix_to_flags(self.radix);

        // Validation.
        let is_invalid =
            !flags::is_valid_digit_separator(self.digit_separator)
            || !flags::is_valid_decimal_point(self.decimal_point)
            || !flags::is_valid_exponent(self.exponent)
            || !flags::is_valid_exponent_backup(self.exponent_backup)
            || !flags::is_valid_punctuation(self.digit_separator, self.decimal_point, self.exponent, self.exponent_backup)
            || !flags::is_valid_radix(self.radix)
            || format.intersects(NumberFormat::NO_EXPONENT_NOTATION) && format.intersects(NumberFormat::EXPONENT_FLAG_MASK)
            || self.no_positive_mantissa_sign && self.required_mantissa_sign
            || self.no_positive_exponent_sign && self.required_exponent_sign
            || self.no_special && (self.case_sensitive_special || self.special_digit_separator)
            || format & NumberFormat::INTEGER_DIGIT_SEPARATOR_FLAG_MASK == NumberFormat::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR
            || format & NumberFormat::FRACTION_DIGIT_SEPARATOR_FLAG_MASK == NumberFormat::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR
            || format & NumberFormat::EXPONENT_DIGIT_SEPARATOR_FLAG_MASK == NumberFormat::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR
            || self.incorrect && self.lossy;

        match is_invalid {
            true  => None,
            false => Some(format)
        }
    }
}

impl Buildable for NumberFormat {
    type Builder = NumberFormatBuilder;

    #[inline(always)]
    fn builder() -> Self::Builder {
        Self::Builder::new()
    }

    #[inline]
    fn rebuild(&self) -> Self::Builder {
        Self::Builder {
            radix: flags::radix_from_flags(self.bits),
            digit_separator: self.digit_separator(),
            decimal_point: self.decimal_point(),
            exponent: self.exponent(),
            exponent_backup: flags::exponent_backup_from_flags(self.bits),
            required_integer_digits: self.required_integer_digits(),
            required_fraction_digits: self.required_fraction_digits(),
            required_exponent_digits: self.required_exponent_digits(),
            no_positive_mantissa_sign: self.no_positive_mantissa_sign(),
            required_mantissa_sign: self.required_mantissa_sign(),
            no_exponent_notation: self.no_exponent_notation(),
            no_positive_exponent_sign: self.no_positive_exponent_sign(),
            required_exponent_sign: self.required_exponent_sign(),
            no_exponent_without_fraction: self.no_exponent_without_fraction(),
            no_special: self.no_special(),
            case_sensitive_special: self.case_sensitive_special(),
            no_integer_leading_zeros: self.no_integer_leading_zeros(),
            no_float_leading_zeros: self.no_float_leading_zeros(),
            integer_internal_digit_separator: self.integer_internal_digit_separator(),
            fraction_internal_digit_separator: self.fraction_internal_digit_separator(),
            exponent_internal_digit_separator: self.exponent_internal_digit_separator(),
            integer_leading_digit_separator: self.integer_leading_digit_separator(),
            fraction_leading_digit_separator: self.fraction_leading_digit_separator(),
            exponent_leading_digit_separator: self.exponent_leading_digit_separator(),
            integer_trailing_digit_separator: self.integer_trailing_digit_separator(),
            fraction_trailing_digit_separator: self.fraction_trailing_digit_separator(),
            exponent_trailing_digit_separator: self.exponent_trailing_digit_separator(),
            integer_consecutive_digit_separator: self.integer_consecutive_digit_separator(),
            fraction_consecutive_digit_separator: self.fraction_consecutive_digit_separator(),
            exponent_consecutive_digit_separator: self.exponent_consecutive_digit_separator(),
            special_digit_separator: self.special_digit_separator(),
            incorrect: self.incorrect(),
            lossy: self.lossy()
        }
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_compile() {
        // TODO(ahuszagh) Use the builder interface
        // Test all false
        let flag = NumberFormat::compile(10, b'_', b'.', b'e', b'^', false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false).unwrap();
        assert_eq!(flag.flags(), NumberFormat::default());
        assert_eq!(flag.digit_separator(), 0);
    }

    #[test]
    fn test_permissive() {
        let flag = NumberFormat::ignore(b'_').unwrap();
        assert_eq!(flag.flags(), NumberFormat::DIGIT_SEPARATOR_FLAG_MASK);
    }

    #[test]
    fn test_ignore() {
        let flag = NumberFormat::ignore(b'_').unwrap();
        assert_eq!(flag.flags(), NumberFormat::DIGIT_SEPARATOR_FLAG_MASK);
        assert_eq!(flag.digit_separator(), b'_');
        assert_eq!(flag.decimal_point(), b'.');
        assert_eq!(flag.exponent(), b'e');
        assert_eq!(flag.required_integer_digits(), false);
        assert_eq!(flag.required_fraction_digits(), false);
        assert_eq!(flag.required_exponent_digits(), false);
        assert_eq!(flag.required_digits(), false);
        assert_eq!(flag.no_positive_mantissa_sign(), false);
        assert_eq!(flag.required_mantissa_sign(), false);
        assert_eq!(flag.no_exponent_notation(), false);
        assert_eq!(flag.no_positive_exponent_sign(), false);
        assert_eq!(flag.required_exponent_sign(), false);
        assert_eq!(flag.no_exponent_without_fraction(), false);
        assert_eq!(flag.no_special(), false);
        assert_eq!(flag.case_sensitive_special(), false);
        assert_eq!(flag.no_integer_leading_zeros(), false);
        assert_eq!(flag.no_float_leading_zeros(), false);
        assert_eq!(flag.integer_internal_digit_separator(), true);
        assert_eq!(flag.fraction_internal_digit_separator(), true);
        assert_eq!(flag.exponent_internal_digit_separator(), true);
        assert_eq!(flag.internal_digit_separator(), true);
        assert_eq!(flag.integer_leading_digit_separator(), true);
        assert_eq!(flag.fraction_leading_digit_separator(), true);
        assert_eq!(flag.exponent_leading_digit_separator(), true);
        assert_eq!(flag.leading_digit_separator(), true);
        assert_eq!(flag.integer_trailing_digit_separator(), true);
        assert_eq!(flag.fraction_trailing_digit_separator(), true);
        assert_eq!(flag.exponent_trailing_digit_separator(), true);
        assert_eq!(flag.trailing_digit_separator(), true);
        assert_eq!(flag.integer_consecutive_digit_separator(), true);
        assert_eq!(flag.fraction_consecutive_digit_separator(), true);
        assert_eq!(flag.exponent_consecutive_digit_separator(), true);
        assert_eq!(flag.consecutive_digit_separator(), true);
        assert_eq!(flag.special_digit_separator(), true);
        assert_eq!(flag.incorrect(), false);
        assert_eq!(flag.lossy(), false);

        #[cfg(feature ="radix")]
        assert_eq!(flag.radix(), 10);   // TODO(ahuszagh) Failing...

        #[cfg(feature ="radix")]
        assert_eq!(flag.exponent_backup(), b'^');
    }

    #[test]
    fn test_flags() {
        let flags = [
            NumberFormat::REQUIRED_INTEGER_DIGITS,
            NumberFormat::REQUIRED_FRACTION_DIGITS,
            NumberFormat::REQUIRED_EXPONENT_DIGITS,
            NumberFormat::NO_POSITIVE_MANTISSA_SIGN,
            NumberFormat::REQUIRED_MANTISSA_SIGN,
            NumberFormat::NO_EXPONENT_NOTATION,
            NumberFormat::NO_POSITIVE_EXPONENT_SIGN,
            NumberFormat::REQUIRED_EXPONENT_SIGN,
            NumberFormat::NO_EXPONENT_WITHOUT_FRACTION,
            NumberFormat::NO_SPECIAL,
            NumberFormat::CASE_SENSITIVE_SPECIAL,
            NumberFormat::NO_INTEGER_LEADING_ZEROS,
            NumberFormat::NO_FLOAT_LEADING_ZEROS,
            NumberFormat::INTEGER_INTERNAL_DIGIT_SEPARATOR,
            NumberFormat::FRACTION_INTERNAL_DIGIT_SEPARATOR,
            NumberFormat::EXPONENT_INTERNAL_DIGIT_SEPARATOR,
            NumberFormat::INTEGER_LEADING_DIGIT_SEPARATOR,
            NumberFormat::FRACTION_LEADING_DIGIT_SEPARATOR,
            NumberFormat::EXPONENT_LEADING_DIGIT_SEPARATOR,
            NumberFormat::INTEGER_TRAILING_DIGIT_SEPARATOR,
            NumberFormat::FRACTION_TRAILING_DIGIT_SEPARATOR,
            NumberFormat::EXPONENT_TRAILING_DIGIT_SEPARATOR,
            NumberFormat::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR,
            NumberFormat::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR,
            NumberFormat::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR,
            NumberFormat::SPECIAL_DIGIT_SEPARATOR
        ];
        for &flag in flags.iter() {
            assert_eq!(flag.flags(), flag);
            assert_eq!(flag.digit_separator(), 0);
        }
    }

    #[test]
    fn test_constants() {
        let flags = [
            NumberFormat::RUST_LITERAL,
            NumberFormat::RUST_STRING,
            NumberFormat::RUST_STRING_STRICT,
            NumberFormat::PYTHON_LITERAL,
            NumberFormat::PYTHON_STRING,
            NumberFormat::CXX17_LITERAL,
            NumberFormat::CXX17_STRING,
            NumberFormat::CXX14_LITERAL,
            NumberFormat::CXX14_STRING,
            NumberFormat::CXX11_LITERAL,
            NumberFormat::CXX11_STRING,
            NumberFormat::CXX03_LITERAL,
            NumberFormat::CXX03_STRING,
            NumberFormat::CXX98_LITERAL,
            NumberFormat::CXX98_STRING,
            NumberFormat::C18_LITERAL,
            NumberFormat::C18_STRING,
            NumberFormat::C11_LITERAL,
            NumberFormat::C11_STRING,
            NumberFormat::C99_LITERAL,
            NumberFormat::C99_STRING,
            NumberFormat::C90_LITERAL,
            NumberFormat::C90_STRING,
            NumberFormat::C89_LITERAL,
            NumberFormat::C89_STRING,
            NumberFormat::RUBY_LITERAL,
            NumberFormat::RUBY_STRING,
            NumberFormat::SWIFT_LITERAL,
            NumberFormat::SWIFT_STRING,
            NumberFormat::GO_LITERAL,
            NumberFormat::GO_STRING,
            NumberFormat::HASKELL_LITERAL,
            NumberFormat::HASKELL_STRING,
            NumberFormat::JAVASCRIPT_LITERAL,
            NumberFormat::JAVASCRIPT_STRING,
            NumberFormat::PERL_LITERAL,
            NumberFormat::PERL_STRING,
            NumberFormat::PHP_LITERAL,
            NumberFormat::PHP_STRING,
            NumberFormat::JAVA_LITERAL,
            NumberFormat::JAVA_STRING,
            NumberFormat::R_LITERAL,
            NumberFormat::R_STRING,
            NumberFormat::KOTLIN_LITERAL,
            NumberFormat::KOTLIN_STRING,
            NumberFormat::JULIA_LITERAL,
            NumberFormat::JULIA_STRING,
            NumberFormat::CSHARP7_LITERAL,
            NumberFormat::CSHARP7_STRING,
            NumberFormat::CSHARP6_LITERAL,
            NumberFormat::CSHARP6_STRING,
            NumberFormat::CSHARP5_LITERAL,
            NumberFormat::CSHARP5_STRING,
            NumberFormat::CSHARP4_LITERAL,
            NumberFormat::CSHARP4_STRING,
            NumberFormat::CSHARP3_LITERAL,
            NumberFormat::CSHARP3_STRING,
            NumberFormat::CSHARP2_LITERAL,
            NumberFormat::CSHARP2_STRING,
            NumberFormat::CSHARP1_LITERAL,
            NumberFormat::CSHARP1_STRING,
            NumberFormat::KAWA_LITERAL,
            NumberFormat::KAWA_STRING,
            NumberFormat::GAMBITC_LITERAL,
            NumberFormat::GAMBITC_STRING,
            NumberFormat::GUILE_LITERAL,
            NumberFormat::GUILE_STRING,
            NumberFormat::CLOJURE_LITERAL,
            NumberFormat::CLOJURE_STRING,
            NumberFormat::ERLANG_LITERAL,
            NumberFormat::ERLANG_STRING,
            NumberFormat::ELM_LITERAL,
            NumberFormat::ELM_STRING,
            NumberFormat::SCALA_LITERAL,
            NumberFormat::SCALA_STRING,
            NumberFormat::ELIXIR_LITERAL,
            NumberFormat::ELIXIR_STRING,
            NumberFormat::FORTRAN_LITERAL,
            NumberFormat::FORTRAN_STRING,
            NumberFormat::D_LITERAL,
            NumberFormat::D_STRING,
            NumberFormat::COFFEESCRIPT_LITERAL,
            NumberFormat::COFFEESCRIPT_STRING,
            NumberFormat::COBOL_LITERAL,
            NumberFormat::COBOL_STRING,
            NumberFormat::FSHARP_LITERAL,
            NumberFormat::FSHARP_STRING,
            NumberFormat::VB_LITERAL,
            NumberFormat::VB_STRING,
            NumberFormat::OCAML_LITERAL,
            NumberFormat::OCAML_STRING,
            NumberFormat::OBJECTIVEC_LITERAL,
            NumberFormat::OBJECTIVEC_STRING,
            NumberFormat::REASONML_LITERAL,
            NumberFormat::REASONML_STRING,
            NumberFormat::OCTAVE_LITERAL,
            NumberFormat::OCTAVE_STRING,
            NumberFormat::MATLAB_LITERAL,
            NumberFormat::MATLAB_STRING,
            NumberFormat::ZIG_LITERAL,
            NumberFormat::ZIG_STRING,
            NumberFormat::SAGE_LITERAL,
            NumberFormat::SAGE_STRING,
            NumberFormat::JSON,
            NumberFormat::TOML,
            NumberFormat::YAML,
            NumberFormat::XML,
            NumberFormat::SQLITE,
            NumberFormat::POSTGRESQL,
            NumberFormat::MYSQL,
            NumberFormat::MONGODB
        ];
        for &flag in flags.iter() {
            // Just wanna check the flags are defined.
            assert!((flag.bits == 0) | true);
            assert!((flag.digit_separator() == 0) | true);
        }
    }

    // TODO(ahuszagh) Test the builder, and rebuild.
}
