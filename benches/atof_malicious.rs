//! Sample data invoking the worst-case scenario.

#[macro_use]
extern crate bencher;
extern crate lexical_core;

use bencher::{black_box, Bencher};
use lexical_core::*;

// BENCH GENERATORS

// Lexical atoi generator.
macro_rules! lexical_generator {
    ($name:ident, $data:ident, $cb:ident) => (
        fn $name(bench: &mut Bencher) {
            bench.iter(|| {
                $data.iter().for_each(|x| {
                    black_box($cb(x.as_bytes()).unwrap());
                })
            })
        }
    );
}

// Parse atoi generator.
macro_rules! parse_generator {
    ($name:ident, $data:ident, $t:tt) => (
        fn $name(bench: &mut Bencher) {
            bench.iter(|| {
                $data.iter().for_each(|x| {
                    black_box(x.parse::<$t>().unwrap());
                })
            })
        }
    );
}

// F32

const F32_DATA: [&'static str; 2] = ["1.7014118346046927e+38", "170141183460469250621729695946768384000"];

lexical_generator!(atof_malicious_f32_lexical, F32_DATA, atof32);
lexical_generator!(atof_malicious_f32_lexical_lossy, F32_DATA, atof32_lossy);
parse_generator!(atof_malicious_f32_parse, F32_DATA, f32);

// F64

const F64_DATA: [&'static str; 2] = ["2.808895523222369e+306", "2808895523222368917686604633622079529188233041591539331521444526420434043771916119662550082894079617220372964810094217066950621375059876624667086135812280080428078132487487958048119593255470919674956589830984467943652626599596155679087859556560442277125192857671791932218094505800533594923639420624044032000"];

lexical_generator!(atof_malicious_f64_lexical, F64_DATA, atof64);
lexical_generator!(atof_malicious_f64_lexical_lossy, F64_DATA, atof64_lossy);
parse_generator!(atof_malicious_f64_parse, F64_DATA, f64);

// MAIN

benchmark_group!(f32_benches, atof_malicious_f32_lexical, atof_malicious_f32_lexical_lossy, atof_malicious_f32_parse);
benchmark_group!(f64_benches, atof_malicious_f64_lexical, atof_malicious_f64_lexical_lossy, atof_malicious_f64_parse);
benchmark_main!(f32_benches, f64_benches);
