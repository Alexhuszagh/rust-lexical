use lexical_parse_integer::FromLexical;

#[test]
fn issue_91_test() {
    // Derived from:
    //  https://github.com/Alexhuszagh/rust-lexical/issues/91
    assert!(u8::from_lexical(b"354").is_err());
    assert!(u8::from_lexical(b"355").is_err());
    assert!(u8::from_lexical(b"356").is_err());
    assert!(u8::from_lexical(b"357").is_err());
    assert!(u8::from_lexical(b"358").is_err());
    assert!(u8::from_lexical(b"510").is_err());
    assert!(u8::from_lexical(b"511").is_err());
    assert!(u8::from_lexical(b"512").is_err());
    assert!(u8::from_lexical(b"513").is_err());
    assert!(u8::from_lexical(b"514").is_err());
    assert!(u8::from_lexical(b"612").is_err());
    assert!(u8::from_lexical(b"999").is_err());
    assert!(u8::from_lexical(b"1000").is_err());

    let n = u32::MAX as u64 + 1_000_000_000;
    assert!(u32::from_lexical((n - 1).to_string().as_bytes()).is_err());
    assert!(u32::from_lexical(n.to_string().as_bytes()).is_err());
    assert!(u32::from_lexical((n + 1).to_string().as_bytes()).is_err());

    assert!(u8::from_lexical(b"357").is_err());
}
