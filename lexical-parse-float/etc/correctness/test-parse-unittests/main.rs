// Copyright 2018, Alex Huszagh. Unlicensed.
// See https://unlicense.org/

#![allow(non_snake_case)]

use lexical_parse_float::FromLexical;
use serde::Deserialize;

// STRUCTS
// Derived structs for the Toml parser.

#[derive(Debug, Deserialize)]
struct StrtodTests {
    negativeFormattingTests: Vec<String>,
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

/// Return the `target/debug` or `target/release` directory path.
pub fn build_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .expect("unittest executable path")
        .parent()
        .expect("debug/release directory")
        .to_path_buf()
}

/// Return the `target` directory path.
pub fn target_dir() -> std::path::PathBuf {
    build_dir().parent().expect("target directory").to_path_buf()
}

/// Return the project directory path.
pub fn project_dir() -> std::path::PathBuf {
    target_dir().parent().expect("project directory").to_path_buf()
}

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
    let negative_tests_count = tests.negativeFormattingTests.len();
    let formatting_tests_count = tests.FormattingTests.len();
    let conversion_tests_count = tests.ConversionTests.len();
    for test in tests.negativeFormattingTests {
        assert!(f64::from_lexical(test.as_bytes()).is_err());
    }
    for test in tests.FormattingTests {
        run_test(&test.str, &test.hex)
    }
    for test in tests.ConversionTests {
        run_test(&test.str, &test.hex)
    }
    println!("Ran {} negative tests.", negative_tests_count);
    println!("Ran {} formatting tests.", formatting_tests_count);
    println!("Ran {} conversion tests.", conversion_tests_count);
    println!("");
}

fn parse_tests(name: &str) -> StrtodTests {
    let mut test_path = project_dir();
    test_path.push("test-parse-unittests");
    test_path.push(name);
    let test_data = std::fs::read_to_string(test_path).unwrap();

    toml::from_str(&test_data).unwrap()
}

fn main() {
    let filenames = ["strtod_tests.toml", "rust_parse_tests.toml"];
    for filename in filenames.iter() {
        println!("Running Test: {}", filename);
        run_tests(parse_tests(filename));
    }
}
