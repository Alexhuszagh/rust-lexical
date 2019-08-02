//! Benchmarks for the lexical string-to-float conversion routines.

use std::env;
use std::fs;
use std::path::PathBuf;

#[macro_use]
extern crate bencher;

#[macro_use]
extern crate lazy_static;

extern crate lexical_core;
extern crate serde_json;

use bencher::{black_box, Bencher};
use lexical_core::*;

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
        black_box(atof64_slice(data[0].as_bytes()).unwrap());
    })
}

fn denormal20(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[1].as_bytes()).unwrap());
    })
}

fn denormal30(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[2].as_bytes()).unwrap());
    })
}

fn denormal40(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[3].as_bytes()).unwrap());
    })
}

fn denormal50(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[4].as_bytes()).unwrap());
    })
}

fn denormal100(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[5].as_bytes()).unwrap());
    })
}

fn denormal200(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[6].as_bytes()).unwrap());
    })
}

fn denormal400(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[7].as_bytes()).unwrap());
    })
}

fn denormal800(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[8].as_bytes()).unwrap());
    })
}

fn denormal1600(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[9].as_bytes()).unwrap());
    })
}

fn denormal3200(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[10].as_bytes()).unwrap());
    })
}

fn denormal6400(bench: &mut Bencher) {
    let data: &[String] = &DENORMAL_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[11].as_bytes()).unwrap());
    })
}

fn large10(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[0].as_bytes()).unwrap());
    })
}

fn large20(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[1].as_bytes()).unwrap());
    })
}

fn large30(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[2].as_bytes()).unwrap());
    })
}

fn large40(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[3].as_bytes()).unwrap());
    })
}

fn large50(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[4].as_bytes()).unwrap());
    })
}

fn large100(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[5].as_bytes()).unwrap());
    })
}

fn large200(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[6].as_bytes()).unwrap());
    })
}

fn large400(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[7].as_bytes()).unwrap());
    })
}

fn large800(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[8].as_bytes()).unwrap());
    })
}

fn large1600(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[9].as_bytes()).unwrap());
    })
}

fn large3200(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[10].as_bytes()).unwrap());
    })
}

fn large6400(bench: &mut Bencher) {
    let data: &[String] = &LARGE_DATA;
    bench.iter(|| {
        black_box(atof64_slice(data[11].as_bytes()).unwrap());
    })
}

fn digits2(bench: &mut Bencher) {
    let data: &[String] = &DIGITS2_DATA;
    bench.iter(|| {
        for value in data.iter() {
            black_box(atof64_slice(value.as_bytes()).unwrap());
        }
    })
}

fn digits8(bench: &mut Bencher) {
    let data: &[String] = &DIGITS8_DATA;
    bench.iter(|| {
        for value in data.iter() {
            black_box(atof64_slice(value.as_bytes()).unwrap());
        }
    })
}

fn digits16(bench: &mut Bencher) {
    let data: &[String] = &DIGITS16_DATA;
    bench.iter(|| {
        for value in data.iter() {
            black_box(atof64_slice(value.as_bytes()).unwrap());
        }
    })
}

fn digits32(bench: &mut Bencher) {
    let data: &[String] = &DIGITS32_DATA;
    bench.iter(|| {
        for value in data.iter() {
            black_box(atof64_slice(value.as_bytes()).unwrap());
        }
    })
}

fn digits64(bench: &mut Bencher) {
    let data: &[String] = &DIGITS64_DATA;
    bench.iter(|| {
        for value in data.iter() {
            black_box(atof64_slice(value.as_bytes()).unwrap());
        }
    })
}

benchmark_group!(denormal, denormal10, denormal20, denormal30, denormal40, denormal50, denormal100, denormal200, denormal400, denormal800, denormal1600, denormal3200, denormal6400);
benchmark_group!(large, large10, large20, large30, large40, large50, large100, large200, large400, large800, large1600, large3200, large6400);
benchmark_group!(digits, digits2, digits8, digits16, digits32, digits64);
benchmark_main!(denormal, large, digits);

