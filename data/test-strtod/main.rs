// Copyright 2018, Alex Huszagh. Unlicensed.
// See https://unlicense.org/

#![allow(non_snake_case)]

extern crate lexical;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

// STRUCTS
// Derived structs for the Toml parser.

#[derive(Debug, Deserialize)]
struct StrtodTests {
    negativeFormattingTests: Vec<String>,
    FormattingTests: Vec<FormattingTest>,
    ConversionTests: Vec<ConversionTest>,
}

#[derive(Debug, Deserialize)]
struct FormattingTest {
    UID: String,
    str: String,
    hex: String,
    int: String,
}

#[derive(Debug, Deserialize)]
struct ConversionTest {
    UID: String,
    str: String,
    hex: String,
    int: String,
}

// PATH

/// Return the `target/debug` or `target/release` directory path.
pub fn build_dir() -> PathBuf {
    env::current_exe()
        .expect("unittest executable path")
        .parent()
        .expect("debug/release directory")
        .to_path_buf()
}

/// Return the `target` directory path.
pub fn target_dir() -> PathBuf {
    build_dir()
        .parent()
        .expect("target directory")
        .to_path_buf()
}

/// Return the project directory path.
pub fn project_dir() -> PathBuf {
    target_dir()
        .parent()
        .expect("project directory")
        .to_path_buf()
}

/// Return the `data` directory path.
pub fn data_dir() -> PathBuf {
    let mut dir = project_dir();
    dir.push("data");
    dir
}

fn run_test(string: &str, hex: &str) {
    // We toggle between "inf" and "infinity" as valid Infinity identifiers.
    let lower = string.to_lowercase();
    if lower == "nan" || !lower.contains("nan") {
        let float: f64 = lexical::parse(string).unwrap();
        let int: u64 = float.to_bits();
        // Rust literals for NaN are not standard conforming:
        // Rust uses 0x7ff8000000000000, not 0x7ff0000000000001
        // We want to pad on the left with 0s, up to 16 characters.
        if float.is_finite() {
            assert_eq!(hex, format!("{:0>16x}", int));
        }
    }
}

fn main() {
    let mut test_path = data_dir();
    test_path.push("test-strtod");
    test_path.push("strtod_tests.toml");
    let test_data = read_to_string(test_path).unwrap();
    let tests: StrtodTests = toml::from_str(&test_data).unwrap();

    for test in tests.FormattingTests {
        run_test(&test.str, &test.hex)
    }
    for test in tests.ConversionTests {
        run_test(&test.str, &test.hex)
    }
}
