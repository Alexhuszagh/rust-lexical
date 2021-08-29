#![cfg(feature = "compact")]

use core::num;
use lexical_util::constants::BUFFER_SIZE;
use lexical_util::format::NumberFormatBuilder;
use lexical_util::num::Float;
use lexical_write_float::float::{ExtendedFloat80, RawFloat};
use lexical_write_float::{compact, Options, RoundMode};
use proptest::prelude::*;
use quickcheck::quickcheck;

const DECIMAL: u128 = NumberFormatBuilder::decimal();

fn check_normalize(mant: u64, exp: i32, ymant: u64, yexp: i32) {
    let mut x = ExtendedFloat80 {
        mant,
        exp,
    };
    if x.mant != 0 {
        assert_eq!(x.mant & (1 << 63), 0);
        compact::normalize(&mut x);
        assert_eq!(x.mant & (1 << 63), 1 << 63);
    }
    assert_eq!(
        x,
        ExtendedFloat80 {
            mant: ymant,
            exp: yexp
        }
    );
}

#[test]
fn normalize_test() {
    // f32 cases
    check_normalize(0, 0, 0, 0);
    check_normalize(1, -149, 9223372036854775808, -212);
    check_normalize(71362, -149, 10043308644012916736, -196);
    check_normalize(12379400, -90, 13611294244890214400, -130);
    check_normalize(8388608, -23, 9223372036854775808, -63);
    check_normalize(11368684, 43, 12500000250510966784, 3);
    check_normalize(16777213, 104, 18446740775174668288, 64);

    // Test a few cases from radix float writer errors.
    check_normalize(5178144, -22, 11386859076597055488, -63);

    // f64 cases
    check_normalize(1, -1074, 9223372036854775808, -1137);
    check_normalize(6448907850777164, -883, 13207363278391631872, -894);
    check_normalize(7371020360979573, -551, 15095849699286165504, -562);
    check_normalize(6427752177035961, -202, 13164036458569648128, -213);
    check_normalize(4903985730770844, -185, 10043362776618688512, -196);
    check_normalize(6646139978924579, -119, 13611294676837537792, -130);
    check_normalize(4503599627370496, -52, 9223372036854775808, -63);
    check_normalize(6103515625000000, 14, 12500000000000000000, 3);
    check_normalize(8271806125530277, 80, 16940658945086007296, 69);
    check_normalize(5503284107318959, 446, 11270725851789228032, 435);
    check_normalize(6290184345309700, 778, 12882297539194265600, 767);
    check_normalize(9007199254740991, 971, 18446744073709549568, 960);

    // Check with errors from power-of-two.
    check_normalize(72057594037927936, -1078, 9223372036854775808, -1085);
}

#[test]
fn normalized_boundaries_test() {
    let fp = ExtendedFloat80 {
        mant: 4503599627370496,
        exp: -50,
    };
    let u = ExtendedFloat80 {
        mant: 9223372036854775296,
        exp: -61,
    };
    let l = ExtendedFloat80 {
        mant: 9223372036854776832,
        exp: -61,
    };
    let (upper, lower) = compact::normalized_boundaries::<f64>(&fp);
    assert_eq!(upper, u);
    assert_eq!(lower, l);
}

#[test]
fn from_f32_test() {
    assert_eq!(
        compact::from_float(0.0f32),
        ExtendedFloat80 {
            mant: 0,
            exp: -149
        }
    );
    assert_eq!(
        compact::from_float(-0.0f32),
        ExtendedFloat80 {
            mant: 0,
            exp: -149
        }
    );
    assert_eq!(
        compact::from_float(1e-45f32),
        ExtendedFloat80 {
            mant: 1,
            exp: -149
        }
    );
    assert_eq!(
        compact::from_float(1e-40f32),
        ExtendedFloat80 {
            mant: 71362,
            exp: -149
        }
    );
    assert_eq!(
        compact::from_float(2e-40f32),
        ExtendedFloat80 {
            mant: 142725,
            exp: -149
        }
    );
    assert_eq!(
        compact::from_float(1e-20f32),
        ExtendedFloat80 {
            mant: 12379400,
            exp: -90
        }
    );
    assert_eq!(
        compact::from_float(2e-20f32),
        ExtendedFloat80 {
            mant: 12379400,
            exp: -89
        }
    );
    assert_eq!(
        compact::from_float(1.0f32),
        ExtendedFloat80 {
            mant: 8388608,
            exp: -23
        }
    );
    assert_eq!(
        compact::from_float(2.0f32),
        ExtendedFloat80 {
            mant: 8388608,
            exp: -22
        }
    );
    assert_eq!(
        compact::from_float(1e20f32),
        ExtendedFloat80 {
            mant: 11368684,
            exp: 43
        }
    );
    assert_eq!(
        compact::from_float(2e20f32),
        ExtendedFloat80 {
            mant: 11368684,
            exp: 44
        }
    );
    assert_eq!(
        compact::from_float(3.402823e38f32),
        ExtendedFloat80 {
            mant: 16777213,
            exp: 104
        }
    );
}

#[test]
fn from_f64_test() {
    assert_eq!(
        compact::from_float(0.0f64),
        ExtendedFloat80 {
            mant: 0,
            exp: -1074
        }
    );
    assert_eq!(
        compact::from_float(-0.0f64),
        ExtendedFloat80 {
            mant: 0,
            exp: -1074
        }
    );
    assert_eq!(
        compact::from_float(5e-324f64),
        ExtendedFloat80 {
            mant: 1,
            exp: -1074
        }
    );
    assert_eq!(
        compact::from_float(1e-250f64),
        ExtendedFloat80 {
            mant: 6448907850777164,
            exp: -883
        }
    );
    assert_eq!(
        compact::from_float(1e-150f64),
        ExtendedFloat80 {
            mant: 7371020360979573,
            exp: -551
        }
    );
    assert_eq!(
        compact::from_float(1e-45f64),
        ExtendedFloat80 {
            mant: 6427752177035961,
            exp: -202
        }
    );
    assert_eq!(
        compact::from_float(1e-40f64),
        ExtendedFloat80 {
            mant: 4903985730770844,
            exp: -185
        }
    );
    assert_eq!(
        compact::from_float(2e-40f64),
        ExtendedFloat80 {
            mant: 4903985730770844,
            exp: -184
        }
    );
    assert_eq!(
        compact::from_float(1e-20f64),
        ExtendedFloat80 {
            mant: 6646139978924579,
            exp: -119
        }
    );
    assert_eq!(
        compact::from_float(2e-20f64),
        ExtendedFloat80 {
            mant: 6646139978924579,
            exp: -118
        }
    );
    assert_eq!(
        compact::from_float(1.0f64),
        ExtendedFloat80 {
            mant: 4503599627370496,
            exp: -52
        }
    );
    assert_eq!(
        compact::from_float(2.0f64),
        ExtendedFloat80 {
            mant: 4503599627370496,
            exp: -51
        }
    );
    assert_eq!(
        compact::from_float(1e20f64),
        ExtendedFloat80 {
            mant: 6103515625000000,
            exp: 14
        }
    );
    assert_eq!(
        compact::from_float(2e20f64),
        ExtendedFloat80 {
            mant: 6103515625000000,
            exp: 15
        }
    );
    assert_eq!(
        compact::from_float(1e40f64),
        ExtendedFloat80 {
            mant: 8271806125530277,
            exp: 80
        }
    );
    assert_eq!(
        compact::from_float(2e40f64),
        ExtendedFloat80 {
            mant: 8271806125530277,
            exp: 81
        }
    );
    assert_eq!(
        compact::from_float(1e150f64),
        ExtendedFloat80 {
            mant: 5503284107318959,
            exp: 446
        }
    );
    assert_eq!(
        compact::from_float(1e250f64),
        ExtendedFloat80 {
            mant: 6290184345309700,
            exp: 778
        }
    );
    assert_eq!(
        compact::from_float(1.7976931348623157e308),
        ExtendedFloat80 {
            mant: 9007199254740991,
            exp: 971
        }
    );
}

fn check_mul(xmant: u64, xexp: i32, ymant: u64, yexp: i32, zmant: u64, zexp: i32) {
    let x = ExtendedFloat80 {
        mant: xmant,
        exp: xexp,
    };
    let y = ExtendedFloat80 {
        mant: ymant,
        exp: yexp,
    };
    let z = ExtendedFloat80 {
        mant: zmant,
        exp: zexp,
    };
    let r = compact::mul(&x, &y);
    assert_eq!(r, z);
}

#[test]
fn mul_test() {
    // Normalized (64-bit mantissa)
    check_mul(13164036458569648128, -213, 9223372036854775808, -62, 6582018229284824064, -211);

    // Check both values need high bits set.
    check_mul(1 << 32, -31, 1 << 32, -31, 1, 2);
    check_mul(10 << 31, -31, 10 << 31, -31, 25, 2);
}

fn grisu<T: RawFloat>(f: T, expected: &str, k: i32) {
    let mut buffer = [b'\x00'; 32];
    let (count, real_k) = unsafe { compact::grisu(f, &mut buffer) };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
    assert_eq!(k, real_k);
}

#[test]
fn grisu_test() {
    // Useful test to ensure we don't have trailing zeros.
    grisu(1.0f64, "1", 0);
    grisu(0.1f64, "1", -1);
    grisu(10.0f64, "1", 1);
    grisu(100000000.0f64, "1", 8);
    grisu(100001000.0f64, "100001", 3);
    grisu(100111000.0f64, "100111", 3);
}

fn write_float<T: RawFloat, const FORMAT: u128>(f: T, options: &Options, expected: &str) {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let count = unsafe { compact::write_float::<_, FORMAT>(f, &mut buffer, options) };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_test() {
    let options = Options::builder().build().unwrap();
    write_float::<_, DECIMAL>(0.0f64, &options, "0.0");
    write_float::<_, DECIMAL>(1.0f64, &options, "1.0");
    write_float::<_, DECIMAL>(1.5f64, &options, "1.5");
    write_float::<_, DECIMAL>(1.2345678901234567890e0f64, &options, "1.2345678901234567");
    write_float::<_, DECIMAL>(1.0e-17f64, &options, "1.0e-17");
    write_float::<_, DECIMAL>(9.99999999999999e-16f64, &options, "9.99999999999999e-16");
    write_float::<_, DECIMAL>(9.99999999999999e-15f64, &options, "9.99999999999999e-15");
    write_float::<_, DECIMAL>(0.00999999999999999f64, &options, "0.00999999999999999");
    write_float::<_, DECIMAL>(0.0999999999999999f64, &options, "0.0999999999999999");
    write_float::<_, DECIMAL>(0.999999999999999f64, &options, "0.999999999999999");
    write_float::<_, DECIMAL>(9.99999999999999f64, &options, "9.99999999999999");
    write_float::<_, DECIMAL>(99.9999999999999f64, &options, "99.9999999999999");
    write_float::<_, DECIMAL>(999.999999999999f64, &options, "999.999999999999");
    write_float::<_, DECIMAL>(1000.0f64, &options, "1000.0");
    write_float::<_, DECIMAL>(1.7976931348623157e308f64, &options, "1.7976931348623157e308");
    write_float::<_, DECIMAL>(2.2250738585072014e-308f64, &options, "2.2250738585072014e-308");

    let options =
        Options::builder().min_significant_digits(num::NonZeroUsize::new(50)).build().unwrap();
    write_float::<_, DECIMAL>(
        0.0f64,
        &options,
        "0.0000000000000000000000000000000000000000000000000",
    );
    write_float::<_, DECIMAL>(
        1.0f64,
        &options,
        "1.0000000000000000000000000000000000000000000000000",
    );
    write_float::<_, DECIMAL>(
        1.5f64,
        &options,
        "1.5000000000000000000000000000000000000000000000000",
    );
    write_float::<_, DECIMAL>(
        1.2345678901234567890e0f64,
        &options,
        "1.2345678901234567000000000000000000000000000000000",
    );
    write_float::<_, DECIMAL>(
        1.0e-17f64,
        &options,
        "1.0000000000000000000000000000000000000000000000000e-17",
    );
    write_float::<_, DECIMAL>(
        9.99999999999999e-16f64,
        &options,
        "9.9999999999999900000000000000000000000000000000000e-16",
    );
    write_float::<_, DECIMAL>(
        9.99999999999999e-15f64,
        &options,
        "9.9999999999999900000000000000000000000000000000000e-15",
    );
    write_float::<_, DECIMAL>(
        0.00999999999999999f64,
        &options,
        "0.0099999999999999900000000000000000000000000000000000",
    );
    write_float::<_, DECIMAL>(
        0.0999999999999999f64,
        &options,
        "0.099999999999999900000000000000000000000000000000000",
    );
    write_float::<_, DECIMAL>(
        0.999999999999999f64,
        &options,
        "0.99999999999999900000000000000000000000000000000000",
    );
    write_float::<_, DECIMAL>(
        9.99999999999999f64,
        &options,
        "9.9999999999999900000000000000000000000000000000000",
    );
    write_float::<_, DECIMAL>(
        99.9999999999999f64,
        &options,
        "99.999999999999900000000000000000000000000000000000",
    );
    write_float::<_, DECIMAL>(
        999.999999999999f64,
        &options,
        "999.99999999999900000000000000000000000000000000000",
    );
    write_float::<_, DECIMAL>(
        1000.0f64,
        &options,
        "1000.0000000000000000000000000000000000000000000000",
    );
    write_float::<_, DECIMAL>(
        1.7976931348623157e308f64,
        &options,
        "1.7976931348623157000000000000000000000000000000000e308",
    );
    write_float::<_, DECIMAL>(
        2.2250738585072014e-308f64,
        &options,
        "2.2250738585072014000000000000000000000000000000000e-308",
    );

    let options =
        Options::builder().max_significant_digits(num::NonZeroUsize::new(5)).build().unwrap();
    write_float::<_, DECIMAL>(0.0f64, &options, "0.0");
    write_float::<_, DECIMAL>(1.0f64, &options, "1.0");
    write_float::<_, DECIMAL>(1.5f64, &options, "1.5");
    write_float::<_, DECIMAL>(1.2345678901234567890e0f64, &options, "1.2346");
    write_float::<_, DECIMAL>(1.0e-17f64, &options, "1.0e-17");
    write_float::<_, DECIMAL>(9.99999999999999e-16f64, &options, "1.0e-15");
    write_float::<_, DECIMAL>(9.99999999999999e-15f64, &options, "1.0e-14");
    write_float::<_, DECIMAL>(0.00999999999999999f64, &options, "0.01");
    write_float::<_, DECIMAL>(0.0999999999999999f64, &options, "0.1");
    write_float::<_, DECIMAL>(0.999999999999999f64, &options, "1.0");
    write_float::<_, DECIMAL>(9.99999999999999f64, &options, "10.0");
    write_float::<_, DECIMAL>(99.9999999999999f64, &options, "100.0");
    write_float::<_, DECIMAL>(999.999999999999f64, &options, "1000.0");
    write_float::<_, DECIMAL>(1000.0f64, &options, "1000.0");
    write_float::<_, DECIMAL>(1.7976931348623157e308f64, &options, "1.7977e308");
    write_float::<_, DECIMAL>(2.2250738585072014e-308f64, &options, "2.2251e-308");

    let options = Options::builder().trim_floats(true).build().unwrap();
    write_float::<_, DECIMAL>(0.0f64, &options, "0");
    write_float::<_, DECIMAL>(1.0f64, &options, "1");
    write_float::<_, DECIMAL>(1.5f64, &options, "1.5");
    write_float::<_, DECIMAL>(1.2345678901234567890e0f64, &options, "1.2345678901234567");
    write_float::<_, DECIMAL>(1.0e-17f64, &options, "1e-17");
    write_float::<_, DECIMAL>(9.99999999999999e-16f64, &options, "9.99999999999999e-16");
    write_float::<_, DECIMAL>(9.99999999999999e-15f64, &options, "9.99999999999999e-15");
    write_float::<_, DECIMAL>(0.00999999999999999f64, &options, "0.00999999999999999");
    write_float::<_, DECIMAL>(0.0999999999999999f64, &options, "0.0999999999999999");
    write_float::<_, DECIMAL>(0.999999999999999f64, &options, "0.999999999999999");
    write_float::<_, DECIMAL>(9.99999999999999f64, &options, "9.99999999999999");
    write_float::<_, DECIMAL>(99.9999999999999f64, &options, "99.9999999999999");
    write_float::<_, DECIMAL>(999.999999999999f64, &options, "999.999999999999");
    write_float::<_, DECIMAL>(1000.0f64, &options, "1000");
    write_float::<_, DECIMAL>(1.7976931348623157e308f64, &options, "1.7976931348623157e308");
    write_float::<_, DECIMAL>(2.2250738585072014e-308f64, &options, "2.2250738585072014e-308");

    let options = Options::builder()
        .min_significant_digits(num::NonZeroUsize::new(50))
        .trim_floats(true)
        .build()
        .unwrap();
    write_float::<_, DECIMAL>(1.0e17f64, &options, "1e17");
    write_float::<_, DECIMAL>(1.0e-17f64, &options, "1e-17");
    write_float::<_, DECIMAL>(1000.0f64, &options, "1000");
    write_float::<_, DECIMAL>(
        9.99999999999999e16f64,
        &options,
        "9.9999999999999900000000000000000000000000000000000e16",
    );
    write_float::<_, DECIMAL>(
        9.99999999999999e-16f64,
        &options,
        "9.9999999999999900000000000000000000000000000000000e-16",
    );

    let truncate = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(4))
        .round_mode(RoundMode::Truncate)
        .build()
        .unwrap();
    let round = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(4))
        .round_mode(RoundMode::Round)
        .build()
        .unwrap();

    write_float::<_, DECIMAL>(1.2345678901234567890e0f64, &truncate, "1.234");
    write_float::<_, DECIMAL>(1.2345678901234567890e0f64, &round, "1.235");
    write_float::<_, DECIMAL>(1.2345678901234567890e1f64, &truncate, "12.34");
    write_float::<_, DECIMAL>(1.2345678901234567890e1f64, &round, "12.35");
    write_float::<_, DECIMAL>(1.2345678901234567890e2f64, &truncate, "123.4");
    write_float::<_, DECIMAL>(1.2345678901234567890e2f64, &round, "123.5");
    write_float::<_, DECIMAL>(1.2345678901234567890e3f64, &truncate, "1234.0");
    write_float::<_, DECIMAL>(1.2345678901234567890e3f64, &round, "1235.0");

    // Check min and max digits
    let options = Options::builder()
        .min_significant_digits(num::NonZeroUsize::new(3))
        .max_significant_digits(num::NonZeroUsize::new(4))
        .round_mode(RoundMode::Truncate)
        .build()
        .unwrap();
    write_float::<_, DECIMAL>(0.0f64, &options, "0.00");
    write_float::<_, DECIMAL>(1.5f64, &options, "1.50");
    write_float::<_, DECIMAL>(1.2345678901234567890e0f64, &options, "1.234");
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
}

#[test]
fn f32_roundtrip_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let options = Options::builder().build().unwrap();
    for &float in F32_DATA.iter() {
        let count = unsafe { compact::write_float::<_, DECIMAL>(float, &mut buffer, &options) };
        let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
        let roundtrip = actual.parse::<f32>();
        assert_eq!(roundtrip, Ok(float));
    }
}

fn write_float_scientific(digits: &mut [u8], k: i32, options: &Options, expected: &str) {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let ndigits = digits.len();
    let count = unsafe {
        compact::write_float_scientific::<DECIMAL>(&mut buffer, digits, ndigits, k, &options)
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

macro_rules! mut_b {
    ($x:literal) => {
        &mut $x.to_vec()
    };
}

#[test]
fn write_float_scientific_test() {
    let options = Options::new();
    write_float_scientific(mut_b!(b"1"), 0, &options, "1.0e0");
    write_float_scientific(mut_b!(b"999999999999999"), -1, &options, "9.99999999999999e-1");
    write_float_scientific(mut_b!(b"999999999999999"), 0, &options, "9.99999999999999e0");
    write_float_scientific(mut_b!(b"999999999999999"), -2, &options, "9.99999999999999e-2");
    write_float_scientific(mut_b!(b"17976931348623157"), 308, &options, "1.7976931348623157e308");
    write_float_scientific(mut_b!(b"22250738585072014"), -308, &options, "2.2250738585072014e-308");

    let options = Options::builder().trim_floats(true).build().unwrap();
    write_float_scientific(mut_b!(b"1"), 0, &options, "1e0");
    write_float_scientific(mut_b!(b"999999999999999"), -1, &options, "9.99999999999999e-1");
    write_float_scientific(mut_b!(b"999999999999999"), 0, &options, "9.99999999999999e0");
    write_float_scientific(mut_b!(b"999999999999999"), -2, &options, "9.99999999999999e-2");
    write_float_scientific(mut_b!(b"17976931348623157"), 308, &options, "1.7976931348623157e308");
    write_float_scientific(mut_b!(b"22250738585072014"), -308, &options, "2.2250738585072014e-308");
}

fn write_float_positive_exponent(digits: &mut [u8], k: i32, options: &Options, expected: &str) {
    let mut buffer = [b'\x00'; 512];
    let ndigits = digits.len();
    let count = unsafe {
        compact::write_float_positive_exponent::<DECIMAL>(&mut buffer, digits, ndigits, k, &options)
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_positive_exponent_test() {
    let options = Options::new();
    write_float_positive_exponent(&mut [b'1'], 0, &options, "1.0");
    write_float_positive_exponent(mut_b!(b"999999999999999"), 0, &options, "9.99999999999999");
    write_float_positive_exponent(mut_b!(b"999999999999999"), 1, &options, "99.9999999999999");
    write_float_positive_exponent(mut_b!(b"999999999999999"), 2, &options, "999.999999999999");
    write_float_positive_exponent(mut_b!(b"17976931348623157"), 308, &options, "179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0");

    let options = Options::builder().trim_floats(true).build().unwrap();
    write_float_positive_exponent(&mut [b'1'], 0, &options, "1");
    write_float_positive_exponent(mut_b!(b"999999999999999"), 0, &options, "9.99999999999999");
    write_float_positive_exponent(mut_b!(b"999999999999999"), 1, &options, "99.9999999999999");
    write_float_positive_exponent(mut_b!(b"999999999999999"), 2, &options, "999.999999999999");
    write_float_positive_exponent(mut_b!(b"17976931348623157"), 308, &options, "179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
}

fn write_float_negative_exponent(digits: &mut [u8], k: i32, options: &Options, expected: &str) {
    let mut buffer = [b'\x00'; 512];
    let ndigits = digits.len();
    let count = unsafe {
        compact::write_float_negative_exponent::<DECIMAL>(&mut buffer, digits, ndigits, k, &options)
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_negative_exponent_test() {
    let options = Options::new();
    write_float_negative_exponent(&mut [b'1'], -1, &options, "0.1");
    write_float_negative_exponent(mut_b!(b"999999999999999"), -3, &options, "0.00999999999999999");
    write_float_negative_exponent(mut_b!(b"999999999999999"), -2, &options, "0.0999999999999999");
    write_float_negative_exponent(mut_b!(b"999999999999999"), -1, &options, "0.999999999999999");
    write_float_negative_exponent(mut_b!(b"22250738585072014"), -308, &options, "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022250738585072014");

    let options = Options::builder().trim_floats(true).build().unwrap();
    write_float_negative_exponent(&mut [b'1'], -1, &options, "0.1");
    write_float_negative_exponent(mut_b!(b"999999999999999"), -3, &options, "0.00999999999999999");
    write_float_negative_exponent(mut_b!(b"999999999999999"), -2, &options, "0.0999999999999999");
    write_float_negative_exponent(mut_b!(b"999999999999999"), -1, &options, "0.999999999999999");
    write_float_negative_exponent(mut_b!(b"22250738585072014"), -308, &options, "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022250738585072014");
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
        let count = unsafe { compact::write_float::<_, DECIMAL>(float, &mut buffer, &options) };
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
            let count = unsafe { compact::write_float::<_, DECIMAL>(f, &mut buffer, &options) };
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
            let count = unsafe { compact::write_float::<_, DECIMAL>(f, &mut buffer, &options) };
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
            let count = unsafe { compact::write_float::<_, DECIMAL>(f, &mut buffer, &options) };
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
            let count = unsafe { compact::write_float::<_, DECIMAL>(f, &mut buffer, &options) };
            let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
            let roundtrip = actual.parse::<f64>();
            prop_assert_eq!(roundtrip, Ok(f))
        }
    }
}
