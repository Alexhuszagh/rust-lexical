#![cfg(not(feature = "compact"))]

mod util;

use lexical_util::num::UnsignedInteger;
use lexical_write_integer::decimal::{self, Decimal, DecimalCount};

#[test]
fn fast_log10_test() {
    // Check the first, even if illogical case works.
    assert_eq!(decimal::fast_log10(0u32), 0);
    assert_eq!(decimal::fast_log10(10u32), 0);
    assert_eq!(decimal::fast_log10(15u32), 0);
    assert_eq!(decimal::fast_log10(20u32), 1);
    assert_eq!(decimal::fast_log10(100u32), 1);
    assert_eq!(decimal::fast_log10(200u32), 2);
}

#[test]
fn u32_decimal_count_test() {
    assert_eq!(u32::decimal_count(0), 1);
    assert_eq!(u32::decimal_count(1), 1);
    assert_eq!(u32::decimal_count(9), 1);
    assert_eq!(u32::decimal_count(10), 2);
    assert_eq!(u32::decimal_count(11), 2);

    assert_eq!(u32::decimal_count((1 << 16) - 1), 5);
    assert_eq!(u32::decimal_count(1 << 16), 5);
    assert_eq!(u32::decimal_count((1 << 16) + 1), 5);

    assert_eq!(u32::decimal_count(u32::MAX), 10);
}

#[test]
fn u64_decimal_count_test() {
    assert_eq!(u64::decimal_count(0), 1);
    assert_eq!(u64::decimal_count(1), 1);
    assert_eq!(u64::decimal_count(9), 1);
    assert_eq!(u64::decimal_count(10), 2);
    assert_eq!(u64::decimal_count(11), 2);

    assert_eq!(u64::decimal_count((1 << 16) - 1), 5);
    assert_eq!(u64::decimal_count(1 << 16), 5);
    assert_eq!(u64::decimal_count((1 << 16) + 1), 5);

    assert_eq!(u64::decimal_count(u32::MAX as u64), 10);
    assert_eq!(u64::decimal_count(u64::MAX), 20);
}

#[test]
fn i64_19digit_test() {
    let mut buffer = [0u8; 19];
    assert_eq!((5i64 as u64).decimal_signed(&mut buffer), 1);
    assert_eq!(&buffer[..1], b"5");
}

#[test]
#[should_panic]
fn u64_19digit_test() {
    let mut buffer = [0u8; 19];
    assert_eq!(5u64.decimal(&mut buffer), 1);
    assert_eq!(&buffer[..1], b"5");
}

#[test]
fn u128_decimal_count_test() {
    assert_eq!(u128::decimal_count(u128::MAX), 39);
}

#[test]
fn u32toa_test() {
    let mut buffer = [b'\x00'; 16];
    assert_eq!(5u32.decimal(&mut buffer), 1);
    assert_eq!(&buffer[..1], b"5");

    assert_eq!(11u32.decimal(&mut buffer), 2);
    assert_eq!(&buffer[..2], b"11");

    assert_eq!(99u32.decimal(&mut buffer), 2);
    assert_eq!(&buffer[..2], b"99");

    assert_eq!(101u32.decimal(&mut buffer), 3);
    assert_eq!(&buffer[..3], b"101");

    assert_eq!(999u32.decimal(&mut buffer), 3);
    assert_eq!(&buffer[..3], b"999");

    assert_eq!(1001u32.decimal(&mut buffer), 4);
    assert_eq!(&buffer[..4], b"1001");

    assert_eq!(9999u32.decimal(&mut buffer), 4);
    assert_eq!(&buffer[..4], b"9999");

    assert_eq!(10001u32.decimal(&mut buffer), 5);
    assert_eq!(&buffer[..5], b"10001");

    assert_eq!(65535u32.decimal(&mut buffer), 5);
    assert_eq!(&buffer[..5], b"65535");

    assert_eq!(99999u32.decimal(&mut buffer), 5);
    assert_eq!(&buffer[..5], b"99999");

    assert_eq!(100001u32.decimal(&mut buffer), 6);
    assert_eq!(&buffer[..6], b"100001");

    assert_eq!(999999u32.decimal(&mut buffer), 6);
    assert_eq!(&buffer[..6], b"999999");

    assert_eq!(1000001u32.decimal(&mut buffer), 7);
    assert_eq!(&buffer[..7], b"1000001");

    assert_eq!(9999999u32.decimal(&mut buffer), 7);
    assert_eq!(&buffer[..7], b"9999999");

    assert_eq!(10000001u32.decimal(&mut buffer), 8);
    assert_eq!(&buffer[..8], b"10000001");

    assert_eq!(99999999u32.decimal(&mut buffer), 8);
    assert_eq!(&buffer[..8], b"99999999");

    assert_eq!(100000001u32.decimal(&mut buffer), 9);
    assert_eq!(&buffer[..9], b"100000001");

    assert_eq!(999999999u32.decimal(&mut buffer), 9);
    assert_eq!(&buffer[..9], b"999999999");

    assert_eq!(1000000001u32.decimal(&mut buffer), 10);
    assert_eq!(&buffer[..10], b"1000000001");

    assert_eq!(4294967295u32.decimal(&mut buffer), 10);
    assert_eq!(&buffer[..10], b"4294967295");
}

#[test]
fn u64toa_test() {
    let mut buffer = [b'\x00'; 32];
    assert_eq!(5u64.decimal(&mut buffer), 1);
    assert_eq!(&buffer[..1], b"5");

    assert_eq!(11u64.decimal(&mut buffer), 2);
    assert_eq!(&buffer[..2], b"11");

    assert_eq!(99u64.decimal(&mut buffer), 2);
    assert_eq!(&buffer[..2], b"99");

    assert_eq!(101u64.decimal(&mut buffer), 3);
    assert_eq!(&buffer[..3], b"101");

    assert_eq!(999u64.decimal(&mut buffer), 3);
    assert_eq!(&buffer[..3], b"999");

    assert_eq!(1001u64.decimal(&mut buffer), 4);
    assert_eq!(&buffer[..4], b"1001");

    assert_eq!(9999u64.decimal(&mut buffer), 4);
    assert_eq!(&buffer[..4], b"9999");

    assert_eq!(10001u64.decimal(&mut buffer), 5);
    assert_eq!(&buffer[..5], b"10001");

    assert_eq!(65535u64.decimal(&mut buffer), 5);
    assert_eq!(&buffer[..5], b"65535");

    assert_eq!(99999u64.decimal(&mut buffer), 5);
    assert_eq!(&buffer[..5], b"99999");

    assert_eq!(100001u64.decimal(&mut buffer), 6);
    assert_eq!(&buffer[..6], b"100001");

    assert_eq!(999999u64.decimal(&mut buffer), 6);
    assert_eq!(&buffer[..6], b"999999");

    assert_eq!(1000001u64.decimal(&mut buffer), 7);
    assert_eq!(&buffer[..7], b"1000001");

    assert_eq!(9999999u64.decimal(&mut buffer), 7);
    assert_eq!(&buffer[..7], b"9999999");

    assert_eq!(10000001u64.decimal(&mut buffer), 8);
    assert_eq!(&buffer[..8], b"10000001");

    assert_eq!(99999999u64.decimal(&mut buffer), 8);
    assert_eq!(&buffer[..8], b"99999999");

    assert_eq!(100000001u64.decimal(&mut buffer), 9);
    assert_eq!(&buffer[..9], b"100000001");

    assert_eq!(999999999u64.decimal(&mut buffer), 9);
    assert_eq!(&buffer[..9], b"999999999");

    assert_eq!(1000000001u64.decimal(&mut buffer), 10);
    assert_eq!(&buffer[..10], b"1000000001");

    assert_eq!(9999999999u64.decimal(&mut buffer), 10);
    assert_eq!(&buffer[..10], b"9999999999");

    assert_eq!(10000000001u64.decimal(&mut buffer), 11);
    assert_eq!(&buffer[..11], b"10000000001");

    assert_eq!(99999999999u64.decimal(&mut buffer), 11);
    assert_eq!(&buffer[..11], b"99999999999");

    assert_eq!(100000000001u64.decimal(&mut buffer), 12);
    assert_eq!(&buffer[..12], b"100000000001");

    assert_eq!(999999999999u64.decimal(&mut buffer), 12);
    assert_eq!(&buffer[..12], b"999999999999");

    assert_eq!(1000000000001u64.decimal(&mut buffer), 13);
    assert_eq!(&buffer[..13], b"1000000000001");

    assert_eq!(9999999999999u64.decimal(&mut buffer), 13);
    assert_eq!(&buffer[..13], b"9999999999999");

    assert_eq!(10000000000001u64.decimal(&mut buffer), 14);
    assert_eq!(&buffer[..14], b"10000000000001");

    assert_eq!(99999999999999u64.decimal(&mut buffer), 14);
    assert_eq!(&buffer[..14], b"99999999999999");

    assert_eq!(100000000000001u64.decimal(&mut buffer), 15);
    assert_eq!(&buffer[..15], b"100000000000001");

    assert_eq!(999999999999999u64.decimal(&mut buffer), 15);
    assert_eq!(&buffer[..15], b"999999999999999");

    assert_eq!(1000000000000001u64.decimal(&mut buffer), 16);
    assert_eq!(&buffer[..16], b"1000000000000001");

    assert_eq!(9999999999999999u64.decimal(&mut buffer), 16);
    assert_eq!(&buffer[..16], b"9999999999999999");

    assert_eq!(10000000000000001u64.decimal(&mut buffer), 17);
    assert_eq!(&buffer[..17], b"10000000000000001");

    assert_eq!(99999999999999999u64.decimal(&mut buffer), 17);
    assert_eq!(&buffer[..17], b"99999999999999999");

    assert_eq!(100000000000000001u64.decimal(&mut buffer), 18);
    assert_eq!(&buffer[..18], b"100000000000000001");

    assert_eq!(999999999999999999u64.decimal(&mut buffer), 18);
    assert_eq!(&buffer[..18], b"999999999999999999");

    assert_eq!(1000000000000000001u64.decimal(&mut buffer), 19);
    assert_eq!(&buffer[..19], b"1000000000000000001");

    assert_eq!(9999999999999999999u64.decimal(&mut buffer), 19);
    assert_eq!(&buffer[..19], b"9999999999999999999");

    assert_eq!(10000000000000000001u64.decimal(&mut buffer), 20);
    assert_eq!(&buffer[..20], b"10000000000000000001");

    assert_eq!(18446744073709551615u64.decimal(&mut buffer), 20);
    assert_eq!(&buffer[..20], b"18446744073709551615");
}

#[test]
fn u128toa_test() {
    let mut buffer = [b'\x00'; 48];
    assert_eq!(5u128.decimal(&mut buffer), 1);
    assert_eq!(&buffer[..1], b"5");

    assert_eq!(11u128.decimal(&mut buffer), 2);
    assert_eq!(&buffer[..2], b"11");

    assert_eq!(99u128.decimal(&mut buffer), 2);
    assert_eq!(&buffer[..2], b"99");

    assert_eq!(101u128.decimal(&mut buffer), 3);
    assert_eq!(&buffer[..3], b"101");

    assert_eq!(999u128.decimal(&mut buffer), 3);
    assert_eq!(&buffer[..3], b"999");

    assert_eq!(1001u128.decimal(&mut buffer), 4);
    assert_eq!(&buffer[..4], b"1001");

    assert_eq!(9999u128.decimal(&mut buffer), 4);
    assert_eq!(&buffer[..4], b"9999");

    assert_eq!(10001u128.decimal(&mut buffer), 5);
    assert_eq!(&buffer[..5], b"10001");

    assert_eq!(65535u128.decimal(&mut buffer), 5);
    assert_eq!(&buffer[..5], b"65535");

    assert_eq!(99999u128.decimal(&mut buffer), 5);
    assert_eq!(&buffer[..5], b"99999");

    assert_eq!(100001u128.decimal(&mut buffer), 6);
    assert_eq!(&buffer[..6], b"100001");

    assert_eq!(999999u128.decimal(&mut buffer), 6);
    assert_eq!(&buffer[..6], b"999999");

    assert_eq!(1000001u128.decimal(&mut buffer), 7);
    assert_eq!(&buffer[..7], b"1000001");

    assert_eq!(9999999u128.decimal(&mut buffer), 7);
    assert_eq!(&buffer[..7], b"9999999");

    assert_eq!(10000001u128.decimal(&mut buffer), 8);
    assert_eq!(&buffer[..8], b"10000001");

    assert_eq!(99999999u128.decimal(&mut buffer), 8);
    assert_eq!(&buffer[..8], b"99999999");

    assert_eq!(100000001u128.decimal(&mut buffer), 9);
    assert_eq!(&buffer[..9], b"100000001");

    assert_eq!(999999999u128.decimal(&mut buffer), 9);
    assert_eq!(&buffer[..9], b"999999999");

    assert_eq!(1000000001u128.decimal(&mut buffer), 10);
    assert_eq!(&buffer[..10], b"1000000001");

    assert_eq!(9999999999u128.decimal(&mut buffer), 10);
    assert_eq!(&buffer[..10], b"9999999999");

    assert_eq!(10000000001u128.decimal(&mut buffer), 11);
    assert_eq!(&buffer[..11], b"10000000001");

    assert_eq!(99999999999u128.decimal(&mut buffer), 11);
    assert_eq!(&buffer[..11], b"99999999999");

    assert_eq!(100000000001u128.decimal(&mut buffer), 12);
    assert_eq!(&buffer[..12], b"100000000001");

    assert_eq!(999999999999u128.decimal(&mut buffer), 12);
    assert_eq!(&buffer[..12], b"999999999999");

    assert_eq!(1000000000001u128.decimal(&mut buffer), 13);
    assert_eq!(&buffer[..13], b"1000000000001");

    assert_eq!(9999999999999u128.decimal(&mut buffer), 13);
    assert_eq!(&buffer[..13], b"9999999999999");

    assert_eq!(10000000000001u128.decimal(&mut buffer), 14);
    assert_eq!(&buffer[..14], b"10000000000001");

    assert_eq!(99999999999999u128.decimal(&mut buffer), 14);
    assert_eq!(&buffer[..14], b"99999999999999");

    assert_eq!(100000000000001u128.decimal(&mut buffer), 15);
    assert_eq!(&buffer[..15], b"100000000000001");

    assert_eq!(999999999999999u128.decimal(&mut buffer), 15);
    assert_eq!(&buffer[..15], b"999999999999999");

    assert_eq!(1000000000000001u128.decimal(&mut buffer), 16);
    assert_eq!(&buffer[..16], b"1000000000000001");

    assert_eq!(9999999999999999u128.decimal(&mut buffer), 16);
    assert_eq!(&buffer[..16], b"9999999999999999");

    assert_eq!(10000000000000001u128.decimal(&mut buffer), 17);
    assert_eq!(&buffer[..17], b"10000000000000001");

    assert_eq!(99999999999999999u128.decimal(&mut buffer), 17);
    assert_eq!(&buffer[..17], b"99999999999999999");

    assert_eq!(100000000000000001u128.decimal(&mut buffer), 18);
    assert_eq!(&buffer[..18], b"100000000000000001");

    assert_eq!(999999999999999999u128.decimal(&mut buffer), 18);
    assert_eq!(&buffer[..18], b"999999999999999999");

    assert_eq!(1000000000000000001u128.decimal(&mut buffer), 19);
    assert_eq!(&buffer[..19], b"1000000000000000001");

    assert_eq!(9999999999999999999u128.decimal(&mut buffer), 19);
    assert_eq!(&buffer[..19], b"9999999999999999999");

    assert_eq!(10000000000000000001u128.decimal(&mut buffer), 20);
    assert_eq!(&buffer[..20], b"10000000000000000001");

    assert_eq!(999999999999999999999999u128.decimal(&mut buffer), 24);
    assert_eq!(&buffer[..24], b"999999999999999999999999");

    assert_eq!(1000000000000000000000001u128.decimal(&mut buffer), 25);
    assert_eq!(&buffer[..25], b"1000000000000000000000001");

    assert_eq!(66620387370000000000000000000u128.decimal(&mut buffer), 29);
    assert_eq!(&buffer[..29], b"66620387370000000000000000000");

    assert_eq!(99999999999999999999999999999u128.decimal(&mut buffer), 29);
    assert_eq!(&buffer[..29], b"99999999999999999999999999999");

    assert_eq!(100000000000000000000000000001u128.decimal(&mut buffer), 30);
    assert_eq!(&buffer[..30], b"100000000000000000000000000001");

    assert_eq!(9999999999999999999999999999999999u128.decimal(&mut buffer), 34);
    assert_eq!(&buffer[..34], b"9999999999999999999999999999999999");

    assert_eq!(10000000000000000000000000000000001u128.decimal(&mut buffer), 35);
    assert_eq!(&buffer[..35], b"10000000000000000000000000000000001");

    assert_eq!(340282366920938463463374607431768211455u128.decimal(&mut buffer), 39);
    assert_eq!(&buffer[..39], b"340282366920938463463374607431768211455");
}

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
