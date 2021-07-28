#![cfg(feature = "parse")]

use lexical_util::iterator::ByteIter;
use lexical_util::noskip::{NoSkipIter, NoSkipIterator};

#[test]
fn noskip_iterator_test() {
    assert!(NoSkipIterator::IS_CONTIGUOUS);

    let digits = b"12345";
    let iter1 = NoSkipIterator::new(digits);
    let iter2 = NoSkipIterator::new(digits);
    assert!(iter1.eq(iter2));

    let mut iter = digits.noskip_iter();
    assert_eq!(iter.as_slice(), &digits[..]);
    assert_eq!(iter.as_ptr(), digits.as_ptr());
    assert_eq!(iter.is_consumed(), false);
    assert_eq!(ByteIter::is_empty(&iter), false);
    assert_eq!(iter.read::<u32>().unwrap(), 0x34333231);
    assert_eq!(iter.length(), 5);
    assert_eq!(iter.cursor(), 0);
    unsafe {
        iter.step_by_unchecked(4);
    }
    assert_eq!(iter.length(), 5);
    assert_eq!(iter.cursor(), 4);
    assert_eq!(unsafe { iter.peek_unchecked() }, &b'5');
    assert_eq!(iter.peek(), Some(&b'5'));
    assert_eq!(iter.next(), Some(&b'5'));
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.next(), None);

    let mut iter = digits.noskip_iter();
    assert_eq!(iter.read::<u64>(), None);
    assert_eq!(iter.nth(4).unwrap(), &b'5');
    assert_eq!(iter.as_slice(), &digits[digits.len()..]);
    assert_eq!(iter.as_ptr(), digits[digits.len()..].as_ptr());

    let mut iter = digits.noskip_iter();
    assert_eq!(iter.peek(), Some(&b'1'));
    unsafe {
        iter.step_unchecked();
    }
    assert_eq!(iter.peek(), Some(&b'2'));
    unsafe {
        iter.step_unchecked();
    }
}

#[test]
#[cfg(feature = "format")]
fn skip_iterator_test() {
    use lexical_util::skip::{self, SkipIter, SkipIterator};

    // TODO(ahuszagh) In the future, needs to have a digit separator.
    type Iter<'a> = SkipIterator<'a, { skip::ILTC }>;
    assert!(!Iter::IS_CONTIGUOUS);

    let digits = b"123_45";
    let iter1 = Iter::new(digits);
    let iter2 = Iter::new(digits);
    assert!(iter1.eq(iter2));

    let mut iter = digits.skip_iter::<{ skip::ILTC }>();
    assert_eq!(iter.as_slice(), &digits[..]);
    assert_eq!(iter.as_ptr(), digits.as_ptr());
    assert_eq!(iter.is_consumed(), false);
    assert_eq!(ByteIter::is_empty(&iter), false);
    assert_eq!(iter.length(), 6);
    assert_eq!(iter.cursor(), 0);

    assert_eq!(unsafe { iter.peek_unchecked() }, &b'1');
    assert_eq!(iter.peek(), Some(&b'1'));
    assert_eq!(iter.next(), Some(&b'1'));
    assert_eq!(iter.next(), Some(&b'2'));
    assert_eq!(iter.next(), Some(&b'3'));
    assert_eq!(iter.next(), Some(&b'4'));
    assert_eq!(iter.next(), Some(&b'5'));
    assert_eq!(iter.next(), None);

    let mut iter = digits.skip_iter::<{ skip::ILTC }>();
    assert_eq!(iter.nth(4).unwrap(), &b'5');
    assert_eq!(iter.as_slice(), &digits[digits.len()..]);
    assert_eq!(iter.as_ptr(), digits[digits.len()..].as_ptr());
}
