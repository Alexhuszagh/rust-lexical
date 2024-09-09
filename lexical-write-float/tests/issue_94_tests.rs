use core::str;

use lexical_util::constants::BUFFER_SIZE;
use lexical_write_float::ToLexical;

#[test]
fn issue_94_test() {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let neg0: f64 = -0.0;
    let result = neg0.to_lexical(&mut buffer);
    assert_eq!(str::from_utf8(result), Ok("-0.0"));
}
