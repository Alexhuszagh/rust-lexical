use core::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastrand::Rng;
use lexical_parse_integer::FromLexical;
use lexical_util::num::Integer;

// Default random data size.
const COUNT: usize = 1000;

// RANDOM

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RandomGen {
    Uniform,
    Simple,
    SimpleSigned,
    Large,
    LargeSigned,
}

trait IntegerRng: Integer {
    fn uniform(rng: &mut Rng) -> Self;
    fn simple(rng: &mut Rng) -> Self;
    fn large(rng: &mut Rng) -> Self;
    fn simple_signed(rng: &mut Rng) -> Self;
    fn large_signed(rng: &mut Rng) -> Self;

    fn gen(strategy: RandomGen, rng: &mut Rng) -> Self {
        match strategy {
            RandomGen::Uniform => Self::uniform(rng),
            RandomGen::Simple => Self::simple(rng),
            RandomGen::SimpleSigned => Self::simple_signed(rng),
            RandomGen::Large => Self::large(rng),
            RandomGen::LargeSigned => Self::large_signed(rng),
        }
    }
}

macro_rules! unsigned_rng_impl {
    ($($t:ident $smin:literal $smax:literal $lmin:literal $lmax:literal ; )*) => ($(
        impl IntegerRng for $t {
            #[inline]
            fn uniform(rng: &mut Rng) -> Self {
                rng.$t(<$t>::MIN..<$t>::MAX)
            }

            #[inline]
            fn simple(rng: &mut Rng) -> Self {
                rng.$t($smin..$smax)
            }

            #[inline]
            fn simple_signed(_: &mut Rng) -> Self {
                unimplemented!()
            }

            #[inline]
            fn large(rng: &mut Rng) -> Self {
                rng.$t($lmin..$lmax)
            }

            #[inline]
            fn large_signed(_: &mut Rng) -> Self {
                unimplemented!()
            }
        }
    )*);
}

unsigned_rng_impl! {
    u8 0 50 100 255 ;
    u16 0 1000 1024 65535 ;
    u32 0 1000 67108864 4294967295 ;
    u64 0 1000 288230376151711744 18446744073709551615 ;
    u128 0 1000 5316911983139663491615228241121378304 340282366920938463463374607431768211455 ;
}

macro_rules! signed_rng_impl {
    ($(
        $t:ident
        $smin:literal $smax:literal $lmin:literal $lmax:literal
        $ssmin:literal $ssmax:literal $lsmin:literal $lsmax:literal
        ;
    )*) => ($(
        impl IntegerRng for $t {
            #[inline]
            fn uniform(rng: &mut Rng) -> Self {
                rng.$t(<$t>::MIN..<$t>::MAX)
            }

            #[inline]
            fn simple(rng: &mut Rng) -> Self {
                rng.$t($smin..$smax)
            }

            #[inline]
            fn simple_signed(rng: &mut Rng) -> Self {
                rng.$t($ssmin..$ssmax)
            }

            #[inline]
            fn large(rng: &mut Rng) -> Self {
                rng.$t($lmin..$lmax)
            }

            #[inline]
            fn large_signed(rng: &mut Rng) -> Self {
                rng.$t($lsmin..$lsmax)
            }
        }
    )*);
}

signed_rng_impl! {
    i8 0 50 100 127 -50 50 -127 -100 ;
    i16 0 1000 1024 32767 -1000 1000 -32767 -1024 ;
    i32 0 1000 67108864 2147483647 -1000 1000 -2147483647 -67108864 ;
    i64 0 1000 288230376151711744 9223372036854775807 -1000 1000 -9223372036854775807 -288230376151711744 ;
    i128 0 1000 5316911983139663491615228241121378304 170141183460469231731687303715884105727 -1000 1000 -170141183460469231731687303715884105727 -5316911983139663491615228241121378304 ;
}

// Generate a static array of random values.
fn genarray<T: IntegerRng, const N: usize>(strategy: RandomGen, seed: u64) -> Vec<String> {
    let mut rng = Rng::with_seed(seed);
    let mut vec: Vec<String> = Vec::with_capacity(N);
    for _ in 0..N {
        vec.push(T::gen(strategy, &mut rng).to_string());
    }
    vec
}

// GENERATORS

macro_rules! lexical_generator {
    ($group:ident, $name:expr, $data:expr, $t:ty) => {{
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $data.iter().for_each(|x| {
                    black_box(<$t>::from_lexical(x.as_bytes()).unwrap());
                })
            })
        });
    }};
}

macro_rules! core_generator {
    ($group:ident, $name:expr, $data:expr, $t:ty) => {{
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $data.iter().for_each(|x| {
                    black_box(x.parse::<$t>().unwrap());
                })
            })
        });
    }};
}

macro_rules! generator {
    ($group:ident, $type:literal, $iter:expr, $t:ty) => {{
        lexical_generator!($group, concat!("parse_", $type, "_lexical"), $iter, $t);
        core_generator!($group, concat!("parse_", $type, "_core"), $iter, $t);
    }};
}

// BENCHES

macro_rules! bench {
    ($fn:ident, $name:literal, $strategy:expr) => {
        fn $fn(criterion: &mut Criterion) {
            let mut group = criterion.benchmark_group($name);
            group.measurement_time(Duration::from_secs(5));
            let seed = fastrand::u64(..);

            let u8_data = genarray::<u8, COUNT>($strategy, seed);
            let u16_data = genarray::<u16, COUNT>($strategy, seed);
            let u32_data = genarray::<u32, COUNT>($strategy, seed);
            let u64_data = genarray::<u64, COUNT>($strategy, seed);
            let u128_data = genarray::<u128, COUNT>($strategy, seed);
            let i8_data = genarray::<i8, COUNT>($strategy, seed);
            let i16_data = genarray::<i16, COUNT>($strategy, seed);
            let i32_data = genarray::<i32, COUNT>($strategy, seed);
            let i64_data = genarray::<i64, COUNT>($strategy, seed);
            let i128_data = genarray::<i128, COUNT>($strategy, seed);

            generator!(group, "u8", u8_data, u8);
            generator!(group, "u16", u16_data, u16);
            generator!(group, "u32", u32_data, u32);
            generator!(group, "u64", u64_data, u64);
            generator!(group, "u128", u128_data, u128);
            generator!(group, "i8", i8_data, i8);
            generator!(group, "i16", i16_data, i16);
            generator!(group, "i32", i32_data, i32);
            generator!(group, "i64", i64_data, i64);
            generator!(group, "i128", i128_data, i128);
        }
    };
}

macro_rules! bench_signed {
    ($fn:ident, $name:literal, $strategy:expr) => {
        fn $fn(criterion: &mut Criterion) {
            let mut group = criterion.benchmark_group($name);
            group.measurement_time(Duration::from_secs(5));
            let seed = fastrand::u64(..);

            let i8_data = genarray::<i8, COUNT>($strategy, seed);
            let i16_data = genarray::<i16, COUNT>($strategy, seed);
            let i32_data = genarray::<i32, COUNT>($strategy, seed);
            let i64_data = genarray::<i64, COUNT>($strategy, seed);
            let i128_data = genarray::<i128, COUNT>($strategy, seed);

            generator!(group, "i8", i8_data, i8);
            generator!(group, "i16", i16_data, i16);
            generator!(group, "i32", i32_data, i32);
            generator!(group, "i64", i64_data, i64);
            generator!(group, "i128", i128_data, i128);
        }
    };
}

bench!(uniform, "random:uniform", RandomGen::Uniform);
bench!(simple, "random:simple", RandomGen::Simple);
bench!(large, "random:large", RandomGen::Large);
bench_signed!(simple_signed, "random:simple_signed", RandomGen::SimpleSigned);
bench_signed!(large_signed, "random:large_signed", RandomGen::LargeSigned);

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
