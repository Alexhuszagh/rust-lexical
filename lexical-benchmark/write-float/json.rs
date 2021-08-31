#[macro_use]
mod input;

use core::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_write_float::ToLexical;
use serde::Deserialize;

#[derive(Deserialize)]
struct TestData {
    #[serde(rename = "f32")]
    f32_data: Vec<String>,

    #[serde(rename = "f64")]
    f64_data: Vec<String>,
}

json_data!(TestData, "float.json");
static_data! {
    f32_data json_data f32_data f32 ;
    f64_data json_data f64_data f64 ;
}

// BENCHES

fn json(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("json");
    group.measurement_time(Duration::from_secs(5));

    write_float_generator!(group, "f32", f32_data().iter(), format32);
    write_float_generator!(group, "f64", f64_data().iter(), format64);
}

criterion_group!(json_benches, json);
criterion_main!(json_benches);
