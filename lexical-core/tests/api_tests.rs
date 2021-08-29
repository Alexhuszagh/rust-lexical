#[test]
#[cfg(feature = "write-integers")]
fn integer_to_string_test() {
    let mut buffer = [b'0'; lexical_core::BUFFER_SIZE];
    assert_eq!(lexical_core::write(12345u32, &mut buffer), b"12345");
    let options = lexical_core::WriteIntegerOptions::new();
    const FORMAT: u128 = lexical_core::format::STANDARD;
    assert_eq!(
        lexical_core::write_with_options::<_, FORMAT>(12345u32, &mut buffer, &options),
        b"12345"
    );
}

#[test]
#[cfg(feature = "write-floats")]
fn float_to_string_test() {
    let mut buffer = [b'0'; lexical_core::BUFFER_SIZE];
    assert_eq!(lexical_core::write(12345.0f32, &mut buffer), b"12345.0");
    let options = lexical_core::WriteFloatOptions::new();
    const FORMAT: u128 = lexical_core::format::STANDARD;
    assert_eq!(
        lexical_core::write_with_options::<_, FORMAT>(12345.0f32, &mut buffer, &options),
        b"12345.0"
    );
}

#[test]
#[cfg(feature = "parse-integers")]
fn string_to_integer_test() {
    assert_eq!(lexical_core::parse(b"12345"), Ok(12345u32));
    assert_eq!(lexical_core::parse_partial(b"12345"), Ok((12345u32, 5)));

    let options = lexical_core::ParseIntegerOptions::new();
    const FORMAT: u128 = lexical_core::format::STANDARD;
    assert_eq!(lexical_core::parse_with_options::<_, FORMAT>(b"12345", &options), Ok(12345u32));
    assert_eq!(
        lexical_core::parse_partial_with_options::<_, FORMAT>(b"12345", &options),
        Ok((12345u32, 5))
    );
}

#[test]
#[cfg(feature = "parse-floats")]
fn string_to_float_test() {
    assert_eq!(lexical_core::parse(b"12345.0"), Ok(12345.0f32));
    assert_eq!(lexical_core::parse_partial(b"12345.0"), Ok((12345.0f32, 7)));

    let options = lexical_core::ParseFloatOptions::new();
    const FORMAT: u128 = lexical_core::format::STANDARD;
    assert_eq!(lexical_core::parse_with_options::<_, FORMAT>(b"12345.0", &options), Ok(12345.0f32));
    assert_eq!(
        lexical_core::parse_partial_with_options::<_, FORMAT>(b"12345.0", &options),
        Ok((12345.0f32, 7))
    );
}
