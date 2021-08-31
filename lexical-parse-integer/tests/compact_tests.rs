#![cfg(feature = "compact")]

use lexical_parse_integer::compact;
use lexical_util::format::STANDARD;

#[test]
fn algorithm_test() {
    let parse_u32 = |digits: &[u8]| compact::algorithm_partial::<u32, u32, STANDARD>(digits);
    let parse_i32 = |digits: &[u8]| compact::algorithm_partial::<i32, u32, STANDARD>(digits);

    assert_eq!(parse_u32(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_u32(b"+12345"), Ok((12345, 6)));
    // This just parses 0 digits, since it's an unsigned type.
    assert_eq!(parse_u32(b"-12345"), Ok((0, 0)));
    assert_eq!(parse_i32(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_i32(b"-12345"), Ok((-12345, 6)));
    assert_eq!(parse_i32(b"+12345"), Ok((12345, 6)));
    assert_eq!(parse_i32(b"+123.45"), Ok((123, 4)));
}
