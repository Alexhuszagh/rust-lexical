//! Implementation of `SyntaxFormat` with the `format` feature enabled.

#![cfg(feature = "format")]

use bitflags::bitflags;

use super::flags;

// SYNTAX FORMAT

bitflags! {
    /// Bitflags for a number format.
    ///
    /// This is used to derive the high-level bitflags. The default
    /// representation has no digit separators, no required integer or
    /// fraction digits, and required exponent digits.
    ///
    /// Bit Flags Layout
    /// ----------------
    ///
    /// The bitflags has the lower bits designated for flags that modify
    /// the syntax verification of lexical. The first 8 bits are
    /// designated for digit separator (so, if the compiler is
    /// smart enough, it can easily fit in a register).
    ///
    /// Bits 8-32 are reserved for float component flags, such
    /// as for example if base prefixes or postfixes are case-sensitive,
    /// if leading zeros in a float are valid, etc.
    ///
    /// Bits 32-64 are reserved for digit separator flags. These
    /// define which locations within a float or integer digit separators
    /// are valid, for example, before any digits in the integer component,
    /// whether consecutive digit separators are allowed, and more.
    ///
    /// ```text
    /// 0   1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |       Digit Separator         |I/R|F/R|E/R|+/M|R/M|e/e|+/E|R/E|
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 0   1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |e/F|S/S|S/C|N/I|N/F|R/e|e/C|e/P|e/S|                           |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 16  17  18  19  20  21  22  23  24  25  26  27  28  29  30  31  32
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |       |                                                       |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 32  33  34  35  36  37  38  39  40  41 42  43  44  45  46  47   48
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |I/I|F/I|E/I|I/L|F/L|E/L|I/T|F/T|E/T|I/C|F/C|E/C|S/D|   |       |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 48  49  50  51  52  53  54  55  56  57  58  59  60  62  62  63  64
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |                                   |     Digit Separator       |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// Where:
    ///     Non-Digit Separator Flags:
    ///         I/R = Required integer digits.
    ///         F/R = Required fraction digits.
    ///         E/R = Required exponent digits.
    ///         +/M = No mantissa positive sign.
    ///         R/M = Required positive sign.
    ///         e/e = No exponent notation.
    ///         +/E = No exponent positive sign.
    ///         R/E = Required exponent sign.
    ///         e/F = No exponent without fraction.
    ///         S/S = No special (non-finite) values.
    ///         S/C = Case-sensitive special (non-finite) values.
    ///         N/I = No integer leading zeros.
    ///         N/F = No float leading zeros.
    ///         R/e = Required exponent characters.
    ///         e/C = Case-sensitive exponent character.
    ///         e/P = Case-sensitive base prefix.
    ///         e/S = Case-sensitive base suffix.
    ///
    ///     Digit Separator Flags:
    ///         I/I = Integer internal digit separator.
    ///         F/I = Fraction internal digit separator.
    ///         E/I = Exponent internal digit separator.
    ///         I/L = Integer leading digit separator.
    ///         F/L = Fraction leading digit separator.
    ///         E/L = Exponent leading digit separator.
    ///         I/T = Integer trailing digit separator.
    ///         F/T = Fraction trailing digit separator.
    ///         E/T = Exponent trailing digit separator.
    ///         I/C = Integer consecutive digit separator.
    ///         F/C = Fraction consecutive digit separator.
    ///         E/C = Exponent consecutive digit separator.
    ///         S/D = Special (non-finite) digit separator.
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
    /// O: '1.0'        // No required exponent notation.
    /// P: '3.0E7'      // Case-insensitive exponent character.
    /// P: '0x3.0'      // Case-insensitive base prefix.
    /// P: '3.0H'       // Case-insensitive base postfix.
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
    #[repr(align(8))]
    #[derive(Default)]
    pub struct SyntaxFormat: u64 {
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

        #[doc(hidden)]
        const REQUIRED_EXPONENT_NOTATION            = flags::REQUIRED_EXPONENT_NOTATION;

        #[doc(hidden)]
        const CASE_SENSITIVE_EXPONENT               = flags::CASE_SENSITIVE_EXPONENT;

        #[doc(hidden)]
        const CASE_SENSITIVE_BASE_PREFIX            = flags::CASE_SENSITIVE_BASE_PREFIX;

        #[doc(hidden)]
        const CASE_SENSITIVE_BASE_SUFFIX            = flags::CASE_SENSITIVE_BASE_SUFFIX;

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

        // MASKS

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
            | Self::REQUIRED_EXPONENT_NOTATION.bits
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
            | Self::REQUIRED_EXPONENT_NOTATION.bits
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
            | Self::REQUIRED_EXPONENT_NOTATION.bits
            | Self::EXPONENT_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_LEADING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_TRAILING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

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
        /// Number format for a Rust literal floating-point number.
        const RUST_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // RUST STRING [0134567MN]
        /// Number format to parse a Rust float from string.
        const RUST_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // RUST STRING STRICT [01345678MN]
        /// `RUST_STRING`, but enforces strict equality for special values.
        const RUST_STRING_STRICT = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        /// Number format for a Python literal floating-point number.
        const PYTHON_LITERAL = Self::PYTHON3_LITERAL.bits;

        /// Number format to parse a Python float from string.
        const PYTHON_STRING = Self::PYTHON3_STRING.bits;

        // PYTHON3 LITERAL [013456N]
        /// Number format for a Python3 literal floating-point number.
        const PYTHON3_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
        );

        // PYTHON3 STRING [0134567MN]
        /// Number format to parse a Python3 float from string.
        const PYTHON3_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // PYTHON2 LITERAL [013456MN]
        /// Number format for a Python2 literal floating-point number.
        const PYTHON2_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // PYTHON2 STRING [0134567MN]
        /// Number format to parse a Python2 float from string.
        const PYTHON2_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        /// Number format for a C++ literal floating-point number.
        const CXX_LITERAL = Self::CXX17_LITERAL.bits;

        /// Number format to parse a C++ float from string.
        const CXX_STRING = Self::CXX17_STRING.bits;

        // C++17 LITERAL [01345689ABMN-']
        /// Number format for a C++17 literal floating-point number.
        const CXX17_LITERAL = (
            flags::digit_separator_to_flags(b'\'')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // C++17 STRING [013456MN]
        /// Number format for a C++17 string floating-point number.
        const CXX17_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++14 LITERAL [01345689ABMN-']
        /// Number format for a C++14 literal floating-point number.
        const CXX14_LITERAL = (
            flags::digit_separator_to_flags(b'\'')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // C++14 STRING [013456MN]
        /// Number format to parse a C++14 float from string.
        const CXX14_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++11 LITERAL [0134568MN]
        /// Number format for a C++11 literal floating-point number.
        const CXX11_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C++11 STRING [013456MN]
        /// Number format to parse a C++11 float from string.
        const CXX11_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++03 LITERAL [0134567MN]
        /// Number format for a C++03 literal floating-point number.
        const CXX03_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C++03 STRING [013456MN]
        /// Number format to parse a C++03 float from string.
        const CXX03_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C++98 LITERAL [0134567MN]
        /// Number format for a C++98 literal floating-point number.
        const CXX98_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C++98 STRING [013456MN]
        /// Number format to parse a C++98 float from string.
        const CXX98_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        /// Number format for a C literal floating-point number.
        const C_LITERAL = Self::C18_LITERAL.bits;

        /// Number format to parse a C float from string.
        const C_STRING = Self::C18_STRING.bits;

        // C18 LITERAL [0134568MN]
        /// Number format for a C18 literal floating-point number.
        const C18_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C18 STRING [013456MN]
        /// Number format to parse a C18 float from string.
        const C18_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C11 LITERAL [0134568MN]
        /// Number format for a C11 literal floating-point number.
        const C11_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C11 STRING [013456MN]
        /// Number format to parse a C11 float from string.
        const C11_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C99 LITERAL [0134568MN]
        /// Number format for a C99 literal floating-point number.
        const C99_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // C99 STRING [013456MN]
        /// Number format to parse a C99 float from string.
        const C99_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C90 LITERAL [0134567MN]
        /// Number format for a C90 literal floating-point number.
        const C90_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C90 STRING [013456MN]
        /// Number format to parse a C90 float from string.
        const C90_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // C89 LITERAL [0134567MN]
        /// Number format for a C89 literal floating-point number.
        const C89_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // C89 STRING [013456MN]
        /// Number format to parse a C89 float from string.
        const C89_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // RUBY LITERAL [345689AM-_]
        /// Number format for a Ruby literal floating-point number.
        const RUBY_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // RUBY STRING [01234569ABMN-_]
        /// Number format to parse a Ruby float from string.
        // Note: Amazingly, Ruby 1.8+ do not allow parsing special values.
        const RUBY_STRING = (
            flags::digit_separator_to_flags(b'_')
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // SWIFT LITERAL [34569ABFGHIJKMN-_]
        /// Number format for a Swift literal floating-point number.
        const SWIFT_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // SWIFT STRING [13456MN]
        /// Number format to parse a Swift float from string.
        const SWIFT_STRING = (
            Self::REQUIRED_FRACTION_DIGITS.bits
        );

        // GO LITERAL [0134567MN]
        /// Number format for a Golang literal floating-point number.
        const GO_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GO STRING [013456MN]
        /// Number format to parse a Golang float from string.
        const GO_STRING = (
            Self::REQUIRED_FRACTION_DIGITS.bits
        );

        // HASKELL LITERAL [456MN]
        /// Number format for a Haskell literal floating-point number.
        const HASKELL_LITERAL = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::NO_SPECIAL.bits
        );

        // HASKELL STRING [45678MN]
        /// Number format to parse a Haskell float from string.
        const HASKELL_STRING = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // JAVASCRIPT LITERAL [01345678M]
        /// Number format for a Javascript literal floating-point number.
        const JAVASCRIPT_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // JAVASCRIPT STRING [012345678MN]
        /// Number format to parse a Javascript float from string.
        const JAVASCRIPT_STRING = (
            Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // PERL LITERAL [0134569ABDEFGHIJKMN-_]
        /// Number format for a Perl literal floating-point number.
        const PERL_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // PERL STRING [01234567MN]
        /// Number format to parse a Perl float from string.
        const PERL_STRING = Self::PERMISSIVE.bits;

        // PHP LITERAL [01345678MN]
        /// Number format for a PHP literal floating-point number.
        const PHP_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // PHP STRING [0123456MN]
        /// Number format to parse a PHP float from string.
        const PHP_STRING = (
            Self::NO_SPECIAL.bits
        );

        // JAVA LITERAL [0134569ABIJKMN-_]
        /// Number format for a Java literal floating-point number.
        const JAVA_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // JAVA STRING [01345678MN]
        /// Number format to parse a Java float from string.
        const JAVA_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // R LITERAL [01345678MN]
        /// Number format for a R literal floating-point number.
        const R_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // R STRING [01234567MN]
        /// Number format to parse a R float from string.
        const R_STRING = Self::PERMISSIVE.bits;

        // KOTLIN LITERAL [0134569ABIJKN-_]
        /// Number format for a Kotlin literal floating-point number.
        const KOTLIN_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // KOTLIN STRING [0134568MN]
        /// Number format to parse a Kotlin float from string.
        const KOTLIN_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // JULIA LITERAL [01345689AMN-_]
        /// Number format for a Julia literal floating-point number.
        const JULIA_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTEGER_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_INTERNAL_DIGIT_SEPARATOR.bits
        );

        // JULIA STRING [01345678MN]
        /// Number format to parse a Julia float from string.
        const JULIA_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        /// Number format for a C# literal floating-point number.
        const CSHARP_LITERAL = Self::CSHARP7_LITERAL.bits;

        /// Number format to parse a C# float from string.
        const CSHARP_STRING = Self::CSHARP7_STRING.bits;

        // CSHARP7 LITERAL [034569ABIJKMN-_]
        /// Number format for a C#7 literal floating-point number.
        const CSHARP7_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // CSHARP7 STRING [0134568MN]
        /// Number format to parse a C#7 float from string.
        const CSHARP7_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP6 LITERAL [03456MN]
        /// Number format for a C#6 literal floating-point number.
        const CSHARP6_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP6 STRING [0134568MN]
        /// Number format to parse a C#6 float from string.
        const CSHARP6_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP5 LITERAL [03456MN]
        /// Number format for a C#5 literal floating-point number.
        const CSHARP5_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP5 STRING [0134568MN]
        /// Number format to parse a C#5 float from string.
        const CSHARP5_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP4 LITERAL [03456MN]
        /// Number format for a C#4 literal floating-point number.
        const CSHARP4_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP4 STRING [0134568MN]
        /// Number format to parse a C#4 float from string.
        const CSHARP4_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP3 LITERAL [03456MN]
        /// Number format for a C#3 literal floating-point number.
        const CSHARP3_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP3 STRING [0134568MN]
        /// Number format to parse a C#3 float from string.
        const CSHARP3_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP2 LITERAL [03456MN]
        /// Number format for a C#2 literal floating-point number.
        const CSHARP2_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP2 STRING [0134568MN]
        /// Number format to parse a C#2 float from string.
        const CSHARP2_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // CSHARP1 LITERAL [03456MN]
        /// Number format for a C#1 literal floating-point number.
        const CSHARP1_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CSHARP1 STRING [0134568MN]
        /// Number format to parse a C#1 float from string.
        const CSHARP1_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // KAWA LITERAL [013456MN]
        /// Number format for a Kawa literal floating-point number.
        const KAWA_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // KAWA STRING [013456MN]
        /// Number format to parse a Kawa float from string.
        const KAWA_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GAMBITC LITERAL [013456MN]
        /// Number format for a Gambit-C literal floating-point number.
        const GAMBITC_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GAMBITC STRING [013456MN]
        /// Number format to parse a Gambit-C float from string.
        const GAMBITC_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GUILE LITERAL [013456MN]
        /// Number format for a Guile literal floating-point number.
        const GUILE_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // GUILE STRING [013456MN]
        /// Number format to parse a Guile float from string.
        const GUILE_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CLOJURE LITERAL [13456MN]
        /// Number format for a Clojure literal floating-point number.
        const CLOJURE_LITERAL = (
            Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // CLOJURE STRING [01345678MN]
        /// Number format to parse a Clojure float from string.
        const CLOJURE_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // ERLANG LITERAL [34578MN]
        /// Number format for an Erlang literal floating-point number.
        const ERLANG_LITERAL = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // ERLANG STRING [345MN]
        /// Number format to parse an Erlang float from string.
        const ERLANG_STRING = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_SPECIAL.bits
        );

        // ELM LITERAL [456]
        /// Number format for an Elm literal floating-point number.
        const ELM_LITERAL = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // ELM STRING [01345678MN]
        /// Number format to parse an Elm float from string.
        // Note: There is no valid representation of NaN, just Infinity.
        const ELM_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // SCALA LITERAL [3456]
        /// Number format for a Scala literal floating-point number.
        const SCALA_LITERAL = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // SCALA STRING [01345678MN]
        /// Number format to parse a Scala float from string.
        const SCALA_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // ELIXIR LITERAL [3459ABMN-_]
        /// Number format for an Elixir literal floating-point number.
        const ELIXIR_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // ELIXIR STRING [345MN]
        /// Number format to parse an Elixir float from string.
        const ELIXIR_STRING = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_SPECIAL.bits
        );

        // FORTRAN LITERAL [013456MN]
        /// Number format for a FORTRAN literal floating-point number.
        const FORTRAN_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // FORTRAN STRING [0134567MN]
        /// Number format to parse a FORTRAN float from string.
        const FORTRAN_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // D LITERAL [0134569ABFGHIJKN-_]
        /// Number format for a D literal floating-point number.
        const D_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // D STRING [01345679AFGMN-_]
        /// Number format to parse a D float from string.
        const D_STRING = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTEGER_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::INTEGER_TRAILING_DIGIT_SEPARATOR.bits
            | Self::FRACTION_TRAILING_DIGIT_SEPARATOR.bits
        );

        // COFFEESCRIPT LITERAL [01345678]
        /// Number format for a Coffeescript literal floating-point number.
        const COFFEESCRIPT_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // COFFEESCRIPT STRING [012345678MN]
        /// Number format to parse a Coffeescript float from string.
        const COFFEESCRIPT_STRING = (
            Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // COBOL LITERAL [0345MN]
        /// Number format for a Cobol literal floating-point number.
        const COBOL_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::NO_SPECIAL.bits
        );

        // COBOL STRING [012356MN]
        /// Number format to parse a Cobol float from string.
        const COBOL_STRING = (
            Self::REQUIRED_EXPONENT_SIGN.bits
            | Self::NO_SPECIAL.bits
        );

        // FSHARP LITERAL [13456789ABIJKMN-_]
        /// Number format for a F# literal floating-point number.
        const FSHARP_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // FSHARP STRING [013456789ABCDEFGHIJKLMN-_]
        /// Number format to parse a F# float from string.
        const FSHARP_STRING = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
            | Self::SPECIAL_DIGIT_SEPARATOR.bits
        );

        // VB LITERAL [03456MN]
        /// Number format for a Visual Basic literal floating-point number.
        const VB_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // VB STRING [01345678MN]
        /// Number format to parse a Visual Basic float from string.
        // Note: To my knowledge, Visual Basic cannot parse infinity.
        const VB_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // OCAML LITERAL [1456789ABDFGHIJKMN-_]
        /// Number format for an OCaml literal floating-point number.
        const OCAML_LITERAL = (
            flags::digit_separator_to_flags(b'_')
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
        /// Number format to parse an OCaml float from string.
        const OCAML_STRING = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
            | Self::SPECIAL_DIGIT_SEPARATOR.bits
        );

        // OBJECTIVEC LITERAL [013456MN]
        /// Number format for an Objective-C literal floating-point number.
        const OBJECTIVEC_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // OBJECTIVEC STRING [013456MN]
        /// Number format to parse an Objective-C float from string.
        const OBJECTIVEC_STRING = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // REASONML LITERAL [13456789ABDFGHIJKMN-_]
        /// Number format for a ReasonML literal floating-point number.
        const REASONML_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // REASONML STRING [01345679ABCDEFGHIJKLMN-_]
        /// Number format to parse a ReasonML float from string.
        const REASONML_STRING = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
            | Self::SPECIAL_DIGIT_SEPARATOR.bits
        );

        // OCTAVE LITERAL [013456789ABDFGHIJKMN-_]
        /// Number format for an Octave literal floating-point number.
        // Note: Octave accepts both NaN and nan, Inf and inf.
        const OCTAVE_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // OCTAVE STRING [01345679ABCDEFGHIJKMN-,]
        /// Number format to parse an Octave float from string.
        const OCTAVE_STRING = (
            flags::digit_separator_to_flags(b',')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // MATLAB LITERAL [013456789ABDFGHIJKMN-_]
        /// Number format for an Matlab literal floating-point number.
        // Note: Matlab accepts both NaN and nan, Inf and inf.
        const MATLAB_LITERAL = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // MATLAB STRING [01345679ABCDEFGHIJKMN-,]
        /// Number format to parse an Matlab float from string.
        const MATLAB_STRING = (
            flags::digit_separator_to_flags(b',')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // ZIG LITERAL [1456MN]
        /// Number format for a Zig literal floating-point number.
        const ZIG_LITERAL = (
            Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::NO_SPECIAL.bits
        );

        // ZIG STRING [01234567MN]
        /// Number format to parse a Zig float from string.
        const ZIG_STRING = Self::PERMISSIVE.bits;

        // SAGE LITERAL [012345678MN]
        /// Number format for a Sage literal floating-point number.
        // Note: Both Infinity and infinity are accepted.
        const SAGE_LITERAL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // SAGE STRING [01345679ABMN-_]
        /// Number format to parse a Sage float from string.
        const SAGE_STRING = (
            flags::digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // JSON [456]
        /// Number format for a JSON literal floating-point number.
        const JSON = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::NO_SPECIAL.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // TOML [34569AB]
        /// Number format for a TOML literal floating-point number.
        const TOML = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_SPECIAL.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::NO_INTEGER_LEADING_ZEROS.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // YAML (defined in-terms of JSON schema).
        /// Number format for a YAML literal floating-point number.
        const YAML = Self::JSON.bits;

        // XML [01234578MN]
        /// Number format for a XML literal floating-point number.
        const XML = (
            Self::CASE_SENSITIVE_SPECIAL.bits
        );

        // SQLITE [013456MN]
        /// Number format for a SQLite literal floating-point number.
        const SQLITE = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // POSTGRESQL [013456MN]
        /// Number format for a PostgreSQL literal floating-point number.
        const POSTGRESQL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // MYSQL [013456MN]
        /// Number format for a MySQL literal floating-point number.
        const MYSQL = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_SPECIAL.bits
        );

        // MONGODB [01345678M]
        /// Number format for a MongoDB literal floating-point number.
        const MONGODB = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::CASE_SENSITIVE_SPECIAL.bits
            | Self::NO_FLOAT_LEADING_ZEROS.bits
        );

        // HIDDEN DEFAULTS AND INTERFACES

        /// Number format when no flags are set.
        #[doc(hidden)]
        const PERMISSIVE = 0;

        /// Permissive interface float format flags.
        #[doc(hidden)]
        const PERMISSIVE_INTERFACE = Self::PERMISSIVE.bits & Self::INTERFACE_FLAG_MASK.bits;

        /// Standard float format.
        #[doc(hidden)]
        const STANDARD = Self::RUST_STRING.bits;

        /// Standard interface float format flags.
        #[doc(hidden)]
        const STANDARD_INTERFACE = Self::STANDARD.bits & Self::INTERFACE_FLAG_MASK.bits;

        /// Number format when all digit separator flags are set.
        #[doc(hidden)]
        const IGNORE = (
            flags::digit_separator_to_flags(b'_')
            | Self::DIGIT_SEPARATOR_FLAG_MASK.bits
        );

        /// Ignore interface float format flags.
        #[doc(hidden)]
        const IGNORE_INTERFACE = Self::IGNORE.bits & Self::INTERFACE_FLAG_MASK.bits;
    }
}

impl SyntaxFormat {
    // CONSTRUCTORS

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

    // FLAGS

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

    // DIGIT SEPARATOR

    /// Get the digit separator for the number format.
    #[inline(always)]
    pub const fn digit_separator(self) -> u8 {
        flags::digit_separator_from_flags(self.bits)
    }

    // NON-DIGIT SEPARATOR FLAGS & MASKS

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

    /// Get if exponent notation is required.
    #[inline(always)]
    pub const fn required_exponent_notation(self) -> bool {
        self.intersects(Self::REQUIRED_EXPONENT_NOTATION)
    }

    /// Get if exponent characters are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_exponent(self) -> bool {
        self.intersects(Self::CASE_SENSITIVE_EXPONENT)
    }

    /// Get if base prefixes are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_base_prefix(self) -> bool {
        self.intersects(Self::CASE_SENSITIVE_BASE_PREFIX)
    }

    /// Get if base suffixes are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_base_suffix(self) -> bool {
        self.intersects(Self::CASE_SENSITIVE_BASE_SUFFIX)
    }

    // DIGIT SEPARATOR FLAGS & MASKS

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
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignore() {
        let flag = SyntaxFormat::IGNORE;
        let flag = flag | SyntaxFormat::from_digit_separator(b'_');
        assert_eq!(flag.flags(), SyntaxFormat::DIGIT_SEPARATOR_FLAG_MASK);
        assert_eq!(flag.digit_separator(), b'_');
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
        assert_eq!(flag.required_exponent_notation(), false);
        assert_eq!(flag.case_sensitive_exponent(), false);
        assert_eq!(flag.case_sensitive_base_prefix(), false);
        assert_eq!(flag.case_sensitive_base_suffix(), false);
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
    }

    #[test]
    fn test_properties() {
        let flag = SyntaxFormat::STANDARD;
        assert_eq!(flag.flags(), flag);
        assert_eq!(flag.interface_flags(), flag);
        assert_eq!(flag.digit_separator(), b'\x00');
        assert_eq!(flag.required_integer_digits(), false);
        assert_eq!(flag.required_fraction_digits(), false);
        assert_eq!(flag.required_exponent_digits(), true);
        assert_eq!(flag.required_digits(), true);
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
        assert_eq!(flag.no_exponent_notation(), false);
        assert_eq!(flag.required_exponent_notation(), false);
        assert_eq!(flag.case_sensitive_exponent(), false);
        assert_eq!(flag.case_sensitive_base_prefix(), false);
        assert_eq!(flag.case_sensitive_base_suffix(), false);
        assert_eq!(flag.integer_internal_digit_separator(), false);
        assert_eq!(flag.fraction_internal_digit_separator(), false);
        assert_eq!(flag.exponent_internal_digit_separator(), false);
        assert_eq!(flag.internal_digit_separator(), false);
        assert_eq!(flag.integer_leading_digit_separator(), false);
        assert_eq!(flag.fraction_leading_digit_separator(), false);
        assert_eq!(flag.exponent_leading_digit_separator(), false);
        assert_eq!(flag.leading_digit_separator(), false);
        assert_eq!(flag.integer_trailing_digit_separator(), false);
        assert_eq!(flag.fraction_trailing_digit_separator(), false);
        assert_eq!(flag.exponent_trailing_digit_separator(), false);
        assert_eq!(flag.trailing_digit_separator(), false);
        assert_eq!(flag.integer_consecutive_digit_separator(), false);
        assert_eq!(flag.fraction_consecutive_digit_separator(), false);
        assert_eq!(flag.exponent_consecutive_digit_separator(), false);
        assert_eq!(flag.consecutive_digit_separator(), false);
        assert_eq!(flag.special_digit_separator(), false);
    }

    #[test]
    fn test_flags() {
        let flags = [
            SyntaxFormat::REQUIRED_INTEGER_DIGITS,
            SyntaxFormat::REQUIRED_FRACTION_DIGITS,
            SyntaxFormat::REQUIRED_EXPONENT_DIGITS,
            SyntaxFormat::NO_POSITIVE_MANTISSA_SIGN,
            SyntaxFormat::REQUIRED_MANTISSA_SIGN,
            SyntaxFormat::NO_EXPONENT_NOTATION,
            SyntaxFormat::NO_POSITIVE_EXPONENT_SIGN,
            SyntaxFormat::REQUIRED_EXPONENT_SIGN,
            SyntaxFormat::NO_EXPONENT_WITHOUT_FRACTION,
            SyntaxFormat::NO_SPECIAL,
            SyntaxFormat::CASE_SENSITIVE_SPECIAL,
            SyntaxFormat::NO_INTEGER_LEADING_ZEROS,
            SyntaxFormat::NO_FLOAT_LEADING_ZEROS,
            SyntaxFormat::REQUIRED_EXPONENT_NOTATION,
            SyntaxFormat::INTEGER_INTERNAL_DIGIT_SEPARATOR,
            SyntaxFormat::FRACTION_INTERNAL_DIGIT_SEPARATOR,
            SyntaxFormat::EXPONENT_INTERNAL_DIGIT_SEPARATOR,
            SyntaxFormat::INTEGER_LEADING_DIGIT_SEPARATOR,
            SyntaxFormat::FRACTION_LEADING_DIGIT_SEPARATOR,
            SyntaxFormat::EXPONENT_LEADING_DIGIT_SEPARATOR,
            SyntaxFormat::INTEGER_TRAILING_DIGIT_SEPARATOR,
            SyntaxFormat::FRACTION_TRAILING_DIGIT_SEPARATOR,
            SyntaxFormat::EXPONENT_TRAILING_DIGIT_SEPARATOR,
            SyntaxFormat::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR,
            SyntaxFormat::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR,
            SyntaxFormat::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR,
            SyntaxFormat::SPECIAL_DIGIT_SEPARATOR,
        ];
        for &flag in flags.iter() {
            assert_eq!(flag.flags(), flag);
            assert_eq!(flag.digit_separator(), 0);
        }
    }

    #[test]
    fn test_constants() {
        let flags = [
            SyntaxFormat::RUST_LITERAL,
            SyntaxFormat::RUST_STRING,
            SyntaxFormat::RUST_STRING_STRICT,
            SyntaxFormat::PYTHON_LITERAL,
            SyntaxFormat::PYTHON_STRING,
            SyntaxFormat::CXX17_LITERAL,
            SyntaxFormat::CXX17_STRING,
            SyntaxFormat::CXX14_LITERAL,
            SyntaxFormat::CXX14_STRING,
            SyntaxFormat::CXX11_LITERAL,
            SyntaxFormat::CXX11_STRING,
            SyntaxFormat::CXX03_LITERAL,
            SyntaxFormat::CXX03_STRING,
            SyntaxFormat::CXX98_LITERAL,
            SyntaxFormat::CXX98_STRING,
            SyntaxFormat::C18_LITERAL,
            SyntaxFormat::C18_STRING,
            SyntaxFormat::C11_LITERAL,
            SyntaxFormat::C11_STRING,
            SyntaxFormat::C99_LITERAL,
            SyntaxFormat::C99_STRING,
            SyntaxFormat::C90_LITERAL,
            SyntaxFormat::C90_STRING,
            SyntaxFormat::C89_LITERAL,
            SyntaxFormat::C89_STRING,
            SyntaxFormat::RUBY_LITERAL,
            SyntaxFormat::RUBY_STRING,
            SyntaxFormat::SWIFT_LITERAL,
            SyntaxFormat::SWIFT_STRING,
            SyntaxFormat::GO_LITERAL,
            SyntaxFormat::GO_STRING,
            SyntaxFormat::HASKELL_LITERAL,
            SyntaxFormat::HASKELL_STRING,
            SyntaxFormat::JAVASCRIPT_LITERAL,
            SyntaxFormat::JAVASCRIPT_STRING,
            SyntaxFormat::PERL_LITERAL,
            SyntaxFormat::PERL_STRING,
            SyntaxFormat::PHP_LITERAL,
            SyntaxFormat::PHP_STRING,
            SyntaxFormat::JAVA_LITERAL,
            SyntaxFormat::JAVA_STRING,
            SyntaxFormat::R_LITERAL,
            SyntaxFormat::R_STRING,
            SyntaxFormat::KOTLIN_LITERAL,
            SyntaxFormat::KOTLIN_STRING,
            SyntaxFormat::JULIA_LITERAL,
            SyntaxFormat::JULIA_STRING,
            SyntaxFormat::CSHARP7_LITERAL,
            SyntaxFormat::CSHARP7_STRING,
            SyntaxFormat::CSHARP6_LITERAL,
            SyntaxFormat::CSHARP6_STRING,
            SyntaxFormat::CSHARP5_LITERAL,
            SyntaxFormat::CSHARP5_STRING,
            SyntaxFormat::CSHARP4_LITERAL,
            SyntaxFormat::CSHARP4_STRING,
            SyntaxFormat::CSHARP3_LITERAL,
            SyntaxFormat::CSHARP3_STRING,
            SyntaxFormat::CSHARP2_LITERAL,
            SyntaxFormat::CSHARP2_STRING,
            SyntaxFormat::CSHARP1_LITERAL,
            SyntaxFormat::CSHARP1_STRING,
            SyntaxFormat::KAWA_LITERAL,
            SyntaxFormat::KAWA_STRING,
            SyntaxFormat::GAMBITC_LITERAL,
            SyntaxFormat::GAMBITC_STRING,
            SyntaxFormat::GUILE_LITERAL,
            SyntaxFormat::GUILE_STRING,
            SyntaxFormat::CLOJURE_LITERAL,
            SyntaxFormat::CLOJURE_STRING,
            SyntaxFormat::ERLANG_LITERAL,
            SyntaxFormat::ERLANG_STRING,
            SyntaxFormat::ELM_LITERAL,
            SyntaxFormat::ELM_STRING,
            SyntaxFormat::SCALA_LITERAL,
            SyntaxFormat::SCALA_STRING,
            SyntaxFormat::ELIXIR_LITERAL,
            SyntaxFormat::ELIXIR_STRING,
            SyntaxFormat::FORTRAN_LITERAL,
            SyntaxFormat::FORTRAN_STRING,
            SyntaxFormat::D_LITERAL,
            SyntaxFormat::D_STRING,
            SyntaxFormat::COFFEESCRIPT_LITERAL,
            SyntaxFormat::COFFEESCRIPT_STRING,
            SyntaxFormat::COBOL_LITERAL,
            SyntaxFormat::COBOL_STRING,
            SyntaxFormat::FSHARP_LITERAL,
            SyntaxFormat::FSHARP_STRING,
            SyntaxFormat::VB_LITERAL,
            SyntaxFormat::VB_STRING,
            SyntaxFormat::OCAML_LITERAL,
            SyntaxFormat::OCAML_STRING,
            SyntaxFormat::OBJECTIVEC_LITERAL,
            SyntaxFormat::OBJECTIVEC_STRING,
            SyntaxFormat::REASONML_LITERAL,
            SyntaxFormat::REASONML_STRING,
            SyntaxFormat::OCTAVE_LITERAL,
            SyntaxFormat::OCTAVE_STRING,
            SyntaxFormat::MATLAB_LITERAL,
            SyntaxFormat::MATLAB_STRING,
            SyntaxFormat::ZIG_LITERAL,
            SyntaxFormat::ZIG_STRING,
            SyntaxFormat::SAGE_LITERAL,
            SyntaxFormat::SAGE_STRING,
            SyntaxFormat::JSON,
            SyntaxFormat::TOML,
            SyntaxFormat::YAML,
            SyntaxFormat::XML,
            SyntaxFormat::SQLITE,
            SyntaxFormat::POSTGRESQL,
            SyntaxFormat::MYSQL,
            SyntaxFormat::MONGODB,
        ];
        for &flag in flags.iter() {
            // Just wanna check the flags are defined.
            assert!((flag.bits == 0) | true);
            assert!((flag.digit_separator() == 0) | true);
        }
    }
}
