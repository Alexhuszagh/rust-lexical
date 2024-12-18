#![cfg(feature = "power-of-two")]

mod parse_radix;
mod util;

use lexical_util::constants::BUFFER_SIZE;
use lexical_util::format::NumberFormatBuilder;
use lexical_util::num::Float;
use lexical_write_float::{binary, Options};
use parse_radix::{parse_f32, parse_f64};
use proptest::prelude::*;

use crate::util::default_proptest_config;

const BINARY: u128 = NumberFormatBuilder::binary();
const OCTAL: u128 = NumberFormatBuilder::octal();

default_quickcheck! {
    fn f32_binary_quickcheck(f: f32) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() {
            true
        } else {
            let f = f.abs();
            let count = binary::write_float::<_, BINARY>(f, &mut buffer, &options);
            let roundtrip = parse_f32(&buffer[..count], 2, b'e');
            roundtrip == f
        }
    }

    fn f32_octal_quickcheck(f: f32) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() {
            true
        } else {
            let f = f.abs();
            let count = binary::write_float::<_, OCTAL>(f, &mut buffer, &options);
            let roundtrip = parse_f32(&buffer[..count], 8, b'e');
            roundtrip == f
        }
    }

    fn f64_binary_quickcheck(f: f64) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() {
            true
        } else {
            let f = f.abs();
            let count = binary::write_float::<_, BINARY>(f, &mut buffer, &options);
            let roundtrip = parse_f64(&buffer[..count], 2, b'e');
            roundtrip == f
        }
    }

    fn f64_octal_quickcheck(f: f64) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() {
            true
        } else {
            let f = f.abs();
            let count = binary::write_float::<_, OCTAL>(f, &mut buffer, &options);
            let roundtrip = parse_f64(&buffer[..count], 8, b'e');
            roundtrip == f
        }
    }
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    fn f32_binary_proptest(f in f32::MIN..f32::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !f.is_special() {
            let f = f.abs();
            let count = binary::write_float::<_, BINARY>(f, &mut buffer, &options);
            let roundtrip = parse_f32(&buffer[..count], 2, b'e');
            prop_assert_eq!(roundtrip, f)
        }
    }

    #[test]
    fn f32_octal_proptest(f in f32::MIN..f32::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !f.is_special() {
            let f = f.abs();
            let count = binary::write_float::<_, OCTAL>(f, &mut buffer, &options);
            let roundtrip = parse_f32(&buffer[..count], 8, b'e');
            prop_assert_eq!(roundtrip, f)
        }
    }

    #[test]
    fn f64_binary_proptest(f in f64::MIN..f64::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !f.is_special() {
            let f = f.abs();
            let count = binary::write_float::<_, BINARY>(f, &mut buffer, &options);
            let roundtrip = parse_f64(&buffer[..count], 2, b'e');
            prop_assert_eq!(roundtrip, f)
        }
    }

    #[test]
    fn f64_octal_proptest(f in f64::MIN..f64::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !f.is_special() {
            let f = f.abs();
            let count = binary::write_float::<_, OCTAL>(f, &mut buffer, &options);
            let roundtrip = parse_f64(&buffer[..count], 8, b'e');
            prop_assert_eq!(roundtrip, f)
        }
    }
}
