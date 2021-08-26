mod input;

use core::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_util::constants::BUFFER_SIZE;
use lexical_write_float::ToLexical;

// Default random data size.
const COUNT: usize = 1000;

// GENERATORS

macro_rules! lexical_generator {
    ($group:ident, $name:expr, $iter:expr) => {{
        let mut buffer: [u8; BUFFER_SIZE] = [b'0'; BUFFER_SIZE];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|&x| {
                    black_box(unsafe { x.to_lexical_unchecked(&mut buffer) });
                })
            })
        });
    }};
}

macro_rules! dtoa_generator {
    ($group:ident, $name:expr, $iter:expr) => {{
        let mut buffer = vec![b'0'; 256];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|x| {
                    dtoa::write(&mut buffer, *x).unwrap();
                    black_box(&buffer);
                    unsafe {
                        buffer.set_len(0);
                    } // Way faster than Vec::clear().
                })
            })
        });
    }};
}

macro_rules! ryu_generator {
    ($group:ident, $name:expr, $iter:expr, $fmt:ident) => {{
        let mut buffer: [u8; 256] = [b'0'; 256];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|x| unsafe {
                    black_box(ryu::raw::$fmt(*x, buffer.as_mut_ptr()));
                })
            })
        });
    }};
}

macro_rules! fmt_generator {
    ($group:ident, $name:expr, $iter:expr) => {{
        use std::io::Write;
        let mut buffer = vec![b'0'; 256];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|x| {
                    black_box(buffer.write_fmt(format_args!("{}", x)).unwrap());
                    unsafe {
                        buffer.set_len(0);
                    } // Way faster than Vec::clear().
                })
            })
        });
    }};
}

macro_rules! generator {
    ($group:ident, $type:expr, $iter:expr, $fmt:ident) => {{
        lexical_generator!($group, concat!("write_", $type, "_lexical"), $iter);
        dtoa_generator!($group, concat!("write_", $type, "_dtoa"), $iter);
        ryu_generator!($group, concat!("write_", $type, "_ryu"), $iter, $fmt);
        fmt_generator!($group, concat!("write_", $type, "_fmt"), $iter);
    }};
}

// BENCHES

macro_rules! bench {
    ($fn:ident, $name:literal, $strategy:expr) => {
        fn $fn(criterion: &mut Criterion) {
            let mut group = criterion.benchmark_group($name);
            group.measurement_time(Duration::from_secs(5));
            let seed = fastrand::u64(..);

            let f32_data: Vec<f32> = input::from_random::<f32>($strategy, COUNT, seed)
                .iter()
                .map(|x| x.parse::<f32>().unwrap())
                .collect();
            let f64_data: Vec<f64> = input::from_random::<f64>($strategy, COUNT, seed)
                .iter()
                .map(|x| x.parse::<f64>().unwrap())
                .collect();

            generator!(group, "f32", f32_data.iter(), format32);
            generator!(group, "f64", f64_data.iter(), format64);
        }
    };
}

bench!(uniform, "random:uniform", input::RandomGen::Uniform);
bench!(one_over_rand32, "random:one_over_rand32", input::RandomGen::OneOverRand32);
bench!(simple_uniform32, "random:simple_uniform32", input::RandomGen::SimpleUniform32);
bench!(simple_int32, "random:simple_int32", input::RandomGen::SimpleInt32);
bench!(simple_int64, "random:simple_int64", input::RandomGen::SimpleInt64);
bench!(big_int_dot_int, "random:big_int_dot_int", input::RandomGen::BigIntDotInt);
bench!(big_ints, "random:big_ints", input::RandomGen::BigInts);

criterion_group!(uniform_benches, uniform);
criterion_group!(one_over_rand32_benches, one_over_rand32);
criterion_group!(simple_uniform32_benches, simple_uniform32);
criterion_group!(simple_int32_benches, simple_int32);
criterion_group!(simple_int64_benches, simple_int64);
criterion_group!(big_int_dot_int_benches, big_int_dot_int);
criterion_group!(big_ints_benches, big_ints);
criterion_main!(
    uniform_benches,
    one_over_rand32_benches,
    simple_uniform32_benches,
    simple_int32_benches,
    simple_int64_benches,
    big_int_dot_int_benches,
    big_ints_benches
);
