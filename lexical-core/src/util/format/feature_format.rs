//! Implementation of `format` with the feature enabled.

#![cfg(feature = "format")]

use bitflags::bitflags;

use super::flags;

// NUMBER FORMAT

bitflags! {
    /// Bitflags for a number format.
    ///
    /// This is used to derive the high-level bitflags. The default
    /// representation has no digit separators, no required integer or
    /// fraction digits, required exponent digits, a b'.' character
    /// for the decimal point, a b'e' character for decimal the exponent,
    /// and a b'^' character for the backup exponent.
    ///
    /// Bit Flags Layout
    /// ----------------
    ///
    /// The bitflags has the lower bits designated for flags that modify
    /// the parsing behavior of lexical, with 7 bits each set for the
    /// decimal point, decimal exponent, backup exponent, and digit
    /// separator, allowing any valid ASCII character as punctuation.
    /// The first 12-bits are reserved for non-digit separator flags,
    /// bits 18-25 for the decimal exponent character, bits 25-32 for
    /// the backup exponent character, bits 32-48 are reserved for digit
    /// separator flags, bits 50-57 for the decimal point character, and
    /// the last 7 bits for the digit separator character.
    ///
    /// ```text
    /// 0   1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |I/R|F/R|E/R|+/M|R/M|e/e|+/E|R/E|e/F|S/S|S/C|N/I|N/F|           |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 16  17  18  19  20  21  22  23  24  25  26  27  28  29  30  31  32
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |       |     Exponent Decimal      |     Exponent Backup       |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 32  33  34  35  36  37  38  39  40  41 42  43  44  45  46  47   48
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |I/I|F/I|E/I|I/L|F/L|E/L|I/T|F/T|E/T|I/C|F/C|E/C|S/D|   |       |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 48  49  50  51  52  53  54  55  56  57  58  59  60  62  62  63  64
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |       |      Decimal Point        |     Digit Separator       |
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
    ///     N/I = No integer leading zeros.
    ///     N/F = No float leading zeros.
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
    #[repr(C)]
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
        #[doc(hidden)]
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
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // RUST STRING STRICT [01345678MN]
        /// `RUST_STRING`, but enforces strict equality for special values.
        const RUST_STRING_STRICT = (
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
        );

        // PYTHON3 STRING [0134567MN]
        /// Float format to parse a Python3 float from string.
        const PYTHON3_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // PYTHON2 LITERAL [013456MN]
        /// Float format for a Python2 literal floating-point number.
        const PYTHON2_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // PYTHON2 STRING [0134567MN]
        /// Float format to parse a Python2 float from string.
        const PYTHON2_STRING = (
            flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b'\'')
            | flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // C++17 STRING [013456MN]
        const CXX17_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++14 LITERAL [01345689ABMN-']
        /// Float format for a C++14 literal floating-point number.
        const CXX14_LITERAL = (
            flags::digit_separator_to_flags(b'\'')
            | flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // C++14 STRING [013456MN]
        /// Float format to parse a C++14 float from string.
        const CXX14_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++11 LITERAL [0134568MN]
        /// Float format for a C++11 literal floating-point number.
        const CXX11_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C++11 STRING [013456MN]
        /// Float format to parse a C++11 float from string.
        const CXX11_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++03 LITERAL [0134567MN]
        /// Float format for a C++03 literal floating-point number.
        const CXX03_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C++03 STRING [013456MN]
        /// Float format to parse a C++03 float from string.
        const CXX03_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++98 LITERAL [0134567MN]
        /// Float format for a C++98 literal floating-point number.
        const CXX98_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C++98 STRING [013456MN]
        /// Float format to parse a C++98 float from string.
        const CXX98_STRING = (
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C18 STRING [013456MN]
        /// Float format to parse a C18 float from string.
        const C18_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C11 LITERAL [0134568MN]
        /// Float format for a C11 literal floating-point number.
        const C11_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C11 STRING [013456MN]
        /// Float format to parse a C11 float from string.
        const C11_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C99 LITERAL [0134568MN]
        /// Float format for a C99 literal floating-point number.
        const C99_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C99 STRING [013456MN]
        /// Float format to parse a C99 float from string.
        const C99_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C90 LITERAL [0134567MN]
        /// Float format for a C90 literal floating-point number.
        const C90_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C90 STRING [013456MN]
        /// Float format to parse a C90 float from string.
        const C90_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C89 LITERAL [0134567MN]
        /// Float format for a C89 literal floating-point number.
        const C89_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C89 STRING [013456MN]
        /// Float format to parse a C89 float from string.
        const C89_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // RUBY LITERAL [345689AM-_]
        /// Float format for a Ruby literal floating-point number.
        const RUBY_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // SWIFT LITERAL [34569ABFGHIJKMN-_]
        /// Float format for a Swift literal floating-point number.
        const SWIFT_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
        );

        // GO LITERAL [0134567MN]
        /// Float format for a Golang literal floating-point number.
        const GO_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GO STRING [013456MN]
        /// Float format to parse a Golang float from string.
        const GO_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
        );

        // HASKELL LITERAL [456MN]
        /// Float format for a Haskell literal floating-point number.
        const HASKELL_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::NO_SPECIAL.bits
        );

        // HASKELL STRING [45678MN]
        /// Float format to parse a Haskell float from string.
        const HASKELL_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // JAVASCRIPT LITERAL [01345678M]
        /// Float format for a Javascript literal floating-point number.
        const JAVASCRIPT_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // JAVASCRIPT STRING [012345678MN]
        /// Float format to parse a Javascript float from string.
        const JAVASCRIPT_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // PERL LITERAL [0134569ABDEFGHIJKMN-_]
        /// Float format for a Perl literal floating-point number.
        const PERL_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // PHP STRING [0123456MN]
        /// Float format to parse a PHP float from string.
        const PHP_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::NO_SPECIAL.bits
        );

        // JAVA LITERAL [0134569ABIJKMN-_]
        /// Float format for a Java literal floating-point number.
        const JAVA_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // R LITERAL [01345678MN]
        /// Float format for a R literal floating-point number.
        const R_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // JULIA LITERAL [01345689AMN-_]
        /// Float format for a Julia literal floating-point number.
        const JULIA_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP6 LITERAL [03456MN]
        /// Float format for a C#6 literal floating-point number.
        const CSHARP6_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP6 STRING [0134568MN]
        /// Float format to parse a C#6 float from string.
        const CSHARP6_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP5 LITERAL [03456MN]
        /// Float format for a C#5 literal floating-point number.
        const CSHARP5_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP5 STRING [0134568MN]
        /// Float format to parse a C#5 float from string.
        const CSHARP5_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP4 LITERAL [03456MN]
        /// Float format for a C#4 literal floating-point number.
        const CSHARP4_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP4 STRING [0134568MN]
        /// Float format to parse a C#4 float from string.
        const CSHARP4_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP3 LITERAL [03456MN]
        /// Float format for a C#3 literal floating-point number.
        const CSHARP3_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP3 STRING [0134568MN]
        /// Float format to parse a C#3 float from string.
        const CSHARP3_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP2 LITERAL [03456MN]
        /// Float format for a C#2 literal floating-point number.
        const CSHARP2_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP2 STRING [0134568MN]
        /// Float format to parse a C#2 float from string.
        const CSHARP2_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP1 LITERAL [03456MN]
        /// Float format for a C#1 literal floating-point number.
        const CSHARP1_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP1 STRING [0134568MN]
        /// Float format to parse a C#1 float from string.
        const CSHARP1_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // KAWA LITERAL [013456MN]
        /// Float format for a Kawa literal floating-point number.
        const KAWA_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // KAWA STRING [013456MN]
        /// Float format to parse a Kawa float from string.
        const KAWA_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GAMBITC LITERAL [013456MN]
        /// Float format for a Gambit-C literal floating-point number.
        const GAMBITC_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GAMBITC STRING [013456MN]
        /// Float format to parse a Gambit-C float from string.
        const GAMBITC_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GUILE LITERAL [013456MN]
        /// Float format for a Guile literal floating-point number.
        const GUILE_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GUILE STRING [013456MN]
        /// Float format to parse a Guile float from string.
        const GUILE_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CLOJURE LITERAL [13456MN]
        /// Float format for a Clojure literal floating-point number.
        const CLOJURE_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CLOJURE STRING [01345678MN]
        /// Float format to parse a Clojure float from string.
        const CLOJURE_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // ERLANG LITERAL [34578MN]
        /// Float format for an Erlang literal floating-point number.
        const ERLANG_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // ERLANG STRING [345MN]
        /// Float format to parse an Erlang float from string.
        const ERLANG_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_SPECIAL.bits
        );

        // ELM LITERAL [456]
        /// Float format for an Elm literal floating-point number.
        const ELM_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // SCALA LITERAL [3456]
        /// Float format for a Scala literal floating-point number.
        const SCALA_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // ELIXIR LITERAL [3459ABMN-_]
        /// Float format for an Elixir literal floating-point number.
        const ELIXIR_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_SPECIAL.bits
        );

        // FORTRAN LITERAL [013456MN]
        /// Float format for a FORTRAN literal floating-point number.
        const FORTRAN_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // FORTRAN STRING [0134567MN]
        /// Float format to parse a FORTRAN float from string.
        const FORTRAN_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // D LITERAL [0134569ABFGHIJKN-_]
        /// Float format for a D literal floating-point number.
        const D_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // COBOL LITERAL [0345MN]
        /// Float format for a Cobol literal floating-point number.
        const COBOL_LITERAL = (
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_SIGN.bits
            | Self::NO_SPECIAL.bits
        );

        // FSHARP LITERAL [13456789ABIJKMN-_]
        /// Float format for a F# literal floating-point number.
        const FSHARP_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // OCAML LITERAL [1456789ABDFGHIJKMN-_]
        /// Float format for an OCaml literal floating-point number.
        const OCAML_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // OBJECTIVEC STRING [013456MN]
        /// Float format to parse an Objective-C float from string.
        const OBJECTIVEC_STRING = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // REASONML LITERAL [13456789ABDFGHIJKMN-_]
        /// Float format for a ReasonML literal floating-point number.
        const REASONML_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b',')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::digit_separator_to_flags(b',')
            | flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // SAGE STRING [01345679ABMN-_]
        /// Float format to parse a Sage float from string.
        const SAGE_STRING = (
            flags::digit_separator_to_flags(b'_')
            | flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // JSON [456]
        /// Float format for a JSON literal floating-point number.
        const JSON = (
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // SQLITE [013456MN]
        /// Float format for a SQLite literal floating-point number.
        const SQLITE = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // POSTGRESQL [013456MN]
        /// Float format for a PostgreSQL literal floating-point number.
        const POSTGRESQL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // MYSQL [013456MN]
        /// Float format for a MySQL literal floating-point number.
        const MYSQL = (
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // MONGODB [01345678M]
        /// Float format for a MongoDB literal floating-point number.
        const MONGODB = (
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
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
            flags::exponent_decimal_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
            | Self::DIGIT_SEPARATOR_FLAG_MASK.bits
        );

        /// Ignore interface float format flags.
        #[doc(hidden)]
        const IGNORE_INTERFACE = Self::IGNORE.bits & Self::INTERFACE_FLAG_MASK.bits;
    }
}

impl NumberFormat {
    /// Create new format from bits.
    /// This method should **NEVER** be public, use the builder API.
    #[inline(always)]
    pub(crate) const fn new(bits: u64) -> Self {
        Self { bits }
    }

    /// Create new format from digit separator.
    /// This method should **NEVER** be public, use the builder API.
    #[inline(always)]
    #[cfg(test)]
    pub(crate) const fn from_digit_separator(digit_separator: u8) -> Self {
        Self::new(flags::digit_separator_to_flags(digit_separator))
    }

    /// Get the flag bits from the compiled float format.
    #[inline(always)]
    pub const fn flags(self) -> Self {
        Self::new(self.bits() & Self::FLAG_MASK.bits())
    }

    /// Get the interface flag bits from the compiled float format.
    #[inline(always)]
    pub const fn interface_flags(self) -> Self {
        Self::new(self.bits() & Self::INTERFACE_FLAG_MASK.bits())
    }

    /// Get the digit separator for the number format.
    #[inline(always)]
    pub const fn digit_separator(self) -> u8 {
        flags::digit_separator_from_flags(self.bits)
    }

    /// Get the decimal point character for the number format.
    #[inline(always)]
    pub const fn decimal_point(self) -> u8 {
        flags::decimal_point_from_flags(self.bits)
    }

    /// Get the decimal exponent character for the number format.
    #[inline(always)]
    pub const fn exponent_decimal(self) -> u8 {
        flags::exponent_decimal_from_flags(self.bits)
    }

    /// Get the backup exponent character for the number format.
    #[inline(always)]
    pub const fn exponent_backup(self) -> u8 {
        flags::exponent_backup_from_flags(self.bits)
    }

    const_fn!(
    /// Get the exponent character based on the radix.
    #[inline(always)]
    pub const fn exponent(self, radix: u32) -> u8 {
        if cfg!(feature = "radix") && radix != 10 {
            self.exponent_backup()
        } else {
            self.exponent_decimal()
        }
    });

    /// Get if digits are required before the decimal point.
    #[inline(always)]
    pub const fn required_integer_digits(self) -> bool {
        self.intersects(Self::REQUIRED_INTEGER_DIGITS)
    }

    /// Get if digits are required after the decimal point.
    #[inline(always)]
    pub const fn required_fraction_digits(self) -> bool {
        self.intersects(Self::REQUIRED_FRACTION_DIGITS)
    }

    /// Get if digits are required after the exponent character.
    #[inline(always)]
    pub const fn required_exponent_digits(self) -> bool {
        self.intersects(Self::REQUIRED_EXPONENT_DIGITS)
    }

    /// Get if digits are required before or after the decimal point.
    #[inline(always)]
    pub const fn required_digits(self) -> bool {
        self.intersects(Self::REQUIRED_DIGITS)
    }

    /// Get if a positive sign before the mantissa is not allowed.
    #[inline(always)]
    pub const fn no_positive_mantissa_sign(self) -> bool {
        self.intersects(Self::NO_POSITIVE_MANTISSA_SIGN)
    }

    /// Get if a sign symbol before the mantissa is required.
    #[inline(always)]
    pub const fn required_mantissa_sign(self) -> bool {
        self.intersects(Self::REQUIRED_MANTISSA_SIGN)
    }

    /// Get if exponent notation is not allowed.
    #[inline(always)]
    pub const fn no_exponent_notation(self) -> bool {
        self.intersects(Self::NO_EXPONENT_NOTATION)
    }

    /// Get if a positive sign before the exponent is not allowed.
    #[inline(always)]
    pub const fn no_positive_exponent_sign(self) -> bool {
        self.intersects(Self::NO_POSITIVE_EXPONENT_SIGN)
    }

    /// Get if a sign symbol before the exponent is required.
    #[inline(always)]
    pub const fn required_exponent_sign(self) -> bool {
        self.intersects(Self::REQUIRED_EXPONENT_SIGN)
    }

    /// Get if an exponent without fraction is not allowed.
    #[inline(always)]
    pub const fn no_exponent_without_fraction(self) -> bool {
        self.intersects(Self::NO_EXPONENT_WITHOUT_FRACTION)
    }

    /// Get if special (non-finite) values are not allowed.
    #[inline(always)]
    pub const fn no_special(self) -> bool {
        self.intersects(Self::NO_SPECIAL)
    }

    /// Get if special (non-finite) values are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_special(self) -> bool {
        self.intersects(Self::CASE_SENSITIVE_SPECIAL)
    }

    /// Get if leading zeros before an integer are not allowed.
    #[inline(always)]
    pub const fn no_integer_leading_zeros(self) -> bool {
        self.intersects(Self::NO_INTEGER_LEADING_ZEROS)
    }

    /// Get if leading zeros before a float are not allowed.
    #[inline(always)]
    pub const fn no_float_leading_zeros(self) -> bool {
        self.intersects(Self::NO_FLOAT_LEADING_ZEROS)
    }

    /// Get if digit separators are allowed between integer digits.
    #[inline(always)]
    pub const fn integer_internal_digit_separator(self) -> bool {
        self.intersects(Self::INTEGER_INTERNAL_DIGIT_SEPARATOR)
    }

    /// Get if digit separators are allowed between fraction digits.
    #[inline(always)]
    pub const fn fraction_internal_digit_separator(self) -> bool {
        self.intersects(Self::FRACTION_INTERNAL_DIGIT_SEPARATOR)
    }

    /// Get if digit separators are allowed between exponent digits.
    #[inline(always)]
    pub const fn exponent_internal_digit_separator(self) -> bool {
        self.intersects(Self::EXPONENT_INTERNAL_DIGIT_SEPARATOR)
    }

    /// Get if digit separators are allowed between digits.
    #[inline(always)]
    pub const fn internal_digit_separator(self) -> bool {
        self.intersects(Self::INTERNAL_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed before any integer digits.
    #[inline(always)]
    pub const fn integer_leading_digit_separator(self) -> bool {
        self.intersects(Self::INTEGER_LEADING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed before any fraction digits.
    #[inline(always)]
    pub const fn fraction_leading_digit_separator(self) -> bool {
        self.intersects(Self::FRACTION_LEADING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed before any exponent digits.
    #[inline(always)]
    pub const fn exponent_leading_digit_separator(self) -> bool {
        self.intersects(Self::EXPONENT_LEADING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed before any digits.
    #[inline(always)]
    pub const fn leading_digit_separator(self) -> bool {
        self.intersects(Self::LEADING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed after any integer digits.
    #[inline(always)]
    pub const fn integer_trailing_digit_separator(self) -> bool {
        self.intersects(Self::INTEGER_TRAILING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed after any fraction digits.
    #[inline(always)]
    pub const fn fraction_trailing_digit_separator(self) -> bool {
        self.intersects(Self::FRACTION_TRAILING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed after any exponent digits.
    #[inline(always)]
    pub const fn exponent_trailing_digit_separator(self) -> bool {
        self.intersects(Self::EXPONENT_TRAILING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed after any digits.
    #[inline(always)]
    pub const fn trailing_digit_separator(self) -> bool {
        self.intersects(Self::TRAILING_DIGIT_SEPARATOR)
    }

    /// Get if multiple consecutive integer digit separators are allowed.
    #[inline(always)]
    pub const fn integer_consecutive_digit_separator(self) -> bool {
        self.intersects(Self::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR)
    }

    /// Get if multiple consecutive fraction digit separators are allowed.
    #[inline(always)]
    pub const fn fraction_consecutive_digit_separator(self) -> bool {
        self.intersects(Self::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR)
    }

    /// Get if multiple consecutive exponent digit separators are allowed.
    #[inline(always)]
    pub const fn exponent_consecutive_digit_separator(self) -> bool {
        self.intersects(Self::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR)
    }

    /// Get if multiple consecutive digit separators are allowed.
    #[inline(always)]
    pub const fn consecutive_digit_separator(self) -> bool {
        self.intersects(Self::CONSECUTIVE_DIGIT_SEPARATOR)
    }

    /// Get if any digit separators are allowed in special (non-finite) values.
    #[inline(always)]
    pub const fn special_digit_separator(self) -> bool {
        self.intersects(Self::SPECIAL_DIGIT_SEPARATOR)
    }

    // BUILDERS

    /// Create new builder to instantiate `NumberFormat`.
    #[inline(always)]
    pub const fn builder() -> NumberFormatBuilder {
        NumberFormatBuilder::new()
    }

    /// Recreate `NumberFormatBuilder` using current `NumberFormat` values.
    #[inline(always)]
    pub const fn rebuild(&self) -> NumberFormatBuilder {
        NumberFormatBuilder {
            digit_separator: self.digit_separator(),
            decimal_point: self.decimal_point(),
            exponent_decimal: self.exponent_decimal(),
            exponent_backup: self.exponent_backup(),
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
        }
    }
}

// NUMBER FORMAT BUILDER

/// Build float format value from specifications.
///
/// * `digit_separator`                         - Character to separate digits.
/// * `decimal_point`                           - Character to designate the decimal point.
/// * `exponent_decimal`                        - Character to designate the exponent for decimal strings.
/// * `exponent_backup`                         - Character to designate the exponent for non-decimal strings.
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
///
/// Returns the format on calling build if it was able to compile the format,
/// otherwise, returns None.
#[derive(Debug, Clone)]
pub struct NumberFormatBuilder {
    digit_separator: u8,
    decimal_point: u8,
    exponent_decimal: u8,
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
}

impl NumberFormatBuilder {
    /// Create new NumberFormatBuilder with default arguments.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            digit_separator: b'\x00',
            decimal_point: b'.',
            exponent_decimal: b'e',
            exponent_backup: b'^',
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
        }
    }

    // GETTERS

    /// Get the digit separator for the number format.
    #[inline(always)]
    pub const fn get_digit_separator(&self) -> u8 {
        self.digit_separator
    }

    /// Get the decimal point character for the number format.
    #[inline(always)]
    pub const fn get_decimal_point(&self) -> u8 {
        self.decimal_point
    }

    /// Get the decimal exponent character for the number format.
    #[inline(always)]
    pub const fn get_exponent_decimal(&self) -> u8 {
        self.exponent_decimal
    }

    /// Get the backup exponent character for the number format.
    #[inline(always)]
    pub const fn get_exponent_backup(&self) -> u8 {
        self.exponent_backup
    }

    /// Get if digits are required before the decimal point.
    #[inline(always)]
    pub const fn get_required_integer_digits(&self) -> bool {
        self.required_integer_digits
    }

    /// Get if digits are required after the decimal point.
    #[inline(always)]
    pub const fn get_required_fraction_digits(&self) -> bool {
        self.required_fraction_digits
    }

    /// Get if digits are required after the exponent character.
    #[inline(always)]
    pub const fn get_required_exponent_digits(&self) -> bool {
        self.required_exponent_digits
    }

    /// Get if a positive sign before the mantissa is not allowed.
    #[inline(always)]
    pub const fn get_no_positive_mantissa_sign(&self) -> bool {
        self.no_positive_mantissa_sign
    }

    /// Get if a sign symbol before the mantissa is required.
    #[inline(always)]
    pub const fn get_required_mantissa_sign(&self) -> bool {
        self.required_mantissa_sign
    }

    /// Get if exponent notation is not allowed.
    #[inline(always)]
    pub const fn get_no_exponent_notation(&self) -> bool {
        self.no_exponent_notation
    }

    /// Get if a positive sign before the exponent is not allowed.
    #[inline(always)]
    pub const fn get_no_positive_exponent_sign(&self) -> bool {
        self.no_positive_exponent_sign
    }

    /// Get if a sign symbol before the exponent is required.
    #[inline(always)]
    pub const fn get_required_exponent_sign(&self) -> bool {
        self.required_exponent_sign
    }

    /// Get if an exponent without fraction is not allowed.
    #[inline(always)]
    pub const fn get_no_exponent_without_fraction(&self) -> bool {
        self.no_exponent_without_fraction
    }

    /// Get if special (non-finite) values are not allowed.
    #[inline(always)]
    pub const fn get_no_special(&self) -> bool {
        self.no_special
    }

    /// Get if special (non-finite) values are case-sensitive.
    #[inline(always)]
    pub const fn get_case_sensitive_special(&self) -> bool {
        self.case_sensitive_special
    }

    /// Get if leading zeros before an integer are not allowed.
    #[inline(always)]
    pub const fn get_no_integer_leading_zeros(&self) -> bool {
        self.no_integer_leading_zeros
    }

    /// Get if leading zeros before a float are not allowed.
    #[inline(always)]
    pub const fn get_no_float_leading_zeros(&self) -> bool {
        self.no_float_leading_zeros
    }

    /// Get if digit separators are allowed between integer digits.
    #[inline(always)]
    pub const fn get_integer_internal_digit_separator(&self) -> bool {
        self.integer_internal_digit_separator
    }

    /// Get if digit separators are allowed between fraction digits.
    #[inline(always)]
    pub const fn get_fraction_internal_digit_separator(&self) -> bool {
        self.fraction_internal_digit_separator
    }

    /// Get if digit separators are allowed between exponent digits.
    #[inline(always)]
    pub const fn get_exponent_internal_digit_separator(&self) -> bool {
        self.exponent_internal_digit_separator
    }

    /// Get if a digit separator is allowed before any integer digits.
    #[inline(always)]
    pub const fn get_integer_leading_digit_separator(&self) -> bool {
        self.integer_leading_digit_separator
    }

    /// Get if a digit separator is allowed before any fraction digits.
    #[inline(always)]
    pub const fn get_fraction_leading_digit_separator(&self) -> bool {
        self.fraction_leading_digit_separator
    }

    /// Get if a digit separator is allowed before any exponent digits.
    #[inline(always)]
    pub const fn get_exponent_leading_digit_separator(&self) -> bool {
        self.exponent_leading_digit_separator
    }

    /// Get if a digit separator is allowed after any integer digits.
    #[inline(always)]
    pub const fn get_integer_trailing_digit_separator(&self) -> bool {
        self.integer_trailing_digit_separator
    }

    /// Get if a digit separator is allowed after any fraction digits.
    #[inline(always)]
    pub const fn get_fraction_trailing_digit_separator(&self) -> bool {
        self.fraction_trailing_digit_separator
    }

    /// Get if a digit separator is allowed after any exponent digits.
    #[inline(always)]
    pub const fn get_exponent_trailing_digit_separator(&self) -> bool {
        self.exponent_trailing_digit_separator
    }

    /// Get if multiple consecutive integer digit separators are allowed.
    #[inline(always)]
    pub const fn get_integer_consecutive_digit_separator(&self) -> bool {
        self.integer_consecutive_digit_separator
    }

    /// Get if multiple consecutive fraction digit separators are allowed.
    #[inline(always)]
    pub const fn get_fraction_consecutive_digit_separator(&self) -> bool {
        self.fraction_consecutive_digit_separator
    }

    /// Get if multiple consecutive exponent digit separators are allowed.
    #[inline(always)]
    pub const fn get_exponent_consecutive_digit_separator(&self) -> bool {
        self.exponent_consecutive_digit_separator
    }

    /// Get if any digit separators are allowed in special (non-finite) values.
    #[inline(always)]
    pub const fn get_special_digit_separator(&self) -> bool {
        self.special_digit_separator
    }

    // SETTERS

    /// Set the digit separator for the number format.
    #[inline(always)]
    pub const fn digit_separator(mut self, digit_separator: u8) -> Self {
        self.digit_separator = digit_separator;
        self
    }

    /// Set the decimal point character for the number format.
    #[inline(always)]
    pub const fn decimal_point(mut self, decimal_point: u8) -> Self {
        self.decimal_point = decimal_point;
        self
    }

    const_fn!(
    /// Set the decimal exponent character for the number format.
    #[inline(always)]
    pub const fn exponent_decimal(mut self, exponent_decimal: u8) -> Self {
        self.exponent_decimal = flags::to_ascii_lowercase(exponent_decimal);
        self
    });

    const_fn!(
    /// Set the backup exponent character for the number format.
    #[inline(always)]
    pub const fn exponent_backup(mut self, exponent_backup: u8) -> Self {
        self.exponent_backup = flags::to_ascii_lowercase(exponent_backup);
        self
    });

    /// Set if digits are required before the decimal point.
    #[inline(always)]
    pub const fn required_integer_digits(mut self, required_integer_digits: bool) -> Self {
        self.required_integer_digits = required_integer_digits;
        self
    }

    /// Set if digits are required after the decimal point.
    #[inline(always)]
    pub const fn required_fraction_digits(mut self, required_fraction_digits: bool) -> Self {
        self.required_fraction_digits = required_fraction_digits;
        self
    }

    /// Set if digits are required after the exponent character.
    #[inline(always)]
    pub const fn required_exponent_digits(mut self, required_exponent_digits: bool) -> Self {
        self.required_exponent_digits = required_exponent_digits;
        self
    }

    /// Set if digits are required for all float components.
    #[inline(always)]
    pub const fn required_digits(mut self, flag: bool) -> Self {
        self = self.required_integer_digits(flag);
        self = self.required_fraction_digits(flag);
        self = self.required_exponent_digits(flag);
        self
    }

    /// Set if a positive sign before the mantissa is not allowed.
    #[inline(always)]
    pub const fn no_positive_mantissa_sign(mut self, no_positive_mantissa_sign: bool) -> Self {
        self.no_positive_mantissa_sign = no_positive_mantissa_sign;
        self
    }

    /// Set if a sign symbol before the mantissa is required.
    #[inline(always)]
    pub const fn required_mantissa_sign(mut self, required_mantissa_sign: bool) -> Self {
        self.required_mantissa_sign = required_mantissa_sign;
        self
    }

    /// Set if exponent notation is not allowed.
    #[inline(always)]
    pub const fn no_exponent_notation(mut self, no_exponent_notation: bool) -> Self {
        self.no_exponent_notation = no_exponent_notation;
        self
    }

    /// Set if a positive sign before the exponent is not allowed.
    #[inline(always)]
    pub const fn no_positive_exponent_sign(mut self, no_positive_exponent_sign: bool) -> Self {
        self.no_positive_exponent_sign = no_positive_exponent_sign;
        self
    }

    /// Set if a sign symbol before the exponent is required.
    #[inline(always)]
    pub const fn required_exponent_sign(mut self, required_exponent_sign: bool) -> Self {
        self.required_exponent_sign = required_exponent_sign;
        self
    }

    /// Set if an exponent without fraction is not allowed.
    #[inline(always)]
    pub const fn no_exponent_without_fraction(mut self, no_exponent_without_fraction: bool) -> Self {
        self.no_exponent_without_fraction = no_exponent_without_fraction;
        self
    }

    /// Set if special (non-finite) values are not allowed.
    #[inline(always)]
    pub const fn no_special(mut self, no_special: bool) -> Self {
        self.no_special = no_special;
        self
    }

    /// Set if special (non-finite) values are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_special(mut self, case_sensitive_special: bool) -> Self {
        self.case_sensitive_special = case_sensitive_special;
        self
    }

    /// Set if leading zeros before an integer are not allowed.
    #[inline(always)]
    pub const fn no_integer_leading_zeros(mut self, no_integer_leading_zeros: bool) -> Self {
        self.no_integer_leading_zeros = no_integer_leading_zeros;
        self
    }

    /// Set if leading zeros before a float are not allowed.
    #[inline(always)]
    pub const fn no_float_leading_zeros(mut self, no_float_leading_zeros: bool) -> Self {
        self.no_float_leading_zeros = no_float_leading_zeros;
        self
    }

    /// Set if digit separators are allowed between integer digits.
    #[inline(always)]
    pub const fn integer_internal_digit_separator(mut self, integer_internal_digit_separator: bool) -> Self {
        self.integer_internal_digit_separator = integer_internal_digit_separator;
        self
    }

    /// Set if digit separators are allowed between fraction digits.
    #[inline(always)]
    pub const fn fraction_internal_digit_separator(mut self, fraction_internal_digit_separator: bool) -> Self {
        self.fraction_internal_digit_separator = fraction_internal_digit_separator;
        self
    }

    /// Set if digit separators are allowed between exponent digits.
    #[inline(always)]
    pub const fn exponent_internal_digit_separator(mut self, exponent_internal_digit_separator: bool) -> Self {
        self.exponent_internal_digit_separator = exponent_internal_digit_separator;
        self
    }

    /// Set if a digit separator is allowed before any integer digits.
    #[inline(always)]
    pub const fn integer_leading_digit_separator(mut self, integer_leading_digit_separator: bool) -> Self {
        self.integer_leading_digit_separator = integer_leading_digit_separator;
        self
    }

    /// Set if a digit separator is allowed before any fraction digits.
    #[inline(always)]
    pub const fn fraction_leading_digit_separator(mut self, fraction_leading_digit_separator: bool) -> Self {
        self.fraction_leading_digit_separator = fraction_leading_digit_separator;
        self
    }

    /// Set if a digit separator is allowed before any exponent digits.
    #[inline(always)]
    pub const fn exponent_leading_digit_separator(mut self, exponent_leading_digit_separator: bool) -> Self {
        self.exponent_leading_digit_separator = exponent_leading_digit_separator;
        self
    }

    /// Set if a digit separator is allowed after any integer digits.
    #[inline(always)]
    pub const fn integer_trailing_digit_separator(mut self, integer_trailing_digit_separator: bool) -> Self {
        self.integer_trailing_digit_separator = integer_trailing_digit_separator;
        self
    }

    /// Set if a digit separator is allowed after any fraction digits.
    #[inline(always)]
    pub const fn fraction_trailing_digit_separator(mut self, fraction_trailing_digit_separator: bool) -> Self {
        self.fraction_trailing_digit_separator = fraction_trailing_digit_separator;
        self
    }

    /// Set if a digit separator is allowed after any exponent digits.
    #[inline(always)]
    pub const fn exponent_trailing_digit_separator(mut self, exponent_trailing_digit_separator: bool) -> Self {
        self.exponent_trailing_digit_separator = exponent_trailing_digit_separator;
        self
    }

    /// Set if multiple consecutive integer digit separators are allowed.
    #[inline(always)]
    pub const fn integer_consecutive_digit_separator(mut self, integer_consecutive_digit_separator: bool) -> Self {
        self.integer_consecutive_digit_separator = integer_consecutive_digit_separator;
        self
    }

    /// Set if multiple consecutive fraction digit separators are allowed.
    #[inline(always)]
    pub const fn fraction_consecutive_digit_separator(mut self, fraction_consecutive_digit_separator: bool) -> Self {
        self.fraction_consecutive_digit_separator = fraction_consecutive_digit_separator;
        self
    }

    /// Set if multiple consecutive exponent digit separators are allowed.
    #[inline(always)]
    pub const fn exponent_consecutive_digit_separator(mut self, exponent_consecutive_digit_separator: bool) -> Self {
        self.exponent_consecutive_digit_separator = exponent_consecutive_digit_separator;
        self
    }

    /// Set if any digit separators are allowed in special (non-finite) values.
    #[inline(always)]
    pub const fn special_digit_separator(mut self, special_digit_separator: bool) -> Self {
        self.special_digit_separator = special_digit_separator;
        self
    }

    /// Set all integer digit separator flag masks.
    #[inline(always)]
    pub const fn digit_separator_flag_mask(mut self, flag: bool) -> Self {
        self = self.integer_digit_separator_flag_mask(flag);
        self = self.fraction_digit_separator_flag_mask(flag);
        self = self.exponent_digit_separator_flag_mask(flag);
        self = self.special_digit_separator(flag);
        self
    }

    /// Set all integer digit separator flag masks.
    #[inline(always)]
    pub const fn integer_digit_separator_flag_mask(mut self, flag: bool) -> Self {
        self = self.integer_internal_digit_separator(flag);
        self = self.integer_leading_digit_separator(flag);
        self = self.integer_trailing_digit_separator(flag);
        self = self.integer_consecutive_digit_separator(flag);
        self
    }

    /// Set all fraction digit separator flag masks.
    #[inline(always)]
    pub const fn fraction_digit_separator_flag_mask(mut self, flag: bool) -> Self {
        self = self.fraction_internal_digit_separator(flag);
        self = self.fraction_leading_digit_separator(flag);
        self = self.fraction_trailing_digit_separator(flag);
        self = self.fraction_consecutive_digit_separator(flag);
        self
    }

    /// Set all exponent digit separator flag masks.
    #[inline(always)]
    pub const fn exponent_digit_separator_flag_mask(mut self, flag: bool) -> Self {
        self = self.exponent_internal_digit_separator(flag);
        self = self.exponent_leading_digit_separator(flag);
        self = self.exponent_trailing_digit_separator(flag);
        self = self.exponent_consecutive_digit_separator(flag);
        self
    }

    // BUILDER

    const_fn!(
    /// Create `NumberFormat` from builder options.
    ///
    /// If the format is invalid, this function will return `None`.
    #[inline]
    pub const fn build(&self) -> Option<NumberFormat> {
        let mut format = NumberFormat::new(0);
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

        // Add punctuation characters.
        if format.intersects(NumberFormat::DIGIT_SEPARATOR_FLAG_MASK) {
            format.bits |= flags::digit_separator_to_flags(self.digit_separator);
        }
        format.bits |= flags::decimal_point_to_flags(self.decimal_point);
        format.bits |= flags::exponent_decimal_to_flags(self.exponent_decimal);
        format.bits |= flags::exponent_backup_to_flags(self.exponent_backup);

        // Validation.
        let is_invalid =
            !flags::is_valid_digit_separator(self.digit_separator)
            || !flags::is_valid_decimal_point(self.decimal_point)
            || !flags::is_valid_exponent_decimal(self.exponent_decimal)
            || !flags::is_valid_exponent_backup(self.exponent_backup)
            || !flags::is_valid_punctuation(self.digit_separator, self.decimal_point, self.exponent_decimal, self.exponent_backup)
            || format.intersects(NumberFormat::NO_EXPONENT_NOTATION) && format.intersects(NumberFormat::EXPONENT_FLAG_MASK)
            || self.no_positive_mantissa_sign && self.required_mantissa_sign
            || self.no_positive_exponent_sign && self.required_exponent_sign
            || self.no_special && (self.case_sensitive_special || self.special_digit_separator)
            || check_flag!(format, INTEGER_DIGIT_SEPARATOR_FLAG_MASK, INTEGER_CONSECUTIVE_DIGIT_SEPARATOR)
            || check_flag!(format, FRACTION_DIGIT_SEPARATOR_FLAG_MASK, FRACTION_CONSECUTIVE_DIGIT_SEPARATOR)
            || check_flag!(format, EXPONENT_DIGIT_SEPARATOR_FLAG_MASK, EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR);

        match is_invalid {
            true  => None,
            false => Some(format)
        }
    });
}

impl Default for NumberFormatBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignore() {
        let flag = NumberFormat::IGNORE;
        let flag = flag | NumberFormat::from_digit_separator(b'_');
        assert_eq!(flag.flags(), NumberFormat::DIGIT_SEPARATOR_FLAG_MASK);
        assert_eq!(flag.digit_separator(), b'_');
        assert_eq!(flag.decimal_point(), b'.');
        assert_eq!(flag.exponent_decimal(), b'e');
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
            // Check these values are properly set.
            assert_eq!(flag.decimal_point(), b'.');
            assert_eq!(flag.exponent_decimal(), b'e');
            assert_eq!(flag.exponent_backup(), b'^');
        }
    }

    #[test]
    fn test_builder() {
        // Test a few invalid ones.
        let flag = NumberFormat::builder().exponent_decimal(b'.').build();
        assert_eq!(flag, None);

        // Test a few valid ones.
        let flag = NumberFormat::builder().decimal_point(b'.').build();
        assert!(flag.is_some());
        let flag = flag.unwrap();
        assert_eq!(flag.digit_separator(), b'\x00');
        assert_eq!(flag.decimal_point(), b'.');
        assert_eq!(flag.exponent_decimal(), b'e');
        assert_eq!(flag.exponent_backup(), b'^');
        assert_eq!(flag.required_integer_digits(), false);
        assert_eq!(flag.required_fraction_digits(), false);
        assert_eq!(flag.required_exponent_digits(), false);
    }

    #[test]
    fn test_rebuild() {
        let flag = NumberFormat::CSHARP7_LITERAL;
        let rebuilt = flag.rebuild().decimal_point(b',').build().unwrap();
        assert_eq!(flag.digit_separator(), b'_');
        assert_eq!(rebuilt.digit_separator(), b'_');
        assert_eq!(rebuilt.flags(), flag.flags());
        assert_eq!(flag.decimal_point(), b'.');
        assert_eq!(rebuilt.decimal_point(), b',');
    }
}
