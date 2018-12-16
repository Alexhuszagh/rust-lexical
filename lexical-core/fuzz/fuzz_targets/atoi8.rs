#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate lexical_core;

fuzz_target!(|data: &[u8]| {
    let _ = lexical_core::atoi::try_atoi8_slice(10, data);
});
