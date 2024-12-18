#![cfg(not(feature = "compact"))]

mod util;

use lexical_util::constants::BUFFER_SIZE;
use lexical_util::format::NumberFormatBuilder;
use lexical_util::num::Float;
use lexical_write_float::{algorithm, Options};
use proptest::prelude::*;

use crate::util::default_proptest_config;

const DECIMAL: u128 = NumberFormatBuilder::decimal();

default_quickcheck! {
    fn f32_quickcheck(f: f32) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        let f = f.abs();
        if f.is_special() {
            true
        } else {
            let count = algorithm::write_float::<_, DECIMAL>(f, &mut buffer, &options);
            let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
            let roundtrip = actual.parse::<f32>();
            roundtrip == Ok(f)
        }
    }

    fn f64_quickcheck(f: f64) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        let f = f.abs();
        if f.is_special() {
            true
        } else {
            let count = algorithm::write_float::<_, DECIMAL>(f, &mut buffer, &options);
            let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
            let roundtrip = actual.parse::<f64>();
            roundtrip == Ok(f)
        }
    }
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    fn f32_proptest(f in f32::MIN..f32::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        let f = f.abs();
        if !f.is_special() {
            let count = algorithm::write_float::<_, DECIMAL>(f, &mut buffer, &options);
            let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
            let roundtrip = actual.parse::<f32>();
            prop_assert_eq!(roundtrip, Ok(f))
        }
    }

    #[test]
    fn f64_proptest(f in f64::MIN..f64::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        let f = f.abs();
        if !f.is_special() {
            let count = algorithm::write_float::<_, DECIMAL>(f, &mut buffer, &options);
            let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
            let roundtrip = actual.parse::<f64>();
            prop_assert_eq!(roundtrip, Ok(f))
        }
    }
}
