#![cfg(feature = "radix")]

use lexical_util::constants::BUFFER_SIZE;
use lexical_write_float::{options, NumberFormatBuilder, ToLexicalWithOptions};

#[test]
fn issue_224_test() {
    const RADIX: u128 = NumberFormatBuilder::from_radix(3);
    let f = f64::MAX;
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let bytes = f.to_lexical_with_options::<RADIX>(&mut buffer, &options::JAVASCRIPT_LITERAL);
    let actual = unsafe { std::str::from_utf8_unchecked(bytes) };
    assert_eq!(actual, "1.0020200012020012100112000100111212e212221");
}
