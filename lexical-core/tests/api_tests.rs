#[test]
#[cfg(feature = "write-integers")]
fn integer_to_string_test() {
    let mut buffer = [b'0'; lexical_core::BUFFER_SIZE];
    assert_eq!(lexical_core::write(12345u32, &mut buffer), b"12345");
    const OPTIONS: lexical_write_integer::Options = lexical_core::WriteIntegerOptions::new();
    const FORMAT: u128 = lexical_core::format::STANDARD;
    assert_eq!(
        lexical_core::write_with_options::<_, FORMAT>(12345u32, &mut buffer, &OPTIONS),
        b"12345"
    );
}

#[test]
#[cfg(feature = "write-floats")]
fn float_to_string_test() {
    let mut buffer = [b'0'; lexical_core::BUFFER_SIZE];
    assert_eq!(lexical_core::write(12345.0f32, &mut buffer), b"12345.0");
    const OPTIONS: lexical_write_float::Options = lexical_core::WriteFloatOptions::new();
    const FORMAT: u128 = lexical_core::format::STANDARD;
    assert_eq!(
        lexical_core::write_with_options::<_, FORMAT>(12345.0f32, &mut buffer, &OPTIONS),
        b"12345.0"
    );
}

#[test]
#[cfg(feature = "parse-integers")]
fn string_to_integer_test() {
    assert_eq!(lexical_core::parse(b"12345"), Ok(12345u32));
    assert_eq!(lexical_core::parse_partial(b"12345"), Ok((12345u32, 5)));

    const OPTIONS: lexical_parse_integer::Options = lexical_core::ParseIntegerOptions::new();
    const FORMAT: u128 = lexical_core::format::STANDARD;
    assert_eq!(lexical_core::parse_with_options::<_, FORMAT>(b"12345", &OPTIONS), Ok(12345u32));
    assert_eq!(
        lexical_core::parse_partial_with_options::<_, FORMAT>(b"12345", &OPTIONS),
        Ok((12345u32, 5))
    );
}

#[test]
#[cfg(feature = "parse-floats")]
fn string_to_float_test() {
    assert_eq!(lexical_core::parse(b"12345.0"), Ok(12345.0f32));
    assert_eq!(lexical_core::parse_partial(b"12345.0"), Ok((12345.0f32, 7)));

    const OPTIONS: lexical_parse_float::Options = lexical_core::ParseFloatOptions::new();
    const FORMAT: u128 = lexical_core::format::STANDARD;
    assert_eq!(lexical_core::parse_with_options::<_, FORMAT>(b"12345.0", &OPTIONS), Ok(12345.0f32));
    assert_eq!(
        lexical_core::parse_partial_with_options::<_, FORMAT>(b"12345.0", &OPTIONS),
        Ok((12345.0f32, 7))
    );
}

/// Test that converting the specified value into a buffer of FORMATTED_SIZE
/// yields the expected string
#[cfg(feature = "write-integers")]
macro_rules! test_format {
    ($t:ty, $value:expr, $expected:expr) => {{
        use lexical_core::FormattedSize;
        let mut buffer = [b'0'; <$t>::FORMATTED_SIZE];
        let formatted = lexical_core::write($value, &mut buffer);
        assert_eq!(
            formatted,
            $expected.as_bytes(),
            "formatted: {}, expected: {}",
            String::from_utf8_lossy(formatted),
            $expected
        );
    }};
}

#[test]
#[cfg(feature = "write-integers")]
fn numeric_limit_string_tests() {
    test_format!(u8, u8::MIN, "0");
    test_format!(u8, u8::MAX, "255");
    test_format!(u16, u16::MIN, "0");
    test_format!(u16, u16::MAX, "65535");
    test_format!(u32, u32::MIN, "0");
    test_format!(u32, u32::MAX, "4294967295");
    test_format!(u64, u64::MIN, "0");
    test_format!(u64, u64::MAX, "18446744073709551615");
    test_format!(i8, i8::MIN, "-128");
    test_format!(i8, i8::MAX, "127");
    test_format!(i16, i16::MIN, "-32768");
    test_format!(i16, i16::MAX, "32767");
    test_format!(i32, i32::MIN, "-2147483648");
    test_format!(i32, i32::MAX, "2147483647");
    test_format!(i64, i64::MIN, "-9223372036854775808");
    test_format!(i64, i64::MAX, "9223372036854775807");
}
