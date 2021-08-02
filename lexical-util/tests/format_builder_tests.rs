use lexical_util::format::{NumberFormat, NumberFormatBuilder};

#[test]
fn decimal_test() {
    const FORMAT: u128 = NumberFormatBuilder::decimal();
    let format = NumberFormat::<FORMAT> {};
    assert!(format.is_valid());
    assert_eq!(format.radix(), 10);
    assert_eq!(format.mantissa_radix(), 10);
    assert_eq!(format.exponent_base(), 10);
    assert_eq!(format.exponent_radix(), 10);
}

#[test]
#[cfg(feature = "power-of-two")]
fn binary_test() {
    const FORMAT: u128 = NumberFormatBuilder::binary();
    let format = NumberFormat::<FORMAT> {};
    assert!(format.is_valid());
    assert_eq!(format.radix(), 2);
    assert_eq!(format.mantissa_radix(), 2);
    assert_eq!(format.exponent_base(), 2);
    assert_eq!(format.exponent_radix(), 2);
}

#[test]
#[cfg(feature = "power-of-two")]
fn octal_test() {
    const FORMAT: u128 = NumberFormatBuilder::octal();
    let format = NumberFormat::<FORMAT> {};
    assert!(format.is_valid());
    assert_eq!(format.radix(), 8);
    assert_eq!(format.mantissa_radix(), 8);
    assert_eq!(format.exponent_base(), 8);
    assert_eq!(format.exponent_radix(), 8);
}

#[test]
#[cfg(feature = "power-of-two")]
fn hexadecimal_test() {
    const FORMAT: u128 = NumberFormatBuilder::hexadecimal();
    let format = NumberFormat::<FORMAT> {};
    assert!(format.is_valid());
    assert_eq!(format.radix(), 16);
    assert_eq!(format.mantissa_radix(), 16);
    assert_eq!(format.exponent_base(), 16);
    assert_eq!(format.exponent_radix(), 16);
}

#[test]
#[cfg(feature = "power-of-two")]
fn from_radix_test() {
    const FORMAT: u128 = NumberFormatBuilder::from_radix(32);
    let format = NumberFormat::<FORMAT> {};
    assert!(format.is_valid());
    assert_eq!(format.radix(), 32);
    assert_eq!(format.mantissa_radix(), 32);
    assert_eq!(format.exponent_base(), 32);
    assert_eq!(format.exponent_radix(), 32);
}

// TODO(ahuszagh) Restore
//#[test]
//fn test_ignore() {
//    let flag = NumberFormat::IGNORE;
//    let flag = flag | NumberFormat::from_digit_separator(b'_');
//    assert_eq!(flag.flags(), NumberFormat::DIGIT_SEPARATOR_FLAG_MASK);
//    assert_eq!(flag.digit_separator(), b'_');
//    assert_eq!(flag.decimal_point(), b'.');
//    assert_eq!(flag.exponent_decimal(), b'e');
//    assert_eq!(flag.required_integer_digits(), false);
//    assert_eq!(flag.required_fraction_digits(), false);
//    assert_eq!(flag.required_exponent_digits(), false);
//    assert_eq!(flag.required_digits(), false);
//    assert_eq!(flag.no_positive_mantissa_sign(), false);
//    assert_eq!(flag.required_mantissa_sign(), false);
//    assert_eq!(flag.no_exponent_notation(), false);
//    assert_eq!(flag.no_positive_exponent_sign(), false);
//    assert_eq!(flag.required_exponent_sign(), false);
//    assert_eq!(flag.no_exponent_without_fraction(), false);
//    assert_eq!(flag.no_special(), false);
//    assert_eq!(flag.case_sensitive_special(), false);
//    assert_eq!(flag.no_integer_leading_zeros(), false);
//    assert_eq!(flag.no_float_leading_zeros(), false);
//    assert_eq!(flag.required_exponent_notation(), false);
//    assert_eq!(flag.integer_internal_digit_separator(), true);
//    assert_eq!(flag.fraction_internal_digit_separator(), true);
//    assert_eq!(flag.exponent_internal_digit_separator(), true);
//    assert_eq!(flag.internal_digit_separator(), true);
//    assert_eq!(flag.integer_leading_digit_separator(), true);
//    assert_eq!(flag.fraction_leading_digit_separator(), true);
//    assert_eq!(flag.exponent_leading_digit_separator(), true);
//    assert_eq!(flag.leading_digit_separator(), true);
//    assert_eq!(flag.integer_trailing_digit_separator(), true);
//    assert_eq!(flag.fraction_trailing_digit_separator(), true);
//    assert_eq!(flag.exponent_trailing_digit_separator(), true);
//    assert_eq!(flag.trailing_digit_separator(), true);
//    assert_eq!(flag.integer_consecutive_digit_separator(), true);
//    assert_eq!(flag.fraction_consecutive_digit_separator(), true);
//    assert_eq!(flag.exponent_consecutive_digit_separator(), true);
//    assert_eq!(flag.consecutive_digit_separator(), true);
//    assert_eq!(flag.special_digit_separator(), true);
//
//    #[cfg(feature = "power_of_two")]
//    assert_eq!(flag.exponent_backup(), b'^');
//}
//
//#[test]
//fn test_flags() {
//    let flags = [
//        NumberFormat::REQUIRED_INTEGER_DIGITS,
//        NumberFormat::REQUIRED_FRACTION_DIGITS,
//        NumberFormat::REQUIRED_EXPONENT_DIGITS,
//        NumberFormat::NO_POSITIVE_MANTISSA_SIGN,
//        NumberFormat::REQUIRED_MANTISSA_SIGN,
//        NumberFormat::NO_EXPONENT_NOTATION,
//        NumberFormat::NO_POSITIVE_EXPONENT_SIGN,
//        NumberFormat::REQUIRED_EXPONENT_SIGN,
//        NumberFormat::NO_EXPONENT_WITHOUT_FRACTION,
//        NumberFormat::NO_SPECIAL,
//        NumberFormat::CASE_SENSITIVE_SPECIAL,
//        NumberFormat::NO_INTEGER_LEADING_ZEROS,
//        NumberFormat::NO_FLOAT_LEADING_ZEROS,
//        NumberFormat::REQUIRED_EXPONENT_NOTATION,
//        NumberFormat::INTEGER_INTERNAL_DIGIT_SEPARATOR,
//        NumberFormat::FRACTION_INTERNAL_DIGIT_SEPARATOR,
//        NumberFormat::EXPONENT_INTERNAL_DIGIT_SEPARATOR,
//        NumberFormat::INTEGER_LEADING_DIGIT_SEPARATOR,
//        NumberFormat::FRACTION_LEADING_DIGIT_SEPARATOR,
//        NumberFormat::EXPONENT_LEADING_DIGIT_SEPARATOR,
//        NumberFormat::INTEGER_TRAILING_DIGIT_SEPARATOR,
//        NumberFormat::FRACTION_TRAILING_DIGIT_SEPARATOR,
//        NumberFormat::EXPONENT_TRAILING_DIGIT_SEPARATOR,
//        NumberFormat::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR,
//        NumberFormat::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR,
//        NumberFormat::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR,
//        NumberFormat::SPECIAL_DIGIT_SEPARATOR,
//    ];
//    for &flag in flags.iter() {
//        assert_eq!(flag.flags(), flag);
//        assert_eq!(flag.digit_separator(), 0);
//    }
//}
//
//#[test]
//fn test_constants() {
//    let flags = [
//        NumberFormat::RUST_LITERAL,
//        NumberFormat::RUST_STRING,
//        NumberFormat::RUST_STRING_STRICT,
//        NumberFormat::PYTHON_LITERAL,
//        NumberFormat::PYTHON_STRING,
//        NumberFormat::CXX17_LITERAL,
//        NumberFormat::CXX17_STRING,
//        NumberFormat::CXX14_LITERAL,
//        NumberFormat::CXX14_STRING,
//        NumberFormat::CXX11_LITERAL,
//        NumberFormat::CXX11_STRING,
//        NumberFormat::CXX03_LITERAL,
//        NumberFormat::CXX03_STRING,
//        NumberFormat::CXX98_LITERAL,
//        NumberFormat::CXX98_STRING,
//        NumberFormat::C18_LITERAL,
//        NumberFormat::C18_STRING,
//        NumberFormat::C11_LITERAL,
//        NumberFormat::C11_STRING,
//        NumberFormat::C99_LITERAL,
//        NumberFormat::C99_STRING,
//        NumberFormat::C90_LITERAL,
//        NumberFormat::C90_STRING,
//        NumberFormat::C89_LITERAL,
//        NumberFormat::C89_STRING,
//        NumberFormat::RUBY_LITERAL,
//        NumberFormat::RUBY_STRING,
//        NumberFormat::SWIFT_LITERAL,
//        NumberFormat::SWIFT_STRING,
//        NumberFormat::GO_LITERAL,
//        NumberFormat::GO_STRING,
//        NumberFormat::HASKELL_LITERAL,
//        NumberFormat::HASKELL_STRING,
//        NumberFormat::JAVASCRIPT_LITERAL,
//        NumberFormat::JAVASCRIPT_STRING,
//        NumberFormat::PERL_LITERAL,
//        NumberFormat::PERL_STRING,
//        NumberFormat::PHP_LITERAL,
//        NumberFormat::PHP_STRING,
//        NumberFormat::JAVA_LITERAL,
//        NumberFormat::JAVA_STRING,
//        NumberFormat::R_LITERAL,
//        NumberFormat::R_STRING,
//        NumberFormat::KOTLIN_LITERAL,
//        NumberFormat::KOTLIN_STRING,
//        NumberFormat::JULIA_LITERAL,
//        NumberFormat::JULIA_STRING,
//        NumberFormat::CSHARP7_LITERAL,
//        NumberFormat::CSHARP7_STRING,
//        NumberFormat::CSHARP6_LITERAL,
//        NumberFormat::CSHARP6_STRING,
//        NumberFormat::CSHARP5_LITERAL,
//        NumberFormat::CSHARP5_STRING,
//        NumberFormat::CSHARP4_LITERAL,
//        NumberFormat::CSHARP4_STRING,
//        NumberFormat::CSHARP3_LITERAL,
//        NumberFormat::CSHARP3_STRING,
//        NumberFormat::CSHARP2_LITERAL,
//        NumberFormat::CSHARP2_STRING,
//        NumberFormat::CSHARP1_LITERAL,
//        NumberFormat::CSHARP1_STRING,
//        NumberFormat::KAWA_LITERAL,
//        NumberFormat::KAWA_STRING,
//        NumberFormat::GAMBITC_LITERAL,
//        NumberFormat::GAMBITC_STRING,
//        NumberFormat::GUILE_LITERAL,
//        NumberFormat::GUILE_STRING,
//        NumberFormat::CLOJURE_LITERAL,
//        NumberFormat::CLOJURE_STRING,
//        NumberFormat::ERLANG_LITERAL,
//        NumberFormat::ERLANG_STRING,
//        NumberFormat::ELM_LITERAL,
//        NumberFormat::ELM_STRING,
//        NumberFormat::SCALA_LITERAL,
//        NumberFormat::SCALA_STRING,
//        NumberFormat::ELIXIR_LITERAL,
//        NumberFormat::ELIXIR_STRING,
//        NumberFormat::FORTRAN_LITERAL,
//        NumberFormat::FORTRAN_STRING,
//        NumberFormat::D_LITERAL,
//        NumberFormat::D_STRING,
//        NumberFormat::COFFEESCRIPT_LITERAL,
//        NumberFormat::COFFEESCRIPT_STRING,
//        NumberFormat::COBOL_LITERAL,
//        NumberFormat::COBOL_STRING,
//        NumberFormat::FSHARP_LITERAL,
//        NumberFormat::FSHARP_STRING,
//        NumberFormat::VB_LITERAL,
//        NumberFormat::VB_STRING,
//        NumberFormat::OCAML_LITERAL,
//        NumberFormat::OCAML_STRING,
//        NumberFormat::OBJECTIVEC_LITERAL,
//        NumberFormat::OBJECTIVEC_STRING,
//        NumberFormat::REASONML_LITERAL,
//        NumberFormat::REASONML_STRING,
//        NumberFormat::OCTAVE_LITERAL,
//        NumberFormat::OCTAVE_STRING,
//        NumberFormat::MATLAB_LITERAL,
//        NumberFormat::MATLAB_STRING,
//        NumberFormat::ZIG_LITERAL,
//        NumberFormat::ZIG_STRING,
//        NumberFormat::SAGE_LITERAL,
//        NumberFormat::SAGE_STRING,
//        NumberFormat::JSON,
//        NumberFormat::TOML,
//        NumberFormat::YAML,
//        NumberFormat::XML,
//        NumberFormat::SQLITE,
//        NumberFormat::POSTGRESQL,
//        NumberFormat::MYSQL,
//        NumberFormat::MONGODB,
//    ];
//    for &flag in flags.iter() {
//        // Just wanna check the flags are defined.
//        assert!((flag.bits == 0) | true);
//        assert!((flag.digit_separator() == 0) | true);
//        // Check these values are properly set.
//        assert_eq!(flag.decimal_point(), b'.');
//        assert_eq!(flag.exponent_decimal(), b'e');
//        assert_eq!(flag.exponent_backup(), b'^');
//    }
//}
//
//#[test]
//fn test_builder() {
//    // Test a few invalid ones.
//    let flag = NumberFormat::builder().exponent_decimal(b'.').build();
//    assert_eq!(flag, None);
//
//    // Test a few valid ones.
//    let flag = NumberFormat::builder().decimal_point(b'.').build();
//    assert!(flag.is_some());
//    let flag = flag.unwrap();
//    assert_eq!(flag.digit_separator(), b'\x00');
//    assert_eq!(flag.decimal_point(), b'.');
//    assert_eq!(flag.exponent_decimal(), b'e');
//    assert_eq!(flag.exponent_backup(), b'^');
//    assert_eq!(flag.required_integer_digits(), false);
//    assert_eq!(flag.required_fraction_digits(), false);
//    assert_eq!(flag.required_exponent_digits(), false);
//}
//
//#[test]
//fn test_rebuild() {
//    let flag = NumberFormat::CSHARP7_LITERAL;
//    let rebuilt = flag.rebuild().decimal_point(b',').build().unwrap();
//    assert_eq!(flag.digit_separator(), b'_');
//    assert_eq!(rebuilt.digit_separator(), b'_');
//    assert_eq!(rebuilt.flags(), flag.flags());
//    assert_eq!(flag.decimal_point(), b'.');
//    assert_eq!(rebuilt.decimal_point(), b',');
//}

// TODO(ahuszagh) Restore...
//#[test]
//fn test_properties() {
//    let flag = NumberFormat::STANDARD;
//    assert_eq!(flag.flags(), flag);
//    assert_eq!(flag.interface_flags(), flag);
//    assert_eq!(flag.digit_separator(), b'\x00');
//    assert_eq!(flag.decimal_point(), b'.');
//    assert_eq!(flag.exponent_decimal(), b'e');
//    assert_eq!(flag.required_integer_digits(), false);
//    assert_eq!(flag.required_fraction_digits(), false);
//    assert_eq!(flag.required_exponent_digits(), true);
//    assert_eq!(flag.required_digits(), false);
//    assert_eq!(flag.no_positive_mantissa_sign(), false);
//    assert_eq!(flag.required_mantissa_sign(), false);
//    assert_eq!(flag.no_exponent_notation(), false);
//    assert_eq!(flag.no_positive_exponent_sign(), false);
//    assert_eq!(flag.required_exponent_sign(), false);
//    assert_eq!(flag.no_exponent_without_fraction(), false);
//    assert_eq!(flag.no_special(), false);
//    assert_eq!(flag.case_sensitive_special(), false);
//    assert_eq!(flag.no_integer_leading_zeros(), false);
//    assert_eq!(flag.no_float_leading_zeros(), false);
//    assert_eq!(flag.no_exponent_notation(), false);
//    assert_eq!(flag.integer_internal_digit_separator(), false);
//    assert_eq!(flag.fraction_internal_digit_separator(), false);
//    assert_eq!(flag.exponent_internal_digit_separator(), false);
//    assert_eq!(flag.internal_digit_separator(), false);
//    assert_eq!(flag.integer_leading_digit_separator(), false);
//    assert_eq!(flag.fraction_leading_digit_separator(), false);
//    assert_eq!(flag.exponent_leading_digit_separator(), false);
//    assert_eq!(flag.leading_digit_separator(), false);
//    assert_eq!(flag.integer_trailing_digit_separator(), false);
//    assert_eq!(flag.fraction_trailing_digit_separator(), false);
//    assert_eq!(flag.exponent_trailing_digit_separator(), false);
//    assert_eq!(flag.trailing_digit_separator(), false);
//    assert_eq!(flag.integer_consecutive_digit_separator(), false);
//    assert_eq!(flag.fraction_consecutive_digit_separator(), false);
//    assert_eq!(flag.exponent_consecutive_digit_separator(), false);
//    assert_eq!(flag.consecutive_digit_separator(), false);
//    assert_eq!(flag.special_digit_separator(), false);
//
//    #[cfg(feature = "power_of_two")]
//    assert_eq!(flag.exponent_backup(), b'^');
//}
//
//#[test]
//fn test_builder() {
//    // Test a few invalid ones.
//    let flag = NumberFormat::builder().exponent_decimal(b'.').build();
//    assert_eq!(flag, None);
//
//    // Test a few valid ones.
//    let flag = NumberFormat::builder().decimal_point(b'.').build();
//    assert!(flag.is_some());
//    let flag = flag.unwrap();
//    assert_eq!(flag.decimal_point(), b'.');
//    assert_eq!(flag.exponent_decimal(), b'e');
//    assert_eq!(flag.exponent_backup(), b'^');
//}
//
//#[test]
//fn test_rebuild() {
//    let flag = NumberFormat::STANDARD;
//    let flag = flag.rebuild().decimal_point(b',').build().unwrap();
//    assert_eq!(flag.decimal_point(), b',');
//    assert_eq!(flag.exponent_decimal(), b'e');
//    assert_eq!(flag.exponent_backup(), b'^');
//
//    let flag = flag.rebuild().exponent_decimal(b'f').build().unwrap();
//    assert_eq!(flag.decimal_point(), b',');
//    assert_eq!(flag.exponent_decimal(), b'f');
//    assert_eq!(flag.exponent_backup(), b'^');
//
//    let flag = flag.rebuild().exponent_backup(b'$').build().unwrap();
//    assert_eq!(flag.decimal_point(), b',');
//    assert_eq!(flag.exponent_decimal(), b'f');
//    assert_eq!(flag.exponent_backup(), b'$');
//}
