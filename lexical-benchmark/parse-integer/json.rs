use std::path::PathBuf;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use lexical_parse_integer::FromLexical;
use serde::Deserialize;

// PATHS

/// Return the `target/debug` directory path.
pub fn debug_dir() -> PathBuf {
    std::env::current_exe()
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
    target_dir().parent().expect("bench directory").to_path_buf()
}

// JSON

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
            path.push("data");
            path.push("integer.json");
            let file = std::fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        };
    }
    &*DATA
}

// GENERATORS

macro_rules! lexical_generator {
    ($group:ident, $name:expr, $iter:expr, $t:ty) => {{
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|x| {
                    black_box(<$t>::from_lexical(x.as_bytes()).unwrap());
                })
            })
        });
    }};
}

macro_rules! core_generator {
    ($group:ident, $name:expr, $iter:expr, $t:ty) => {{
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|x| {
                    black_box(x.parse::<$t>().unwrap());
                })
            })
        });
    }};
}

macro_rules! generator {
    ($group:ident, $type:literal, $iter:expr, $t:ty) => {{
        lexical_generator!($group, concat!("parse_", $type, "_lexical"), $iter, $t);
        core_generator!($group, concat!("parse_", $type, "_core"), $iter, $t);
    }};
}

// BENCHES

fn simple(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("json:simple");
    group.measurement_time(Duration::from_secs(5));

    generator!(group, "u8", json_data().simple.u8_data.iter(), u8);
    generator!(group, "u16", json_data().simple.u16_data.iter(), u16);
    generator!(group, "u32", json_data().simple.u32_data.iter(), u32);
    generator!(group, "u64", json_data().simple.u64_data.iter(), u64);
    generator!(group, "u128", json_data().simple.u128_data.iter(), u128);
}

fn random(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("json:random");
    group.measurement_time(Duration::from_secs(5));

    generator!(group, "u8", json_data().random.u8_data.iter(), u8);
    generator!(group, "u16", json_data().random.u16_data.iter(), u16);
    generator!(group, "u32", json_data().random.u32_data.iter(), u32);
    generator!(group, "u64", json_data().random.u64_data.iter(), u64);
    generator!(group, "u128", json_data().random.u128_data.iter(), u128);

    generator!(group, "i8", json_data().random.i8_data.iter(), i8);
    generator!(group, "i16", json_data().random.i16_data.iter(), i16);
    generator!(group, "i32", json_data().random.i32_data.iter(), i32);
    generator!(group, "i64", json_data().random.i64_data.iter(), i64);
    generator!(group, "i128", json_data().random.i128_data.iter(), i128);
}

criterion_group!(simple_benches, simple);
criterion_group!(random_benches, random);
criterion_main!(simple_benches, random_benches);
