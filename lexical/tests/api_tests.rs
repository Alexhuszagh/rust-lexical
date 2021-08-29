#[test]
#[cfg(feature = "write-integers")]
fn integer_to_string_test() {
    assert_eq!(lexical::to_string(12345u32), "12345");
    let options = lexical::WriteIntegerOptions::new();
    const FORMAT: u128 = lexical::format::STANDARD;
    assert_eq!(lexical::to_string_with_options::<_, FORMAT>(12345u32, &options), "12345");
}

#[test]
#[cfg(feature = "write-floats")]
fn float_to_string_test() {
    assert_eq!(lexical::to_string(12345.0f32), "12345.0");
    let options = lexical::WriteFloatOptions::new();
    const FORMAT: u128 = lexical::format::STANDARD;
    assert_eq!(lexical::to_string_with_options::<_, FORMAT>(12345.0f32, &options), "12345.0");
}

#[test]
#[cfg(feature = "parse-integers")]
fn string_to_integer_test() {
    assert_eq!(lexical::parse("12345"), Ok(12345u32));
    assert_eq!(lexical::parse_partial("12345"), Ok((12345u32, 5)));

    let options = lexical::ParseIntegerOptions::new();
    const FORMAT: u128 = lexical::format::STANDARD;
    assert_eq!(lexical::parse_with_options::<_, _, FORMAT>("12345", &options), Ok(12345u32));
    assert_eq!(
        lexical::parse_partial_with_options::<_, _, FORMAT>("12345", &options),
        Ok((12345u32, 5))
    );
}

#[test]
#[cfg(feature = "parse-floats")]
fn string_to_float_test() {
    assert_eq!(lexical::parse("12345.0"), Ok(12345.0f32));
    assert_eq!(lexical::parse_partial("12345.0"), Ok((12345.0f32, 7)));

    let options = lexical::ParseFloatOptions::new();
    const FORMAT: u128 = lexical::format::STANDARD;
    assert_eq!(lexical::parse_with_options::<_, _, FORMAT>("12345.0", &options), Ok(12345.0f32));
    assert_eq!(
        lexical::parse_partial_with_options::<_, _, FORMAT>("12345.0", &options),
        Ok((12345.0f32, 7))
    );
}
