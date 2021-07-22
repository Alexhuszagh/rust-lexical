#[test]
fn slice_iterator_test() {
    use lexical_util::iterator::Iterator;

    let digits = b"12345";
    let mut iter = digits.iter();
    assert_eq!(iter.as_slice(), &digits[..]);
    assert_eq!(iter.as_ptr(), digits.as_ptr());
    assert_eq!(iter.read::<u32>().unwrap(), 0x34333231);

    let mut iter = digits.iter();
    assert_eq!(iter.read::<u64>(), None);
    assert_eq!(iter.nth(4).unwrap(), &b'5');
    assert_eq!(iter.as_slice(), &digits[digits.len()..]);
    assert_eq!(iter.as_ptr(), digits[digits.len()..].as_ptr());
}
