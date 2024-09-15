use lexical_parse_integer::options::{Options, OptionsBuilder};

#[test]
fn options_tests() {
    let builder = OptionsBuilder::new();
    assert!(builder.is_valid());
    assert!(builder.build_unchecked().is_valid());
    assert!(OptionsBuilder::default().is_valid());

    let options: Options = Options::new();
    assert!(options.is_valid());
    assert_eq!(options, Options::default());
    assert!(OptionsBuilder::new().build().is_ok());
    assert!(OptionsBuilder::default().build().is_ok());
    assert!(OptionsBuilder::default().is_valid());
    assert_eq!(options.rebuild(), Options::builder());
}
