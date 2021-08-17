//! Benchmark sample floats meant to invoke certain code paths.

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lexical_parse_float::FromLexical;

macro_rules! core_generator {
    ($group:ident, $name:literal, $str:ident, $t:ty) => {
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                black_box($str.parse::<$t>().unwrap());
            })
        });
    };
}

macro_rules! lexical_generator {
    ($group:ident, $name:literal, $str:ident, $t:ty) => {
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                black_box(<$t>::from_lexical($str.as_bytes()).unwrap());
            })
        });
    };
}

// FLOATS

// NOTE: Rust currently doesn't handle large, denormal floats
// with more than 25 significant digits. Use the 25 significant
// digits for both large and denormal.

// Example fast-path value.
const FAST_PATH: &str = "1.2345e22";
// Example disguised fast-path value.
const DISGUISED_FAST_PATH: &str = "1.2345e30";
// Example moderate path value: clearly not halfway `1 << 53`.
const MODERATE_PATH: &str = "9007199254740992.0";
// Example exactly-halfway value `(1<<53) + 1`.
const HALFWAY: &str = "9007199254740993.0";
// Example large, near-halfway value.
const LARGE: &str = "8.988465674311580536566680e307";
// Example long, large, near-halfway value.
const LARGE_LONG: &str = "8.9884656743115805365666807213050294962762414131308158973971342756154045415486693752413698006024096935349884403114202125541629105369684531108613657287705365884742938136589844238179474556051429647415148697857438797685859063890851407391008830874765563025951597582513936655578157348020066364210154316532161708032e307";
// Example denormal, near-halfway value.
const DENORMAL: &str = "8.442911973260991817129021e-309";
// Example of a long, denormal, near-halfway value.
const DENORMAL_LONG: &str = "2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328124999e-324";

fn core(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("core");
    group.measurement_time(Duration::from_secs(5));
    core_generator!(group, "fast", FAST_PATH, f64);
    core_generator!(group, "disguised", DISGUISED_FAST_PATH, f64);
    core_generator!(group, "moderate", MODERATE_PATH, f64);
    core_generator!(group, "halfway", HALFWAY, f64);
    core_generator!(group, "large", LARGE, f64);
    core_generator!(group, "large_long", LARGE_LONG, f64);
    core_generator!(group, "denormal", DENORMAL, f64);
    core_generator!(group, "denormal_long", DENORMAL_LONG, f64);
}

fn lexical(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("lexical");
    group.measurement_time(Duration::from_secs(5));
    lexical_generator!(group, "fast", FAST_PATH, f64);
    lexical_generator!(group, "disguised", DISGUISED_FAST_PATH, f64);
    lexical_generator!(group, "moderate", MODERATE_PATH, f64);
    lexical_generator!(group, "halfway", HALFWAY, f64);
    lexical_generator!(group, "large", LARGE, f64);
    lexical_generator!(group, "large_long", LARGE_LONG, f64);
    lexical_generator!(group, "denormal", DENORMAL, f64);
    lexical_generator!(group, "denormal_long", DENORMAL_LONG, f64);
}

// MAIN

criterion_group!(core_benches, core);
criterion_group!(lexical_benches, lexical);
criterion_main!(core_benches, lexical_benches);
