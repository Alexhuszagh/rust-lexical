#![cfg(all(feature = "format", feature = "power-of-two"))]

use lexical_parse_float::{options::HEX_FLOAT, FromLexicalWithOptions};
use lexical_util::format::C_HEX_STRING;

#[test]
fn issue_87_test() {
    assert_eq!(
        f64::from_lexical_with_options::<{ C_HEX_STRING }>(b"1f.5p-2", &HEX_FLOAT),
        Ok(0x1f5 as f64 / (16 * 4) as f64)
    );
    assert_eq!(
        f64::from_lexical_with_options::<{ C_HEX_STRING }>(b"c2.a8p6", &HEX_FLOAT),
        Ok((0xc2a8 * (1 << 6)) as f64 / 256 as f64)
    );
}
