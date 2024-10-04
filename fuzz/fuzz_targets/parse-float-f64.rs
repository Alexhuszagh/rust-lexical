#![no_main]
use libfuzzer_sys::fuzz_target;
use lexical_parse_float::FromLexical;

fuzz_target!(|data: &[u8]| {
    let _ = f64::from_lexical(data);
});
