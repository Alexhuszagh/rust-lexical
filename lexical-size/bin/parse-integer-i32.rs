#![feature(llvm_asm)]

mod black_box;
#[cfg(not(feature = "lexical"))]
mod core_parse;

use black_box::black_box;
#[cfg(not(feature = "lexical"))]
use core_parse::from_str_radix;
#[cfg(feature = "lexical")]
use lexical_parse_integer::FromLexical;

#[inline(never)]
fn parse(s: &str) -> i32 {
    #[cfg(feature = "lexical")]
    return i32::from_lexical(s.as_bytes()).unwrap();

    #[cfg(not(feature = "lexical"))]
    return from_str_radix::<i32>(s, 10).unwrap();
}

pub fn main() {
    let _ = black_box(parse(black_box("12")));
}
