use lexical_parse_float::limits::{self, ExactFloat, MaxDigits};

#[test]
fn mantissa_limit_test() {
    assert_eq!(f32::mantissa_limit(10), 7);
    assert_eq!(f64::mantissa_limit(10), 15);
}

#[test]
fn exponent_limit_test() {
    assert_eq!(f32::exponent_limit(10), (-10, 10));
    assert_eq!(f64::exponent_limit(10), (-22, 22));
}

#[test]
fn power_limit_test() {
    assert_eq!(limits::u32_power_limit(5), 13);
    assert_eq!(limits::u32_power_limit(10), 9);
    assert_eq!(limits::u64_power_limit(5), 27);
    assert_eq!(limits::u64_power_limit(10), 19);
}

#[test]
fn max_digit_test() {
    assert_eq!(f32::max_digits(10), Some(114));
    assert_eq!(f64::max_digits(10), Some(769));
}
