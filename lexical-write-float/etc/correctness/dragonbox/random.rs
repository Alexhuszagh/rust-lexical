// Copyright 2020 Junekey Jeon
// Modified 2021 Alex Huszagh
// Distributed under an Apache2.0/Boost license.

mod opts;
mod roundtrip;

use clap::Clap;
use lexical_write_float::float::RawFloat;
use lexical_write_float::{BUFFER_SIZE, ToLexical};
use opts::Opts;
use roundtrip::roundtrip;

trait FloatRng: RawFloat {
    fn uniform() -> Self;
    fn one_over_rand32() -> Self;
    fn simple_uniform32() -> Self;
    fn simple_int32() -> Self;
    fn int_e_int() -> Self;
    fn simple_int64() -> Self;
    fn big_int_dot_int() -> Self;
    fn big_ints() -> Self;
}

macro_rules! float_rng {
    ($($t:ident)*) => ($(
        impl FloatRng for $t {
            #[inline]
            fn uniform() -> Self {
                fastrand::$t()
            }

            #[inline]
            fn one_over_rand32() -> Self {
                1. / fastrand::u32(1..) as $t
            }

            #[inline]
            fn simple_uniform32() -> Self {
                fastrand::u32(..) as $t / u32::MAX as $t
            }

            #[inline]
            fn simple_int32() -> Self {
                fastrand::u32(..) as $t
            }

            #[inline]
            fn int_e_int() -> Self {
                format!("{}e{}", fastrand::u32(..), fastrand::u32(..99)).parse::<$t>().unwrap()
            }

            #[inline]
            fn simple_int64() -> Self {
                fastrand::u64(..) as $t
            }

            #[inline]
            fn big_int_dot_int() -> Self {
                format!("{}.{}", fastrand::u32(..), fastrand::u32(..)).parse::<$t>().unwrap()
            }

            #[inline]
            fn big_ints() -> Self {
                let x = format!("{}{}{}", fastrand::u64(..), fastrand::u64(..), fastrand::u64(..));
                x.parse::<$t>().unwrap()
            }
        }
    )*);
}

float_rng! { f32 f64 }

macro_rules! random {
    ($name:ident, $cb:ident) => (
        fn $name<F>(count: usize) -> Result<(), String>
        where
            F: FloatRng + ToLexical + std::str::FromStr + std::string::ToString,
        {
            let mut buffer = [b'0'; BUFFER_SIZE];
            for _ in 0..=count {
                let float = F::$cb();
                roundtrip(float, &mut buffer)?;
            }
            Ok(())
        }
    );
}

random!(uniform_random, uniform);
random!(one_over_rand32_random, one_over_rand32);
random!(simple_uniform32_random, simple_uniform32);
random!(simple_int32_random, simple_int32);
random!(int_e_int_random, int_e_int);
random!(simple_int64_random, simple_int64);
random!(big_int_dot_int_random, big_int_dot_int);
random!(big_ints_random, big_ints);

pub fn main() -> Result<(), String> {
    let opts: Opts = Opts::parse();

    uniform_random::<f32>(opts.iterations)?;
    one_over_rand32_random::<f32>(opts.iterations)?;
    simple_uniform32_random::<f32>(opts.iterations)?;
    simple_int32_random::<f32>(opts.iterations)?;
    int_e_int_random::<f32>(opts.iterations)?;
    simple_int64_random::<f32>(opts.iterations)?;
    big_int_dot_int_random::<f32>(opts.iterations)?;
    big_ints_random::<f32>(opts.iterations)?;

    uniform_random::<f64>(opts.iterations)?;
    one_over_rand32_random::<f64>(opts.iterations)?;
    simple_uniform32_random::<f64>(opts.iterations)?;
    simple_int32_random::<f64>(opts.iterations)?;
    int_e_int_random::<f64>(opts.iterations)?;
    simple_int64_random::<f64>(opts.iterations)?;
    big_int_dot_int_random::<f64>(opts.iterations)?;
    big_ints_random::<f64>(opts.iterations)?;

    Ok(())
}
