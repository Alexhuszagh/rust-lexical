mod util;

use lexical_parse_integer::FromLexical;
#[cfg(feature = "power-of-two")]
use lexical_parse_integer::{FromLexicalWithOptions, Options};
use proptest::prelude::*;
#[cfg(feature = "power-of-two")]
use util::from_radix;

use crate::util::default_proptest_config;

macro_rules! is_error {
    ($result:expr, $check:ident) => {{
        let result = $result;
        prop_assert!(result.is_err());
        let err = result.err().unwrap();
        prop_assert!(err.$check());
    }};
}

macro_rules! is_invalid_digit {
    ($result:expr) => {
        is_error!($result, is_invalid_digit)
    };
}

macro_rules! is_empty {
    ($result:expr) => {
        is_error!($result, is_empty)
    };
}

macro_rules! is_overflow {
    ($result:expr) => {
        is_error!($result, is_overflow)
    };
}

macro_rules! is_underflow {
    ($result:expr) => {
        is_error!($result, is_underflow)
    };
}

macro_rules! is_invalid_digit_match {
    ($result:expr, $p1:pat_param $(| $prest:pat_param)*) => {{
        let result = $result;
        prop_assert!(result.is_err());
        let err = result.err().unwrap();
        prop_assert!(err.is_invalid_digit());
        prop_assert!(matches!(*err.index().unwrap(), $p1 $(| $prest)*));
    }};
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    #[cfg(feature = "power-of-two")]
    fn i32_binary_roundtrip_display_proptest(i in i32::MIN..i32::MAX) {
        let options = Options::new();
        const FORMAT: u128 = from_radix(2);
        let digits = if i < 0 {
            format!("-{:b}", (i as i64).wrapping_neg())
        } else {
            format!("{:b}", i)
        };
        let result = i32::from_lexical_with_options::<FORMAT>(digits.as_bytes(), &options);
        prop_assert_eq!(i, result.unwrap());
    }

    #[test]
    fn u8_invalid_proptest(i in r"[+]?[0-9]{2}\D") {
        is_invalid_digit_match!(u8::from_lexical(i.as_bytes()), 2 | 3);
    }

    #[test]
    fn u8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}") {
        is_overflow!(u8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u8_negative_proptest(i in r"[-][1-9][0-9]{2}") {
        is_invalid_digit!(u8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u8_double_sign_proptest(i in r"[+]{2}[0-9]{2}") {
        is_invalid_digit_match!(u8::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn u8_sign_only_proptest(i in r"[+]") {
        is_empty!(u8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u8_trailing_digits_proptest(i in r"[+]?[0-9]{2}\D[0-9]{2}") {
        is_invalid_digit_match!(u8::from_lexical(i.as_bytes()), 2 | 3);
    }

    #[test]
    fn i8_invalid_proptest(i in r"[+-]?[0-9]{2}\D") {
        is_invalid_digit_match!(i8::from_lexical(i.as_bytes()), 2 | 3);
    }

    #[test]
    fn i8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}") {
        is_overflow!(i8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i8_underflow_proptest(i in r"[-][1-9][0-9]{3}") {
        is_underflow!(i8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i8_double_sign_proptest(i in r"[+-]{2}[0-9]{2}") {
        is_invalid_digit_match!(i8::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn i8_sign_only_proptest(i in r"[+-]") {
        is_empty!(i8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i8_trailing_digits_proptest(i in r"[+-]?[0-9]{2}\D[0-9]{2}") {
        is_invalid_digit_match!(i8::from_lexical(i.as_bytes()), 2 | 3);
    }

    #[test]
    fn u16_invalid_proptest(i in r"[+]?[0-9]{4}\D") {
        is_invalid_digit_match!(u16::from_lexical(i.as_bytes()), 4 | 5);
    }

    #[test]
    fn u16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}") {
        is_overflow!(u16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u16_negative_proptest(i in r"[-][1-9][0-9]{4}") {
        is_invalid_digit!(u16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u16_double_sign_proptest(i in r"[+]{2}[0-9]{4}") {
        is_invalid_digit_match!(u16::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn u16_sign_only_proptest(i in r"[+]") {
        is_empty!(u16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u16_trailing_digits_proptest(i in r"[+]?[0-9]{4}\D[0-9]{2}") {
        is_invalid_digit_match!(u16::from_lexical(i.as_bytes()), 4 | 5);
    }

    #[test]
    fn i16_invalid_proptest(i in r"[+-]?[0-9]{4}\D") {
        is_invalid_digit_match!(i16::from_lexical(i.as_bytes()), 4 | 5);
    }

    #[test]
    fn i16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}") {
        is_overflow!(i16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i16_underflow_proptest(i in r"[-][1-9][0-9]{5}") {
        is_underflow!(i16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i16_double_sign_proptest(i in r"[+-]{2}[0-9]{4}") {
        is_invalid_digit_match!(i16::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn i16_sign_only_proptest(i in r"[+-]") {
        is_empty!(i16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i16_trailing_digits_proptest(i in r"[+-]?[0-9]{4}\D[0-9]{2}") {
        is_invalid_digit_match!(i16::from_lexical(i.as_bytes()), 4 | 5);
    }

    #[test]
    fn u32_invalid_proptest(i in r"[+]?[0-9]{9}\D") {
        is_invalid_digit_match!(u32::from_lexical(i.as_bytes()), 9 | 10);
    }

    #[test]
    fn u32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}") {
        is_overflow!(u32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u32_negative_proptest(i in r"[-][1-9][0-9]{9}") {
        is_invalid_digit!(u32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u32_double_sign_proptest(i in r"[+]{2}[0-9]{9}") {
        is_invalid_digit_match!(u32::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn u32_sign_only_proptest(i in r"[+]") {
        is_empty!(u32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u32_trailing_digits_proptest(i in r"[+]?[0-9]{9}\D[0-9]{2}") {
        is_invalid_digit_match!(u32::from_lexical(i.as_bytes()), 9 | 10);
    }

    #[test]
    fn i32_invalid_proptest(i in r"[+-]?[0-9]{9}\D") {
        is_invalid_digit_match!(i32::from_lexical(i.as_bytes()), 9 | 10);
    }

    #[test]
    fn i32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}") {
        is_overflow!(i32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i32_underflow_proptest(i in r"-[1-9][0-9]{10}") {
        is_underflow!(i32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i32_double_sign_proptest(i in r"[+-]{2}[0-9]{9}") {
        is_invalid_digit_match!(i32::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn i32_sign_only_proptest(i in r"[+-]") {
        is_empty!(i32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i32_trailing_digits_proptest(i in r"[+-]?[0-9]{9}\D[0-9]{2}") {
        is_invalid_digit_match!(i32::from_lexical(i.as_bytes()), 9 | 10);
    }

    #[test]
    fn u64_invalid_proptest(i in r"[+]?[0-9]{19}\D") {
        is_invalid_digit_match!(u64::from_lexical(i.as_bytes()), 19 | 20);
    }

    #[test]
    fn u64_overflow_proptest(i in r"[+]?[1-9][0-9]{21}") {
        is_overflow!(u64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u64_negative_proptest(i in r"[-][1-9][0-9]{21}") {
        is_invalid_digit!(u64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u64_double_sign_proptest(i in r"[+]{2}[0-9]{19}") {
        is_invalid_digit_match!(u64::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn u64_sign_only_proptest(i in r"[+]") {
        is_empty!(u64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u64_trailing_digits_proptest(i in r"[+]?[0-9]{19}\D[0-9]{2}") {
        is_invalid_digit_match!(u64::from_lexical(i.as_bytes()), 19 | 20);
    }

    #[test]
    fn i64_invalid_proptest(i in r"[+-]?[0-9]{18}\D") {
        is_invalid_digit_match!(i64::from_lexical(i.as_bytes()), 18 | 19);
    }

    #[test]
    fn i64_overflow_proptest(i in r"[+]?[1-9][0-9]{19}") {
        is_overflow!(i64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i64_underflow_proptest(i in r"-[1-9][0-9]{19}") {
        is_underflow!(i64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i64_double_sign_proptest(i in r"[+-]{2}[0-9]{18}") {
        is_invalid_digit_match!(i64::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn i64_sign_only_proptest(i in r"[+-]") {
        is_empty!(i64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i64_trailing_digits_proptest(i in r"[+-]?[0-9]{18}\D[0-9]{2}") {
        is_invalid_digit_match!(i64::from_lexical(i.as_bytes()), 18 | 19);
    }

    #[test]
    fn u128_invalid_proptest(i in r"[+]?[0-9]{38}\D") {
        is_invalid_digit_match!(u128::from_lexical(i.as_bytes()), 38 | 39);
    }

    #[test]
    fn u128_overflow_proptest(i in r"[+]?[1-9][0-9]{39}") {
        is_overflow!(u128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u128_negative_proptest(i in r"[-][1-9][0-9]{39}") {
        is_invalid_digit!(u128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u128_double_sign_proptest(i in r"[+]{2}[0-9]{38}") {
        is_invalid_digit_match!(u128::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn u128_sign_only_proptest(i in r"[+]") {
        is_empty!(u128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u128_trailing_digits_proptest(i in r"[+]?[0-9]{38}\D[0-9]{2}") {
        is_invalid_digit_match!(u128::from_lexical(i.as_bytes()), 38 | 39);
    }

    #[test]
    fn i128_invalid_proptest(i in r"[+-]?[0-9]{38}\D") {
        is_invalid_digit_match!(i128::from_lexical(i.as_bytes()), 38 | 39);
    }

    #[test]
    fn i128_overflow_proptest(i in r"[+]?[1-9][0-9]{39}") {
        is_overflow!(i128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i128_underflow_proptest(i in r"-[1-9][0-9]{39}") {
        is_underflow!(i128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i128_double_sign_proptest(i in r"[+-]{2}[0-9]{38}") {
        is_invalid_digit_match!(i128::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn i128_sign_only_proptest(i in r"[+-]") {
        is_empty!(i128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i128_trailing_digits_proptest(i in r"[+-]?[0-9]{38}\D[0-9]{2}") {
        is_invalid_digit_match!(i128::from_lexical(i.as_bytes()), 38 | 39);
    }
}
