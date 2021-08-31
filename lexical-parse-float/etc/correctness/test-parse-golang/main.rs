// Copyright 2021, Alex Huszagh. Unlicensed.
// See https://unlicense.org/

use lexical_parse_float::FromLexical;

use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, fs, io};

// PATH

/// Return the `target/debug` or `target/release` directory path.
pub fn build_dir() -> PathBuf {
    env::current_exe()
        .expect("unittest executable path")
        .parent()
        .expect("debug/release directory")
        .to_path_buf()
}

/// Return the `target` directory path.
pub fn target_dir() -> PathBuf {
    build_dir().parent().expect("target directory").to_path_buf()
}

/// Return the project directory path.
pub fn project_dir() -> PathBuf {
    target_dir().parent().expect("project directory").to_path_buf()
}

/// Return the `data` directory path.
pub fn data_dir() -> PathBuf {
    let mut dir = project_dir();
    dir.push("test-parse-golang");
    dir.push("parse-number-fxx-test-data");
    dir.push("data");
    dir
}

fn run_test(line: &str) {
    // Tests have the following format:
    //      hhhh ssssssss dddddddddddddddddd ....
    // The `hhhh` part is the hexadecimal representation for f16,
    // the `ssssssss` part is the hexadecimal representation of f32,
    // the `dddddddddddddddddd` is the hex representation of f64,
    // and the remaining bytes are the string to parse.
    let hex32 = line[5..13].to_lowercase();
    let hex64 = line[14..30].to_lowercase();
    let string = &line[31..];

    let float32 = f32::from_lexical(string.as_bytes()).unwrap();
    let float64 = f64::from_lexical(string.as_bytes()).unwrap();
    assert_eq!(hex32, format!("{:0>8x}", float32.to_bits()));
    assert_eq!(hex64, format!("{:0>16x}", float64.to_bits()));
}

fn main() {
    // Iterate over all .txt files in the directory.
    let paths = fs::read_dir(data_dir()).expect("Please update the Git submodule");
    for direntry in paths {
        let path = direntry.unwrap().path();
        if path.extension().unwrap() == "txt" {
            // Have a data file, parse and run the tests.
            let filename = path.file_name().unwrap().to_str().unwrap();
            println!("Running Test: {}", filename);
            let file = fs::File::open(path).unwrap();
            let reader = io::BufReader::new(file);
            let mut count: usize = 0;
            for line in reader.lines() {
                let line = line.unwrap();
                run_test(&line);
                count += 1;
            }
            println!("Ran {} tests.", count);
        }
    }
}
