#[macro_use]
mod input;

use core::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_parse_float::FromLexical;
use serde::Deserialize;

#[derive(Deserialize)]
struct TestData {
    #[serde(rename = "f32")]
    f32_data: Vec<String>,

    #[serde(rename = "f64")]
    f64_data: Vec<String>,
}

json_data!(TestData, "float.json");

// BENCHES

fn json(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("json");
    group.measurement_time(Duration::from_secs(5));

    parse_float_generator!(group, "f32", json_data().f32_data.iter(), f32);
    parse_float_generator!(group, "f64", json_data().f64_data.iter(), f64);
}

criterion_group!(json_benches, json);
criterion_main!(json_benches);
