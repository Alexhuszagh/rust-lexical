#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
use lexical_parse_float::FromLexical;

fuzz_target!(|data: &[u8]| {
    let _ = f32::from_lexical(data);
});
