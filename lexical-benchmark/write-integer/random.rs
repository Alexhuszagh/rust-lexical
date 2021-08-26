#[macro_use]
mod input;

use core::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_write_integer::ToLexical;

// Default random data size.
const COUNT: usize = 1000;

// BENCHES

macro_rules! bench {
    ($fn:ident, $name:literal, $strategy:expr) => {
        fn $fn(criterion: &mut Criterion) {
            let mut group = criterion.benchmark_group($name);
            group.measurement_time(Duration::from_secs(5));
            let seed = fastrand::u64(..);

            let u8_data = input::type_from_random::<u8>($strategy, COUNT, seed);
            let u16_data = input::type_from_random::<u16>($strategy, COUNT, seed);
            let u32_data = input::type_from_random::<u32>($strategy, COUNT, seed);
            let u64_data = input::type_from_random::<u64>($strategy, COUNT, seed);
            let u128_data = input::type_from_random::<u128>($strategy, COUNT, seed);
            let i8_data = input::type_from_random::<i8>($strategy, COUNT, seed);
            let i16_data = input::type_from_random::<i16>($strategy, COUNT, seed);
            let i32_data = input::type_from_random::<i32>($strategy, COUNT, seed);
            let i64_data = input::type_from_random::<i64>($strategy, COUNT, seed);
            let i128_data = input::type_from_random::<i128>($strategy, COUNT, seed);

            write_integer_generator!(group, "u8", u8_data.iter());
            write_integer_generator!(group, "u16", u16_data.iter());
            write_integer_generator!(group, "u32", u32_data.iter());
            write_integer_generator!(group, "u64", u64_data.iter());
            write_integer_generator!(group, "u128", u128_data.iter());
            write_integer_generator!(group, "i8", i8_data.iter());
            write_integer_generator!(group, "i16", i16_data.iter());
            write_integer_generator!(group, "i32", i32_data.iter());
            write_integer_generator!(group, "i64", i64_data.iter());
            write_integer_generator!(group, "i128", i128_data.iter());
        }
    };
}

macro_rules! bench_signed {
    ($fn:ident, $name:literal, $strategy:expr) => {
        fn $fn(criterion: &mut Criterion) {
            let mut group = criterion.benchmark_group($name);
            group.measurement_time(Duration::from_secs(5));
            let seed = fastrand::u64(..);

            let i8_data = input::type_from_random::<i8>($strategy, COUNT, seed);
            let i16_data = input::type_from_random::<i16>($strategy, COUNT, seed);
            let i32_data = input::type_from_random::<i32>($strategy, COUNT, seed);
            let i64_data = input::type_from_random::<i64>($strategy, COUNT, seed);
            let i128_data = input::type_from_random::<i128>($strategy, COUNT, seed);

            write_integer_generator!(group, "i8", i8_data.iter());
            write_integer_generator!(group, "i16", i16_data.iter());
            write_integer_generator!(group, "i32", i32_data.iter());
            write_integer_generator!(group, "i64", i64_data.iter());
            write_integer_generator!(group, "i128", i128_data.iter());
        }
    };
}

bench!(uniform, "random:uniform", input::RandomGen::Uniform);
bench!(simple, "random:simple", input::RandomGen::Simple);
bench!(large, "random:large", input::RandomGen::Large);
bench_signed!(simple_signed, "random:simple_signed", input::RandomGen::SimpleSigned);
bench_signed!(large_signed, "random:large_signed", input::RandomGen::LargeSigned);

criterion_group!(uniform_benches, uniform);
criterion_group!(simple_benches, simple);
criterion_group!(large_benches, large);
criterion_group!(simple_signed_benches, simple_signed);
criterion_group!(large_signed_benches, large_signed);
criterion_main!(
    uniform_benches,
    simple_benches,
    large_benches,
    simple_signed_benches,
    large_signed_benches
);
