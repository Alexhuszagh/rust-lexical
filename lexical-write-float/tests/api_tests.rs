use lexical_util::constants::BUFFER_SIZE;
#[cfg(any(feature = "format", feature = "power-of-two"))]
use lexical_util::format::NumberFormatBuilder;
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

#[test]
#[should_panic]
#[cfg(feature = "format")]
fn unsupported_test() {
    const FORMAT: u128 = NumberFormatBuilder::new().supports_writing_floats(false).build_strict();
    const OPTIONS: Options = Options::new();

    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let float = 12345.0f64;
    _ = float.to_lexical_with_options::<FORMAT>(&mut buffer, &OPTIONS);
}

#[test]
#[cfg(feature = "format")]
fn supported_test() {
    const FORMAT: u128 = NumberFormatBuilder::new()
        .supports_parsing_integers(false)
        .supports_parsing_floats(false)
        .supports_writing_integers(false)
        .build_strict();
    const OPTIONS: Options = Options::new();

    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let float = 12345.0f64;
    assert_eq!(b"12345.0", float.to_lexical_with_options::<FORMAT>(&mut buffer, &OPTIONS));
}

#[test]
#[cfg(all(feature = "format", feature = "power-of-two"))]
fn require_base_prefix_test() {
    use core::num;

    const PREFIX: u128 = NumberFormatBuilder::new()
        .base_prefix(num::NonZeroU8::new(b'd'))
        .required_base_prefix(true)
        .build_strict();
    const OPTIONS: Options = Options::new();
    const TRIM: Options = Options::builder().trim_floats(true).build_strict();

    const PREFIX_SIZE: usize = OPTIONS.buffer_size_const::<f64, PREFIX>();
    let mut buffer = [b'\x00'; PREFIX_SIZE];
    let pos = 12345.0f64;
    assert_eq!(b"0d12345.0", pos.to_lexical_with_options::<PREFIX>(&mut buffer, &OPTIONS));
    assert_eq!(b"0d12345", pos.to_lexical_with_options::<PREFIX>(&mut buffer, &TRIM));

    let neg = -12345.0f64;
    assert_eq!(b"-0d12345.0", neg.to_lexical_with_options::<PREFIX>(&mut buffer, &OPTIONS));
    assert_eq!(b"-0d12345", neg.to_lexical_with_options::<PREFIX>(&mut buffer, &TRIM));

    const SUFFIX: u128 = NumberFormatBuilder::rebuild(PREFIX)
        .base_suffix(num::NonZeroU8::new(b'z'))
        .required_base_suffix(true)
        .build_strict();
    const SUFFIX_SIZE: usize = OPTIONS.buffer_size_const::<f64, SUFFIX>();
    let mut buffer = [b'\x00'; SUFFIX_SIZE];
    assert_eq!(b"0d12345.0z", pos.to_lexical_with_options::<SUFFIX>(&mut buffer, &OPTIONS));
    assert_eq!(b"0d12345z", pos.to_lexical_with_options::<SUFFIX>(&mut buffer, &TRIM));

    assert_eq!(b"-0d12345.0z", neg.to_lexical_with_options::<SUFFIX>(&mut buffer, &OPTIONS));
    assert_eq!(b"-0d12345z", neg.to_lexical_with_options::<SUFFIX>(&mut buffer, &TRIM));
}
