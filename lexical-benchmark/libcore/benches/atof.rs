//! Benchmarks for Rust's libcore string-to-float conversion routines.

use std::env;
use std::fs;
use std::path::PathBuf;

#[macro_use]
extern crate lazy_static;

extern crate criterion;
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

fn denormal10(criterion: &mut Criterion) {
    let data: &[String] = &DENORMAL_DATA;
    criterion.bench_function("denormal10", |b| b.iter(|| {
        black_box(data[0].parse::<f64>().unwrap());
    }));
}

fn large10(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large10", |b| b.iter(|| {
        black_box(data[0].parse::<f64>().unwrap());
    }));
}

fn large20(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large20", |b| b.iter(|| {
        black_box(data[1].parse::<f64>().unwrap());
    }));
}

fn large30(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large30", |b| b.iter(|| {
        black_box(data[2].parse::<f64>().unwrap());
    }));
}

fn large40(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large40", |b| b.iter(|| {
        black_box(data[3].parse::<f64>().unwrap());
    }));
}

fn large50(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large50", |b| b.iter(|| {
        black_box(data[4].parse::<f64>().unwrap());
    }));
}

fn large100(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large100", |b| b.iter(|| {
        black_box(data[5].parse::<f64>().unwrap());
    }));
}

fn large200(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large200", |b| b.iter(|| {
        black_box(data[6].parse::<f64>().unwrap());
    }));
}

fn large400(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large400", |b| b.iter(|| {
        black_box(data[7].parse::<f64>().unwrap());
    }));
}

fn large800(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large800", |b| b.iter(|| {
        black_box(data[8].parse::<f64>().unwrap());
    }));
}

fn large1600(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large1600", |b| b.iter(|| {
        black_box(data[9].parse::<f64>().unwrap());
    }));
}

fn large3200(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large3200", |b| b.iter(|| {
        black_box(data[10].parse::<f64>().unwrap());
    }));
}

fn large6400(criterion: &mut Criterion) {
    let data: &[String] = &LARGE_DATA;
    criterion.bench_function("large6400", |b| b.iter(|| {
        black_box(data[11].parse::<f64>().unwrap());
    }));
}

fn digits2(criterion: &mut Criterion) {
    let data: &[String] = &DIGITS2_DATA;
    criterion.bench_function("digits2", |b| b.iter(|| {
        for value in data.iter() {
            black_box(value.parse::<f64>().unwrap());
        }
    }));
}

fn digits8(criterion: &mut Criterion) {
    let data: &[String] = &DIGITS8_DATA;
    criterion.bench_function("digits8", |b| b.iter(|| {
        for value in data.iter() {
            black_box(value.parse::<f64>().unwrap());
        }
    }));
}

fn digits16(criterion: &mut Criterion) {
    let data: &[String] = &DIGITS16_DATA;
    criterion.bench_function("digits16", |b| b.iter(|| {
        for value in data.iter() {
            black_box(value.parse::<f64>().unwrap());
        }
    }));
}

fn digits32(criterion: &mut Criterion) {
    let data: &[String] = &DIGITS32_DATA;
    criterion.bench_function("digits32", |b| b.iter(|| {
        for value in data.iter() {
            black_box(value.parse::<f64>().unwrap());
        }
    }));
}

fn digits64(criterion: &mut Criterion) {
    let data: &[String] = &DIGITS64_DATA;
    criterion.bench_function("digits64", |b| b.iter(|| {
        for value in data.iter() {
            black_box(value.parse::<f64>().unwrap());
        }
    }));
}

criterion_group!(denormal, denormal10);
criterion_group!(large, large10, large20, large30, large40, large50, large100, large200, large400, large800, large1600, large3200, large6400);
criterion_group!(digits, digits2, digits8, digits16, digits32, digits64);
criterion_main!(denormal, large, digits);
