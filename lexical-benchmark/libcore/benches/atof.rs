//! Benchmarks for Rust's libcore string-to-float conversion routines.

use std::env;
use std::fs;
use std::path::PathBuf;

#[macro_use]
extern crate bencher;

#[macro_use]
extern crate lazy_static;

extern crate serde_json;

use bencher::{black_box, Bencher};

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

fn denormal10(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(data[0].parse::<f64>().unwrap());
    })
}

fn large10(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[0].parse::<f64>().unwrap());
    })
}

fn large20(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[1].parse::<f64>().unwrap());
    })
}

fn large30(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[2].parse::<f64>().unwrap());
    })
}

fn large40(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[3].parse::<f64>().unwrap());
    })
}

fn large50(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[4].parse::<f64>().unwrap());
    })
}

fn large100(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[5].parse::<f64>().unwrap());
    })
}

fn large200(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[6].parse::<f64>().unwrap());
    })
}

fn large400(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[7].parse::<f64>().unwrap());
    })
}

fn large800(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[8].parse::<f64>().unwrap());
    })
}

fn large1600(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[9].parse::<f64>().unwrap());
    })
}

fn large3200(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[10].parse::<f64>().unwrap());
    })
}

fn large6400(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(data[11].parse::<f64>().unwrap());
    })
}

fn digits2(bench: &mut Bencher) {
    let data: &[String] = &DIGITS2_DATA;
    bench.iter(|| {
        for value in data.iter() {
            black_box(value.parse::<f64>().unwrap());
        }
    })
}

fn digits8(bench: &mut Bencher) {
    let data: &[String] = &DIGITS8_DATA;
    bench.iter(|| {
        for value in data.iter() {
            black_box(value.parse::<f64>().unwrap());
        }
    })
}

fn digits16(bench: &mut Bencher) {
    let data: &[String] = &DIGITS16_DATA;
    bench.iter(|| {
        for value in data.iter() {
            black_box(value.parse::<f64>().unwrap());
        }
    })
}

fn digits32(bench: &mut Bencher) {
    let data: &[String] = &DIGITS32_DATA;
    bench.iter(|| {
        for value in data.iter() {
            black_box(value.parse::<f64>().unwrap());
        }
    })
}

fn digits64(bench: &mut Bencher) {
    let data: &[String] = &DIGITS64_DATA;
    bench.iter(|| {
        for value in data.iter() {
            black_box(value.parse::<f64>().unwrap());
        }
    })
}

benchmark_group!(denormal, denormal10);
benchmark_group!(large, large10, large20, large30, large40, large50, large100, large200, large400, large800, large1600, large3200, large6400);
benchmark_group!(digits, digits2, digits8, digits16, digits32, digits64);
benchmark_main!(denormal, large, digits);
