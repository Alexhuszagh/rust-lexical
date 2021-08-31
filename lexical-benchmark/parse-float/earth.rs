#[macro_use]
mod input;

use core::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_parse_float::FromLexical;

// BENCHES

fn earth(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("earth");
    group.measurement_time(Duration::from_secs(5));

    let data = input::read_csv("earth.csv");
    parse_float_generator!(group, "earth", data.iter(), f64);
}

criterion_group!(earth_benches, earth);
criterion_main!(earth_benches);
