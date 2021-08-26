#[macro_use]
mod input;

use core::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_parse_integer::FromLexical;
use serde::Deserialize;

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

json_data!(TestData, "integer.json");

// BENCHES

fn simple(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("json:simple");
    group.measurement_time(Duration::from_secs(5));

    parse_integer_generator!(group, "u8", json_data().simple.u8_data.iter(), u8);
    parse_integer_generator!(group, "u16", json_data().simple.u16_data.iter(), u16);
    parse_integer_generator!(group, "u32", json_data().simple.u32_data.iter(), u32);
    parse_integer_generator!(group, "u64", json_data().simple.u64_data.iter(), u64);
    parse_integer_generator!(group, "u128", json_data().simple.u128_data.iter(), u128);
}

fn random(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("json:random");
    group.measurement_time(Duration::from_secs(5));

    parse_integer_generator!(group, "u8", json_data().random.u8_data.iter(), u8);
    parse_integer_generator!(group, "u16", json_data().random.u16_data.iter(), u16);
    parse_integer_generator!(group, "u32", json_data().random.u32_data.iter(), u32);
    parse_integer_generator!(group, "u64", json_data().random.u64_data.iter(), u64);
    parse_integer_generator!(group, "u128", json_data().random.u128_data.iter(), u128);

    parse_integer_generator!(group, "i8", json_data().random.i8_data.iter(), i8);
    parse_integer_generator!(group, "i16", json_data().random.i16_data.iter(), i16);
    parse_integer_generator!(group, "i32", json_data().random.i32_data.iter(), i32);
    parse_integer_generator!(group, "i64", json_data().random.i64_data.iter(), i64);
    parse_integer_generator!(group, "i128", json_data().random.i128_data.iter(), i128);
}

criterion_group!(simple_benches, simple);
criterion_group!(random_benches, random);
criterion_main!(simple_benches, random_benches);
