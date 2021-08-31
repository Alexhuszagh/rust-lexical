#[macro_use]
mod input;

use core::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_parse_float::FromLexical;

// BENCHES

fn canada(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("canada");
    group.measurement_time(Duration::from_secs(5));

    let data = input::read_lines("canada.txt");
    parse_float_generator!(group, "canada", data.iter(), f64);
}

criterion_group!(canada_benches, canada);
criterion_main!(canada_benches);
