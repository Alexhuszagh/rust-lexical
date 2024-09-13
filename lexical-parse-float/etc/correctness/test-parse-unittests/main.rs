// Copyright 2018, Alex Huszagh. Unlicensed.
// See https://unlicense.org/

#![allow(non_snake_case)]

use lexical_parse_float::FromLexical;
use serde::Deserialize;
use std::collections::HashMap;

// STRUCTS
// Derived structs for the Toml parser.

#[derive(Debug, Deserialize)]
struct StrtodTests {
    NegativeFormattingTests: Vec<String>,
    FormattingTests: Vec<FormattingTest>,
    ConversionTests: Vec<ConversionTest>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct FormattingTest {
    UID: String,
    str: String,
    hex: String,
    int: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ConversionTest {
    UID: String,
    str: String,
    hex: String,
    int: String,
}

// PATH

fn run_test(string: &str, hex: &str) {
    // We toggle between "inf" and "infinity" as valid Infinity identifiers.
    let lower = string.to_lowercase();
    if lower == "nan" || !lower.contains("nan") {
        let float: f64 = f64::from_lexical(string.as_bytes()).unwrap();
        let int: u64 = float.to_bits();
        // Rust literals for NaN are not standard conforming:
        // Rust uses 0x7ff8000000000000, not 0x7ff0000000000001
        // We want to pad on the left with 0s, up to 16 characters.
        if float.is_finite() {
            assert_eq!(hex, format!("{:0>16x}", int));
        }
    }
}

fn run_tests(tests: StrtodTests) {
    let negative_tests_count = tests.NegativeFormattingTests.len();
    let formatting_tests_count = tests.FormattingTests.len();
    let conversion_tests_count = tests.ConversionTests.len();
    // Unfortunately, randomize the data with miri is too expensive so we just use it normally.
    let mut count = 0;
    for test in tests.NegativeFormattingTests {
        if cfg!(miri) && count % 10 == 0 {
            println!("Running test {count} for negative formatting.");
        }
        assert!(f64::from_lexical(test.as_bytes()).is_err());
        count += 1;
        if cfg!(miri) && count > 500 {
            break;
        }
    }
    for test in tests.FormattingTests {
        if cfg!(miri) && count % 10 == 0 {
            println!("Running test {count} for positive formatting.");
        }
        run_test(&test.str, &test.hex);
        count += 1;
        if cfg!(miri) && count > 1500 {
            break;
        }
    }
    for test in tests.ConversionTests {
        if cfg!(miri) && count % 10 == 0 {
            println!("Running test {count} for conversion tests.");
        }
        run_test(&test.str, &test.hex);
        if cfg!(miri) && count > 2500 {
            break;
        }
    }
    println!("Ran {} negative tests.", negative_tests_count);
    println!("Ran {} formatting tests.", formatting_tests_count);
    println!("Ran {} conversion tests.\n", conversion_tests_count);
}

fn main() {
    // NOTE: Miri does not play nicely with directories so we just compile them in.
    let tests: HashMap<&str, &str> = HashMap::from([
        ("strtod_tests.toml", include_str!("strtod_tests.toml")),
        ("rust_parse_tests.toml", include_str!("rust_parse_tests.toml")),
    ]);
    for (&filename, &data) in tests.iter() {
        println!("Running Test: {}", filename);
        run_tests(toml::from_str(data).unwrap());
    }
}
