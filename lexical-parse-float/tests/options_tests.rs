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
    builder = builder.nan_string(b"naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaan");
    assert!(!builder.is_valid());
    builder = builder.nan_string(b"inf");
    assert!(!builder.is_valid());
    builder = builder.nan_string(b"na00n");
    assert!(!builder.is_valid());
    assert!(builder.build().is_err());
    builder = builder.nan_string(b"nan");
    assert!(builder.is_valid());
    assert!(builder.build().is_ok());
}

#[test]
fn invalid_inf_test() {
    let mut builder = OptionsBuilder::default();
    builder = builder.inf_string(b"innnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnf");
    assert!(!builder.is_valid());
    builder = builder.inf_string(b"nan");
    assert!(!builder.is_valid());
    builder = builder.inf_string(b"in00f");
    assert!(!builder.is_valid());
    assert!(builder.build().is_err());
    builder = builder.inf_string(b"i");
    assert!(builder.is_valid());
    builder = builder.inf_string(b"inf");
    assert!(builder.is_valid());
    assert!(builder.build().is_ok());
}

#[test]
fn invalid_infinity_test() {
    let mut builder = OptionsBuilder::default();
    builder = builder.infinity_string(b"innnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnf");
    assert!(!builder.is_valid());
    builder = builder.infinity_string(b"nan");
    assert!(!builder.is_valid());
    builder = builder.infinity_string(b"i");
    assert!(!builder.is_valid());
    builder = builder.inf_string(b"infi000nity");
    assert!(!builder.is_valid());
    assert!(builder.build().is_err());
    builder = builder.inf_string(b"i");
    assert!(builder.is_valid());
    builder = builder.infinity_string(b"infinity");
    assert!(builder.is_valid());
    assert!(builder.build().is_ok());
}

#[test]
fn builder_test() {
    let mut builder = OptionsBuilder::default();

    builder = builder.lossy(true);
    builder = builder.exponent(b'^');
    builder = builder.decimal_point(b',');
    builder = builder.nan_string(b"nan");
    builder = builder.inf_string(b"Infinity");
    builder = builder.infinity_string(b"Infiniiiiiity");

    assert_eq!(builder.get_lossy(), true);
    assert_eq!(builder.get_exponent(), b'^');
    assert_eq!(builder.get_decimal_point(), b',');
    assert_eq!(builder.get_nan_string(), b"nan");
    assert_eq!(builder.get_inf_string(), b"Infinity");
    assert_eq!(builder.get_infinity_string(), b"Infiniiiiiity");

    assert!(builder.is_valid());
    assert_eq!(builder.build(), Ok(unsafe { builder.build_unchecked() }));
}

#[test]
fn options_test() {
    let mut opts = Options::new();

    unsafe {
        opts.set_lossy(true);
        opts.set_exponent(b'^');
        opts.set_decimal_point(b',');
        opts.set_nan_string(b"nan");
        opts.set_inf_string(b"Infinity");
        opts.set_infinity_string(b"Infiniiiiiity");
    }

    assert_eq!(opts.lossy(), true);
    assert_eq!(opts.exponent(), b'^');
    assert_eq!(opts.decimal_point(), b',');
    assert_eq!(opts.nan_string(), b"nan");
    assert_eq!(opts.inf_string(), b"Infinity");
    assert_eq!(opts.infinity_string(), b"Infiniiiiiity");
    assert!(opts.is_valid());

    assert_eq!(Options::builder(), OptionsBuilder::new());
    assert_eq!(opts.rebuild().build(), Ok(opts));
}
