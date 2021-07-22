use lexical_write_integer::decimal;
use quickcheck::quickcheck;

#[test]
fn fast_log2_test() {
    // Check the first, even if illogical case works.
    assert_eq!(decimal::fast_log2(0), 0);
    assert_eq!(decimal::fast_log2(1), 0);
    assert_eq!(decimal::fast_log2(2), 1);
    assert_eq!(decimal::fast_log2(3), 1);

    assert_eq!(decimal::fast_log2((1 << 16) - 1), 15);
    assert_eq!(decimal::fast_log2(1 << 16), 16);
    assert_eq!(decimal::fast_log2((1 << 16) + 1), 16);

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

fn slow_log2(x: u32) -> usize {
    // Slow approach to calculating a log2, using floats.
    if x == 0 {
        0
    } else {
        (x as f64).log2().floor() as usize
    }
}

fn slow_digit_count(x: u32) -> usize {
    if x < 10 {
        1
    } else if x < 100 {
        2
    } else if x < 1000 {
        3
    } else if x < 10000 {
        4
    } else if x < 100000 {
        5
    } else if x < 1000000 {
        6
    } else if x < 10000000 {
        7
    } else if x < 100000000 {
        8
    } else if x < 1000000000 {
        9
    } else {
        10
    }
}

quickcheck! {
    fn fast_log2_quickcheck(x: u32) -> bool {
        slow_log2(x) == decimal::fast_log2(x)
    }

    fn fast_digit_count_quickcheck(x: u32) -> bool {
        slow_digit_count(x) == decimal::fast_digit_count(x)
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
}
