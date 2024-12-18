mod util;

use lexical_parse_float::FromLexical;
#[cfg(feature = "f16")]
use lexical_util::bf16::bf16;
#[cfg(feature = "f16")]
use lexical_util::f16::f16;
use lexical_util::num::Float;
use proptest::prelude::*;

use crate::util::default_proptest_config;

fn float_equal<F: Float>(x: F, y: F) -> bool {
    if x.is_nan() {
        y.is_nan()
    } else {
        y == x
    }
}

default_quickcheck! {
    fn f32_roundtrip_quickcheck(x: f32) -> bool {
        let string = x.to_string();
        let result = f32::from_lexical(string.as_bytes());
        result.map_or(false, |y| float_equal(x, y))
    }

    fn f32_short_decimal_quickcheck(x: f32) -> bool {
        let string = format!("{:.4}", x);
        let actual = f32::from_lexical(string.as_bytes());
        let expected = string.parse::<f32>();
        actual.map_or(false, |y| expected.map_or(false, |x| float_equal(x, y)))
    }

    fn f32_long_decimal_quickcheck(x: f32) -> bool {
        let string = format!("{:.100}", x);
        let actual = f32::from_lexical(string.as_bytes());
        let expected = string.parse::<f32>();
        actual.map_or(false, |y| expected.map_or(false, |x| float_equal(x, y)))
    }

    fn f32_short_exponent_quickcheck(x: f32) -> bool {
        let string = format!("{:.4e}", x);
        let actual = f32::from_lexical(string.as_bytes());
        let expected = string.parse::<f32>();
        actual.map_or(false, |y| expected.map_or(false, |x| float_equal(x, y)))
    }

    fn f32_long_exponent_quickcheck(x: f32) -> bool {
        let string = format!("{:.100e}", x);
        let actual = f32::from_lexical(string.as_bytes());
        let expected = string.parse::<f32>();
        actual.map_or(false, |y| expected.map_or(false, |x| float_equal(x, y)))
    }

    fn f64_roundtrip_quickcheck(x: f64) -> bool {
        let string = x.to_string();
        let result = f64::from_lexical(string.as_bytes());
        result.map_or(false, |y| float_equal(x, y))
    }

    fn f64_short_decimal_quickcheck(x: f64) -> bool {
        let string = format!("{:.4}", x);
        let actual = f64::from_lexical(string.as_bytes());
        let expected = string.parse::<f64>();
        actual.map_or(false, |y| expected.map_or(false, |x| float_equal(x, y)))
    }

    fn f64_long_decimal_quickcheck(x: f64) -> bool {
        let string = format!("{:.100}", x);
        let actual = f64::from_lexical(string.as_bytes());
        let expected = string.parse::<f64>();
        actual.map_or(false, |y| expected.map_or(false, |x| float_equal(x, y)))
    }

    fn f64_short_exponent_quickcheck(x: f64) -> bool {
        let string = format!("{:.4e}", x);
        let actual = f64::from_lexical(string.as_bytes());
        let expected = string.parse::<f64>();
        actual.map_or(false, |y| expected.map_or(false, |x| float_equal(x, y)))
    }

    fn f64_long_exponent_quickcheck(x: f64) -> bool {
        let string = format!("{:.100e}", x);
        let actual = f64::from_lexical(string.as_bytes());
        let expected = string.parse::<f64>();
        actual.map_or(false, |y| expected.map_or(false, |x| float_equal(x, y)))
    }

    #[cfg(feature = "f16")]
    fn f16_roundtrip_quickcheck(bits: u16) -> bool {
        let x = f16::from_bits(bits);
        let string = x.as_f32().to_string();
        let result = f16::from_lexical(string.as_bytes());
        result.map_or(false, |y| float_equal(x, y))
    }

    #[cfg(feature = "f16")]
    fn bf16_roundtrip_quickcheck(bits: u16) -> bool {
        let x = bf16::from_bits(bits);
        let string = x.as_f32().to_string();
        let result = bf16::from_lexical(string.as_bytes());
        result.map_or(false, |y| float_equal(x, y))
    }
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    fn f32_invalid_proptest(i in r"[+-]?[0-9]{2}[^\deE]?\.[^\deE]?[0-9]{2}[^\deE]?e[+-]?[0-9]+[^\deE]") {
        let res = f32::from_lexical(i.as_bytes());
        prop_assert!(res.is_err());
        prop_assert!(res.err().unwrap().is_invalid_digit());
    }

    #[test]
    fn f32_double_sign_proptest(i in r"[+-]{2}[0-9]{2}\.[0-9]{2}e[+-]?[0-9]+") {
        let res = f32::from_lexical(i.as_bytes());
        prop_assert!(res.is_err());
        prop_assert!(
            res.err().unwrap().is_invalid_digit() ||
            res.err().unwrap().is_empty_mantissa()
        );
    }

    #[test]
    fn f32_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
        let res = f32::from_lexical(i.as_bytes());
        prop_assert!(res.is_err());
        prop_assert!(
            res.err().unwrap().is_empty() ||
            res.err().unwrap().is_empty_mantissa()
        );
    }

    #[test]
    fn f32_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
        let res = f32::from_lexical(i.as_bytes());
        prop_assert!(res.is_err());
        prop_assert!(res.err().unwrap().is_empty_exponent());
    }

    #[test]
    fn f32_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
        let res = f32::from_lexical(i.as_bytes());
        prop_assert!(res.is_err());
        prop_assert!(res.err().unwrap().is_empty_exponent());
    }

    #[test]
    fn f32_roundtrip_display_proptest(i in f32::MIN..f32::MAX) {
        let input: String = format!("{}", i);
        prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
    }

    #[test]
    fn f32_roundtrip_debug_proptest(i in f32::MIN..f32::MAX) {
        let input: String = format!("{:?}", i);
        prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
    }

    #[test]
    fn f32_roundtrip_scientific_proptest(i in f32::MIN..f32::MAX) {
        let input: String = format!("{:e}", i);
        prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
    }

    #[test]
    fn f64_invalid_proptest(i in r"[+-]?[0-9]{2}[^\deE]?\.[^\deE]?[0-9]{2}[^\deE]?e[+-]?[0-9]+[^\deE]") {
        let res = f64::from_lexical(i.as_bytes());
        prop_assert!(res.is_err());
        prop_assert!(res.err().unwrap().is_invalid_digit());
    }

    #[test]
    fn f64_double_sign_proptest(i in r"[+-]{2}[0-9]{2}\.[0-9]{2}e[+-]?[0-9]+") {
        let res = f64::from_lexical(i.as_bytes());
        prop_assert!(res.is_err());
        prop_assert!(
            res.err().unwrap().is_invalid_digit() ||
            res.err().unwrap().is_empty_mantissa()
        );
    }

    #[test]
    fn f64_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
        let res = f64::from_lexical(i.as_bytes());
        prop_assert!(res.is_err());
        prop_assert!(
            res.err().unwrap().is_empty() ||
            res.err().unwrap().is_empty_mantissa()
        );
    }

    #[test]
    fn f64_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
        let res = f64::from_lexical(i.as_bytes());
        prop_assert!(res.is_err());
        prop_assert!(res.err().unwrap().is_empty_exponent());
    }

    #[test]
    fn f64_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
        let res = f64::from_lexical(i.as_bytes());
        prop_assert!(res.is_err());
        prop_assert!(res.err().unwrap().is_empty_exponent());
    }

    #[test]
    fn f64_roundtrip_display_proptest(i in f64::MIN..f64::MAX) {
        let input: String = format!("{}", i);
        prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
    }

    #[test]
    fn f64_roundtrip_debug_proptest(i in f64::MIN..f64::MAX) {
        let input: String = format!("{:?}", i);
        prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
    }

    #[test]
    fn f64_roundtrip_scientific_proptest(i in f64::MIN..f64::MAX) {
        let input: String = format!("{:e}", i);
        prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
    }
}
