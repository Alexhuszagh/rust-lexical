#[macro_use]
mod input;

use core::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use lexical_parse_float::FromLexical;
use serde::Deserialize;

#[derive(Deserialize)]
struct TestData {
    #[serde(rename = "f32")]
    f32_data: Vec<String>,

    #[serde(rename = "f64")]
    f64_data: Vec<String>,
}

fn json_data() -> &'static TestData {
    lazy_static! {
        static ref DATA: TestData = input::read_json("float.json");
    }
    &*DATA
}

// BENCHES

fn json(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("json");
    group.measurement_time(Duration::from_secs(5));

    generator!(group, "f32", json_data().f32_data.iter(), f32);
    generator!(group, "f64", json_data().f64_data.iter(), f64);
}

criterion_group!(json_benches, json);
criterion_main!(json_benches);
