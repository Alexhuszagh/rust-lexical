use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastrand::Rng;
use lexical_util::constants::BUFFER_SIZE;
use lexical_util::num::Integer;
use lexical_write_integer::ToLexical;

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
fn genarray<T: IntegerRng, const N: usize>(strategy: RandomGen, seed: u64) -> [T; N] {
    let mut rng = Rng::with_seed(seed);
    let mut array: [T; N] = [T::ZERO; N];
    for index in 0..N {
        array[index] = T::gen(strategy, &mut rng);
    }
    array
}

// GENERATORS

macro_rules! lexical_generator {
    ($group:ident, $name:expr, $data:expr) => {{
        let mut buffer: [u8; BUFFER_SIZE] = [b'0'; BUFFER_SIZE];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $data.iter().for_each(|&x| {
                    black_box(x.to_lexical(&mut buffer));
                })
            })
        });
    }};
}

macro_rules! itoa_generator {
    ($group:ident, $name:expr, $data:expr) => {{
        let mut buffer = vec![b'0'; 256];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $data.iter().for_each(|&x| {
                    itoa::write(&mut buffer, x).unwrap();
                    black_box(&buffer);
                    unsafe {
                        buffer.set_len(0);
                    }
                })
            })
        });
    }};
}

macro_rules! fmt_generator {
    ($group:ident, $name:expr, $data:expr) => {{
        use std::io::Write;
        let mut buffer = vec![b'0'; 256];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $data.iter().for_each(|&x| {
                    black_box(buffer.write_fmt(format_args!("{}", x)).unwrap());
                    unsafe {
                        buffer.set_len(0);
                    }
                })
            })
        });
    }};
}

macro_rules! generator {
    ($group:ident, $name:literal, $type:literal, $data:expr) => {{
        lexical_generator!($group, concat!("write_", $name, "_", $type, "_lexical"), $data);
        itoa_generator!($group, concat!("write_", $name, "_", $type, "_itoa"), $data);
        fmt_generator!($group, concat!("write_", $name, "_", $type, "_fmt"), $data);
    }};
}

// BENCHES

macro_rules! bench {
    ($fn:ident, $name:literal, $strategy:expr) => {
        fn $fn(criterion: &mut Criterion) {
            let mut group = criterion.benchmark_group($name);
            group.measurement_time(Duration::from_secs(5));
            let seed = fastrand::u64(..);

            //let u8_data = genarray::<u8, COUNT>($strategy, seed);
            //let u16_data = genarray::<u16, COUNT>($strategy, seed);
            //let u32_data = genarray::<u32, COUNT>($strategy, seed);
            //let u64_data = genarray::<u64, COUNT>($strategy, seed);
            let u128_data = genarray::<u128, COUNT>($strategy, seed);
            //let i8_data = genarray::<i8, COUNT>($strategy, seed);
            //let i16_data = genarray::<i16, COUNT>($strategy, seed);
            //let i32_data = genarray::<i32, COUNT>($strategy, seed);
            //let i64_data = genarray::<i64, COUNT>($strategy, seed);
            //let i128_data = genarray::<i128, COUNT>($strategy, seed);

            //generator!(group, $name, "u8", u8_data);
            //generator!(group, $name, "u16", u16_data);
            //generator!(group, $name, "u32", u32_data);
            //generator!(group, $name, "u64", u64_data);
            generator!(group, $name, "u128", u128_data);
            //generator!(group, $name, "i8", i8_data);
            //generator!(group, $name, "i16", i16_data);
            //generator!(group, $name, "i32", i32_data);
            //generator!(group, $name, "i64", i64_data);
            //generator!(group, $name, "i128", i128_data);
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

            generator!(group, $name, "i8", i8_data);
            generator!(group, $name, "i16", i16_data);
            generator!(group, $name, "i32", i32_data);
            generator!(group, $name, "i64", i64_data);
            generator!(group, $name, "i128", i128_data);
        }
    };
}

// TODO(ahuszagh) Restore these...
bench!(uniform, "uniform", RandomGen::Uniform);
bench!(simple, "simple", RandomGen::Simple);
//bench!(large, "large", RandomGen::Large);
//bench_signed!(simple_signed, "simple_signed", RandomGen::SimpleSigned);
//bench_signed!(large_signed, "large_signed", RandomGen::LargeSigned);

criterion_group!(uniform_benches, uniform);
criterion_group!(simple_benches, simple);
//criterion_group!(large_benches, large);
//criterion_group!(simple_signed_benches, simple_signed);
//criterion_group!(large_signed_benches, large_signed);
criterion_main!(
    uniform_benches,
    simple_benches /*, large_benches, simple_signed_benches, large_signed_benches*/
);
