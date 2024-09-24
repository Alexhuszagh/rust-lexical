#![cfg(feature = "parse")]

use lexical_util::iterator::{AsBytes, Bytes, DigitsIter, Iter};

#[test]
#[cfg(not(feature = "format"))]
fn digits_iterator_test() {
    use lexical_util::format::STANDARD;

    type Digits<'a> = Bytes<'a, { STANDARD }>;
    assert!(Digits::IS_CONTIGUOUS);

    let digits = b"12345";
    let mut byte1 = Digits::new(digits);
    let mut byte2 = Digits::new(digits);
    assert!(byte1.integer_iter().eq(byte2.integer_iter()));

    let mut byte = digits.bytes::<{ STANDARD }>();
    let mut iter = byte.integer_iter();
    assert_eq!(iter.as_slice(), &digits[..]);
    assert_eq!(iter.as_ptr(), digits.as_ptr());
    assert_eq!(iter.is_consumed(), false);
    assert_eq!(iter.is_buffer_empty(), false);
    assert_eq!(u32::from_le(iter.peek_u32().unwrap()), 0x34333231);
    assert_eq!(iter.buffer_length(), 5);
    assert_eq!(iter.cursor(), 0);
    assert_eq!(iter.current_count(), 0);
    unsafe {
        iter.step_by_unchecked(4);
    }
    assert_eq!(iter.buffer_length(), 5);
    assert_eq!(iter.cursor(), 4);
    assert_eq!(iter.current_count(), 4);
    assert_eq!(iter.peek(), Some(&b'5'));
    assert_eq!(iter.peek(), Some(&b'5'));
    assert_eq!(iter.next(), Some(&b'5'));
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.next(), None);

    let mut byte = digits.bytes::<{ STANDARD }>();
    let mut iter = byte.integer_iter();
    assert_eq!(iter.peek_u64(), None);
    assert_eq!(iter.nth(4).unwrap(), &b'5');
    assert_eq!(iter.as_slice(), &digits[digits.len()..]);
    assert_eq!(iter.as_ptr(), digits[digits.len()..].as_ptr());

    let mut byte = digits.bytes::<{ STANDARD }>();
    let mut iter = byte.integer_iter();
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
    use core::num;

    use lexical_util::format::{NumberFormat, NumberFormatBuilder};
    use static_assertions::const_assert;

    pub const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .digit_separator_flags(true)
        .build();
    const_assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    type Digits<'a> = Bytes<'a, { FORMAT }>;
    assert!(!Digits::IS_CONTIGUOUS);

    let digits = b"123_45";
    let mut byte1 = Digits::new(digits);
    let mut byte2 = Digits::new(digits);
    assert!(byte1.integer_iter().eq(byte2.integer_iter()));

    let mut byte = digits.bytes::<{ FORMAT }>();
    let mut iter = byte.integer_iter();
    assert_eq!(iter.as_slice(), &digits[..]);
    assert_eq!(iter.as_ptr(), digits.as_ptr());
    assert_eq!(iter.is_consumed(), false);
    assert_eq!(iter.is_buffer_empty(), false);
    assert_eq!(iter.buffer_length(), 6);
    assert_eq!(iter.cursor(), 0);
    assert_eq!(iter.current_count(), 0);
    unsafe { iter.step_unchecked() };
    assert_eq!(iter.cursor(), 1);
    assert_eq!(iter.current_count(), 0);
    iter.next();
    assert_eq!(iter.cursor(), 2);
    assert_eq!(iter.current_count(), 1);

    let mut byte = digits.bytes::<{ FORMAT }>();
    let mut iter = byte.integer_iter();
    assert_eq!(iter.peek(), Some(&b'1'));
    assert_eq!(iter.peek(), Some(&b'1'));
    assert_eq!(iter.next(), Some(&b'1'));
    assert_eq!(iter.next(), Some(&b'2'));
    assert_eq!(iter.next(), Some(&b'3'));
    assert_eq!(iter.cursor(), 3);
    assert_eq!(iter.current_count(), 3);
    assert_eq!(iter.next(), Some(&b'4'));
    assert_eq!(iter.cursor(), 5);
    assert_eq!(iter.current_count(), 4);
    assert_eq!(iter.next(), Some(&b'5'));
    assert_eq!(iter.next(), None);

    let mut byte = digits.bytes::<{ FORMAT }>();
    let mut iter = byte.integer_iter();
    assert_eq!(iter.nth(4).unwrap(), &b'5');
    assert_eq!(iter.as_slice(), &digits[digits.len()..]);
    assert_eq!(iter.as_ptr(), digits[digits.len()..].as_ptr());
}
