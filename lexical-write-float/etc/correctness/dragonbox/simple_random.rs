// Copyright 2015 The Rust Project Developers.
// Modified 2021 Alex Huszagh
// Distributed under an Apache2.0/MIT license.

mod opts;
mod roundtrip;

use clap::Parser;
use lexical_write_float::BUFFER_SIZE;
use opts::Opts;
use roundtrip::roundtrip;

macro_rules! ones {
    ($powers:ident, $t:ident) => {{
        let mut buffer = [b'0'; BUFFER_SIZE];
        for a in &$powers {
            for b in &$powers {
                for c in &$powers {
                    let float = (a | b | c) as $t;
                    roundtrip(float, &mut buffer)?;
                }
            }
        }
        println!("Finished ones for {}", stringify!($t));
        Ok(())
    }};
}

fn f32_ones() -> Result<(), String> {
    let mut powers = [0u32; 32];
    for i in 0..31 {
        powers[i] = 1u32 << i;
    }
    ones!(powers, f32)
}

fn f64_ones() -> Result<(), String> {
    let mut powers = [0u64; 64];
    for i in 0..63 {
        powers[i] = 1u64 << i;
    }
    ones!(powers, f64)
}

macro_rules! u32_small {
    ($t:ident) => {{
        let mut buffer = [b'0'; BUFFER_SIZE];
        for int in 0..(1u32 << 19) {
            roundtrip(int as $t, &mut buffer)?;
        }
        println!("Finished u32_small for {}", stringify!($t));
        Ok(())
    }};
}

fn f32_u32_small() -> Result<(), String> {
    u32_small!(f32)
}

fn f64_u32_small() -> Result<(), String> {
    u32_small!(f64)
}

macro_rules! u64_pow2 {
    ($t:ident, $u:ident) => {{
        let mut buffer = [b'0'; BUFFER_SIZE];
        for exp in 19..64 {
            let power: u64 = 1 << exp;
            for offset in 1..123 {
                roundtrip((power + offset) as $t, &mut buffer)?;
                roundtrip((power - offset) as $t, &mut buffer)?;
            }
        }
        for offset in 0..123 {
            roundtrip(($u::MAX - offset) as $t, &mut buffer)?;
        }
        println!("Finished u64_pow2 for {}", stringify!($t));
        Ok(())
    }};
}

fn f32_u64_pow2() -> Result<(), String> {
    u64_pow2!(f32, u32)
}

fn f64_u64_pow2() -> Result<(), String> {
    u64_pow2!(f64, u64)
}

macro_rules! many_digits {
    ($t:ident, $count:ident, $lower:literal, $upper:literal) => {{
        let mut buffer = [b'0'; BUFFER_SIZE];
        for _ in 0..$count {
            let digit_count = fastrand::usize($lower..$upper);
            let mut digits = String::new();
            for _ in 0..digit_count {
                let digit = char::from_digit(fastrand::u32(0..10), 10).unwrap();
                digits.push(digit);
            }
            let float = digits.parse::<$t>().map_err(|_| digits.clone())?;
            roundtrip(float, &mut buffer)?;
        }
        println!("Finished many_digits for {}", stringify!($t));
        Ok(())
    }};
}

fn f32_many_digits(count: usize) -> Result<(), String> {
    many_digits!(f32, count, 50, 200)
}

fn f64_many_digits(count: usize) -> Result<(), String> {
    many_digits!(f64, count, 100, 400)
}

macro_rules! long_fractions {
    ($t:ident) => {{
        let mut buffer = [b'0'; BUFFER_SIZE];
        for n in 0..10 {
            let digit = char::from_digit(n, 10).unwrap();
            let mut digits = "0.".to_string();
            for _ in 0..400 {
                digits.push(digit);
                let float = digits.parse::<$t>().map_err(|_| digits.clone())?;
                roundtrip(float, &mut buffer)?;
            }
        }
        println!("Finished long_fractions for {}", stringify!($t));
        Ok(())
    }};
}

fn f32_long_fractions() -> Result<(), String> {
    long_fractions!(f32)
}

fn f64_long_fractions() -> Result<(), String> {
    long_fractions!(f64)
}

macro_rules! huge_pow10 {
    ($t:ident, $lower:literal, $upper:literal) => {{
        let mut buffer = [b'0'; BUFFER_SIZE];
        for exponent in $lower..$upper {
            for int in 0..100000 {
                let digits = format!("{}e{}", int, exponent);
                let float = digits.parse::<$t>().map_err(|_| digits.clone())?;
                roundtrip(float, &mut buffer)?;
            }
        }
        println!("Finished huge_pow10 for {}", stringify!($t));
        Ok(())
    }};
}

fn f32_huge_pow10() -> Result<(), String> {
    huge_pow10!(f32, 30, 40)
}

fn f64_huge_pow10() -> Result<(), String> {
    huge_pow10!(f64, 300, 310)
}

macro_rules! tiny_pow10 {
    ($t:ident, $lower:literal, $upper:literal) => {{
        let mut buffer = [b'0'; BUFFER_SIZE];
        for exponent in $lower..$upper {
            for int in 0..100000 {
                let digits = format!("{}e-{}", int, exponent);
                let float = digits.parse::<$t>().map_err(|_| digits.clone())?;
                roundtrip(float, &mut buffer)?;
            }
        }
        println!("Finished tiny_pow10 for {}", stringify!($t));
        Ok(())
    }};
}

fn f32_tiny_pow10() -> Result<(), String> {
    tiny_pow10!(f32, -36, -46)
}

fn f64_tiny_pow10() -> Result<(), String> {
    tiny_pow10!(f64, 301, 327)
}

macro_rules! subnorm {
    ($t:ident, $u:ident, $upper:literal) => {{
        let mut buffer = [b'0'; BUFFER_SIZE];
        for bits in (0 as $u)..(1 << $upper) {
            let float = $t::from_bits(bits);
            roundtrip(float, &mut buffer)?;
        }
        println!("Finished subnorm for {}", stringify!($t));
        Ok(())
    }};
}

fn f32_subnorm() -> Result<(), String> {
    subnorm!(f32, u32, 21)
}

fn f64_subnorm() -> Result<(), String> {
    subnorm!(f64, u64, 25)
}

macro_rules! rand {
    ($t:ident, $count:ident) => {{
        let mut buffer = [b'0'; BUFFER_SIZE];
        let mut i = 0;
        while i < $count {
            let float = fastrand::$t();
            if float.is_finite() {
                roundtrip(float, &mut buffer)?;
                i += 1;
            }
        }
        println!("Finished rand for {}", stringify!($t));
        Ok(())
    }};
}

fn f32_rand(count: usize) -> Result<(), String> {
    rand!(f32, count)
}

fn f64_rand(count: usize) -> Result<(), String> {
    rand!(f64, count)
}

macro_rules! short_decimal {
    ($t:ident, $upper:literal) => {{
        let mut buffer = [b'0'; BUFFER_SIZE];
        for exponent in 1..$upper {
            for int in 0..10000 {
                if int % 10 == 0 {
                    continue;
                }
                let digits = format!("{}e{}", int, exponent);
                let float = digits.parse::<$t>().map_err(|_| digits.clone())?;
                roundtrip(float, &mut buffer)?;

                let digits = format!("{}e-{}", int, exponent);
                let float = digits.parse::<$t>().map_err(|_| digits.clone())?;
                roundtrip(float, &mut buffer)?;
            }
        }
        println!("Finished short_decimal for {}", stringify!($t));
        Ok(())
    }};
}

fn f32_short_decimal() -> Result<(), String> {
    short_decimal!(f32, 32)
}

fn f64_short_decimal() -> Result<(), String> {
    short_decimal!(f64, 301)
}

pub fn main() -> Result<(), String> {
    let opts: Opts = Opts::parse();

    f32_ones()?;
    f32_u32_small()?;
    f32_u64_pow2()?;
    f32_many_digits(opts.iterations)?;
    f32_long_fractions()?;
    f32_huge_pow10()?;
    f32_tiny_pow10()?;
    f32_subnorm()?;
    f32_rand(opts.iterations)?;
    f32_short_decimal()?;

    f64_ones()?;
    f64_u32_small()?;
    f64_u64_pow2()?;
    f64_many_digits(opts.iterations)?;
    f64_long_fractions()?;
    f64_huge_pow10()?;
    f64_tiny_pow10()?;
    f64_subnorm()?;
    f64_rand(opts.iterations)?;
    f64_short_decimal()?;

    Ok(())
}
