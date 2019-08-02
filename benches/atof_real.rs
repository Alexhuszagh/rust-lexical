//! Sample, real dataset for atof benchmarks.

#[macro_use]
extern crate bencher;
#[macro_use]
extern crate lazy_static;
extern crate lexical_core;

use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

use bencher::{black_box, Bencher};
use lexical_core::*;

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

// F32

// Benchmark to real data, downloaded from NASA Earth Observation.
// http://neo.sci.gsfc.nasa.gov/servlet/RenderData?si=1582435&cs=rgb&format=CSV&width=720&height=360

fn atof_real_f32_lexical(bench: &mut Bencher) {
    let data = read_data();
    bench.iter(|| {
        for line in data.lines() {
            for item in line.split(',') {
                black_box(atof32_slice(item.as_bytes()).unwrap());
            }
        }
    })
}

fn atof_real_f32_parse(bench: &mut Bencher) {
    let data = read_data();
    bench.iter(|| {
        for line in data.lines() {
            for item in line.split(',') {
                black_box(item.parse::<f32>().unwrap());
            }
        }
    })
}

// F64

// Benchmark to real data, downloaded from NASA Earth Observation.
// http://neo.sci.gsfc.nasa.gov/servlet/RenderData?si=1582435&cs=rgb&format=CSV&width=720&height=360

fn atof_real_f64_lexical(bench: &mut Bencher) {
    let data = read_data();
    bench.iter(|| {
        //let line = data.lines().nth(45).unwrap();
        for line in data.lines().skip(60).take(20) {
            for item in line.split(',') {
                black_box(atof64_slice(item.as_bytes()).unwrap());
            }
        }
    })
}

fn atof_real_f64_parse(bench: &mut Bencher) {
    let data = read_data();
    bench.iter(|| {
        //let line = data.lines().nth(45).unwrap();
        for line in data.lines().skip(60).take(20) {
            for item in line.split(',') {
                black_box(item.parse::<f64>().unwrap());
            }
        }
    })
}

// MAIN

benchmark_group!(f32_benches, atof_real_f32_lexical, atof_real_f32_parse);
benchmark_group!(f64_benches, atof_real_f64_lexical, atof_real_f64_parse);
benchmark_main!(f32_benches, f64_benches);
