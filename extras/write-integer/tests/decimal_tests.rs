#![cfg(not(feature = "compact"))]

mod util;

use lexical_util::num::UnsignedInteger;
use lexical_write_integer::decimal::{Decimal, DecimalCount};

fn slow_digit_count<T: UnsignedInteger>(x: T) -> usize {
    x.to_string().len()
}

default_quickcheck! {
    fn u32_digit_count_quickcheck(x: u32) -> bool {
        slow_digit_count(x) == x.decimal_count()
    }

    fn u64_digit_count_quickcheck(x: u64) -> bool {
        slow_digit_count(x) == x.decimal_count()
    }

    fn u128_digit_count_quickcheck(x: u128) -> bool {
        slow_digit_count(x) == x.decimal_count()
    }

    fn u32toa_quickcheck(x: u32) -> bool {
        let actual = x.to_string();
        let mut buffer = [b'\x00'; 16];
        actual.len() == x.decimal(&mut buffer) &&
            &buffer[..actual.len()] == actual.as_bytes()
    }

    fn u64toa_quickcheck(x: u64) -> bool {
        let actual = x.to_string();
        let mut buffer = [b'\x00'; 32];
        actual.len() == x.decimal(&mut buffer) &&
            &buffer[..actual.len()] == actual.as_bytes()
    }

    fn u128toa_quickcheck(x: u128) -> bool {
        let actual = x.to_string();
        let mut buffer = [b'\x00'; 48];
        actual.len() == x.decimal(&mut buffer) &&
            &buffer[..actual.len()] == actual.as_bytes()
    }
}
