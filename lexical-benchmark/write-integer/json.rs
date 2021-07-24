use std::{env, fs};
use std::path::PathBuf;
use std::time::Duration;

use lazy_static::lazy_static;
use serde::Deserialize;

//use criterion::{black_box, criterion_group, criterion_main, Criterion};
//use itoa_impl::write as itoa_write;
//use lexical_core::write as lexical_write;

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

/// Return the benchmark directory path.
pub fn bench_dir() -> PathBuf {
    target_dir()
        .parent()
        .expect("bench directory")
        .to_path_buf()
}

// TODO(ahuszagh) These should be changed...
#[derive(Deserialize)]
struct SimpleData {
    #[serde(rename = "u8")]
    u8_data: Vec<String>,

    #[serde(rename = "u16")]
    u16_data: Vec<String>,

    #[serde(rename = "u32")]
    u32_data: Vec<String>,

    #[serde(rename = "u64")]
    u64_data: Vec<String>,

    #[serde(rename = "u128")]
    u128_data: Vec<String>,
}

#[derive(Deserialize)]
struct RandomData {
    #[serde(rename = "u8")]
    u8_data: Vec<String>,

    #[serde(rename = "u16")]
    u16_data: Vec<String>,

    #[serde(rename = "u32")]
    u32_data: Vec<String>,

    #[serde(rename = "u64")]
    u64_data: Vec<String>,

    #[serde(rename = "u128")]
    u128_data: Vec<String>,

    #[serde(rename = "i8")]
    i8_data: Vec<String>,

    #[serde(rename = "i16")]
    i16_data: Vec<String>,

    #[serde(rename = "i32")]
    i32_data: Vec<String>,

    #[serde(rename = "i64")]
    i64_data: Vec<String>,

    #[serde(rename = "i128")]
    i128_data: Vec<String>,
}

#[derive(Deserialize)]
struct TestData {
    simple: SimpleData,
    random: RandomData,
}

fn json_data() -> &'static TestData {
    lazy_static! {
        static ref DATA: TestData = {
            let mut path = bench_dir();
            path.push("write_integer");
            path.push("data.json");
            let file = std::fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        };
    }
    &*DATA
}

// TODO(ahuszagh) Need a macro to parse this...
// First focus on the random data.

// TODO(ahuszagh) Need to fix this.
//  itoa
//  dtoa

pub fn main() {
    // Just check it works...
    json_data();
}
