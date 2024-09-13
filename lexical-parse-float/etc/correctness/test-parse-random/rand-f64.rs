// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod _common;

use _common::{validate, ISAAC_SEED};
use rand::{RngCore, SeedableRng};
use rand_isaac::Isaac64Rng;
use std::mem::transmute;

fn main() {
    let mut rnd = Isaac64Rng::from_seed(ISAAC_SEED);
    let mut i = 0;
    while i < 10_000_000 {
        let bits = rnd.next_u64();
        let x: f64 = unsafe { transmute(bits) };
        if x.is_finite() {
            validate(&format!("{:e}", x));
            i += 1;
        }
    }
}
