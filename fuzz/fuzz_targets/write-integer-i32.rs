#![no_main]
use libfuzzer_sys::fuzz_target;
use lexical_util::constants::BUFFER_SIZE;
use lexical_write_integer::ToLexical;

fuzz_target!(|value: i32| {
    let mut buffer = [b'0'; BUFFER_SIZE];
    let _ = value.to_lexical(&mut buffer);
});
