#![cfg(feature = "parse")]

use std::slice;
use lexical_util::iterator::ByteIter;

#[test]
fn slice_iterator_test() {
    type Iter = slice::Iter<'static, u8>;
    assert!(Iter::IS_CONTIGUOUS);

    let digits = b"12345";
    let iter1 = Iter::new(digits);
    let iter2 = Iter::new(digits);
    assert!(iter1.eq(iter2));

    let mut iter = digits.iter();
    assert_eq!(iter.as_slice(), &digits[..]);
    assert_eq!(iter.as_ptr(), digits.as_ptr());
    assert_eq!(iter.is_consumed(), false);
    assert_eq!(ByteIter::is_empty(&iter), false);
    assert_eq!(iter.read::<u32>().unwrap(), 0x34333231);
    assert_eq!(iter.slice_len(), 5);
    unsafe {
        iter.step_by_unchecked(4);
    }
    assert_eq!(iter.slice_len(), 1);
    assert_eq!(unsafe { iter.peek_unchecked() }, &b'5');
    assert_eq!(iter.peek(), Some(&b'5'));
    assert_eq!(iter.next(), Some(&b'5'));
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.next(), None);

    let mut iter = digits.iter();
    assert_eq!(iter.read::<u64>(), None);
    assert_eq!(iter.nth(4).unwrap(), &b'5');
    assert_eq!(iter.as_slice(), &digits[digits.len()..]);
    assert_eq!(iter.as_ptr(), digits[digits.len()..].as_ptr());

    let mut iter = digits.iter();
    assert_eq!(iter.peek(), Some(&b'1'));
    unsafe { iter.step_unchecked(); }
    assert_eq!(iter.peek(), Some(&b'2'));
    unsafe { iter.step_unchecked(); }
}
