#[macro_use]
mod input;

use core::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_parse_float::FromLexical;

// Default random data size.
const COUNT: usize = 1000;

// BENCHES

macro_rules! bench {
    ($fn:ident, $name:literal, $strategy:expr) => {
        fn $fn(criterion: &mut Criterion) {
            let mut group = criterion.benchmark_group($name);
            group.measurement_time(Duration::from_secs(5));
            let seed = fastrand::u64(..);

            let f32_data = input::string_from_random::<f32>($strategy, COUNT, seed);
            let f64_data = input::string_from_random::<f64>($strategy, COUNT, seed);

            parse_float_generator!(group, "f32", f32_data.iter(), f32);
            parse_float_generator!(group, "f64", f64_data.iter(), f64);
        }
    };
}

bench!(uniform, "random:uniform", input::RandomGen::Uniform);
bench!(one_over_rand32, "random:one_over_rand32", input::RandomGen::OneOverRand32);
bench!(simple_uniform32, "random:simple_uniform32", input::RandomGen::SimpleUniform32);
bench!(simple_int32, "random:simple_int32", input::RandomGen::SimpleInt32);
bench!(int_e_int, "random:int_e_int", input::RandomGen::IntEInt);
bench!(simple_int64, "random:simple_int64", input::RandomGen::SimpleInt64);
bench!(big_int_dot_int, "random:big_int_dot_int", input::RandomGen::BigIntDotInt);
bench!(big_ints, "random:big_ints", input::RandomGen::BigInts);

criterion_group!(uniform_benches, uniform);
criterion_group!(one_over_rand32_benches, one_over_rand32);
criterion_group!(simple_uniform32_benches, simple_uniform32);
criterion_group!(simple_int32_benches, simple_int32);
criterion_group!(int_e_int_benches, int_e_int);
criterion_group!(simple_int64_benches, simple_int64);
criterion_group!(big_int_dot_int_benches, big_int_dot_int);
criterion_group!(big_ints_benches, big_ints);
criterion_main!(
    uniform_benches,
    one_over_rand32_benches,
    simple_uniform32_benches,
    simple_int32_benches,
    int_e_int_benches,
    simple_int64_benches,
    big_int_dot_int_benches,
    big_ints_benches
);
