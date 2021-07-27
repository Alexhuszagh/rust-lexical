#![cfg(feature = "compact")]

use lexical_parse_integer::compact;
use lexical_util::error::ParseErrorCode;
use lexical_util::noskip::NoSkipIter;

#[test]
fn algorithm_test() {
    let parse_u32 = |digits: &[u8]| compact::algorithm::<u32, _, 10, 0>(digits.noskip_iter());
    let parse_i32 = |digits: &[u8]| compact::algorithm::<i32, _, 10, 0>(digits.noskip_iter());

    assert_eq!(parse_u32(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_u32(b"+12345"), Ok((12345, 6)));
    assert_eq!(parse_u32(b"-12345"), Err(ParseErrorCode::InvalidNegativeSign.into()));
    assert_eq!(parse_i32(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_i32(b"-12345"), Ok((-12345, 6)));
    assert_eq!(parse_i32(b"+12345"), Ok((12345, 6)));
    assert_eq!(parse_i32(b"+123.45"), Ok((123, 4)));
}
