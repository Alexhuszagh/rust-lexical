use lexical_parse_float::options::Options;
use lexical_parse_float::parse;
use lexical_util::format::STANDARD;
use lexical_util::iterator::AsBytes;
use lexical_util::step::u64_step;

#[test]
fn parse_complete_test() {
    const FORMAT: u128 = STANDARD;
    let options = Options::new();
    let string = b"1.2345e10";
    let result = parse::parse_complete::<f64, FORMAT>(string, &options);
    assert_eq!(result, Ok(1.2345e10));

    let string = b"1.2345e";
    let result = parse::parse_complete::<f64, FORMAT>(string, &options);
    assert!(result.is_err());

    let string = b"1.2345 ";
    let result = parse::parse_complete::<f64, FORMAT>(string, &options);
    assert!(result.is_err());
}

#[test]
fn fast_path_complete_test() {
    const FORMAT: u128 = STANDARD;
    let options = Options::new();
    let string = b"1.2345e10";
    let result = parse::fast_path_complete::<f64, FORMAT>(string, &options);
    assert_eq!(result, Ok(1.2345e10));

    let string = b"1.2345e";
    let result = parse::fast_path_complete::<f64, FORMAT>(string, &options);
    assert!(result.is_err());

    let string = b"1.2345 ";
    let result = parse::fast_path_complete::<f64, FORMAT>(string, &options);
    assert!(result.is_err());
}

#[test]
fn parse_partial_test() {
    const FORMAT: u128 = STANDARD;
    let options = Options::new();
    let string = b"1.2345e10";
    let result = parse::parse_partial::<f64, FORMAT>(string, &options);
    assert_eq!(result, Ok((1.2345e10, 9)));

    let string = b"1.2345e";
    let result = parse::parse_partial::<f64, FORMAT>(string, &options);
    assert!(result.is_err());

    let string = b"1.2345 ";
    let result = parse::parse_partial::<f64, FORMAT>(string, &options);
    assert_eq!(result, Ok((1.2345, 6)));
}

#[test]
fn fast_path_partial_test() {
    const FORMAT: u128 = STANDARD;
    let options = Options::new();
    let string = b"1.2345e10";
    let result = parse::fast_path_partial::<f64, FORMAT>(string, &options);
    assert_eq!(result, Ok((1.2345e10, 9)));

    let string = b"1.2345e";
    let result = parse::fast_path_partial::<f64, FORMAT>(string, &options);
    assert!(result.is_err());

    let string = b"1.2345 ";
    let result = parse::fast_path_partial::<f64, FORMAT>(string, &options);
    assert_eq!(result, Ok((1.2345, 6)));
}

#[test]
fn parse_number_test() {
    const FORMAT: u128 = STANDARD;
    let options = Options::new();
    let string = b"1.2345e10";
    let byte = string.bytes::<{ FORMAT }>();
    let result = parse::parse_complete_number(byte, false, &options);
    assert!(result.is_ok());
    let num = result.unwrap();
    assert_eq!(num.mantissa, 12345);
    assert_eq!(num.exponent, 6);
    assert_eq!(num.many_digits, false);

    let string = b"1.2345e";
    let byte = string.bytes::<{ FORMAT }>();
    let result = parse::parse_complete_number(byte, false, &options);
    assert!(result.is_err());

    let string = b"1.2345 ";
    let byte = string.bytes::<{ FORMAT }>();
    let result = parse::parse_complete_number(byte, false, &options);
    assert!(result.is_err());
}

#[test]
fn parse_partial_number_test() {
    const FORMAT: u128 = STANDARD;
    let options = Options::new();
    let string = b"1.2345e10";
    let byte = string.bytes::<{ FORMAT }>();
    let result = parse::parse_partial_number(byte, false, &options);
    assert!(result.is_ok());
    let (num, count) = result.unwrap();
    assert_eq!(num.mantissa, 12345);
    assert_eq!(num.exponent, 6);
    assert_eq!(num.many_digits, false);
    assert_eq!(count, 9);

    let string = b"1.2345e";
    let byte = string.bytes::<{ FORMAT }>();
    let result = parse::parse_partial_number(byte, false, &options);
    assert!(result.is_err());

    let string = b"1.2345 ";
    let byte = string.bytes::<{ FORMAT }>();
    let result = parse::parse_partial_number(byte, false, &options);
    assert!(result.is_ok());
    let (num, count) = result.unwrap();
    assert_eq!(num.mantissa, 12345);
    assert_eq!(num.exponent, -4);
    assert_eq!(num.many_digits, false);
    assert_eq!(count, 6);

    // Leading zeros
    let string = b"00000000000000000000001.2345 ";
    let byte = string.bytes::<{ FORMAT }>();
    let result = parse::parse_partial_number(byte, false, &options);
    assert!(result.is_ok());
    let (num, count) = result.unwrap();
    assert_eq!(num.mantissa, 12345);
    assert_eq!(num.exponent, -4);
    assert_eq!(num.many_digits, false);
    assert_eq!(count, 28);

    // Leading zeros
    let string = b"0.00000000000000000000012345 ";
    let byte = string.bytes::<{ FORMAT }>();
    let result = parse::parse_partial_number(byte, false, &options);
    assert!(result.is_ok());
    let (num, count) = result.unwrap();
    assert_eq!(num.mantissa, 12345);
    assert_eq!(num.exponent, -26);
    assert_eq!(num.many_digits, false);
    assert_eq!(count, 28);
}

#[test]
fn parse_digits_test() {
    const FORMAT: u128 = STANDARD;
    let mut mantissa: u64 = 0;
    let digits = b"1234567890123456789012345";
    let mut byte = digits.bytes::<{ FORMAT }>();
    parse::parse_digits::<_, _, FORMAT>(byte.integer_iter(), |digit| {
        mantissa = mantissa.wrapping_mul(10).wrapping_add(digit as _);
    });
    assert_eq!(mantissa, 1096246371337559929);
}

#[test]
#[cfg(not(feature = "compact"))]
fn parse_8digits_test() {
    const FORMAT: u128 = STANDARD;
    let mut mantissa: u64 = 0;
    let digits = b"1234567890123456789012345";
    let mut byte = digits.bytes::<{ FORMAT }>();
    parse::parse_8digits::<_, FORMAT>(byte.integer_iter(), &mut mantissa);
    // We don't check for overflow.
    assert_eq!(mantissa, 11177671081359486962);
}

#[test]
fn parse_u64_digits_test() {
    const FORMAT: u128 = STANDARD;
    let mut mantissa: u64 = 0;
    let mut step = u64_step(10);
    let digits = b"1234567890123456789012345";
    let mut byte = digits.bytes::<{ FORMAT }>();
    parse::parse_u64_digits::<_, FORMAT>(byte.integer_iter(), &mut mantissa, &mut step);
    assert_eq!(mantissa, 1234567890123456789);
    assert_eq!(step, 0);

    let mut mantissa: u64 = 0;
    let mut step = u64_step(10);
    let digits = b"1234567890123456789";
    let mut byte = digits.bytes::<{ FORMAT }>();
    parse::parse_u64_digits::<_, FORMAT>(byte.integer_iter(), &mut mantissa, &mut step);
    assert_eq!(mantissa, 1234567890123456789);
    assert_eq!(step, 0);
}

#[test]
fn is_special_eq_test() {
    const FORMAT: u128 = STANDARD;

    let digits = b"NaN";
    let byte = digits.bytes::<{ FORMAT }>();
    assert_eq!(parse::is_special_eq::<FORMAT>(byte.clone(), b"nan"), 3);

    let byte = digits.bytes::<{ FORMAT }>();
    assert_eq!(parse::is_special_eq::<FORMAT>(byte.clone(), b"NaN"), 3);

    let byte = digits.bytes::<{ FORMAT }>();
    assert_eq!(parse::is_special_eq::<FORMAT>(byte.clone(), b"inf"), 0);
}

#[test]
fn parse_positive_special_test() {
    const FORMAT: u128 = STANDARD;

    let options = Options::new();
    let digits = b"NaN";
    let byte = digits.bytes::<{ FORMAT }>();
    let result = parse::parse_positive_special::<f64, FORMAT>(byte, &options).unwrap();
    assert_eq!(result.1, 3);
    assert!(f64::is_nan(result.0));

    let digits = b"NaN1";
    let byte = digits.bytes::<{ FORMAT }>();
    let result = parse::parse_positive_special::<f64, FORMAT>(byte, &options).unwrap();
    assert_eq!(result.1, 3);
    assert!(f64::is_nan(result.0));

    let digits = b"inf";
    let byte = digits.bytes::<{ FORMAT }>();
    let result = parse::parse_positive_special::<f64, FORMAT>(byte, &options).unwrap();
    assert_eq!(result.1, 3);
    assert!(f64::is_infinite(result.0));

    let digits = b"in";
    let byte = digits.bytes::<{ FORMAT }>();
    let result = parse::parse_positive_special::<f64, FORMAT>(byte, &options);
    assert_eq!(result, None);
}

#[test]
fn parse_partial_special_test() {
    const FORMAT: u128 = STANDARD;

    let options = Options::new();
    let digits = b"NaN";
    let byte = digits.bytes::<{ FORMAT }>();
    let result = parse::parse_partial_special::<f64, FORMAT>(byte, true, &options).unwrap();
    assert_eq!(result.1, 3);
    assert!(f64::is_nan(result.0));
    assert!(f64::is_sign_negative(result.0));
}

#[test]
fn parse_parse_special_test() {
    const FORMAT: u128 = STANDARD;

    let options = Options::new();
    let digits = b"NaN";
    let byte = digits.bytes::<{ FORMAT }>();
    let result = parse::parse_special::<f64, FORMAT>(byte, true, &options).unwrap();
    assert!(f64::is_nan(result));
    assert!(f64::is_sign_negative(result));

    let digits = b"NaN1";
    let byte = digits.bytes::<{ FORMAT }>();
    let result = parse::parse_special::<f64, FORMAT>(byte, true, &options);
    assert_eq!(result, None);
}
