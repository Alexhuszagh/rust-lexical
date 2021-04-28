//! Benchmarks for the lexical string-to-float conversion routines.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;

extern crate criterion;
extern crate minimal_lexical;
extern crate serde_json;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// HELPERS
// -------

// These functions are simple, resuable componetns

/// Find and parse sign and get remaining bytes.
#[inline]
fn parse_sign<'a>(bytes: &'a [u8]) -> (bool, &'a [u8]) {
    match bytes.get(0) {
        Some(&b'+') => (true, &bytes[1..]),
        Some(&b'-') => (false, &bytes[1..]),
        _           => (true, bytes)
    }
}

// Convert u8 to digit.
#[inline]
fn to_digit(c: u8) -> Option<u32> {
    (c as char).to_digit(10)
}

// Add digit from exponent.
#[inline]
fn add_digit_i32(value: i32, digit: u32) -> Option<i32> {
    return value
        .checked_mul(10)?
        .checked_add(digit as i32)
}

// Subtract digit from exponent.
#[inline]
fn sub_digit_i32(value: i32, digit: u32) -> Option<i32> {
    return value
        .checked_mul(10)?
        .checked_sub(digit as i32)
}

// Convert character to digit.
#[inline]
fn is_digit(c: u8) -> bool {
    to_digit(c).is_some()
}

// Split buffer at index.
#[inline]
fn split_at_index<'a>(digits: &'a [u8], index: usize)
    -> (&'a [u8], &'a [u8])
{
    (&digits[..index], &digits[index..])
}

/// Consume until a an invalid digit is found.
///
/// - `digits`      - Slice containing 0 or more digits.
#[inline]
fn consume_digits<'a>(digits: &'a [u8])
    -> (&'a [u8], &'a [u8])
{
    // Consume all digits.
    let mut index = 0;
    while index < digits.len() && is_digit(digits[index]) {
        index += 1;
    }
    split_at_index(digits, index)
}

// Trim leading 0s.
#[inline]
fn ltrim_zero<'a>(bytes: &'a [u8]) -> &'a [u8] {
    let count = bytes.iter().take_while(|&&si| si == b'0').count();
    &bytes[count..]
}

// Trim trailing 0s.
#[inline]
fn rtrim_zero<'a>(bytes: &'a [u8]) -> &'a [u8] {
    let count = bytes.iter().rev().take_while(|&&si| si == b'0').count();
    let index = bytes.len() - count;
    &bytes[..index]
}

// PARSERS
// -------

/// Parse the exponent of the float.
///
/// * `exponent`    - Slice containing the exponent digits.
/// * `is_positive` - If the exponent sign is positive.
fn parse_exponent(exponent: &[u8], is_positive: bool) -> i32 {
    // Parse the sign bit or current data.
    let mut value: i32 = 0;
    match is_positive {
        true  => {
            for c in exponent {
                value = match add_digit_i32(value, to_digit(*c).unwrap()) {
                    Some(v) => v,
                    None    => return i32::max_value(),
                };
            }
        },
        false => {
            for c in exponent {
                value = match sub_digit_i32(value, to_digit(*c).unwrap()) {
                    Some(v) => v,
                    None    => return i32::min_value(),
                };
            }
        }
    }

    value
}

/// Parse float from input bytes, returning the float and the remaining bytes.
///
/// * `bytes`    - Array of bytes leading with float-data.
fn parse_float<'a, F>(bytes: &'a [u8])
    -> (F, &'a [u8])
    where F: minimal_lexical::Float
{
    // Parse the sign.
    let (is_positive, bytes) = parse_sign(bytes);

    // Note: this does not handle special float values.
    // You will have to handle NaN, Inf, and Infinity
    // on your own.

    // Extract and parse the float components:
    //  1. Integer
    //  2. Fraction
    //  3. Exponent
    let (integer_slc, bytes) = consume_digits(bytes);
    let (fraction_slc, bytes) = match bytes.first() {
        Some(&b'.') => consume_digits(&bytes[1..]),
        _           => (&bytes[..0], bytes),
    };
    let (exponent, bytes) = match bytes.first() {
        Some(&b'e') | Some(&b'E') => {
            // Extract and parse the exponent.
            let (is_positive, bytes) = parse_sign(&bytes[1..]);
            let (exponent, bytes) = consume_digits(bytes);
            (parse_exponent(exponent, is_positive), bytes)
        },
        _                         =>  (0, bytes),
    };

    // Note: You may want to check and validate the float data here:
    //  1). Many floats require integer or fraction digits, if a fraction
    //      is present.
    //  2). All floats require either integer or fraction digits.
    //  3). Some floats do not allow a '+' sign before the significant digits.
    //  4). Many floats require exponent digits after the exponent symbol.
    //  5). Some floats do not allow a '+' sign before the exponent.

    // We now need to trim leading and trailing 0s from the integer
    // and fraction, respectively. This is required to make the
    // fast and moderate paths more efficient, and for the slow
    // path.
    let integer_slc = ltrim_zero(integer_slc);
    let fraction_slc = rtrim_zero(fraction_slc);

    // Create the float and return our data.
    let mut float: F = minimal_lexical::parse_float(integer_slc.iter(), fraction_slc.iter(), exponent);
    if !is_positive {
        float = -float;
    }

    (float, bytes)
}

/// Return the `target/debug` directory path.
pub fn debug_dir() -> PathBuf {
    env::current_exe()
        .expect("unittest executable path")
        .parent()
        .expect("unittest executable directory")
        .parent()
        .expect("debug directory")
        .to_path_buf()
}

/// Return the `target` directory path.
pub fn target_dir() -> PathBuf {
    debug_dir()
        .parent()
        .expect("target directory")
        .to_path_buf()
}

/// Return the project directory path.
pub fn project_dir() -> PathBuf {
    target_dir()
        .parent()
        .expect("rust directory")
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

fn parse_json(name: &str) -> Vec<String> {
    let mut path = data_dir();
    path.push(name);
    let s = fs::read_to_string(path).unwrap();
    serde_json::from_str(&s).unwrap()
}

lazy_static! {
    static ref DENORMAL_DATA: Vec<String> = parse_json("denormal_halfway.json");
    static ref LARGE_DATA: Vec<String> = parse_json("large_halfway.json");
    static ref DIGITS2_DATA: Vec<String> = parse_json("digits2.json");
    static ref DIGITS8_DATA: Vec<String> = parse_json("digits8.json");
    static ref DIGITS16_DATA: Vec<String> = parse_json("digits16.json");
    static ref DIGITS32_DATA: Vec<String> = parse_json("digits32.json");
    static ref DIGITS64_DATA: Vec<String> = parse_json("digits64.json");
}

macro_rules! bench_data {
    ($criterion:ident, $group:literal, $name:literal, $data:ident) => {
        let mut group = $criterion.benchmark_group($group);
        group.measurement_time(Duration::from_secs(5));

        let data: &[String] = &$data;
        group.bench_function(concat!($name, "10"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[0].as_bytes()));
        }));
        group.bench_function(concat!($name, "20"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[1].as_bytes()));
        }));
        group.bench_function(concat!($name, "30"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[2].as_bytes()));
        }));
        group.bench_function(concat!($name, "40"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[3].as_bytes()));
        }));
        group.bench_function(concat!($name, "50"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[4].as_bytes()));
        }));
        group.bench_function(concat!($name, "100"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[5].as_bytes()));
        }));
        group.bench_function(concat!($name, "200"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[6].as_bytes()));
        }));
        group.bench_function(concat!($name, "400"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[7].as_bytes()));
        }));
        group.bench_function(concat!($name, "800"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[8].as_bytes()));
        }));
        group.bench_function(concat!($name, "1600"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[9].as_bytes()));
        }));
        group.bench_function(concat!($name, "3200"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[10].as_bytes()));
        }));
        group.bench_function(concat!($name, "6400"), |bench| bench.iter(|| {
            black_box(parse_float::<f64>(data[11].as_bytes()));
        }));
    };
}

macro_rules! bench_digits {
    ($criterion:ident, $group:literal) => {
        let mut group = $criterion.benchmark_group($group);
        group.measurement_time(Duration::from_secs(5));

        let data: &[String] = &DIGITS2_DATA;
        group.bench_function("digits2", |bench| bench.iter(|| {
            for value in data.iter() {
                black_box(parse_float::<f64>(value.as_bytes()));
            }
        }));

        let data: &[String] = &DIGITS8_DATA;
        group.bench_function("digits8", |bench| bench.iter(|| {
            for value in data.iter() {
                black_box(parse_float::<f64>(value.as_bytes()));
            }
        }));

        let data: &[String] = &DIGITS16_DATA;
        group.bench_function("digits16", |bench| bench.iter(|| {
            for value in data.iter() {
                black_box(parse_float::<f64>(value.as_bytes()));
            }
        }));

        let data: &[String] = &DIGITS32_DATA;
        group.bench_function("digits32", |bench| bench.iter(|| {
            for value in data.iter() {
                black_box(parse_float::<f64>(value.as_bytes()));
            }
        }));

        let data: &[String] = &DIGITS64_DATA;
        group.bench_function("digits64", |bench| bench.iter(|| {
            for value in data.iter() {
                black_box(parse_float::<f64>(value.as_bytes()));
            }
        }));
    };
}

fn denormal(criterion: &mut Criterion) {
    bench_data!(criterion, "denormal", "denormal", DENORMAL_DATA);
}

fn large(criterion: &mut Criterion) {
    bench_data!(criterion, "large", "large", LARGE_DATA);
}

fn digits(criterion: &mut Criterion) {
    bench_digits!(criterion, "digits");
}

// MAIN

criterion_group!(denormal_benches, denormal);
criterion_group!(large_benches, large);
criterion_group!(digits_benches, digits);

criterion_main!(denormal_benches, large_benches, digits_benches);
