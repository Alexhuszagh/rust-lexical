#[cfg(feature = "f16")]
use lexical_util::bf16::bf16;
use lexical_util::constants::BUFFER_SIZE;
#[cfg(feature = "f16")]
use lexical_util::f16::f16;
use lexical_util::format::STANDARD;
use lexical_write_float::{Options, ToLexical, ToLexicalWithOptions};
use proptest::prelude::*;
use quickcheck::quickcheck;

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

    let options =
        Options::builder().nan_string(Some(b"nan")).inf_string(Some(b"Infinity")).build().unwrap();
    let bytes = f64::NAN.to_lexical_with_options::<{ STANDARD }>(&mut buffer, &options);
    let actual = unsafe { std::str::from_utf8_unchecked(bytes) };
    assert_eq!(actual, "nan");
    let bytes = f64::INFINITY.to_lexical_with_options::<{ STANDARD }>(&mut buffer, &options);
    let actual = unsafe { std::str::from_utf8_unchecked(bytes) };
    assert_eq!(actual, "Infinity");
}

#[test]
#[should_panic]
fn invalid_nan_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let options = Options::builder().nan_string(None).build().unwrap();
    f64::NAN.to_lexical_with_options::<{ STANDARD }>(&mut buffer, &options);
}

#[test]
#[should_panic]
fn invalid_inf_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let options = Options::builder().inf_string(None).build().unwrap();
    f64::INFINITY.to_lexical_with_options::<{ STANDARD }>(&mut buffer, &options);
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
        .build();
    const HEX_OPTIONS: Options = unsafe { Options::builder().exponent(b'^').build_unchecked() };

    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let float = 12345.0f64;
    let result = float.to_lexical_with_options::<BASE16_2_10>(&mut buffer, &HEX_OPTIONS);
    assert_eq!(result, b"3.039^12");
}

quickcheck! {
    #[cfg_attr(miri, ignore)]
    fn f32_quickcheck(f: f32) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let actual = unsafe { std::str::from_utf8_unchecked(f.to_lexical(&mut buffer)) };
        let roundtrip = actual.parse::<f32>();
        if f.is_nan() {
            roundtrip.is_ok() && roundtrip.unwrap().is_nan()
        } else {
            roundtrip == Ok(f)
        }
    }

    #[cfg_attr(miri, ignore)]
    fn f64_quickcheck(f: f64) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let actual = unsafe { std::str::from_utf8_unchecked(f.to_lexical(&mut buffer)) };
        let roundtrip = actual.parse::<f64>();
        if f.is_nan() {
            roundtrip.is_ok() && roundtrip.unwrap().is_nan()
        } else {
            roundtrip == Ok(f)
        }
    }
}

proptest! {
    #[test]
    #[cfg_attr(miri, ignore)]
    fn f32_proptest(f in f32::MIN..f32::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let actual = unsafe { std::str::from_utf8_unchecked(f.to_lexical(&mut buffer)) };
        let roundtrip = actual.parse::<f32>();
        if f.is_nan() {
            prop_assert!(roundtrip.is_ok() && roundtrip.unwrap().is_nan());
        } else {
            prop_assert_eq!(roundtrip, Ok(f));
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn f64_proptest(f in f64::MIN..f64::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let actual = unsafe { std::str::from_utf8_unchecked(f.to_lexical(&mut buffer)) };
        let roundtrip = actual.parse::<f64>();
        if f.is_nan() {
            prop_assert!(roundtrip.is_ok() && roundtrip.unwrap().is_nan());
        } else {
            prop_assert_eq!(roundtrip, Ok(f));
        }
    }

    #[test]
    #[cfg(feature = "f16")]
    #[cfg_attr(miri, ignore)]
    fn f16_proptest(bits in u16::MIN..u16::MAX) {
        use lexical_util::num::Float;

        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let f = f16::from_bits(bits);
        let actual = unsafe { std::str::from_utf8_unchecked(f.to_lexical(&mut buffer)) };
        let roundtrip = actual.parse::<f32>();
        if f.is_nan() {
            prop_assert!(roundtrip.is_ok() && roundtrip.unwrap().is_nan());
        } else {
            prop_assert_eq!(roundtrip, Ok(f.as_f32()));
        }
    }

    #[test]
    #[cfg(feature = "f16")]
    #[cfg_attr(miri, ignore)]
    fn bf16_proptest(bits in u16::MIN..u16::MAX) {
        use lexical_util::num::Float;

        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let f = bf16::from_bits(bits);
        let actual = unsafe { std::str::from_utf8_unchecked(f.to_lexical(&mut buffer)) };
        let roundtrip = actual.parse::<f32>();
        if f.is_nan() {
            prop_assert!(roundtrip.is_ok() && roundtrip.unwrap().is_nan());
        } else {
            prop_assert_eq!(roundtrip, Ok(f.as_f32()));
        }
    }
}
