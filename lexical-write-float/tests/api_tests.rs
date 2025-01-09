use lexical_util::constants::BUFFER_SIZE;
use lexical_util::format::STANDARD;
use lexical_write_float::{Options, ToLexical, ToLexicalWithOptions};

#[test]
fn error_tests() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let f = 2762159900.0f32;
    let actual = unsafe { std::str::from_utf8_unchecked(f.to_lexical(&mut buffer)) };
    let roundtrip = actual.parse::<f32>();
    assert_eq!(Ok(f), roundtrip);

    let f = 77371252000000000000000000.0f32;
    let actual = unsafe { std::str::from_utf8_unchecked(f.to_lexical(&mut buffer)) };
    let roundtrip = actual.parse::<f32>();
    assert_eq!(Ok(f), roundtrip);
}

#[test]
fn fuzz_tests() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let f = 355259285044678240000000000000000000000000000000000000000000f64;
    let actual = unsafe { std::str::from_utf8_unchecked(f.to_lexical(&mut buffer)) };
    let roundtrip = actual.parse::<f64>();
    assert_eq!(Ok(f), roundtrip);
}

#[test]
fn special_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let actual = unsafe { std::str::from_utf8_unchecked(f64::NAN.to_lexical(&mut buffer)) };
    assert_eq!(actual, "NaN");
    let actual = unsafe { std::str::from_utf8_unchecked(f64::INFINITY.to_lexical(&mut buffer)) };
    assert_eq!(actual, "inf");

    const OPTIONS: Options =
        Options::builder().nan_string(Some(b"nan")).inf_string(Some(b"Infinity")).build_strict();
    let bytes = f64::NAN.to_lexical_with_options::<{ STANDARD }>(&mut buffer, &OPTIONS);
    let actual = unsafe { std::str::from_utf8_unchecked(bytes) };
    assert_eq!(actual, "nan");
    let bytes = f64::INFINITY.to_lexical_with_options::<{ STANDARD }>(&mut buffer, &OPTIONS);
    let actual = unsafe { std::str::from_utf8_unchecked(bytes) };
    assert_eq!(actual, "Infinity");
}

#[test]
#[should_panic]
fn invalid_nan_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    const OPTIONS: Options = Options::builder().nan_string(None).build_strict();
    f64::NAN.to_lexical_with_options::<{ STANDARD }>(&mut buffer, &OPTIONS);
}

#[test]
#[should_panic]
fn invalid_inf_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    const OPTIONS: Options = Options::builder().inf_string(None).build_strict();
    f64::INFINITY.to_lexical_with_options::<{ STANDARD }>(&mut buffer, &OPTIONS);
}

#[test]
#[cfg(feature = "power-of-two")]
fn hex_test() {
    use core::num;

    use lexical_util::format::NumberFormatBuilder;

    const BASE16_2_10: u128 = NumberFormatBuilder::new()
        .mantissa_radix(16)
        .exponent_base(num::NonZeroU8::new(2))
        .exponent_radix(num::NonZeroU8::new(10))
        .build_strict();
    const HEX_OPTIONS: Options = Options::builder().exponent(b'^').build_unchecked();

    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let float = 12345.0f64;
    let result = float.to_lexical_with_options::<BASE16_2_10>(&mut buffer, &HEX_OPTIONS);
    assert_eq!(result, b"3.039^12");
}
