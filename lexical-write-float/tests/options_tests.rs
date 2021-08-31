use core::num;
use lexical_write_float::options::{self, Options, OptionsBuilder};

#[test]
fn invalid_exponent_test() {
    let mut builder = OptionsBuilder::default();
    builder = builder.exponent(b'\x00');
    assert!(!builder.is_valid());
    builder = builder.exponent(b'\x7f');
    assert!(!builder.is_valid());
    assert!(builder.build().is_err());
    builder = builder.exponent(b'^');
    assert!(builder.is_valid());
    assert!(builder.build().is_ok());
}

#[test]
fn invalid_decimal_point_test() {
    let mut builder = OptionsBuilder::default();
    builder = builder.decimal_point(b'\x00');
    assert!(!builder.is_valid());
    builder = builder.decimal_point(b'\x7f');
    assert!(!builder.is_valid());
    assert!(builder.build().is_err());
    builder = builder.decimal_point(b',');
    assert!(builder.is_valid());
    assert!(builder.build().is_ok());
}

#[test]
fn invalid_nan_test() {
    let mut builder = OptionsBuilder::default();
    builder = builder.nan_string(Some(b"naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaan"));
    assert!(!builder.is_valid());
    builder = builder.nan_string(Some(b"inf"));
    assert!(!builder.is_valid());
    builder = builder.nan_string(Some(b"na00n"));
    assert!(!builder.is_valid());
    assert!(builder.build().is_err());
    builder = builder.nan_string(Some(b"nan"));
    assert!(builder.is_valid());
    assert!(builder.build().is_ok());
    builder = builder.nan_string(None);
    assert!(builder.is_valid());
}

#[test]
fn invalid_inf_test() {
    let mut builder = OptionsBuilder::default();
    builder = builder.inf_string(Some(b"innnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnf"));
    assert!(!builder.is_valid());
    builder = builder.inf_string(Some(b"nan"));
    assert!(!builder.is_valid());
    builder = builder.inf_string(Some(b"in00f"));
    assert!(!builder.is_valid());
    assert!(builder.build().is_err());
    builder = builder.inf_string(Some(b"inf"));
    assert!(builder.is_valid());
    assert!(builder.build().is_ok());
    builder = builder.inf_string(None);
    assert!(builder.is_valid());
}

#[test]
fn builder_test() {
    let mut builder = OptionsBuilder::default();

    builder = builder.max_significant_digits(num::NonZeroUsize::new(10));
    builder = builder.min_significant_digits(num::NonZeroUsize::new(5));
    builder = builder.positive_exponent_break(num::NonZeroI32::new(9));
    builder = builder.negative_exponent_break(num::NonZeroI32::new(-9));
    builder = builder.round_mode(options::RoundMode::Truncate);
    builder = builder.trim_floats(true);
    builder = builder.exponent(b'^');
    builder = builder.decimal_point(b',');
    builder = builder.nan_string(Some(b"nan"));
    builder = builder.inf_string(Some(b"Infinity"));

    assert_eq!(builder.get_max_significant_digits().unwrap().get(), 10);
    assert_eq!(builder.get_min_significant_digits().unwrap().get(), 5);
    assert_eq!(builder.get_positive_exponent_break().unwrap().get(), 9);
    assert_eq!(builder.get_negative_exponent_break().unwrap().get(), -9);
    assert_eq!(builder.get_round_mode(), options::RoundMode::Truncate);
    assert_eq!(builder.get_trim_floats(), true);
    assert_eq!(builder.get_exponent(), b'^');
    assert_eq!(builder.get_decimal_point(), b',');
    assert_eq!(builder.get_nan_string(), Some("nan".as_bytes()));
    assert_eq!(builder.get_inf_string(), Some("Infinity".as_bytes()));

    assert!(builder.is_valid());
    assert_eq!(builder.build(), Ok(unsafe { builder.build_unchecked() }));
}

#[test]
fn options_test() {
    let mut opts = Options::new();

    unsafe {
        opts.set_max_significant_digits(num::NonZeroUsize::new(10));
        opts.set_min_significant_digits(num::NonZeroUsize::new(5));
        opts.set_positive_exponent_break(num::NonZeroI32::new(9));
        opts.set_negative_exponent_break(num::NonZeroI32::new(-9));
        opts.set_round_mode(options::RoundMode::Truncate);
        opts.set_trim_floats(true);
        opts.set_exponent(b'^');
        opts.set_decimal_point(b',');
        opts.set_nan_string(Some(b"nan"));
        opts.set_inf_string(Some(b"Infinity"));
    }

    assert_eq!(opts.max_significant_digits().unwrap().get(), 10);
    assert_eq!(opts.min_significant_digits().unwrap().get(), 5);
    assert_eq!(opts.positive_exponent_break().unwrap().get(), 9);
    assert_eq!(opts.negative_exponent_break().unwrap().get(), -9);
    assert_eq!(opts.round_mode(), options::RoundMode::Truncate);
    assert_eq!(opts.trim_floats(), true);
    assert_eq!(opts.exponent(), b'^');
    assert_eq!(opts.decimal_point(), b',');
    assert_eq!(opts.nan_string(), Some("nan".as_bytes()));
    assert_eq!(opts.inf_string(), Some("Infinity".as_bytes()));
    assert!(opts.is_valid());

    assert_eq!(Options::builder(), OptionsBuilder::new());
    assert_eq!(opts.rebuild().build(), Ok(opts));
}
