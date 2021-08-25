#![cfg(not(feature = "compact"))]

use lexical_util::constants::BUFFER_SIZE;
use lexical_util::format::NumberFormatBuilder;
use lexical_util::num::Float;
use lexical_write_float::algorithm::DragonboxFloat;
use lexical_write_float::float::{ExtendedFloat80, RawFloat};
use lexical_write_float::{algorithm, Options};
use proptest::prelude::*;
use quickcheck::quickcheck;

const DECIMAL: u128 = NumberFormatBuilder::decimal();

fn floor_shift(integer: u32, fraction: u64, shift: i32) -> i32 {
    ((integer << shift) | (fraction >> (64 - shift)) as u32) as i32
}

fn dragonbox_log5_2(q: i32) -> i32 {
    let c = floor_shift(0, 0x6e40d1a4143dcb94, 20);
    let s = floor_shift(0, 0, 20);
    (q * c - s) >> 20
}

fn dragonbox_log10_2(q: i32) -> i32 {
    let c = floor_shift(0, 0x4d104d427de7fbcc, 22);
    let s = floor_shift(0, 0, 22);
    (q * c - s) >> 22
}

fn dragonbox_log2_10(q: i32) -> i32 {
    let c = floor_shift(3, 0x5269e12f346e2bf9, 19);
    let s = floor_shift(0, 0, 19);
    (q * c - s) >> 19
}

fn dragonbox_log5_2_sub_log5_3(q: i32) -> i32 {
    let c = floor_shift(0, 0x6e40d1a4143dcb94, 20);
    let s = floor_shift(0, 0xaebf47915d443b24, 20);
    (q * c - s) >> 20
}

fn dragonbox_log10_2_sub_log10_4_div3(q: i32) -> i32 {
    let c = floor_shift(0, 0x4d104d427de7fbcc, 22);
    let s = floor_shift(0, 0x1ffbfc2bbc780375, 22);
    (q * c - s) >> 22
}

#[test]
fn floor_log5_pow2_test() {
    for q in -1492i32..=1492 {
        let actual = algorithm::floor_log5_pow2(q);
        let expected = dragonbox_log5_2(q);
        assert_eq!(actual, expected);
    }
}

#[test]
fn floor_log10_pow2_test() {
    for q in -1700i32..=1700 {
        let actual = algorithm::floor_log10_pow2(q);
        let expected = dragonbox_log10_2(q);
        assert_eq!(actual, expected);
    }
}

#[test]
fn floor_log2_pow10_test() {
    for q in -1233i32..=1233 {
        let actual = algorithm::floor_log2_pow10(q);
        let expected = dragonbox_log2_10(q);
        assert_eq!(actual, expected);
    }
}

#[test]
fn floor_log5_pow2_minus_log5_3_test() {
    for q in -2427i32..=2427 {
        let actual = algorithm::floor_log5_pow2_minus_log5_3(q);
        let expected = dragonbox_log5_2_sub_log5_3(q);
        assert_eq!(actual, expected);
    }
}

#[test]
fn floor_log10_pow2_minus_log10_4_over_3_test() {
    for q in -1700i32..=1700 {
        let actual = algorithm::floor_log10_pow2_minus_log10_4_over_3(q);
        let expected = dragonbox_log10_2_sub_log10_4_div3(q);
        assert_eq!(actual, expected);
    }
}

#[test]
fn max_power_test() {
    assert_eq!(algorithm::max_power::<f32>(), 7);
    assert_eq!(algorithm::max_power::<f64>(), 16);
}

#[test]
fn pow32_test() {
    assert_eq!(algorithm::pow32(10, 1), 10);
    assert_eq!(algorithm::pow32(10, 2), 100);
}

#[test]
fn pow64_test() {
    assert_eq!(algorithm::pow64(10, 1), 10);
    assert_eq!(algorithm::pow64(10, 2), 100);
}

#[test]
fn count_factors_test() {
    assert_eq!(algorithm::count_factors(5, 25), 2);
    assert_eq!(algorithm::count_factors(5, 30), 1);
    assert_eq!(algorithm::count_factors(5, 125), 3);
    assert_eq!(algorithm::count_factors(5, 126), 0);
}

#[test]
fn floor_log2_test() {
    assert_eq!(algorithm::floor_log2(25), 4);
    assert_eq!(algorithm::floor_log2(30), 4);
    assert_eq!(algorithm::floor_log2(125), 6);
    assert_eq!(algorithm::floor_log2(126), 6);
    assert_eq!(algorithm::floor_log2(128), 7);
}

fn to_decimal_f32(float: f32) -> (u64, i32) {
    let fp = algorithm::to_decimal(float);
    (fp.mant, fp.exp)
}

fn to_decimal_f64(float: f64) -> (u64, i32) {
    let fp = algorithm::to_decimal(float);
    (fp.mant, fp.exp)
}

#[test]
fn to_decimal_test() {
    assert_eq!(to_decimal_f32(0.0), (0, 0));
    assert_eq!(to_decimal_f32(0.5), (5, -1));
    assert_eq!(to_decimal_f32(1.0), (1, 0));
    assert_eq!(to_decimal_f32(1.5), (15, -1));
    assert_eq!(to_decimal_f32(1.23456), (123456, -5));
    assert_eq!(to_decimal_f32(2.3786281e+38), (23786281, 31));
    assert_eq!(to_decimal_f32(2147481600.0), (21474816, 2));
    assert_eq!(to_decimal_f32(2147483600.0), (21474836, 2));

    assert_eq!(to_decimal_f64(0.0), (0, 0));
    assert_eq!(to_decimal_f64(0.5), (5000000000000000, -16));
    assert_eq!(to_decimal_f64(1.0), (1000000000000000, -15));
    assert_eq!(to_decimal_f64(1.5), (1500000000000000, -15));
    assert_eq!(to_decimal_f64(1.23456), (1234560000000000, -15));
}

fn write_digits_f32(buffer: &mut [u8], value: u64, expected: &str) {
    let count = unsafe { f32::write_digits(buffer, value) };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_digits_f32_test() {
    let mut buffer = [b'\x00'; 32];
    write_digits_f32(&mut buffer, 0, "0");
    write_digits_f32(&mut buffer, 1, "1");
    write_digits_f32(&mut buffer, 11, "11");
    write_digits_f32(&mut buffer, 23, "23");
    write_digits_f32(&mut buffer, 23786281, "23786281");
    write_digits_f32(&mut buffer, 4294967295, "4294967295");
}

fn write_digits_f64(buffer: &mut [u8], value: u64, expected: &str) {
    let count = unsafe { f64::write_digits(buffer, value) };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_digits_f64_test() {
    let mut buffer = [b'\x00'; 32];
    write_digits_f64(&mut buffer, 0, "0");
    write_digits_f64(&mut buffer, 1, "1");
    write_digits_f64(&mut buffer, 10, "1");
    write_digits_f64(&mut buffer, 11, "11");
    write_digits_f64(&mut buffer, 110, "11");
    write_digits_f64(&mut buffer, 23, "23");
    write_digits_f64(&mut buffer, 230, "23");
    write_digits_f64(&mut buffer, 4294967295, "4294967295");
    write_digits_f64(&mut buffer, 42949672950, "4294967295");
}

#[test]
fn write_float_scientific_test() {
    let options = Options::new();
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let decimal = ExtendedFloat80 {
        mant: 1,
        exp: 0,
    };
    let count = unsafe {
        algorithm::write_float_scientific::<f64, DECIMAL>(&mut buffer, decimal, 0, &options)
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, "1.0e0");

    let options = Options::builder().trim_floats(true).build().unwrap();
    let count = unsafe {
        algorithm::write_float_scientific::<f64, DECIMAL>(&mut buffer, decimal, 0, &options)
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, "1e0");
}

#[test]
fn write_float_positive_exponent_test() {
    let options = Options::new();
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let decimal = ExtendedFloat80 {
        mant: 1,
        exp: 0,
    };
    let count = unsafe {
        algorithm::write_float_positive_exponent::<f64, DECIMAL>(&mut buffer, decimal, 0, &options)
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, "1.0");

    let options = Options::builder().trim_floats(true).build().unwrap();
    let count = unsafe {
        algorithm::write_float_positive_exponent::<f64, DECIMAL>(&mut buffer, decimal, 0, &options)
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, "1");
}

#[test]
fn write_float_negative_exponent_test() {
    let options = Options::new();
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let decimal = ExtendedFloat80 {
        mant: 1,
        exp: -1,
    };
    let count = unsafe {
        algorithm::write_float_negative_exponent::<f64, DECIMAL>(&mut buffer, decimal, -1, &options)
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, "0.1");

    let options = Options::builder().trim_floats(true).build().unwrap();
    let count = unsafe {
        algorithm::write_float_negative_exponent::<f64, DECIMAL>(&mut buffer, decimal, -1, &options)
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, "0.1");
}

// Test data for roundtrips.
const F32_DATA: [f32; 31] = [
    0.,
    0.1,
    1.,
    1.1,
    12.,
    12.1,
    123.,
    123.1,
    1234.,
    1234.1,
    12345.,
    12345.1,
    123456.,
    123456.1,
    1234567.,
    1234567.1,
    12345678.,
    12345678.1,
    123456789.,
    123456789.1,
    123456789.12,
    123456789.123,
    123456789.1234,
    123456789.12345,
    1.2345678912345e8,
    1.2345e+8,
    1.2345e+11,
    1.2345e+38,
    1.2345e-8,
    1.2345e-11,
    1.2345e-38,
];
const F64_DATA: [f64; 33] = [
    0.,
    0.1,
    1.,
    1.1,
    12.,
    12.1,
    123.,
    123.1,
    1234.,
    1234.1,
    12345.,
    12345.1,
    123456.,
    123456.1,
    1234567.,
    1234567.1,
    12345678.,
    12345678.1,
    123456789.,
    123456789.1,
    123456789.12,
    123456789.123,
    123456789.1234,
    123456789.12345,
    1.2345678912345e8,
    1.2345e+8,
    1.2345e+11,
    1.2345e+38,
    1.2345e+308,
    1.2345e-8,
    1.2345e-11,
    1.2345e-38,
    1.2345e-299,
];

fn write_float<T: RawFloat, const FORMAT: u128>(f: T, options: &Options, expected: &str) {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let count = unsafe { algorithm::write_float::<_, FORMAT>(f, &mut buffer, options) };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn f32_test() {
    let options = Options::builder().trim_floats(true).build().unwrap();
    write_float::<_, DECIMAL>(0.0f32, &options, "0");
    write_float::<_, DECIMAL>(1.0f32, &options, "1");
    write_float::<_, DECIMAL>(10.0f32, &options, "10");
    write_float::<_, DECIMAL>(10.0f32, &options, "10");
    write_float::<_, DECIMAL>(1.2345678901234567890e0f32, &options, "1.2345679");
    write_float::<_, DECIMAL>(1.2345678901234567890e1f32, &options, "12.345679");
    write_float::<_, DECIMAL>(1.2345678901234567890e2f32, &options, "123.45679");
    write_float::<_, DECIMAL>(1.2345678901234567890e3f32, &options, "1234.5679");
    write_float::<_, DECIMAL>(2.3786281e+38f32, &options, "2.3786281e38");

    let options = Options::new();
    write_float::<_, DECIMAL>(2.3786281e+38f32, &options, "2.3786281e38");
}

#[test]
fn f32_errors_test() {
    // Errors discovered via quickcheck.
    let options = Options::new();
    write_float::<_, DECIMAL>(0.0f32, &options, "0.0");
    write_float::<_, DECIMAL>(1073741800.0f32, &options, "1073741800.0");
    write_float::<_, DECIMAL>(1610612700.0f32, &options, "1610612700.0");
    write_float::<_, DECIMAL>(1879048200.0f32, &options, "1879048200.0");
    write_float::<_, DECIMAL>(2013265900.0f32, &options, "2013265900.0");
    write_float::<_, DECIMAL>(2080374800.0f32, &options, "2080374800.0");
    write_float::<_, DECIMAL>(2113929200.0f32, &options, "2113929200.0");
    write_float::<_, DECIMAL>(2130706400.0f32, &options, "2130706400.0");
    write_float::<_, DECIMAL>(2139095000.0f32, &options, "2139095000.0");
    write_float::<_, DECIMAL>(2143289300.0f32, &options, "2143289300.0");
    write_float::<_, DECIMAL>(2145386500.0f32, &options, "2145386500.0");
    write_float::<_, DECIMAL>(2146435100.0f32, &options, "2146435100.0");
    write_float::<_, DECIMAL>(2146959400.0f32, &options, "2146959400.0");
    write_float::<_, DECIMAL>(2147221500.0f32, &options, "2147221500.0");
    write_float::<_, DECIMAL>(2147352600.0f32, &options, "2147352600.0");
    write_float::<_, DECIMAL>(2147418100.0f32, &options, "2147418100.0");
    write_float::<_, DECIMAL>(2147450900.0f32, &options, "2147450900.0");
    write_float::<_, DECIMAL>(2147467300.0f32, &options, "2147467300.0");
    write_float::<_, DECIMAL>(2147475500.0f32, &options, "2147475500.0");
    write_float::<_, DECIMAL>(2147479600.0f32, &options, "2147479600.0");
    write_float::<_, DECIMAL>(2147481600.0f32, &options, "2147481600.0");
    write_float::<_, DECIMAL>(2147482600.0f32, &options, "2147482600.0");
    write_float::<_, DECIMAL>(2147483100.0f32, &options, "2147483100.0");
    write_float::<_, DECIMAL>(2147483400.0f32, &options, "2147483400.0");
    write_float::<_, DECIMAL>(2147483500.0f32, &options, "2147483500.0");
    write_float::<_, DECIMAL>(2147483600.0f32, &options, "2147483600.0");
}

#[test]
fn f32_roundtrip_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let options = Options::builder().build().unwrap();
    for &float in F32_DATA.iter() {
        let count = unsafe { algorithm::write_float::<_, DECIMAL>(float, &mut buffer, &options) };
        let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
        let roundtrip = actual.parse::<f32>();
        assert_eq!(roundtrip, Ok(float));
    }
}

#[test]
fn f64_test() {
    let options = Options::builder().trim_floats(true).build().unwrap();
    write_float::<_, DECIMAL>(0.0f64, &options, "0");
    write_float::<_, DECIMAL>(1.0f64, &options, "1");
    write_float::<_, DECIMAL>(10.0f64, &options, "10");
    write_float::<_, DECIMAL>(10.0f64, &options, "10");
    write_float::<_, DECIMAL>(1.2345678901234567890e0f64, &options, "1.2345678901234567");
    write_float::<_, DECIMAL>(1.2345678901234567890e1f64, &options, "12.345678901234567");
    write_float::<_, DECIMAL>(1.2345678901234567890e2f64, &options, "123.45678901234568");
    write_float::<_, DECIMAL>(1.2345678901234567890e3f64, &options, "1234.567890123457");
}

#[test]
fn f64_roundtrip_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let options = Options::builder().build().unwrap();
    for &float in F64_DATA.iter() {
        let count = unsafe { algorithm::write_float::<_, DECIMAL>(float, &mut buffer, &options) };
        let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
        let roundtrip = actual.parse::<f64>();
        assert_eq!(roundtrip, Ok(float));
    }
}

quickcheck! {
    #[cfg_attr(miri, ignore)]
    fn f32_quickcheck(f: f32) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        let f = f.abs();
        if f.is_special() {
            true
        } else {
            let count = unsafe { algorithm::write_float::<_, DECIMAL>(f, &mut buffer, &options) };
            let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
            let roundtrip = actual.parse::<f32>();
            roundtrip == Ok(f)
        }
    }

    #[cfg_attr(miri, ignore)]
    fn f64_quickcheck(f: f64) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        let f = f.abs();
        if f.is_special() {
            true
        } else {
            let count = unsafe { algorithm::write_float::<_, DECIMAL>(f, &mut buffer, &options) };
            let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
            let roundtrip = actual.parse::<f64>();
            roundtrip == Ok(f)
        }
    }
}

proptest! {
    #[test]
    #[cfg_attr(miri, ignore)]
    fn f32_proptest(f in f32::MIN..f32::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        let f = f.abs();
        if !f.is_special() {
            let count = unsafe { algorithm::write_float::<_, DECIMAL>(f, &mut buffer, &options) };
            let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
            let roundtrip = actual.parse::<f32>();
            prop_assert_eq!(roundtrip, Ok(f))
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn f64_proptest(f in f64::MIN..f64::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        let f = f.abs();
        if !f.is_special() {
            let count = unsafe { algorithm::write_float::<_, DECIMAL>(f, &mut buffer, &options) };
            let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
            let roundtrip = actual.parse::<f64>();
            prop_assert_eq!(roundtrip, Ok(f))
        }
    }
}
