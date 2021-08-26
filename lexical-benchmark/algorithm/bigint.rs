use core::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn mul(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("mul");
    group.measurement_time(Duration::from_secs(5));
}

fn pow(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("pow");
    group.measurement_time(Duration::from_secs(5));
}

criterion_group!(mul_benches, mul);
criterion_group!(pow_benches, pow);
criterion_main!(mul_benches, pow_benches);
