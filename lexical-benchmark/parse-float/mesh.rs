#[macro_use]
mod input;

use core::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_parse_float::FromLexical;

// BENCHES

fn mesh(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("mesh");
    group.measurement_time(Duration::from_secs(5));

    let data = input::read_lines("mesh.txt");
    parse_float_generator!(group, "f32", data.iter(), f32);
    parse_float_generator!(group, "f64", data.iter(), f64);
}

criterion_group!(mesh_benches, mesh);
criterion_main!(mesh_benches);
