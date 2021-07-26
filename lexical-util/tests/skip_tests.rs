#![cfg(all(feature = "format", feature = "parse"))]

use lexical_util::skip::{self, SkipIter};

fn skip_next_eq<const FORMAT: u128>(input: &[u8], output: &[u8]) {
    assert!(input.skip_iter::<FORMAT>().eq(output.iter()));
}

fn skip_peek_eq<const FORMAT: u128>(input: &[u8], output: &[u8]) {
    todo!();
    // Need to implement this **properly**.
    //assert!(input.skip_iter::<FORMAT>().eq(output.iter()));
}

fn skip_iter_eq<const FORMAT: u128>(input: &[u8], output: &[u8]) {
    skip_next_eq::<FORMAT>(input, output);
    // TODO(ahuszagh) Restore...
    //skip_peek_eq::<FORMAT>(input, output);
}

#[test]
fn test_i() {
    // Test iterators that skip single, internal-only digit separators.
    //skip_iter_eq::<{ skip::I }>(b"123.45", b"123.45");
    //skip_iter_eq::<{ skip::I }>(b"1e45", b"1e45");
    //skip_iter_eq::<{ skip::I }>(b"1e", b"1e");
    //skip_iter_eq::<{ skip::I }>(b"1", b"1");
    //skip_iter_eq::<{ skip::I }>(b"_45", b"_45");
    //skip_iter_eq::<{ skip::I }>(b"__45", b"__45");
    //skip_iter_eq::<{ skip::I }>(b"_.45", b"_.45");
    //skip_iter_eq::<{ skip::I }>(b"__.45", b"__.45");
    //skip_iter_eq::<{ skip::I }>(b"4_5", b"45");
    //skip_iter_eq::<{ skip::I }>(b"4__5", b"4_5");
    //skip_iter_eq::<{ skip::I }>(b"4_", b"4_");
    // TODO(ahuszagh) This fails incorrectly....
    //skip_iter_eq::<{ skip::I }>(b"4__", b"4__");
    // TODO(ahuszagh) This might be... a bit of an issue...
    // Probably going to need support for other control characters.
    // Should have characters that need to be ignored, such as `E` and `.`
    //  Should be ignored for integers. Not for floats.
    //skip_iter_eq::<{ skip::I }>(b"4_.", b"4.");

    //assert_eq!(consume_digits_lc(b!("4_."), 10, b'_'), (b!("4"), b!("_.")));
    //assert_eq!(consume_digits_lc(b!("4__."), 10, b'_'), (b!("4"), b!("__.")));
    //assert_eq!(consume_digits_lc(b!("_45_5"), 10, b'_'), (b!("_45"), b!("_5")));
    //assert_eq!(consume_digits_lc(b!("__45__5"), 10, b'_'), (b!("__45"), b!("__5")));
    //assert_eq!(consume_digits_lc(b!("_.45_5"), 10, b'_'), (b!("_"), b!(".45_5")));
    //assert_eq!(consume_digits_lc(b!("__.45__5"), 10, b'_'), (b!("__"), b!(".45__5")));
    //assert_eq!(consume_digits_lc(b!("4_5_"), 10, b'_'), (b!("4"), b!("_5_")));
    //assert_eq!(consume_digits_lc(b!("4__5__"), 10, b'_'), (b!("4"), b!("__5__")));
    //assert_eq!(consume_digits_lc(b!("4_5_.5"), 10, b'_'), (b!("4"), b!("_5_.5")));
    //assert_eq!(consume_digits_lc(b!("4__5__.5"), 10, b'_'), (b!("4"), b!("__5__.5")));
    //assert_eq!(consume_digits_lc(b!("_45_"), 10, b'_'), (b!("_45"), b!("_")));
    //assert_eq!(consume_digits_lc(b!("__45__"), 10, b'_'), (b!("__45"), b!("__")));
    //assert_eq!(consume_digits_lc(b!("_45_.56"), 10, b'_'), (b!("_45"), b!("_.56")));
    //assert_eq!(consume_digits_lc(b!("__45__.56"), 10, b'_'), (b!("__45"), b!("__.56")));
    //assert_eq!(consume_digits_lc(b!("_4_5_"), 10, b'_'), (b!("_4"), b!("_5_")));
    //assert_eq!(consume_digits_lc(b!("__4__5__"), 10, b'_'), (b!("__4"), b!("__5__")));
    //assert_eq!(consume_digits_lc(b!("_4_5_.56"), 10, b'_'), (b!("_4"), b!("_5_.56")));
    //assert_eq!(consume_digits_lc(b!("__4__5__.56"), 10, b'_'), (b!("__4"), b!("__5__.56")));
    // TODO(ahuszagh) Going to need peek tests...
}
