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
use rand_isaac::Isaac64Rng;
use rand::distributions::Distribution;
use rand::distributions::uniform::Uniform;
use rand::{Rng, SeedableRng};
use std::char;

fn main() {
    let mut rnd = Isaac64Rng::from_seed(ISAAC_SEED);
    let mut range = Uniform::new(0, 10);
    for _ in 0..5_000_000u64 {
        let num_digits = rnd.gen_range(100..400);
        let digits = gen_digits(num_digits, &mut range, &mut rnd);
        validate(&digits);
    }
}

fn gen_digits(n: u32, range: &mut Uniform<u32>, rnd: &mut Isaac64Rng) -> String {
    let mut s = String::new();
    for _ in 0..n {
        let digit = char::from_digit(range.sample(rnd), 10).unwrap();
        s.push(digit);
    }
    s
}
