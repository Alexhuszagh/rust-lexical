// Copyright 2015 The Rust Project Developers.
// Modified 2021 Alex Huszagh
// Distributed under an Apache2.0/MIT license.

mod roundtrip;

use lexical_write_float::BUFFER_SIZE;
use roundtrip::roundtrip;

fn f32_ones() -> Result<(), String> {
    let mut powers = [0u32; 32];
    for i in 0..31 {
        powers[i] = 1u32 << i;
    }

    let mut buffer = [b'0'; BUFFER_SIZE];
    for a in &powers {
        for b in &powers {
            for c in &powers {
                let float = (a | b | c) as f32;
                roundtrip(float, &mut buffer)?;
            }
        }
    }
    Ok(())
}

fn f64_ones() -> Result<(), String> {
    let mut powers = [0u64; 64];
    for i in 0..63 {
        powers[i] = 1u64 << i;
    }

    let mut buffer = [b'0'; BUFFER_SIZE];
    for a in &powers {
        for b in &powers {
            for c in &powers {
                let float = (a | b | c) as f64;
                roundtrip(float, &mut buffer)?;
            }
        }
    }
    Ok(())
}

pub fn main() -> Result<(), String> {
    f32_ones()?;
    f64_ones()?;
    Ok(())
}
