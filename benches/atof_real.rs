//! Sample, real dataset for atof benchmarks.

#[macro_use]
extern crate lazy_static;
extern crate criterion;
extern crate lexical_core;

use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

// Lexical atof generator.
macro_rules! lexical_generator {
    ($group:ident, $name:literal, $data:ident, $t:ty) => {
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                for line in $data.lines() {
                    for item in line.split(',') {
                        black_box(lexical_core::parse::<$t>(item.as_bytes()).unwrap());
                    }
                }
            })
        });
    };
}

// Parse atof generator.
macro_rules! parse_generator {
    ($group:ident, $name:literal, $data:ident, $t:ty) => {
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                for line in $data.lines() {
                    for item in line.split(',') {
                        black_box(item.parse::<$t>().unwrap());
                    }
                }
            })
        });
    };
}

// Benchmark to real data, downloaded from NASA Earth Observation.
// http://neo.sci.gsfc.nasa.gov/servlet/RenderData?si=1582435&cs=rgb&format=CSV&width=720&height=360

fn lexical(criterion: &mut Criterion) {
    let data = read_data();
    let mut group = criterion.benchmark_group("lexical");
    group.measurement_time(Duration::from_secs(5));
    lexical_generator!(group, "atof_real_f32_lexical", data, f32);
    lexical_generator!(group, "atof_real_f64_lexical", data, f64);
}


fn parse(criterion: &mut Criterion) {
    let data = read_data();
    let mut group = criterion.benchmark_group("core::parse");
    group.measurement_time(Duration::from_secs(5));
    parse_generator!(group, "atof_real_f32_parse", data, f32);
    parse_generator!(group, "atof_real_f64_parse", data, f64);
}

// MAIN

criterion_group!(lexical_benches, lexical);
criterion_group!(parse_benches, parse);
criterion_main!(lexical_benches, parse_benches);
