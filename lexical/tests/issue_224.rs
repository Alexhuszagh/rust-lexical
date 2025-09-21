#![cfg(feature = "radix")]

#[test]
fn issue_224() {
    const RADIX: u128 = lexical::NumberFormatBuilder::from_radix(3);
    let value = f64::MAX;
    let actual = lexical::to_string_with_options::<_, RADIX>(
        value,
        &lexical::write_float_options::JAVASCRIPT_LITERAL,
    );
    assert_eq!(actual, "1.0020200012020012100112000100111212e212221");
}
