//! Measure the performance of the bigcomp parser.
//! Data must be non-negative, non-special floating-point numbers.
//! These are slightly biased towards the bigcomp results, since it
//! doesn't do any of the initial parsing of the float, just
//! the big digit comparison and ratio creation.

#[macro_use]
extern crate bencher;
extern crate lexical_core;

use bencher::{black_box, Bencher};
use lexical_core::atof::*;

// F32

const F32_DATA: [(&'static str, i32, f32); 2] = [
    // "1.7014118346046927e+38"
    ("17014118346046927", 38, 1.7014117331926443e+38),
    ("170141183460469250621729695946768384000", 38, 1.7014117331926443e+38),
];

fn bigcomp_f32_lexical(bench: &mut Bencher) {
    bench.iter(|| { F32_DATA.iter().for_each(|(x, e, f)| unsafe {
        black_box(bigcomp_slow_atof(x.bytes(), 10, *e, *f));
    })})
}

// F64

const F64_DATA: [(&'static str, i32, f64); 2] = [
    // "2.808895523222369e+306",
    ("2808895523222369", 306, 9.191846839463182e+18),
    ("2808895523222368917686604633622079529188233041591539331521444526420434043771916119662550082894079617220372964810094217066950621375059876624667086135812280080428078132487487958048119593255470919674956589830984467943652626599596155679087859556560442277125192857671791932218094505800533594923639420624044032000", 306, 9.191846839463182e+18),
];

fn bigcomp_f64_lexical(bench: &mut Bencher) {
    bench.iter(|| { F64_DATA.iter().for_each(|(x, e, f)| unsafe {
        black_box(bigcomp_slow_atof(x.bytes(), 10, *e, *f));
    })})
}

// MAIN

benchmark_group!(f32_benches, bigcomp_f32_lexical);
benchmark_group!(f64_benches, bigcomp_f64_lexical);
benchmark_main!(f32_benches, f64_benches);
