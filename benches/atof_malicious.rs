//! Sample data invoking the worst-case scenario.

extern crate criterion;
extern crate lexical_core;

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// BENCH GENERATORS

// Lexical atof generator.
macro_rules! lexical_generator {
    ($group:ident, $name:literal, $data:ident, $t:ty) => {
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $data.iter().for_each(|x| {
                    black_box(lexical_core::parse::<$t>(x.as_bytes()).unwrap());
                })
            })
        });
    };
}

// Parse atof generator.
macro_rules! parse_generator {
    ($group:ident, $name:literal, $data:ident, $t:ty) => {
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $data.iter().for_each(|x| {
                    black_box(x.parse::<$t>().unwrap());
                })
            })
        });
    };
}

// F32

const F32_DATA: [&'static str; 2] = [
    "1.7014118346046927e+38",
    "170141183460469250621729695946768384000",
];

// F64

const F64_DATA: [&'static str; 2] = ["2.808895523222369e+306", "2808895523222368917686604633622079529188233041591539331521444526420434043771916119662550082894079617220372964810094217066950621375059876624667086135812280080428078132487487958048119593255470919674956589830984467943652626599596155679087859556560442277125192857671791932218094505800533594923639420624044032000"];


fn lexical(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("lexical");
    group.measurement_time(Duration::from_secs(20));
    lexical_generator!(group, "atof_malicious_f32_lexical", F32_DATA, f32);
    lexical_generator!(group, "atof_malicious_f64_lexical", F64_DATA, f64);
}

fn parse(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("core::parse");
    group.measurement_time(Duration::from_secs(20));
    parse_generator!(group, "atof_malicious_f32_parse", F32_DATA, f32);
    parse_generator!(group, "atof_malicious_f64_parse", F64_DATA, f64);
}

// MAIN

criterion_group!(lexical_benches, lexical);
criterion_group!(parse_benches, parse);
criterion_main!(lexical_benches, parse_benches);
