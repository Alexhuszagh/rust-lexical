// Copyright 2021, Alex Huszagh. Unlicensed.
// See https://unlicense.org/

#![allow(unused_imports)]

use lexical_parse_float::{FromLexicalWithOptions, Options};
use lexical_util::format::{NumberFormatBuilder, STANDARD};
use rand::{Rng, SeedableRng};
use rand_isaac::Isaac64Rng;
use core::{num, str};
use std::collections::HashMap;

#[allow(dead_code)]
pub const ISAAC_SEED: [u8; 32] = [
    49, 52, 49, 53, 57, 50, 54, 53, 51, 53, 56, 57, 55, 57, 51, 50, 51, 56, 52, 54, 50, 54, 52, 51,
    51, 56, 51, 50, 55, 57, 53, 48,
];

#[cfg(feature = "digit-separator")]
lazy_static::lazy_static! {
    static ref SIGN: regex::Regex = regex::Regex::new("(_+)([+-])").unwrap();
}

#[cfg(feature = "digit-separator")]
fn run_test<Random: Rng>(line: &str, rng: &mut Random) {
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .leading_digit_separator(true)
        .internal_digit_separator(true)
        .trailing_digit_separator(true)
        .consecutive_digit_separator(true)
        .build();

    // Tests have the following format:
    //      hhhh ssssssss dddddddddddddddddd ....
    // The `hhhh` part is the hexadecimal representation for f16,
    // the `ssssssss` part is the hexadecimal representation of f32,
    // the `dddddddddddddddddd` is the hex representation of f64,
    // and the remaining bytes are the string to parse.
    let hex32 = line[5..13].to_lowercase();
    let hex64 = line[14..30].to_lowercase();
    let string = &line[31..];
    const OPTIONS: Options = Options::new();

    // now we want to add the number of digit separators we'll use
    let count = rng.gen_range(1..=4);
    let mut vec = string.as_bytes().to_vec();
    let length = vec.len();
    for _ in 0..count {
        let idx = rng.gen_range(0..length);
        vec.insert(idx, b'_');
    }
    // we need to make sure that our digit separators are in the correct location
    // that is, they cannot be before a `+-` symbol
    let string = str::from_utf8(&vec).unwrap();
    let valid = SIGN.replace(string, "${2}${1}");

    let float32 = f32::from_lexical_with_options::<FMT>(valid.as_bytes(), &OPTIONS).unwrap();
    let float64 = f64::from_lexical_with_options::<FMT>(valid.as_bytes(), &OPTIONS).unwrap();
    assert_eq!(hex32, format!("{:0>8x}", float32.to_bits()));
    assert_eq!(hex64, format!("{:0>16x}", float64.to_bits()));
}

#[cfg(not(feature = "digit-separator"))]
fn run_test<Random: Rng>(line: &str, _: &mut Random) {
    const FMT: u128 = STANDARD;

    // Tests have the following format:
    //      hhhh ssssssss dddddddddddddddddd ....
    // The `hhhh` part is the hexadecimal representation for f16,
    // the `ssssssss` part is the hexadecimal representation of f32,
    // the `dddddddddddddddddd` is the hex representation of f64,
    // and the remaining bytes are the string to parse.
    let hex32 = line[5..13].to_lowercase();
    let hex64 = line[14..30].to_lowercase();
    let string = &line[31..];
    const OPTIONS: Options = Options::new();

    let float32 = f32::from_lexical_with_options::<FMT>(string.as_bytes(), &OPTIONS).unwrap();
    let float64 = f64::from_lexical_with_options::<FMT>(string.as_bytes(), &OPTIONS).unwrap();
    assert_eq!(hex32, format!("{:0>8x}", float32.to_bits()));
    assert_eq!(hex64, format!("{:0>16x}", float64.to_bits()));
}

fn main() {
    // Iterate over all .txt files in the directory.
    // NOTE: Miri does not play nicely with directories so we just compile them in.
    let tests: HashMap<&str, &str> = HashMap::from([
        ("freetype-2-7.txt", include_str!("parse-number-fxx-test-data/data/freetype-2-7.txt")),
        (
            "google-double-conversion.txt",
            include_str!("parse-number-fxx-test-data/data/google-double-conversion.txt"),
        ),
        ("google-wuffs.txt", include_str!("parse-number-fxx-test-data/data/google-wuffs.txt")),
        ("ibm-fpgen.txt", include_str!("parse-number-fxx-test-data/data/ibm-fpgen.txt")),
        (
            "lemire-fast-double-parser.txt",
            include_str!("parse-number-fxx-test-data/data/lemire-fast-double-parser.txt"),
        ),
        (
            "lemire-fast-float.txt",
            include_str!("parse-number-fxx-test-data/data/lemire-fast-float.txt"),
        ),
        (
            "more-test-cases.txt",
            include_str!("parse-number-fxx-test-data/data/more-test-cases.txt"),
        ),
        (
            "remyoudompheng-fptest-0.txt",
            include_str!("parse-number-fxx-test-data/data/remyoudompheng-fptest-0.txt"),
        ),
        (
            "remyoudompheng-fptest-1.txt",
            include_str!("parse-number-fxx-test-data/data/remyoudompheng-fptest-1.txt"),
        ),
        (
            "remyoudompheng-fptest-2.txt",
            include_str!("parse-number-fxx-test-data/data/remyoudompheng-fptest-2.txt"),
        ),
        (
            "remyoudompheng-fptest-3.txt",
            include_str!("parse-number-fxx-test-data/data/remyoudompheng-fptest-3.txt"),
        ),
        (
            "tencent-rapidjson.txt",
            include_str!("parse-number-fxx-test-data/data/tencent-rapidjson.txt"),
        ),
        ("ulfjack-ryu.txt", include_str!("parse-number-fxx-test-data/data/ulfjack-ryu.txt")),
    ]);

    // Unfortunately, randomize the data with miri is too expensive so we just use it normally.
    let mut rng = Isaac64Rng::from_seed(ISAAC_SEED);
    for (&filename, data) in tests.iter() {
        println!("Running Test: {}", filename);
        for (count, line) in data.lines().enumerate() {
            if cfg!(miri) && count % 10 == 0 {
                println!("Running test {count} for conversion tests.");
            }
            run_test(line, &mut rng);
            if cfg!(miri) && count > 3000 {
                break;
            }
        }
    }
}
