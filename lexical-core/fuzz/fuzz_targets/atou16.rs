#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate lexical_core;

#[cfg(not(feature = "format"))]
fuzz_target!(|data: &[u8]| {
    let _ = lexical_core::parse::<u16>(data);
});

#[cfg(feature = "format")]
fuzz_target!(|data: &[u8]| {
    let _ = lexical_core::parse_format::<u16>(data, lexical_core::NumberFormat::OCAML_STRING);
});

