use lexical_parse_float::float::{self, RawFloat};
use lexical_parse_float::limits::ExactFloat;
use lexical_util::num::Float;

#[test]
fn exponent_fast_path_test() {
    assert_eq!(f32::min_exponent_fast_path(10), -10);
    assert_eq!(f32::max_exponent_fast_path(10), 10);
    assert_eq!(f32::max_exponent_disguised_fast_path(10), 17);

    assert_eq!(f64::min_exponent_fast_path(10), -22);
    assert_eq!(f64::max_exponent_fast_path(10), 22);
    assert_eq!(f64::max_exponent_disguised_fast_path(10), 37);
}

fn slow_f32_power(exponent: usize, radix: u32) -> f32 {
    let mut value: f32 = 1.0;
    for _ in 0..exponent {
        value *= radix as f32;
    }
    value
}

fn slow_f64_power(exponent: usize, radix: u32) -> f64 {
    let mut value: f64 = 1.0;
    for _ in 0..exponent {
        value *= radix as f64;
    }
    value
}

fn pow_fast_path(radix: u32) {
    for exponent in 0..f32::exponent_limit(radix).1 + 1 {
        let exponent = exponent as usize;
        let actual = f32::pow_fast_path(exponent, radix);
        assert_eq!(actual, slow_f32_power(exponent, radix));
    }
    for exponent in 0..f64::exponent_limit(radix).1 + 1 {
        let exponent = exponent as usize;
        let actual = f64::pow_fast_path(exponent, radix);
        assert_eq!(actual, slow_f64_power(exponent, radix));
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn pow_fast_path_test() {
    pow_fast_path(10);
    if cfg!(feature = "power-of-two") {
        pow_fast_path(2);
        pow_fast_path(4);
        pow_fast_path(8);
        pow_fast_path(16);
        pow_fast_path(32);
    }
    if cfg!(feature = "radix") {
        pow_fast_path(3);
        pow_fast_path(5);
        pow_fast_path(6);
        pow_fast_path(7);
        pow_fast_path(9);
        pow_fast_path(11);
        pow_fast_path(12);
        pow_fast_path(13);
        pow_fast_path(14);
        pow_fast_path(15);
        pow_fast_path(17);
        pow_fast_path(18);
        pow_fast_path(19);
        pow_fast_path(20);
        pow_fast_path(21);
        pow_fast_path(22);
        pow_fast_path(23);
        pow_fast_path(24);
        pow_fast_path(25);
        pow_fast_path(26);
        pow_fast_path(27);
        pow_fast_path(28);
        pow_fast_path(29);
        pow_fast_path(30);
        pow_fast_path(31);
        pow_fast_path(33);
        pow_fast_path(34);
        pow_fast_path(35);
        pow_fast_path(36);
    }
}

fn slow_int_power(exponent: usize, radix: u32) -> u64 {
    let mut value: u64 = 1;
    for _ in 0..exponent {
        value *= radix as u64;
    }
    value
}

fn int_pow_fast_path(radix: u32) {
    for exponent in 0..f64::mantissa_limit(radix) {
        let exponent = exponent as usize;
        let actual = f64::int_pow_fast_path(exponent, radix);
        assert_eq!(actual, slow_int_power(exponent, radix));
    }
}

#[test]
fn int_pow_fast_path_test() {
    int_pow_fast_path(10);
    if cfg!(feature = "power-of-two") {
        int_pow_fast_path(2);
        int_pow_fast_path(4);
        int_pow_fast_path(8);
        int_pow_fast_path(16);
        int_pow_fast_path(32);
    }
    if cfg!(feature = "radix") {
        int_pow_fast_path(3);
        int_pow_fast_path(5);
        int_pow_fast_path(6);
        int_pow_fast_path(7);
        int_pow_fast_path(9);
        int_pow_fast_path(11);
        int_pow_fast_path(12);
        int_pow_fast_path(13);
        int_pow_fast_path(14);
        int_pow_fast_path(15);
        int_pow_fast_path(17);
        int_pow_fast_path(18);
        int_pow_fast_path(19);
        int_pow_fast_path(20);
        int_pow_fast_path(21);
        int_pow_fast_path(22);
        int_pow_fast_path(23);
        int_pow_fast_path(24);
        int_pow_fast_path(25);
        int_pow_fast_path(26);
        int_pow_fast_path(27);
        int_pow_fast_path(28);
        int_pow_fast_path(29);
        int_pow_fast_path(30);
        int_pow_fast_path(31);
        int_pow_fast_path(33);
        int_pow_fast_path(34);
        int_pow_fast_path(35);
        int_pow_fast_path(36);
    }
}

fn extended_to_float<F: RawFloat>(mantissa: u64, exponent: i32, expected: F) {
    let fp = float::ExtendedFloat80 {
        mant: mantissa,
        exp: exponent,
    };
    assert_eq!(float::extended_to_float::<F>(fp), expected);
}

#[test]
fn extended_to_float_test() {
    let max_mant = (1 << f64::MANTISSA_SIZE) - 1;
    let max_exp = f64::INFINITE_POWER - 1;
    extended_to_float::<f64>(0, 0, 0.0);
    extended_to_float::<f64>(1, 0, 5e-324);
    extended_to_float::<f64>(max_mant, max_exp, f64::MAX);
    extended_to_float::<f64>(0, 1076, 9007199254740992.0);
    extended_to_float::<f64>(1, 1076, 9007199254740994.0);
}
