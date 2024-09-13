// Copyright 2021, Alex Huszagh. Unlicensed.
// See https://unlicense.org/

use lexical_parse_float::FromLexical;
use std::collections::HashMap;

fn run_test(line: &str) {
    // Tests have the following format:
    //      hhhh ssssssss dddddddddddddddddd ....
    // The `hhhh` part is the hexadecimal representation for f16,
    // the `ssssssss` part is the hexadecimal representation of f32,
    // the `dddddddddddddddddd` is the hex representation of f64,
    // and the remaining bytes are the string to parse.
    let hex32 = line[5..13].to_lowercase();
    let hex64 = line[14..30].to_lowercase();
    let string = &line[31..];

    let float32 = f32::from_lexical(string.as_bytes()).unwrap();
    let float64 = f64::from_lexical(string.as_bytes()).unwrap();
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
    for (&filename, data) in tests.iter() {
        println!("Running Test: {}", filename);
        for (count, line) in data.lines().enumerate() {
            if cfg!(miri) && count % 10 == 0 {
                println!("Running test {count} for conversion tests.");
            }
            run_test(line);
            if cfg!(miri) && count > 3000 {
                break;
            }
        }
    }
}
