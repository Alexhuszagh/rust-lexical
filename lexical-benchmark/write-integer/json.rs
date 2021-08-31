#[macro_use]
mod input;

use core::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_write_integer::ToLexical;
use serde::Deserialize;

// CHAIN RANDOM

// Uses a PRNG to try to trick the branch predictor.
// Use a simple linear congruent generator with a fixed seed.
//      Xn+1 = (aXn + c) mod c
//          m = 2^32
//          a = 1664525
//          c = 1013904223
//
// The seed was chosen via:
//      np.random.randint(0, 2**32-1)
static mut SEED: u64 = 3937647125;

// Generate next random number in pattern.
fn rand_next() -> u32 {
    unsafe {
        let inner = SEED.saturating_mul(1664525).saturating_add(1013904223);
        SEED = inner % (1 << 32);
        SEED as u32
    }
}

// Calculate a random boolean.
fn rand_bool() -> bool {
    let value = rand_next();
    value <= 0x7FFFFFFF
}

struct ChainRandom<T, U> {
    t: T,
    u: U,
    state: ChainState,
}

enum ChainState {
    Both,
    Front,
    Back,
}

impl<T, U> Iterator for ChainRandom<T, U>
where
    T: Iterator,
    U: Iterator<Item = T::Item>,
{
    type Item = T::Item;

    #[inline]
    fn next(&mut self) -> Option<T::Item> {
        match self.state {
            ChainState::Both => {
                match rand_bool() {
                    // Take the first branch.
                    true => match self.t.next() {
                        Some(v) => Some(v),
                        None => {
                            self.state = ChainState::Back;
                            self.u.next()
                        },
                    },
                    // Take the second branch.
                    false => match self.u.next() {
                        Some(v) => Some(v),
                        None => {
                            self.state = ChainState::Front;
                            self.t.next()
                        },
                    },
                }
            },
            ChainState::Front => self.t.next(),
            ChainState::Back => self.u.next(),
        }
    }

    #[inline]
    fn count(self) -> usize {
        match self.state {
            ChainState::Both => self.t.count() + self.u.count(),
            ChainState::Front => self.t.count(),
            ChainState::Back => self.u.count(),
        }
    }
}

// Generate random chain.
fn chain<T, U>(t: T, u: U) -> ChainRandom<T, U> {
    ChainRandom {
        t,
        u,
        state: ChainState::Both,
    }
}

// JSON

#[derive(Deserialize)]
struct SimpleData {
    #[serde(rename = "u8")]
    u8_data: Vec<String>,

    #[serde(rename = "u16")]
    u16_data: Vec<String>,

    #[serde(rename = "u32")]
    u32_data: Vec<String>,

    #[serde(rename = "u64")]
    u64_data: Vec<String>,

    #[serde(rename = "u128")]
    u128_data: Vec<String>,
}

#[derive(Deserialize)]
struct RandomData {
    #[serde(rename = "u8")]
    u8_data: Vec<String>,

    #[serde(rename = "u16")]
    u16_data: Vec<String>,

    #[serde(rename = "u32")]
    u32_data: Vec<String>,

    #[serde(rename = "u64")]
    u64_data: Vec<String>,

    #[serde(rename = "u128")]
    u128_data: Vec<String>,

    #[serde(rename = "i8")]
    i8_data: Vec<String>,

    #[serde(rename = "i16")]
    i16_data: Vec<String>,

    #[serde(rename = "i32")]
    i32_data: Vec<String>,

    #[serde(rename = "i64")]
    i64_data: Vec<String>,

    #[serde(rename = "i128")]
    i128_data: Vec<String>,
}

#[derive(Deserialize)]
struct TestData {
    simple: SimpleData,
    random: RandomData,
}

json_data!(TestData, "integer.json");
static_data! {
    simple_u8_data json_data simple u8_data u8 ;
    simple_u16_data json_data simple u16_data u16 ;
    simple_u32_data json_data simple u32_data u32 ;
    simple_u64_data json_data simple u64_data u64 ;
    simple_u128_data json_data simple u128_data u128 ;

    random_u8_data json_data random u8_data u8 ;
    random_u16_data json_data random u16_data u16 ;
    random_u32_data json_data random u32_data u32 ;
    random_u64_data json_data random u64_data u64 ;
    random_u128_data json_data random u128_data u128 ;

    random_i8_data json_data random i8_data i8 ;
    random_i16_data json_data random i16_data i16 ;
    random_i32_data json_data random i32_data i32 ;
    random_i64_data json_data random i64_data i64 ;
    random_i128_data json_data random i128_data i128 ;
}

// BENCHES

fn simple(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("json:simple");
    group.measurement_time(Duration::from_secs(5));

    write_integer_generator!(group, "u8", simple_u8_data().iter());
    write_integer_generator!(group, "u16", simple_u16_data().iter());
    write_integer_generator!(group, "u32", simple_u32_data().iter());
    write_integer_generator!(group, "u64", simple_u64_data().iter());
    write_integer_generator!(group, "u128", simple_u128_data().iter());
}

fn random(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("json:random");
    group.measurement_time(Duration::from_secs(5));

    write_integer_generator!(group, "u8", random_u8_data().iter());
    write_integer_generator!(group, "u16", random_u16_data().iter());
    write_integer_generator!(group, "u32", random_u32_data().iter());
    write_integer_generator!(group, "u64", random_u64_data().iter());
    write_integer_generator!(group, "u128", random_u128_data().iter());

    write_integer_generator!(group, "i8", random_i8_data().iter());
    write_integer_generator!(group, "i16", random_i16_data().iter());
    write_integer_generator!(group, "i32", random_i32_data().iter());
    write_integer_generator!(group, "i64", random_i64_data().iter());
    write_integer_generator!(group, "i128", random_i128_data().iter());
}

fn chain_random(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("json:chain_random");
    group.measurement_time(Duration::from_secs(5));

    write_integer_generator!(group, "u8", chain(simple_u8_data().iter(), random_u8_data().iter()));
    write_integer_generator!(
        group,
        "u16",
        chain(simple_u16_data().iter(), random_u16_data().iter())
    );
    write_integer_generator!(
        group,
        "u32",
        chain(simple_u32_data().iter(), random_u32_data().iter())
    );
    write_integer_generator!(
        group,
        "u64",
        chain(simple_u64_data().iter(), random_u64_data().iter())
    );
    write_integer_generator!(
        group,
        "u128",
        chain(simple_u128_data().iter(), random_u128_data().iter())
    );
}

criterion_group!(simple_benches, simple);
criterion_group!(random_benches, random);
criterion_group!(chain_random_benches, chain_random);
criterion_main!(simple_benches, random_benches, chain_random_benches);
