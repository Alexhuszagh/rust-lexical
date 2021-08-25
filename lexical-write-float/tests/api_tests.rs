use lexical_util::constants::BUFFER_SIZE;
use lexical_write_float::ToLexical;
use proptest::prelude::*;
use quickcheck::quickcheck;

quickcheck! {
    #[cfg_attr(miri, ignore)]
    fn f32_quickcheck(f: f32) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let actual = unsafe { std::str::from_utf8_unchecked(f.to_lexical(&mut buffer)) };
        let roundtrip = actual.parse::<f32>();
        if f.is_nan() {
            roundtrip.is_ok() && roundtrip.unwrap().is_nan()
        } else {
            println!("f={:?}", f);
            println!("roundtrip={:?}", roundtrip);
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
}
