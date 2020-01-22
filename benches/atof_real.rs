//! Sample, real dataset for atof benchmarks.

#[macro_use]
extern crate lazy_static;
extern crate criterion;
extern crate lexical_core;

use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_core::parse as lexical_parse;

// PATH

/// Return the `target/debug` or `target/release` directory path.
pub fn build_dir() -> PathBuf {
    env::current_exe()
        .expect("unittest executable path")
        .parent()
        .expect("unittest executable directory")
        .parent()
        .expect("debug/release directory")
        .to_path_buf()
}

/// Return the `target` directory path.
pub fn target_dir() -> PathBuf {
    build_dir()
        .parent()
        .expect("target directory")
        .to_path_buf()
}

/// Return the project directory path.
pub fn project_dir() -> PathBuf {
    target_dir()
        .parent()
        .expect("project directory")
        .to_path_buf()
}

/// Return the `data` directory path.
pub fn data_dir() -> PathBuf {
    let mut dir = project_dir();
    dir.push("data");
    dir
}

// SAMPLE DATA

fn read_data_impl() -> String {
    let mut path = data_dir();
    path.push("AURA_UVI_CLIM_M_2010-12-01_rgb_720x360.CSV");
    read_to_string(&path).unwrap()
}

fn read_data() -> &'static String {
    lazy_static! {
        static ref DATA: String = read_data_impl();
    }
    &DATA
}

// BENCHMARK GENERATORS

// Lexical atoi generator.
macro_rules! lexical_generator {
    ($name:ident, $t:ty) => (
        fn $name(criterion: &mut Criterion) {
            let data = read_data();
            criterion.bench_function(stringify!($name), |b| b.iter(|| {
                for line in data.lines() {
                    for item in line.split(',') {
                        black_box(lexical_parse::<$t>(item.as_bytes()).unwrap());
                    }
                }
            }));
        }
    );
}

// Parse atoi generator.
macro_rules! parse_generator {
    ($name:ident, $t:tt) => (
        fn $name(criterion: &mut Criterion) {
            let data = read_data();
            criterion.bench_function(stringify!($name), |b| b.iter(|| {
                for line in data.lines() {
                    for item in line.split(',') {
                        black_box(item.parse::<$t>().unwrap());
                    }
                }
            }));
        }
    );
}

// F32

// Benchmark to real data, downloaded from NASA Earth Observation.
// http://neo.sci.gsfc.nasa.gov/servlet/RenderData?si=1582435&cs=rgb&format=CSV&width=720&height=360

lexical_generator!(atof_real_f32_lexical, f32);
parse_generator!(atof_real_f32_parse, f32);

// F64

// Benchmark to real data, downloaded from NASA Earth Observation.
// http://neo.sci.gsfc.nasa.gov/servlet/RenderData?si=1582435&cs=rgb&format=CSV&width=720&height=360

lexical_generator!(atof_real_f64_lexical, f64);
parse_generator!(atof_real_f64_parse, f64);

// MAIN

criterion_group!(f32_benches, atof_real_f32_lexical, atof_real_f32_parse);
criterion_group!(f64_benches, atof_real_f64_lexical, atof_real_f64_parse);
criterion_main!(f32_benches, f64_benches);
