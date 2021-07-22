use lexical_util::num::UnsignedInteger;
use lexical_write_integer::decimal;
use quickcheck::quickcheck;

#[test]
fn fast_log2_test() {
    // Check the first, even if illogical case works.
    assert_eq!(decimal::fast_log2(0u32), 0);
    assert_eq!(decimal::fast_log2(1u32), 0);
    assert_eq!(decimal::fast_log2(2u32), 1);
    assert_eq!(decimal::fast_log2(3u32), 1);

    assert_eq!(decimal::fast_log2((1u32 << 16) - 1), 15);
    assert_eq!(decimal::fast_log2(1u32 << 16), 16);
    assert_eq!(decimal::fast_log2((1u32 << 16) + 1), 16);

    assert_eq!(decimal::fast_log2(u32::MAX), 31);
}

#[test]
fn fast_digit_count_test() {
    assert_eq!(decimal::fast_digit_count(0), 1);
    assert_eq!(decimal::fast_digit_count(1), 1);
    assert_eq!(decimal::fast_digit_count(9), 1);
    assert_eq!(decimal::fast_digit_count(10), 2);
    assert_eq!(decimal::fast_digit_count(11), 2);

    assert_eq!(decimal::fast_digit_count((1 << 16) - 1), 5);
    assert_eq!(decimal::fast_digit_count(1 << 16), 5);
    assert_eq!(decimal::fast_digit_count((1 << 16) + 1), 5);

    assert_eq!(decimal::fast_digit_count(u32::MAX), 10);
}

#[test]
fn fallback_digit_count_test() {
    assert_eq!(decimal::fallback_digit_count(0u32), 1);
    assert_eq!(decimal::fallback_digit_count(1u32), 1);
    assert_eq!(decimal::fallback_digit_count(9u32), 1);
    assert_eq!(decimal::fallback_digit_count(10u32), 2);
    assert_eq!(decimal::fallback_digit_count(11u32), 2);

    assert_eq!(decimal::fallback_digit_count((1u32 << 16) - 1), 5);
    assert_eq!(decimal::fallback_digit_count(1u32 << 16), 5);
    assert_eq!(decimal::fallback_digit_count((1u32 << 16) + 1), 5);

    assert_eq!(decimal::fallback_digit_count(u32::MAX), 10);
    assert_eq!(decimal::fallback_digit_count(u64::MAX), 20);
    assert_eq!(decimal::fallback_digit_count(u128::MAX), 39);
}

#[test]
fn u8toa_test() {
    let mut buffer = [b'0'; 16];
    unsafe {
        assert_eq!(decimal::u8toa(5, &mut buffer), 1);
        assert_eq!(&buffer[..1], b"5");

        assert_eq!(decimal::u8toa(11, &mut buffer), 2);
        assert_eq!(&buffer[..2], b"11");

        assert_eq!(decimal::u8toa(99, &mut buffer), 2);
        assert_eq!(&buffer[..2], b"99");

        assert_eq!(decimal::u8toa(101, &mut buffer), 3);
        assert_eq!(&buffer[..3], b"101");

        assert_eq!(decimal::u8toa(255, &mut buffer), 3);
        assert_eq!(&buffer[..3], b"255");
    }
}

#[test]
fn u16toa_test() {
    let mut buffer = [b'0'; 16];
    unsafe {
        assert_eq!(decimal::u16toa(5, &mut buffer), 1);
        assert_eq!(&buffer[..1], b"5");

        assert_eq!(decimal::u16toa(11, &mut buffer), 2);
        assert_eq!(&buffer[..2], b"11");

        assert_eq!(decimal::u16toa(99, &mut buffer), 2);
        assert_eq!(&buffer[..2], b"99");

        assert_eq!(decimal::u16toa(101, &mut buffer), 3);
        assert_eq!(&buffer[..3], b"101");

        assert_eq!(decimal::u16toa(999, &mut buffer), 3);
        assert_eq!(&buffer[..3], b"999");

        assert_eq!(decimal::u16toa(1001, &mut buffer), 4);
        assert_eq!(&buffer[..4], b"1001");

        assert_eq!(decimal::u16toa(9999, &mut buffer), 4);
        assert_eq!(&buffer[..4], b"9999");

        assert_eq!(decimal::u16toa(10001, &mut buffer), 5);
        assert_eq!(&buffer[..5], b"10001");

        assert_eq!(decimal::u16toa(65535, &mut buffer), 5);
        assert_eq!(&buffer[..5], b"65535");
    }
}

#[test]
fn u32toa_test() {
    let mut buffer = [b'0'; 16];
    unsafe {
        assert_eq!(decimal::u32toa(5, &mut buffer), 1);
        assert_eq!(&buffer[..1], b"5");

        assert_eq!(decimal::u32toa(11, &mut buffer), 2);
        assert_eq!(&buffer[..2], b"11");

        assert_eq!(decimal::u32toa(99, &mut buffer), 2);
        assert_eq!(&buffer[..2], b"99");

        assert_eq!(decimal::u32toa(101, &mut buffer), 3);
        assert_eq!(&buffer[..3], b"101");

        assert_eq!(decimal::u32toa(999, &mut buffer), 3);
        assert_eq!(&buffer[..3], b"999");

        assert_eq!(decimal::u32toa(1001, &mut buffer), 4);
        assert_eq!(&buffer[..4], b"1001");

        assert_eq!(decimal::u32toa(9999, &mut buffer), 4);
        assert_eq!(&buffer[..4], b"9999");

        assert_eq!(decimal::u32toa(10001, &mut buffer), 5);
        assert_eq!(&buffer[..5], b"10001");

        assert_eq!(decimal::u32toa(65535, &mut buffer), 5);
        assert_eq!(&buffer[..5], b"65535");

        assert_eq!(decimal::u32toa(99999, &mut buffer), 5);
        assert_eq!(&buffer[..5], b"99999");

        assert_eq!(decimal::u32toa(100001, &mut buffer), 6);
        assert_eq!(&buffer[..6], b"100001");

        assert_eq!(decimal::u32toa(999999, &mut buffer), 6);
        assert_eq!(&buffer[..6], b"999999");

        assert_eq!(decimal::u32toa(1000001, &mut buffer), 7);
        assert_eq!(&buffer[..7], b"1000001");

        assert_eq!(decimal::u32toa(9999999, &mut buffer), 7);
        assert_eq!(&buffer[..7], b"9999999");

        assert_eq!(decimal::u32toa(10000001, &mut buffer), 8);
        assert_eq!(&buffer[..8], b"10000001");

        assert_eq!(decimal::u32toa(99999999, &mut buffer), 8);
        assert_eq!(&buffer[..8], b"99999999");

        assert_eq!(decimal::u32toa(100000001, &mut buffer), 9);
        assert_eq!(&buffer[..9], b"100000001");

        assert_eq!(decimal::u32toa(999999999, &mut buffer), 9);
        assert_eq!(&buffer[..9], b"999999999");

        assert_eq!(decimal::u32toa(1000000001, &mut buffer), 10);
        assert_eq!(&buffer[..10], b"1000000001");

        assert_eq!(decimal::u32toa(4294967295, &mut buffer), 10);
        assert_eq!(&buffer[..10], b"4294967295");
    }
}

#[test]
fn u64toa_test() {
    let mut buffer = [b'0'; 32];
    unsafe {
        assert_eq!(decimal::u64toa(5, &mut buffer), 1);
        assert_eq!(&buffer[..1], b"5");

        assert_eq!(decimal::u64toa(11, &mut buffer), 2);
        assert_eq!(&buffer[..2], b"11");

        assert_eq!(decimal::u64toa(99, &mut buffer), 2);
        assert_eq!(&buffer[..2], b"99");

        assert_eq!(decimal::u64toa(101, &mut buffer), 3);
        assert_eq!(&buffer[..3], b"101");

        assert_eq!(decimal::u64toa(999, &mut buffer), 3);
        assert_eq!(&buffer[..3], b"999");

        assert_eq!(decimal::u64toa(1001, &mut buffer), 4);
        assert_eq!(&buffer[..4], b"1001");

        assert_eq!(decimal::u64toa(9999, &mut buffer), 4);
        assert_eq!(&buffer[..4], b"9999");

        assert_eq!(decimal::u64toa(10001, &mut buffer), 5);
        assert_eq!(&buffer[..5], b"10001");

        assert_eq!(decimal::u64toa(65535, &mut buffer), 5);
        assert_eq!(&buffer[..5], b"65535");

        assert_eq!(decimal::u64toa(99999, &mut buffer), 5);
        assert_eq!(&buffer[..5], b"99999");

        assert_eq!(decimal::u64toa(100001, &mut buffer), 6);
        assert_eq!(&buffer[..6], b"100001");

        assert_eq!(decimal::u64toa(999999, &mut buffer), 6);
        assert_eq!(&buffer[..6], b"999999");

        assert_eq!(decimal::u64toa(1000001, &mut buffer), 7);
        assert_eq!(&buffer[..7], b"1000001");

        assert_eq!(decimal::u64toa(9999999, &mut buffer), 7);
        assert_eq!(&buffer[..7], b"9999999");

        assert_eq!(decimal::u64toa(10000001, &mut buffer), 8);
        assert_eq!(&buffer[..8], b"10000001");

        assert_eq!(decimal::u64toa(99999999, &mut buffer), 8);
        assert_eq!(&buffer[..8], b"99999999");

        assert_eq!(decimal::u64toa(100000001, &mut buffer), 9);
        assert_eq!(&buffer[..9], b"100000001");

        assert_eq!(decimal::u64toa(999999999, &mut buffer), 9);
        assert_eq!(&buffer[..9], b"999999999");

        assert_eq!(decimal::u64toa(1000000001, &mut buffer), 10);
        assert_eq!(&buffer[..10], b"1000000001");

        assert_eq!(decimal::u64toa(9999999999, &mut buffer), 10);
        assert_eq!(&buffer[..10], b"9999999999");

        assert_eq!(decimal::u64toa(10000000001, &mut buffer), 11);
        assert_eq!(&buffer[..11], b"10000000001");

        assert_eq!(decimal::u64toa(99999999999, &mut buffer), 11);
        assert_eq!(&buffer[..11], b"99999999999");

        assert_eq!(decimal::u64toa(100000000001, &mut buffer), 12);
        assert_eq!(&buffer[..12], b"100000000001");

        assert_eq!(decimal::u64toa(999999999999, &mut buffer), 12);
        assert_eq!(&buffer[..12], b"999999999999");

        assert_eq!(decimal::u64toa(1000000000001, &mut buffer), 13);
        assert_eq!(&buffer[..13], b"1000000000001");

        assert_eq!(decimal::u64toa(9999999999999, &mut buffer), 13);
        assert_eq!(&buffer[..13], b"9999999999999");

        assert_eq!(decimal::u64toa(10000000000001, &mut buffer), 14);
        assert_eq!(&buffer[..14], b"10000000000001");

        assert_eq!(decimal::u64toa(99999999999999, &mut buffer), 14);
        assert_eq!(&buffer[..14], b"99999999999999");

        assert_eq!(decimal::u64toa(100000000000001, &mut buffer), 15);
        assert_eq!(&buffer[..15], b"100000000000001");

        assert_eq!(decimal::u64toa(999999999999999, &mut buffer), 15);
        assert_eq!(&buffer[..15], b"999999999999999");

        assert_eq!(decimal::u64toa(1000000000000001, &mut buffer), 16);
        assert_eq!(&buffer[..16], b"1000000000000001");

        assert_eq!(decimal::u64toa(9999999999999999, &mut buffer), 16);
        assert_eq!(&buffer[..16], b"9999999999999999");

        assert_eq!(decimal::u64toa(10000000000000001, &mut buffer), 17);
        assert_eq!(&buffer[..17], b"10000000000000001");

        assert_eq!(decimal::u64toa(99999999999999999, &mut buffer), 17);
        assert_eq!(&buffer[..17], b"99999999999999999");

        assert_eq!(decimal::u64toa(100000000000000001, &mut buffer), 18);
        assert_eq!(&buffer[..18], b"100000000000000001");

        assert_eq!(decimal::u64toa(999999999999999999, &mut buffer), 18);
        assert_eq!(&buffer[..18], b"999999999999999999");

        assert_eq!(decimal::u64toa(1000000000000000001, &mut buffer), 19);
        assert_eq!(&buffer[..19], b"1000000000000000001");

        assert_eq!(decimal::u64toa(9999999999999999999, &mut buffer), 19);
        assert_eq!(&buffer[..19], b"9999999999999999999");

        assert_eq!(decimal::u64toa(10000000000000000001, &mut buffer), 20);
        assert_eq!(&buffer[..20], b"10000000000000000001");

        assert_eq!(decimal::u64toa(18446744073709551615, &mut buffer), 20);
        assert_eq!(&buffer[..20], b"18446744073709551615");
    }
}

#[test]
fn u128toa_test() {
    let mut buffer = [b'0'; 48];
    unsafe {
        assert_eq!(decimal::u128toa(5, &mut buffer), 1);
        assert_eq!(&buffer[..1], b"5");

        assert_eq!(decimal::u128toa(11, &mut buffer), 2);
        assert_eq!(&buffer[..2], b"11");

        assert_eq!(decimal::u128toa(99, &mut buffer), 2);
        assert_eq!(&buffer[..2], b"99");

        assert_eq!(decimal::u128toa(101, &mut buffer), 3);
        assert_eq!(&buffer[..3], b"101");

        assert_eq!(decimal::u128toa(999, &mut buffer), 3);
        assert_eq!(&buffer[..3], b"999");

        assert_eq!(decimal::u128toa(1001, &mut buffer), 4);
        assert_eq!(&buffer[..4], b"1001");

        assert_eq!(decimal::u128toa(9999, &mut buffer), 4);
        assert_eq!(&buffer[..4], b"9999");

        assert_eq!(decimal::u128toa(10001, &mut buffer), 5);
        assert_eq!(&buffer[..5], b"10001");

        assert_eq!(decimal::u128toa(65535, &mut buffer), 5);
        assert_eq!(&buffer[..5], b"65535");

        assert_eq!(decimal::u128toa(99999, &mut buffer), 5);
        assert_eq!(&buffer[..5], b"99999");

        assert_eq!(decimal::u128toa(100001, &mut buffer), 6);
        assert_eq!(&buffer[..6], b"100001");

        assert_eq!(decimal::u128toa(999999, &mut buffer), 6);
        assert_eq!(&buffer[..6], b"999999");

        assert_eq!(decimal::u128toa(1000001, &mut buffer), 7);
        assert_eq!(&buffer[..7], b"1000001");

        assert_eq!(decimal::u128toa(9999999, &mut buffer), 7);
        assert_eq!(&buffer[..7], b"9999999");

        assert_eq!(decimal::u128toa(10000001, &mut buffer), 8);
        assert_eq!(&buffer[..8], b"10000001");

        assert_eq!(decimal::u128toa(99999999, &mut buffer), 8);
        assert_eq!(&buffer[..8], b"99999999");

        assert_eq!(decimal::u128toa(100000001, &mut buffer), 9);
        assert_eq!(&buffer[..9], b"100000001");

        assert_eq!(decimal::u128toa(999999999, &mut buffer), 9);
        assert_eq!(&buffer[..9], b"999999999");

        assert_eq!(decimal::u128toa(1000000001, &mut buffer), 10);
        assert_eq!(&buffer[..10], b"1000000001");

        assert_eq!(decimal::u128toa(9999999999, &mut buffer), 10);
        assert_eq!(&buffer[..10], b"9999999999");

        assert_eq!(decimal::u128toa(10000000001, &mut buffer), 11);
        assert_eq!(&buffer[..11], b"10000000001");

        assert_eq!(decimal::u128toa(99999999999, &mut buffer), 11);
        assert_eq!(&buffer[..11], b"99999999999");

        assert_eq!(decimal::u128toa(100000000001, &mut buffer), 12);
        assert_eq!(&buffer[..12], b"100000000001");

        assert_eq!(decimal::u128toa(999999999999, &mut buffer), 12);
        assert_eq!(&buffer[..12], b"999999999999");

        assert_eq!(decimal::u128toa(1000000000001, &mut buffer), 13);
        assert_eq!(&buffer[..13], b"1000000000001");

        assert_eq!(decimal::u128toa(9999999999999, &mut buffer), 13);
        assert_eq!(&buffer[..13], b"9999999999999");

        assert_eq!(decimal::u128toa(10000000000001, &mut buffer), 14);
        assert_eq!(&buffer[..14], b"10000000000001");

        assert_eq!(decimal::u128toa(99999999999999, &mut buffer), 14);
        assert_eq!(&buffer[..14], b"99999999999999");

        assert_eq!(decimal::u128toa(100000000000001, &mut buffer), 15);
        assert_eq!(&buffer[..15], b"100000000000001");

        assert_eq!(decimal::u128toa(999999999999999, &mut buffer), 15);
        assert_eq!(&buffer[..15], b"999999999999999");

        assert_eq!(decimal::u128toa(1000000000000001, &mut buffer), 16);
        assert_eq!(&buffer[..16], b"1000000000000001");

        assert_eq!(decimal::u128toa(9999999999999999, &mut buffer), 16);
        assert_eq!(&buffer[..16], b"9999999999999999");

        assert_eq!(decimal::u128toa(10000000000000001, &mut buffer), 17);
        assert_eq!(&buffer[..17], b"10000000000000001");

        assert_eq!(decimal::u128toa(99999999999999999, &mut buffer), 17);
        assert_eq!(&buffer[..17], b"99999999999999999");

        assert_eq!(decimal::u128toa(100000000000000001, &mut buffer), 18);
        assert_eq!(&buffer[..18], b"100000000000000001");

        assert_eq!(decimal::u128toa(999999999999999999, &mut buffer), 18);
        assert_eq!(&buffer[..18], b"999999999999999999");

        assert_eq!(decimal::u128toa(1000000000000000001, &mut buffer), 19);
        assert_eq!(&buffer[..19], b"1000000000000000001");

        assert_eq!(decimal::u128toa(9999999999999999999, &mut buffer), 19);
        assert_eq!(&buffer[..19], b"9999999999999999999");

        assert_eq!(decimal::u128toa(10000000000000000001, &mut buffer), 20);
        assert_eq!(&buffer[..20], b"10000000000000000001");

        assert_eq!(decimal::u128toa(999999999999999999999999, &mut buffer), 24);
        assert_eq!(&buffer[..24], b"999999999999999999999999");

        assert_eq!(decimal::u128toa(1000000000000000000000001, &mut buffer), 25);
        assert_eq!(&buffer[..25], b"1000000000000000000000001");

        assert_eq!(decimal::u128toa(66620387370000000000000000000, &mut buffer), 29);
        assert_eq!(&buffer[..29], b"66620387370000000000000000000");

        assert_eq!(decimal::u128toa(99999999999999999999999999999, &mut buffer), 29);
        assert_eq!(&buffer[..29], b"99999999999999999999999999999");

        assert_eq!(decimal::u128toa(100000000000000000000000000001, &mut buffer), 30);
        assert_eq!(&buffer[..30], b"100000000000000000000000000001");

        assert_eq!(decimal::u128toa(9999999999999999999999999999999999, &mut buffer), 34);
        assert_eq!(&buffer[..34], b"9999999999999999999999999999999999");

        assert_eq!(decimal::u128toa(10000000000000000000000000000000001, &mut buffer), 35);
        assert_eq!(&buffer[..35], b"10000000000000000000000000000000001");

        assert_eq!(decimal::u128toa(340282366920938463463374607431768211455, &mut buffer), 39);
        assert_eq!(&buffer[..39], b"340282366920938463463374607431768211455");
    }
}

fn slow_log2(x: u32) -> usize {
    // Slow approach to calculating a log2, using floats.
    if x == 0 {
        0
    } else {
        (x as f64).log2().floor() as usize
    }
}

fn slow_digit_count<T: UnsignedInteger>(x: T) -> usize {
    x.as_u128().to_string().len()
}

quickcheck! {
    fn fast_log2_quickcheck(x: u32) -> bool {
        slow_log2(x) == decimal::fast_log2(x)
    }

    fn fast_digit_count_quickcheck(x: u32) -> bool {
        slow_digit_count(x) == decimal::fast_digit_count(x)
    }

    fn fallback_digit_count_quickcheck(x: u128) -> bool {
        slow_digit_count(x) == decimal::fallback_digit_count(x)
    }

    fn u8toa_quickcheck(x: u8) -> bool {
        let actual = x.to_string();
        let mut buffer = [b'0'; 16];
        actual.len() == unsafe { decimal::u8toa(x, &mut buffer) } &&
            &buffer[..actual.len()] == actual.as_bytes()
    }

    fn u16toa_quickcheck(x: u16) -> bool {
        let actual = x.to_string();
        let mut buffer = [b'0'; 16];
        actual.len() == unsafe { decimal::u16toa(x, &mut buffer) } &&
            &buffer[..actual.len()] == actual.as_bytes()
    }

    fn u32toa_quickcheck(x: u32) -> bool {
        let actual = x.to_string();
        let mut buffer = [b'0'; 16];
        actual.len() == unsafe { decimal::u32toa(x, &mut buffer) } &&
            &buffer[..actual.len()] == actual.as_bytes()
    }

    fn u64toa_quickcheck(x: u64) -> bool {
        let actual = x.to_string();
        let mut buffer = [b'0'; 32];
        actual.len() == unsafe { decimal::u64toa(x, &mut buffer) } &&
            &buffer[..actual.len()] == actual.as_bytes()
    }

    fn u128toa_quickcheck(x: u128) -> bool {
        let actual = x.to_string();
        let mut buffer = [b'0'; 48];
        actual.len() == unsafe { decimal::u128toa(x, &mut buffer) } &&
            &buffer[..actual.len()] == actual.as_bytes()
    }
}
