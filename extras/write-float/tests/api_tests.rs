mod util;

#[cfg(feature = "f16")]
use lexical_util::bf16::bf16;
use lexical_util::constants::BUFFER_SIZE;
#[cfg(feature = "f16")]
use lexical_util::f16::f16;
use lexical_write_float::ToLexical;
use proptest::prelude::*;

use crate::util::default_proptest_config;

default_quickcheck! {
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
    #![proptest_config(default_proptest_config())]

    #[test]
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
    fn f16_proptest(bits in u16::MIN..u16::MAX) {
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
    fn bf16_proptest(bits in u16::MIN..u16::MAX) {
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
