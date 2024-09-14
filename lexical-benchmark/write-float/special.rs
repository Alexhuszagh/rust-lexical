#[macro_use]
mod input;

use core::mem;
use core::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_write_float::ToLexical;

// Default random data size.
const COUNT: usize = 1000;

// BENCHES

macro_rules! gen_vec {
    ($exp_mask:expr, $i:ident, $f:ident) => {{
        let mut vec: Vec<$f> = Vec::with_capacity(COUNT);
        for _ in 0..COUNT {
            let value = fastrand::$i($exp_mask..);
            // NOTE: We want mem::transmute, not from_bits because we
            // don't want the special handling of from_bits
            vec.push(unsafe { mem::transmute::<$i, $f>(value) });
        }
        vec
    }};
}

macro_rules! bench {
    ($fn:ident, $name:literal) => {
        fn $fn(criterion: &mut Criterion) {
            let mut group = criterion.benchmark_group($name);
            group.measurement_time(Duration::from_secs(5));
            let exp32_mask: u32 = 0x7F800000;
            let exp64_mask: u64 = 0x7FF0000000000000;

            let f32_data = gen_vec!(exp32_mask, u32, f32);
            let f64_data = gen_vec!(exp64_mask, u64, f64);

            write_float_generator!(group, "f32", f32_data.iter(), format32);
            write_float_generator!(group, "f64", f64_data.iter(), format64);
        }
    };
}

bench!(random_special, "random:special");
criterion_group!(special_benches, random_special);
criterion_main!(special_benches);
