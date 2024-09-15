use lexical_parse_float::options::{Options, OptionsBuilder};

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
    builder = builder.inf_string(Some(b"i"));
    assert!(builder.is_valid());
    builder = builder.inf_string(Some(b"inf"));
    assert!(builder.is_valid());
    assert!(builder.build().is_ok());
    builder = builder.inf_string(None);
    assert!(builder.is_valid());
    builder = builder.infinity_string(None);
    assert!(builder.is_valid());
}

#[test]
fn invalid_infinity_test() {
    let mut builder = OptionsBuilder::default();
    builder =
        builder.infinity_string(Some(b"innnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnf"));
    assert!(!builder.is_valid());
    builder = builder.infinity_string(Some(b"nan"));
    assert!(!builder.is_valid());
    builder = builder.infinity_string(Some(b"i"));
    assert!(!builder.is_valid());
    builder = builder.inf_string(Some(b"infi000nity"));
    assert!(!builder.is_valid());
    assert!(builder.build().is_err());
    builder = builder.inf_string(Some(b"i"));
    assert!(builder.is_valid());
    builder = builder.infinity_string(Some(b"infinity"));
    assert!(builder.is_valid());
    assert!(builder.build().is_ok());
    builder = builder.infinity_string(None);
    assert!(!builder.is_valid());
    builder = builder.inf_string(None);
    assert!(builder.is_valid());
}

#[test]
fn builder_test() {
    let mut builder = OptionsBuilder::default();

    builder = builder.lossy(true);
    builder = builder.exponent(b'^');
    builder = builder.decimal_point(b',');
    builder = builder.nan_string(Some(b"nan"));
    builder = builder.inf_string(Some(b"Infinity"));
    builder = builder.infinity_string(Some(b"Infiniiiiiity"));

    assert_eq!(builder.get_lossy(), true);
    assert_eq!(builder.get_exponent(), b'^');
    assert_eq!(builder.get_decimal_point(), b',');
    assert_eq!(builder.get_nan_string(), Some("nan".as_bytes()));
    assert_eq!(builder.get_inf_string(), Some("Infinity".as_bytes()));
    assert_eq!(builder.get_infinity_string(), Some("Infiniiiiiity".as_bytes()));

    assert!(builder.is_valid());
    assert_eq!(builder.build(), Ok(builder.build_unchecked()));
}

#[test]
fn options_test() {
    let mut opts = Options::new();

    opts.set_lossy(true);
    opts.set_exponent(b'^');
    opts.set_decimal_point(b',');
    opts.set_nan_string(Some(b"nan"));
    opts.set_inf_string(Some(b"Infinity"));
    opts.set_infinity_string(Some(b"Infiniiiiiity"));

    assert_eq!(opts.lossy(), true);
    assert_eq!(opts.exponent(), b'^');
    assert_eq!(opts.decimal_point(), b',');
    assert_eq!(opts.nan_string(), Some("nan".as_bytes()));
    assert_eq!(opts.inf_string(), Some("Infinity".as_bytes()));
    assert_eq!(opts.infinity_string(), Some("Infiniiiiiity".as_bytes()));
    assert!(opts.is_valid());

    assert_eq!(Options::builder(), OptionsBuilder::new());
    assert_eq!(opts.rebuild().build(), Ok(opts));
}
