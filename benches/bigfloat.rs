//! Measure the performance of the bigfloat parser.
//! Data must be non-negative, non-special floating-point numbers.

#[macro_use]
extern crate bencher;
extern crate lexical;

use bencher::{black_box, Bencher};
use lexical::atof::*;

// F32

const F32_DATA: [&'static str; 2] = ["1.7014118346046927e+38", "170141183460469250621729695946768384000"];

fn bigfloat_f32_lexical(bench: &mut Bencher) {
    bench.iter(|| { F32_DATA.iter().for_each(|x| unsafe {
        let first = x.as_ptr();
        let last = first.add(x.len());
        let (bigfloat, _) = Bigfloat::from_bytes(10, first, last);
        black_box(bigfloat.as_float::<f32>());
    })})
}

// F64

const F64_DATA: [&'static str; 2] = ["2.808895523222369e+306", "2808895523222368917686604633622079529188233041591539331521444526420434043771916119662550082894079617220372964810094217066950621375059876624667086135812280080428078132487487958048119593255470919674956589830984467943652626599596155679087859556560442277125192857671791932218094505800533594923639420624044032000"];

fn bigfloat_f64_lexical(bench: &mut Bencher) {
    bench.iter(|| { F64_DATA.iter().for_each(|x| unsafe {
        let first = x.as_ptr();
        let last = first.add(x.len());
        let (bigfloat, _) = Bigfloat::from_bytes(10, first, last);
        black_box(bigfloat.as_float::<f64>());
    })})
}

// MAIN

benchmark_group!(f32_benches, bigfloat_f32_lexical);
benchmark_group!(f64_benches, bigfloat_f64_lexical);
benchmark_main!(f32_benches, f64_benches);
