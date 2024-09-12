// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use lexical_parse_float::FromLexical;
use std::io;
use std::io::prelude::*;
use std::mem::transmute;

// Nothing up my sleeve: Just (PI - 3) in base 16.
#[allow(dead_code)]
pub const SEED: [u32; 3] = [0x243f_6a88, 0x85a3_08d3, 0x1319_8a2e];
#[allow(dead_code)]
pub const ISAAC_SEED: [u8; 32] = [49, 52, 49, 53, 57, 50, 54, 53, 51, 53, 56, 57, 55, 57, 51, 50, 51, 56, 52, 54, 50, 54, 52, 51, 51, 56, 51, 50, 55, 57, 53, 48];

pub fn validate(text: &str) {
    let mut out = io::stdout();
    let x: f64 = f64::from_lexical(text.as_bytes()).unwrap();
    let f64_bytes: u64 = unsafe { transmute(x) };
    let x: f32 = f32::from_lexical(text.as_bytes()).unwrap();
    let f32_bytes: u32 = unsafe { transmute(x) };
    writeln!(&mut out, "{:016x} {:08x} {}", f64_bytes, f32_bytes, text).unwrap();
}
