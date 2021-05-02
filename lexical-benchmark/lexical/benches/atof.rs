//! Benchmarks for the lexical string-to-float conversion routines.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;

extern crate criterion;
extern crate lexical_core;
extern crate serde_json;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
    debug_dir().parent().expect("target directory").to_path_buf()
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
    ($criterion:ident, $group:literal, $name:literal, $parse:ident, $data:ident $(, $args:expr)*) => {
        let mut group = $criterion.benchmark_group($group);
        group.measurement_time(Duration::from_secs(5));

        let data: &[String] = &$data;
        group.bench_function(concat!($name, "10"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[0].as_bytes() $(, $args)*).unwrap());
        }));
        group.bench_function(concat!($name, "20"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[1].as_bytes() $(, $args)*).unwrap());
        }));
        group.bench_function(concat!($name, "30"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[2].as_bytes() $(, $args)*).unwrap());
        }));
        group.bench_function(concat!($name, "40"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[3].as_bytes() $(, $args)*).unwrap());
        }));
        group.bench_function(concat!($name, "50"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[4].as_bytes() $(, $args)*).unwrap());
        }));
        group.bench_function(concat!($name, "100"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[5].as_bytes() $(, $args)*).unwrap());
        }));
        group.bench_function(concat!($name, "200"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[6].as_bytes() $(, $args)*).unwrap());
        }));
        group.bench_function(concat!($name, "400"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[7].as_bytes() $(, $args)*).unwrap());
        }));
        group.bench_function(concat!($name, "800"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[8].as_bytes() $(, $args)*).unwrap());
        }));
        group.bench_function(concat!($name, "1600"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[9].as_bytes() $(, $args)*).unwrap());
        }));
        group.bench_function(concat!($name, "3200"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[10].as_bytes() $(, $args)*).unwrap());
        }));
        group.bench_function(concat!($name, "6400"), |bench| bench.iter(|| {
            black_box(lexical_core::$parse::<f64>(data[11].as_bytes() $(, $args)*).unwrap());
        }));
    };
}

macro_rules! bench_digits {
    ($criterion:ident, $group:literal, $parse:ident $(, $args:expr)*) => {
        let mut group = $criterion.benchmark_group($group);
        group.measurement_time(Duration::from_secs(5));

        let data: &[String] = &DIGITS2_DATA;
        group.bench_function("digits2", |bench| bench.iter(|| {
            for value in data.iter() {
                black_box(lexical_core::$parse::<f64>(value.as_bytes() $(, $args)*).unwrap());
            }
        }));

        let data: &[String] = &DIGITS8_DATA;
        group.bench_function("digits8", |bench| bench.iter(|| {
            for value in data.iter() {
                black_box(lexical_core::$parse::<f64>(value.as_bytes() $(, $args)*).unwrap());
            }
        }));

        let data: &[String] = &DIGITS16_DATA;
        group.bench_function("digits16", |bench| bench.iter(|| {
            for value in data.iter() {
                black_box(lexical_core::$parse::<f64>(value.as_bytes() $(, $args)*).unwrap());
            }
        }));

        let data: &[String] = &DIGITS32_DATA;
        group.bench_function("digits32", |bench| bench.iter(|| {
            for value in data.iter() {
                black_box(lexical_core::$parse::<f64>(value.as_bytes() $(, $args)*).unwrap());
            }
        }));

        let data: &[String] = &DIGITS64_DATA;
        group.bench_function("digits64", |bench| bench.iter(|| {
            for value in data.iter() {
                black_box(lexical_core::$parse::<f64>(value.as_bytes() $(, $args)*).unwrap());
            }
        }));
    };
}

fn denormal(criterion: &mut Criterion) {
    bench_data!(criterion, "denormal", "denormal", parse, DENORMAL_DATA);
}

fn large(criterion: &mut Criterion) {
    bench_data!(criterion, "large", "large", parse, LARGE_DATA);
}

fn digits(criterion: &mut Criterion) {
    bench_digits!(criterion, "digits", parse);
}

// MAIN

criterion_group!(denormal_benches, denormal);
criterion_group!(large_benches, large);
criterion_group!(digits_benches, digits);

criterion_main!(denormal_benches, large_benches, digits_benches);
