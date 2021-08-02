use lexical_write_integer::options::{Options, OptionsBuilder};

#[test]
fn options_tests() {
    const X: Options = Options::new();
    assert!(X.is_valid());
    assert_eq!(X, Options::default());
    assert!(OptionsBuilder::new().build().is_ok());
    assert!(OptionsBuilder::default().build().is_ok());
    assert!(OptionsBuilder::default().is_valid());
    assert_eq!(X.rebuild(), Options::builder());
}
