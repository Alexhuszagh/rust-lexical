//! Sample data invoking the worst-case scenario.

#[macro_use]
extern crate bencher;
extern crate lexical;

use bencher::{black_box, Bencher};
use lexical::atof::*;

// F32

const F32_DATA: [&'static str; 2] = ["1.7014118346046927e+38", "170141183460469250621729695946768384000"];

fn atof_malicious_f32_lexical(bench: &mut Bencher) {
    bench.iter(|| { F32_DATA.iter().for_each(|x| { black_box(atof32_bytes(10, x.as_bytes())); } ) })
}

fn atof_malicious_f32_lexical_lossy(bench: &mut Bencher) {
    bench.iter(|| { F32_DATA.iter().for_each(|x| { black_box(atof32_lossy_bytes(10, x.as_bytes())); } ) })
}


fn atof_malicious_f32_parse(bench: &mut Bencher) {
    bench.iter(|| { F32_DATA.iter().for_each(|x| { black_box(x.parse::<f32>().unwrap()); } ) })
}

// F64

const F64_DATA: [&'static str; 2] = ["2.808895523222369e+306", "2808895523222368917686604633622079529188233041591539331521444526420434043771916119662550082894079617220372964810094217066950621375059876624667086135812280080428078132487487958048119593255470919674956589830984467943652626599596155679087859556560442277125192857671791932218094505800533594923639420624044032000"];
// Rust fails to parse this, so we need to use it only with Lexical.
const F64_DATA_2: &'static str = "2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125001e-324";

fn atof_malicious_f64_lexical(bench: &mut Bencher) {
    bench.iter(|| { F64_DATA.iter().for_each(|x| { black_box(atof32_bytes(10, x.as_bytes())); } ) })
}

fn atof_malicious_f64_lexical_lossy(bench: &mut Bencher) {
    bench.iter(|| { F64_DATA.iter().for_each(|x| { black_box(atof32_lossy_bytes(10, x.as_bytes())); } ) })
}

fn atof_malicious_f64_parse(bench: &mut Bencher) {
    bench.iter(|| { F64_DATA.iter().for_each(|x| { black_box(x.parse::<f64>().unwrap()); } ) })
}

fn atof_malicious_f64_lexical_2(bench: &mut Bencher) {
    bench.iter(|| { black_box(atof32_bytes(10, F64_DATA_2.as_bytes())); })
}

fn atof_malicious_f64_lexical_2_lossy(bench: &mut Bencher) {
    bench.iter(|| { black_box(atof32_lossy_bytes(10, F64_DATA_2.as_bytes())); })
}

// MAIN

benchmark_group!(f32_benches, atof_malicious_f32_lexical, atof_malicious_f32_lexical_lossy, atof_malicious_f32_parse);
benchmark_group!(f64_benches, atof_malicious_f64_lexical, atof_malicious_f64_lexical_lossy, atof_malicious_f64_parse);
benchmark_group!(f64_benches_2, atof_malicious_f64_lexical_2, atof_malicious_f64_lexical_2_lossy);
benchmark_main!(f32_benches, f64_benches, f64_benches_2);
