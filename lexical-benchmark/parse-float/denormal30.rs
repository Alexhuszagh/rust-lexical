// Inline ASM was stabilized in 1.59.0.
// FIXME: Remove when the MSRV for Rustc >= 1.59.0.
#![allow(stable_features)]
#![cfg_attr(feature = "nightly", feature(asm))]

mod black_box;
use black_box::black_box;
use lexical_parse_float::FromLexical;

const DENORMAL30: &str = "2.4703282292062327208828439643e-324";

pub fn main() {
    for _ in 0..150000 {
        black_box(f64::from_lexical(DENORMAL30.as_bytes()).unwrap());
    }
}
