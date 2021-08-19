//! Input data reader and random-number generator for benchmarks.
//! This is adapted from fast-float-rust.

#![allow(dead_code)]

use fastrand::Rng;
#[cfg(feature = "floats")]
use lexical_util::num::Float;
#[cfg(feature = "integers")]
use lexical_util::num::Integer;

// PATH

/// Return the `target/debug` directory path.
#[inline]
pub fn debug_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .expect("unittest executable path")
        .parent()
        .expect("unittest executable directory")
        .parent()
        .expect("debug directory")
        .to_path_buf()
}

/// Return the `target` directory path.
#[inline]
pub fn target_dir() -> std::path::PathBuf {
    debug_dir().parent().expect("target directory").to_path_buf()
}

/// Return the benchmark directory path.
#[inline]
pub fn bench_dir() -> std::path::PathBuf {
    target_dir().parent().expect("bench directory").to_path_buf()
}

// FILE

/// Parse JSON data from file.
#[inline]
pub fn read_json<T: serde::de::DeserializeOwned>(name: &str) -> T {
    let mut path = bench_dir();
    path.push("data");
    path.push(name);
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

/// Read data as lines from file.
#[inline]
pub fn read_lines(name: &str) -> Vec<String> {
    let mut path = bench_dir();
    path.push("data");
    path.push(name);
    std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

/// Read data as CSV from file.
#[inline]
pub fn read_csv(name: &str) -> Vec<String> {
    let mut path = bench_dir();
    path.push("data");
    path.push(name);
    std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .map(|x| x.split(','))
        .flatten()
        .map(String::from)
        .collect()
}

// RANDOM

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RandomGen {
    // Generic
    Uniform,

    // Integers
    Simple,
    SimpleSigned,
    Large,
    LargeSigned,

    // Floats
    OneOverRand32,
    SimpleUniform32,
    SimpleInt32,
    IntEInt,
    SimpleInt64,
    BigIntDotInt,
    BigInts,
}

pub trait NumberRng: Sized + ToString {
    fn gen(strategy: RandomGen, rng: &mut Rng) -> String;
}

#[cfg(feature = "integers")]
pub trait IntegerRng: NumberRng + Integer {
    fn uniform(rng: &mut Rng) -> String;
    fn simple(rng: &mut Rng) -> String;
    fn large(rng: &mut Rng) -> String;
    fn simple_signed(rng: &mut Rng) -> String;
    fn large_signed(rng: &mut Rng) -> String;
}

#[cfg(feature = "integers")]
macro_rules! unsigned_rng {
    ($($t:ident $smin:literal $smax:literal $lmin:literal $lmax:literal ; )*) => ($(
        impl NumberRng for $t {
            fn gen(strategy: RandomGen, rng: &mut Rng) -> String {
                match strategy {
                    RandomGen::Uniform => Self::uniform(rng),
                    RandomGen::Simple => Self::simple(rng),
                    RandomGen::SimpleSigned => Self::simple_signed(rng),
                    RandomGen::Large => Self::large(rng),
                    RandomGen::LargeSigned => Self::large_signed(rng),
                    _ => unimplemented!(),
                }
            }
        }

        impl IntegerRng for $t {
            #[inline]
            fn uniform(rng: &mut Rng) -> String {
                (rng.$t(<$t>::MIN..<$t>::MAX)).to_string()
            }

            #[inline]
            fn simple(rng: &mut Rng) -> String {
                (rng.$t($smin..$smax)).to_string()
            }

            #[inline]
            fn simple_signed(_: &mut Rng) -> String {
                unimplemented!()
            }

            #[inline]
            fn large(rng: &mut Rng) -> String {
                (rng.$t($lmin..$lmax)).to_string()
            }

            #[inline]
            fn large_signed(_: &mut Rng) -> String {
                unimplemented!()
            }
        }
    )*);
}

#[cfg(feature = "integers")]
macro_rules! signed_rng {
    ($(
        $t:ident
        $smin:literal $smax:literal $lmin:literal $lmax:literal
        $ssmin:literal $ssmax:literal $lsmin:literal $lsmax:literal
        ;
    )*) => ($(
        impl NumberRng for $t {
            fn gen(strategy: RandomGen, rng: &mut Rng) -> String {
                match strategy {
                    RandomGen::Uniform => Self::uniform(rng),
                    RandomGen::Simple => Self::simple(rng),
                    RandomGen::SimpleSigned => Self::simple_signed(rng),
                    RandomGen::Large => Self::large(rng),
                    RandomGen::LargeSigned => Self::large_signed(rng),
                    _ => unimplemented!(),
                }
            }
        }

        impl IntegerRng for $t {
            #[inline]
            fn uniform(rng: &mut Rng) -> String {
                (rng.$t(<$t>::MIN..<$t>::MAX)).to_string()
            }

            #[inline]
            fn simple(rng: &mut Rng) -> String {
                (rng.$t($smin..$smax)).to_string()
            }

            #[inline]
            fn simple_signed(rng: &mut Rng) -> String {
                (rng.$t($ssmin..$ssmax)).to_string()
            }

            #[inline]
            fn large(rng: &mut Rng) -> String {
                (rng.$t($lmin..$lmax)).to_string()
            }

            #[inline]
            fn large_signed(rng: &mut Rng) -> String {
                (rng.$t($lsmin..$lsmax)).to_string()
            }
        }
    )*);
}

#[cfg(feature = "integers")]
unsigned_rng! {
    u8 0 50 100 255 ;
    u16 0 1000 1024 65535 ;
    u32 0 1000 67108864 4294967295 ;
    u64 0 1000 288230376151711744 18446744073709551615 ;
    u128 0 1000 5316911983139663491615228241121378304 340282366920938463463374607431768211455 ;
}

#[cfg(feature = "integers")]
signed_rng! {
    i8 0 50 100 127 -50 50 -127 -100 ;
    i16 0 1000 1024 32767 -1000 1000 -32767 -1024 ;
    i32 0 1000 67108864 2147483647 -1000 1000 -2147483647 -67108864 ;
    i64 0 1000 288230376151711744 9223372036854775807 -1000 1000 -9223372036854775807 -288230376151711744 ;
    i128 0 1000 5316911983139663491615228241121378304 170141183460469231731687303715884105727 -1000 1000 -170141183460469231731687303715884105727 -5316911983139663491615228241121378304 ;
}

#[cfg(feature = "floats")]
pub trait FloatRng: NumberRng + Float {
    fn uniform(rng: &mut Rng) -> String;
    fn one_over_rand32(rng: &mut Rng) -> String;
    fn simple_uniform32(rng: &mut Rng) -> String;
    fn simple_int32(rng: &mut Rng) -> String;
    fn int_e_int(rng: &mut Rng) -> String;
    fn simple_int64(rng: &mut Rng) -> String;
    fn big_int_dot_int(rng: &mut Rng) -> String;
    fn big_ints(rng: &mut Rng) -> String;
}

#[cfg(feature = "floats")]
macro_rules! float_rng {
    ($($t:ident)*) => ($(
        impl NumberRng for $t {
            fn gen(strategy: RandomGen, rng: &mut Rng) -> String {
                match strategy {
                    RandomGen::Uniform => Self::uniform(rng),
                    RandomGen::OneOverRand32 => Self::one_over_rand32(rng),
                    RandomGen::SimpleUniform32 => Self::simple_uniform32(rng),
                    RandomGen::SimpleInt32 => Self::simple_int32(rng),
                    RandomGen::IntEInt => Self::int_e_int(rng),
                    RandomGen::SimpleInt64 => Self::simple_int64(rng),
                    RandomGen::BigIntDotInt => Self::big_int_dot_int(rng),
                    RandomGen::BigInts => Self::big_ints(rng),
                    _ => unimplemented!(),
                }
            }
        }

        impl FloatRng for $t {
            #[inline]
            fn uniform(rng: &mut Rng) -> String {
                (rng.$t()).to_string()
            }

            #[inline]
            fn one_over_rand32(rng: &mut Rng) -> String {
                (1. / rng.u32(1..) as $t).to_string()
            }

            #[inline]
            fn simple_uniform32(rng: &mut Rng) -> String {
                (rng.u32(..) as $t / u32::MAX as $t).to_string()
            }

            #[inline]
            fn simple_int32(rng: &mut Rng) -> String {
                (rng.u32(..) as $t).to_string()
            }

            #[inline]
            fn int_e_int(rng: &mut Rng) -> String {
                format!("{}e{}", rng.u32(..), rng.u32(..99))
            }

            #[inline]
            fn simple_int64(rng: &mut Rng) -> String {
                (rng.u64(..) as $t).to_string()
            }

            #[inline]
            fn big_int_dot_int(rng: &mut Rng) -> String {
                format!("{}.{}", rng.u32(..), rng.u32(..))
            }

            #[inline]
            fn big_ints(rng: &mut Rng) -> String {
                format!("{}{}{}", rng.u64(..), rng.u64(..), rng.u64(..))
            }
        }
    )*);
}

#[cfg(feature = "floats")]
float_rng! { f32 f64 }

// Generate a static array of random values.
#[inline]
pub fn from_random<T: NumberRng>(strategy: RandomGen, count: usize, seed: u64)
    -> Vec<String>
{
    let mut rng = Rng::with_seed(seed);
    let mut vec: Vec<String> = Vec::with_capacity(count);
    for _ in 0..count {
        vec.push(T::gen(strategy, &mut rng));
    }
    vec
}

// GENERATORS

macro_rules! from_lexical_generator {
    ($group:ident, $name:expr, $iter:expr, $t:ty) => {{
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|x| {
                    black_box(<$t>::from_lexical(x.as_bytes()).unwrap());
                })
            })
        });
    }};
}

macro_rules! str_parse_generator {
    ($group:ident, $name:expr, $iter:expr, $t:ty) => {{
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|x| {
                    black_box(x.parse::<$t>().unwrap());
                })
            })
        });
    }};
}

macro_rules! generator {
    ($group:ident, $type:literal, $iter:expr, $t:ty) => {{
        from_lexical_generator!($group, concat!("parse_", $type, "_lexical"), $iter, $t);
        str_parse_generator!($group, concat!("parse_", $type, "_core"), $iter, $t);
    }};
}
