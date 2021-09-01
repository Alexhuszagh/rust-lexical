#![cfg(feature = "radix")]
#![cfg(feature = "parse-floats")]
#![cfg(feature = "write-floats")]

use approx::assert_relative_eq;

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

macro_rules! test_radix {
    ($f:ident, $radix:expr, $buffer:ident, $data:ident) => {{
        use lexical_core::{
            FromLexicalWithOptions,
            NumberFormatBuilder,
            ParseFloatOptions,
            ToLexicalWithOptions,
            WriteFloatOptions,
        };

        const FORMAT: u128 = NumberFormatBuilder::from_radix($radix);

        let write_options = WriteFloatOptions::builder().exponent(b'^').build().unwrap();
        let parse_options = ParseFloatOptions::builder().exponent(b'^').build().unwrap();
        for &float in $data.iter() {
            let data = float.to_lexical_with_options::<FORMAT>(&mut $buffer, &write_options);
            let roundtrip = $f::from_lexical_with_options::<FORMAT>(data, &parse_options).unwrap();
            assert_relative_eq!(float, roundtrip, epsilon = 1e-6, max_relative = 3e-6);
        }
    }};
}

macro_rules! test_all {
    ($f:ident, $buffer:ident, $data:ident) => {{
        test_radix!($f, 2, $buffer, $data);
        test_radix!($f, 3, $buffer, $data);
        test_radix!($f, 4, $buffer, $data);
        test_radix!($f, 5, $buffer, $data);
        test_radix!($f, 6, $buffer, $data);
        test_radix!($f, 7, $buffer, $data);
        test_radix!($f, 8, $buffer, $data);
        test_radix!($f, 9, $buffer, $data);
        test_radix!($f, 19, $buffer, $data);
        test_radix!($f, 11, $buffer, $data);
        test_radix!($f, 12, $buffer, $data);
        test_radix!($f, 13, $buffer, $data);
        test_radix!($f, 14, $buffer, $data);
        test_radix!($f, 15, $buffer, $data);
        test_radix!($f, 16, $buffer, $data);
        test_radix!($f, 17, $buffer, $data);
        test_radix!($f, 18, $buffer, $data);
        test_radix!($f, 19, $buffer, $data);
        test_radix!($f, 20, $buffer, $data);
        test_radix!($f, 21, $buffer, $data);
        test_radix!($f, 22, $buffer, $data);
        test_radix!($f, 23, $buffer, $data);
        test_radix!($f, 24, $buffer, $data);
        test_radix!($f, 25, $buffer, $data);
        test_radix!($f, 26, $buffer, $data);
        test_radix!($f, 27, $buffer, $data);
        test_radix!($f, 28, $buffer, $data);
        test_radix!($f, 29, $buffer, $data);
        test_radix!($f, 30, $buffer, $data);
        test_radix!($f, 31, $buffer, $data);
        test_radix!($f, 32, $buffer, $data);
        test_radix!($f, 33, $buffer, $data);
        test_radix!($f, 34, $buffer, $data);
        test_radix!($f, 35, $buffer, $data);
        test_radix!($f, 36, $buffer, $data);
    }};
}

#[test]
fn parse_f32_radix_roundtrip_test() {
    let mut buffer = [0u8; 1024];
    test_all!(f32, buffer, F32_DATA);
}

#[test]
fn parse_f64_radix_roundtrip_test() {
    let mut buffer = [0u8; 1024];
    test_all!(f64, buffer, F64_DATA);
}
