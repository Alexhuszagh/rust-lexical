#![cfg(not(feature = "compact"))]

use core::num;

use lexical_util::constants::BUFFER_SIZE;
use lexical_util::format::NumberFormatBuilder;
use lexical_util::num::Float;
use lexical_write_float::algorithm::DragonboxFloat;
use lexical_write_float::float::{ExtendedFloat80, RawFloat};
use lexical_write_float::{algorithm, Options, RoundMode};

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
fn issue84_test() {
    let (hi, lo) =
        algorithm::umul192_lower128(15966911296221875, 0xcccccccccccccccc, 0xcccccccccccccccd);
    assert_eq!(hi, 0);
    assert_eq!(lo, 3193382259244375);
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
    assert_eq!(to_decimal_f32(2762159900.0), (27621599, 2));
    assert_eq!(to_decimal_f32(77371252000000000000000000.0), (77371252, 18));

    assert_eq!(to_decimal_f64(0.0), (0, 0));
    assert_eq!(to_decimal_f64(0.5), (5, -1));
    assert_eq!(to_decimal_f64(1.0), (1, 0));
    assert_eq!(to_decimal_f64(1.5), (15, -1));
    assert_eq!(to_decimal_f64(1.23456), (123456, -5));
    assert_eq!(to_decimal_f64(2.2250738585072014e-308), (22250738585072014, -324));
    assert_eq!(to_decimal_f64(1.7976931348623157e+308), (17976931348623157, 292));
}

fn compute_nearest_shorter(float: f64) -> (u64, i32) {
    let fp = algorithm::compute_nearest_shorter(float);
    (fp.mant, fp.exp)
}

#[test]
fn compute_nearest_shorter_test() {
    assert_eq!(compute_nearest_shorter(0.5), (5, -1));
    assert_eq!(compute_nearest_shorter(1.0), (1, 0));
    assert_eq!(compute_nearest_shorter(2.0), (2, 0));
}

fn compute_nearest_normal(float: f64) -> (u64, i32) {
    let fp = algorithm::compute_nearest_normal(float);
    (fp.mant, fp.exp)
}

#[test]
fn compute_nearest_normal_test() {
    assert_eq!(compute_nearest_normal(1.23456), (123456, -5));
    assert_eq!(compute_nearest_normal(13.9999999999999982236431606), (13999999999999998, -15));
}

fn compute_left_closed_directed(float: f64) -> (u64, i32) {
    let fp = algorithm::compute_left_closed_directed(float);
    (fp.mant, fp.exp)
}

#[test]
fn compute_left_closed_directed_test() {
    assert_eq!(compute_left_closed_directed(1.23456), (12345600000000002, -16));
    assert_eq!(
        compute_left_closed_directed(13.9999999999999982236431606),
        (13999999999999999, -15)
    );
}

fn compute_right_closed_directed(float: f64) -> (u64, i32) {
    // Assume we do not have a shorter case.
    let bits = float.to_bits();
    let mantissa_bits = bits & f64::MANTISSA_MASK;
    assert!(mantissa_bits != 0);
    let fp = algorithm::compute_right_closed_directed(float, false);
    (fp.mant, fp.exp)
}

#[test]
fn compute_right_closed_directed_test() {
    assert_eq!(compute_right_closed_directed(1.23456), (123456, -5));
    assert_eq!(
        compute_right_closed_directed(13.9999999999999982236431606),
        (13999999999999982, -15)
    );
}

fn write_digits_f32(buffer: &mut [u8], value: u64, expected: &str) {
    let count = f32::write_digits(buffer, value);
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_digits_f32_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    write_digits_f32(&mut buffer, 0, "0");
    write_digits_f32(&mut buffer, 1, "1");
    write_digits_f32(&mut buffer, 11, "11");
    write_digits_f32(&mut buffer, 23, "23");
    write_digits_f32(&mut buffer, 23786281, "23786281");
    write_digits_f32(&mut buffer, 4294967295, "4294967295");
}

fn write_digits_f64(buffer: &mut [u8], value: u64, expected: &str) {
    let count = f64::write_digits(buffer, value);
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_digits_f64_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    write_digits_f64(&mut buffer, 0, "0");
    write_digits_f64(&mut buffer, 1, "1");
    write_digits_f64(&mut buffer, 11, "11");
    write_digits_f64(&mut buffer, 23, "23");
    write_digits_f64(&mut buffer, 4294967295, "4294967295");
    write_digits_f64(&mut buffer, 4294967296, "4294967296");
}

fn write_float_scientific(mant: u64, exp: i32, options: &Options, expected: &str) {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let fp = ExtendedFloat80 {
        mant,
        exp,
    };
    let digit_count = f64::digit_count(fp.mant);
    let sci_exp = fp.exp + digit_count as i32 - 1;
    let count =
        algorithm::write_float_scientific::<f64, DECIMAL>(&mut buffer, fp, sci_exp, &options);
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_scientific_test() {
    const OPTS1: Options = Options::new();
    write_float_scientific(1, 0, &OPTS1, "1.0e0");
    write_float_scientific(1, 3, &OPTS1, "1.0e3");
    write_float_scientific(1, -12, &OPTS1, "1.0e-12");
    write_float_scientific(999999999999999, -15, &OPTS1, "9.99999999999999e-1");
    write_float_scientific(999999999999999, -14, &OPTS1, "9.99999999999999e0");
    write_float_scientific(999999999999999, -16, &OPTS1, "9.99999999999999e-2");
    write_float_scientific(17976931348623157, 292, &OPTS1, "1.7976931348623157e308");
    write_float_scientific(22250738585072014, -324, &OPTS1, "2.2250738585072014e-308");

    const OPTS2: Options =
        Options::builder().min_significant_digits(num::NonZeroUsize::new(50)).build_strict();
    write_float_scientific(1, 0, &OPTS2, "1.0000000000000000000000000000000000000000000000000e0");
    write_float_scientific(1, 3, &OPTS2, "1.0000000000000000000000000000000000000000000000000e3");
    write_float_scientific(
        1,
        -12,
        &OPTS2,
        "1.0000000000000000000000000000000000000000000000000e-12",
    );
    write_float_scientific(
        999999999999999,
        -15,
        &OPTS2,
        "9.9999999999999900000000000000000000000000000000000e-1",
    );
    write_float_scientific(
        999999999999999,
        -14,
        &OPTS2,
        "9.9999999999999900000000000000000000000000000000000e0",
    );
    write_float_scientific(
        999999999999999,
        -16,
        &OPTS2,
        "9.9999999999999900000000000000000000000000000000000e-2",
    );
    write_float_scientific(
        17976931348623157,
        292,
        &OPTS2,
        "1.7976931348623157000000000000000000000000000000000e308",
    );
    write_float_scientific(
        22250738585072014,
        -324,
        &OPTS2,
        "2.2250738585072014000000000000000000000000000000000e-308",
    );

    const OPTS3: Options =
        Options::builder().max_significant_digits(num::NonZeroUsize::new(5)).build_strict();
    write_float_scientific(1, 0, &OPTS3, "1.0e0");
    write_float_scientific(1, 3, &OPTS3, "1.0e3");
    write_float_scientific(1, -12, &OPTS3, "1.0e-12");
    write_float_scientific(999999999999999, -15, &OPTS3, "1.0e0");
    write_float_scientific(999999999999999, -14, &OPTS3, "1.0e1");
    write_float_scientific(999999999999999, -16, &OPTS3, "1.0e-1");
    write_float_scientific(17976931348623157, 292, &OPTS3, "1.7977e308");
    write_float_scientific(22250738585072014, -324, &OPTS3, "2.2251e-308");

    const OPTS4: Options = Options::builder().trim_floats(true).build_strict();
    write_float_scientific(1, 0, &OPTS4, "1e0");
    write_float_scientific(1, 3, &OPTS4, "1e3");
    write_float_scientific(1, -12, &OPTS4, "1e-12");
    write_float_scientific(999999999999999, -15, &OPTS4, "9.99999999999999e-1");
    write_float_scientific(999999999999999, -14, &OPTS4, "9.99999999999999e0");
    write_float_scientific(999999999999999, -16, &OPTS4, "9.99999999999999e-2");
    write_float_scientific(17976931348623157, 292, &OPTS4, "1.7976931348623157e308");
    write_float_scientific(22250738585072014, -324, &OPTS4, "2.2250738585072014e-308");
}

fn write_float_positive_exponent(mant: u64, exp: i32, options: &Options, expected: &str) {
    let mut buffer = [b'\x00'; 512];
    let fp = ExtendedFloat80 {
        mant,
        exp,
    };
    let digit_count = f64::digit_count(fp.mant);
    let sci_exp = fp.exp + digit_count as i32 - 1;
    let count = algorithm::write_float_positive_exponent::<f64, DECIMAL>(
        &mut buffer,
        fp,
        sci_exp,
        &options,
    );
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_positive_exponent_test() {
    const OPTS1: Options = Options::new();
    write_float_positive_exponent(1, 0, &OPTS1, "1.0");
    write_float_positive_exponent(1, 3, &OPTS1, "1000.0");
    write_float_positive_exponent(1, 12, &OPTS1, "1000000000000.0");
    write_float_positive_exponent(999999999999999, -14, &OPTS1, "9.99999999999999");
    write_float_positive_exponent(999999999999999, -13, &OPTS1, "99.9999999999999");
    write_float_positive_exponent(999999999999999, -12, &OPTS1, "999.999999999999");
    write_float_positive_exponent(17976931348623157, 292, &OPTS1, "179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0");

    const OPTS2: Options =
        Options::builder().min_significant_digits(num::NonZeroUsize::new(50)).build_strict();
    write_float_positive_exponent(
        1,
        0,
        &OPTS2,
        "1.0000000000000000000000000000000000000000000000000",
    );
    write_float_positive_exponent(
        1,
        3,
        &OPTS2,
        "1000.0000000000000000000000000000000000000000000000",
    );
    write_float_positive_exponent(
        1,
        12,
        &OPTS2,
        "1000000000000.0000000000000000000000000000000000000",
    );
    write_float_positive_exponent(
        999999999999999,
        -14,
        &OPTS2,
        "9.9999999999999900000000000000000000000000000000000",
    );
    write_float_positive_exponent(
        999999999999999,
        -13,
        &OPTS2,
        "99.999999999999900000000000000000000000000000000000",
    );
    write_float_positive_exponent(
        999999999999999,
        -12,
        &OPTS2,
        "999.99999999999900000000000000000000000000000000000",
    );
    write_float_positive_exponent(17976931348623157, 292, &OPTS2, "179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0");

    const OPTS3: Options =
        Options::builder().max_significant_digits(num::NonZeroUsize::new(5)).build_strict();
    write_float_positive_exponent(1, 0, &OPTS3, "1.0");
    write_float_positive_exponent(1, 3, &OPTS3, "1000.0");
    write_float_positive_exponent(1, 12, &OPTS3, "1000000000000.0");
    write_float_positive_exponent(999999999999999, -14, &OPTS3, "10.0");
    write_float_positive_exponent(999999999999999, -13, &OPTS3, "100.0");
    write_float_positive_exponent(999999999999999, -12, &OPTS3, "1000.0");
    write_float_positive_exponent(17976931348623157, 292, &OPTS3, "179770000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0");

    const OPTS4: Options = Options::builder().trim_floats(true).build_strict();
    write_float_positive_exponent(1, 0, &OPTS4, "1");
    write_float_positive_exponent(1, 3, &OPTS4, "1000");
    write_float_positive_exponent(1, 12, &OPTS4, "1000000000000");
    write_float_positive_exponent(999999999999999, -14, &OPTS4, "9.99999999999999");
    write_float_positive_exponent(999999999999999, -13, &OPTS4, "99.9999999999999");
    write_float_positive_exponent(999999999999999, -12, &OPTS4, "999.999999999999");
    write_float_positive_exponent(17976931348623157, 292, &OPTS4, "179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
}

fn write_float_negative_exponent(mant: u64, exp: i32, options: &Options, expected: &str) {
    let mut buffer = [b'\x00'; 512];
    let fp = ExtendedFloat80 {
        mant,
        exp,
    };
    let digit_count = f64::digit_count(fp.mant);
    let sci_exp = fp.exp + digit_count as i32 - 1;
    let count = algorithm::write_float_negative_exponent::<f64, DECIMAL>(
        &mut buffer,
        fp,
        sci_exp,
        &options,
    );
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_negative_exponent_test() {
    const OPTS1: Options = Options::new();
    write_float_negative_exponent(1, -1, &OPTS1, "0.1");
    write_float_negative_exponent(1, -3, &OPTS1, "0.001");
    write_float_negative_exponent(1, -12, &OPTS1, "0.000000000001");
    write_float_negative_exponent(999999999999999, -17, &OPTS1, "0.00999999999999999");
    write_float_negative_exponent(999999999999999, -16, &OPTS1, "0.0999999999999999");
    write_float_negative_exponent(999999999999999, -15, &OPTS1, "0.999999999999999");
    write_float_negative_exponent(22250738585072014, -324, &OPTS1, "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022250738585072014");

    const OPTS2: Options =
        Options::builder().min_significant_digits(num::NonZeroUsize::new(50)).build_strict();
    write_float_negative_exponent(
        1,
        -1,
        &OPTS2,
        "0.10000000000000000000000000000000000000000000000000",
    );
    write_float_negative_exponent(
        1,
        -3,
        &OPTS2,
        "0.0010000000000000000000000000000000000000000000000000",
    );
    write_float_negative_exponent(
        1,
        -12,
        &OPTS2,
        "0.0000000000010000000000000000000000000000000000000000000000000",
    );
    write_float_negative_exponent(
        999999999999999,
        -17,
        &OPTS2,
        "0.0099999999999999900000000000000000000000000000000000",
    );
    write_float_negative_exponent(
        999999999999999,
        -16,
        &OPTS2,
        "0.099999999999999900000000000000000000000000000000000",
    );
    write_float_negative_exponent(
        999999999999999,
        -15,
        &OPTS2,
        "0.99999999999999900000000000000000000000000000000000",
    );
    write_float_negative_exponent(22250738585072014, -324, &OPTS2, "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022250738585072014000000000000000000000000000000000");

    const OPTS3: Options =
        Options::builder().max_significant_digits(num::NonZeroUsize::new(5)).build_strict();
    write_float_negative_exponent(1, -1, &OPTS3, "0.1");
    write_float_negative_exponent(1, -3, &OPTS3, "0.001");
    write_float_negative_exponent(1, -12, &OPTS3, "0.000000000001");
    write_float_negative_exponent(999999999999999, -17, &OPTS3, "0.01");
    write_float_negative_exponent(999999999999999, -16, &OPTS3, "0.1");
    write_float_negative_exponent(999999999999999, -15, &OPTS3, "1.0");
    write_float_negative_exponent(22250738585072014, -324, &OPTS3, "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022251");

    const OPTS4: Options = Options::builder().trim_floats(true).build_strict();
    write_float_negative_exponent(1, -1, &OPTS4, "0.1");
    write_float_negative_exponent(1, -3, &OPTS4, "0.001");
    write_float_negative_exponent(1, -12, &OPTS4, "0.000000000001");
    write_float_negative_exponent(999999999999999, -17, &OPTS4, "0.00999999999999999");
    write_float_negative_exponent(999999999999999, -16, &OPTS4, "0.0999999999999999");
    write_float_negative_exponent(999999999999999, -15, &OPTS4, "0.999999999999999");
    write_float_negative_exponent(22250738585072014, -324, &OPTS4, "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022250738585072014");
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
    let count = algorithm::write_float::<_, FORMAT>(f, &mut buffer, options);
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn f32_test() {
    const OPTS1: Options = Options::builder().trim_floats(true).build_strict();
    write_float::<_, DECIMAL>(0.0f32, &OPTS1, "0");
    write_float::<_, DECIMAL>(1.0f32, &OPTS1, "1");
    write_float::<_, DECIMAL>(10.0f32, &OPTS1, "10");
    write_float::<_, DECIMAL>(10.0f32, &OPTS1, "10");
    write_float::<_, DECIMAL>(1.2345678901234567890e0f32, &OPTS1, "1.2345679");
    write_float::<_, DECIMAL>(1.2345678901234567890e1f32, &OPTS1, "12.345679");
    write_float::<_, DECIMAL>(1.2345678901234567890e2f32, &OPTS1, "123.45679");
    write_float::<_, DECIMAL>(1.2345678901234567890e3f32, &OPTS1, "1234.5679");
    write_float::<_, DECIMAL>(2.3786281e+38f32, &OPTS1, "2.3786281e38");

    const OPTS2: Options = Options::new();
    write_float::<_, DECIMAL>(2.3786281e+38f32, &OPTS2, "2.3786281e38");
}

#[test]
fn f32_errors_test() {
    // Errors discovered via quickcheck.
    const OPTIONS: Options = Options::new();
    write_float::<_, DECIMAL>(0.0f32, &OPTIONS, "0.0");
    write_float::<_, DECIMAL>(1073741800.0f32, &OPTIONS, "1073741800.0");
    write_float::<_, DECIMAL>(1610612700.0f32, &OPTIONS, "1610612700.0");
    write_float::<_, DECIMAL>(1879048200.0f32, &OPTIONS, "1879048200.0");
    write_float::<_, DECIMAL>(2013265900.0f32, &OPTIONS, "2013265900.0");
    write_float::<_, DECIMAL>(2080374800.0f32, &OPTIONS, "2080374800.0");
    write_float::<_, DECIMAL>(2113929200.0f32, &OPTIONS, "2113929200.0");
    write_float::<_, DECIMAL>(2130706400.0f32, &OPTIONS, "2130706400.0");
    write_float::<_, DECIMAL>(2139095000.0f32, &OPTIONS, "2139095000.0");
    write_float::<_, DECIMAL>(2143289300.0f32, &OPTIONS, "2143289300.0");
    write_float::<_, DECIMAL>(2145386500.0f32, &OPTIONS, "2145386500.0");
    write_float::<_, DECIMAL>(2146435100.0f32, &OPTIONS, "2146435100.0");
    write_float::<_, DECIMAL>(2146959400.0f32, &OPTIONS, "2146959400.0");
    write_float::<_, DECIMAL>(2147221500.0f32, &OPTIONS, "2147221500.0");
    write_float::<_, DECIMAL>(2147352600.0f32, &OPTIONS, "2147352600.0");
    write_float::<_, DECIMAL>(2147418100.0f32, &OPTIONS, "2147418100.0");
    write_float::<_, DECIMAL>(2147450900.0f32, &OPTIONS, "2147450900.0");
    write_float::<_, DECIMAL>(2147467300.0f32, &OPTIONS, "2147467300.0");
    write_float::<_, DECIMAL>(2147475500.0f32, &OPTIONS, "2147475500.0");
    write_float::<_, DECIMAL>(2147479600.0f32, &OPTIONS, "2147479600.0");
    write_float::<_, DECIMAL>(2147481600.0f32, &OPTIONS, "2147481600.0");
    write_float::<_, DECIMAL>(2147482600.0f32, &OPTIONS, "2147482600.0");
    write_float::<_, DECIMAL>(2147483100.0f32, &OPTIONS, "2147483100.0");
    write_float::<_, DECIMAL>(2147483400.0f32, &OPTIONS, "2147483400.0");
    write_float::<_, DECIMAL>(2147483500.0f32, &OPTIONS, "2147483500.0");
    write_float::<_, DECIMAL>(2147483600.0f32, &OPTIONS, "2147483600.0");
}

#[test]
fn f32_roundtrip_test() {
    let mut buffer: [u8; BUFFER_SIZE] = [b'\x00'; BUFFER_SIZE];
    const OPTIONS: Options = Options::builder().build_strict();
    for &float in F32_DATA.iter() {
        let count = algorithm::write_float::<_, DECIMAL>(float, &mut buffer, &OPTIONS);
        let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
        let roundtrip = actual.parse::<f32>();
        assert_eq!(roundtrip, Ok(float));
    }
}

#[test]
fn f64_test() {
    const TRIM: Options = Options::builder().trim_floats(true).build_strict();
    write_float::<_, DECIMAL>(0.0f64, &TRIM, "0");
    write_float::<_, DECIMAL>(1.0f64, &TRIM, "1");
    write_float::<_, DECIMAL>(10.0f64, &TRIM, "10");
    write_float::<_, DECIMAL>(10.0f64, &TRIM, "10");
    write_float::<_, DECIMAL>(1.2345678901234567890e0f64, &TRIM, "1.2345678901234567");
    write_float::<_, DECIMAL>(1.2345678901234567890e1f64, &TRIM, "12.345678901234567");
    write_float::<_, DECIMAL>(1.2345678901234567890e2f64, &TRIM, "123.45678901234568");
    write_float::<_, DECIMAL>(1.2345678901234567890e3f64, &TRIM, "1234.567890123457");
    write_float::<_, DECIMAL>(1.5f64, &TRIM, "1.5");
    write_float::<_, DECIMAL>(1.0e-17f64, &TRIM, "1e-17");
    write_float::<_, DECIMAL>(9.99999999999999e-16f64, &TRIM, "9.99999999999999e-16");
    write_float::<_, DECIMAL>(9.99999999999999e-15f64, &TRIM, "9.99999999999999e-15");
    write_float::<_, DECIMAL>(0.00999999999999999f64, &TRIM, "0.00999999999999999");
    write_float::<_, DECIMAL>(0.0999999999999999f64, &TRIM, "0.0999999999999999");
    write_float::<_, DECIMAL>(0.999999999999999f64, &TRIM, "0.999999999999999");
    write_float::<_, DECIMAL>(9.99999999999999f64, &TRIM, "9.99999999999999");
    write_float::<_, DECIMAL>(99.9999999999999f64, &TRIM, "99.9999999999999");
    write_float::<_, DECIMAL>(999.999999999999f64, &TRIM, "999.999999999999");
    write_float::<_, DECIMAL>(1000.0f64, &TRIM, "1000");
    write_float::<_, DECIMAL>(1.7976931348623157e308f64, &TRIM, "1.7976931348623157e308");
    write_float::<_, DECIMAL>(2.2250738585072014e-308f64, &TRIM, "2.2250738585072014e-308");

    const MIN_DIGITS: Options = Options::builder()
        .min_significant_digits(num::NonZeroUsize::new(50))
        .trim_floats(true)
        .build_strict();
    write_float::<_, DECIMAL>(1.0e17f64, &MIN_DIGITS, "1e17");
    write_float::<_, DECIMAL>(1.0e-17f64, &MIN_DIGITS, "1e-17");
    write_float::<_, DECIMAL>(1000.0f64, &MIN_DIGITS, "1000");
    write_float::<_, DECIMAL>(
        9.99999999999999e16f64,
        &MIN_DIGITS,
        "9.9999999999999900000000000000000000000000000000000e16",
    );
    write_float::<_, DECIMAL>(
        9.99999999999999e-16f64,
        &MIN_DIGITS,
        "9.9999999999999900000000000000000000000000000000000e-16",
    );

    const TRUNCATE: Options = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(4))
        .round_mode(RoundMode::Truncate)
        .build_strict();
    const ROUND: Options = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(4))
        .round_mode(RoundMode::Round)
        .build_strict();

    write_float::<_, DECIMAL>(1.2345678901234567890e0f64, &TRUNCATE, "1.234");
    write_float::<_, DECIMAL>(1.2345678901234567890e0f64, &ROUND, "1.235");
    write_float::<_, DECIMAL>(1.2345678901234567890e1f64, &TRUNCATE, "12.34");
    write_float::<_, DECIMAL>(1.2345678901234567890e1f64, &ROUND, "12.35");
    write_float::<_, DECIMAL>(1.2345678901234567890e2f64, &TRUNCATE, "123.4");
    write_float::<_, DECIMAL>(1.2345678901234567890e2f64, &ROUND, "123.5");
    write_float::<_, DECIMAL>(1.2345678901234567890e3f64, &TRUNCATE, "1234.0");
    write_float::<_, DECIMAL>(1.2345678901234567890e3f64, &ROUND, "1235.0");
}

#[test]
fn f64_roundtrip_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    const OPTIONS: Options = Options::builder().build_strict();
    for &float in F64_DATA.iter() {
        let count = algorithm::write_float::<_, DECIMAL>(float, &mut buffer, &OPTIONS);
        let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
        let roundtrip = actual.parse::<f64>();
        assert_eq!(roundtrip, Ok(float));
    }
}

#[test]
fn is_endpoint_test() {
    assert_eq!(algorithm::is_endpoint(5, 2, 10), true);
    assert_eq!(algorithm::is_endpoint(5, 6, 10), false);
}

#[test]
fn is_right_endpoint_test() {
    assert_eq!(algorithm::is_right_endpoint::<f64>(1), true);
    assert_eq!(algorithm::is_right_endpoint::<f64>(2), true);
    assert_eq!(algorithm::is_right_endpoint::<f64>(3), true);
    assert_eq!(algorithm::is_right_endpoint::<f64>(4), false);
}

#[test]
fn is_left_endpoint_test() {
    assert_eq!(algorithm::is_left_endpoint::<f64>(1), false);
    assert_eq!(algorithm::is_left_endpoint::<f64>(2), true);
    assert_eq!(algorithm::is_left_endpoint::<f64>(3), true);
    assert_eq!(algorithm::is_left_endpoint::<f64>(4), false);
}
