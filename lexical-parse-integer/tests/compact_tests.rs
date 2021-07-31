// TODO(ahuszagh) Restore...
//#![cfg(feature = "compact")]
//
//mod util;
//
//use lexical_parse_integer::compact;
//use lexical_util::error::ErrorCode;
//use lexical_util::format::STANDARD;
//use lexical_util::iterator::Byte;
//use lexical_util::noskip::AsNoSkip;
//use util::to_format;
//
//#[test]
//fn algorithm_test() {
//    let parse_u32 = |digits: &[u8]| {
//        let mut bytes = digits.noskip();
//        compact::algorithm::<u32, _, STANDARD>(bytes.integer_iter())
//    };
//    let parse_i32 = |digits: &[u8]| {
//        let mut bytes = digits.noskip();
//        compact::algorithm::<i32, _, STANDARD>(bytes.integer_iter())
//    };
//
//    assert_eq!(parse_u32(b"12345"), Ok((12345, 5)));
//    assert_eq!(parse_u32(b"+12345"), Ok((12345, 6)));
//    assert_eq!(parse_u32(b"-12345"), Err(ErrorCode::InvalidNegativeSign.into()));
//    assert_eq!(parse_i32(b"12345"), Ok((12345, 5)));
//    assert_eq!(parse_i32(b"-12345"), Ok((-12345, 6)));
//    assert_eq!(parse_i32(b"+12345"), Ok((12345, 6)));
//    assert_eq!(parse_i32(b"+123.45"), Ok((123, 4)));
//}
