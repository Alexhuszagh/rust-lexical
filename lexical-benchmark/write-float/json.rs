mod input;

use core::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use lexical_util::constants::BUFFER_SIZE;
use lexical_write_float::ToLexical;
use serde::Deserialize;

#[derive(Deserialize)]
struct TestData {
    #[serde(rename = "f32")]
    f32_data: Vec<String>,

    #[serde(rename = "f64")]
    f64_data: Vec<String>,
}

fn json_data() -> &'static TestData {
    lazy_static! {
        static ref DATA: TestData = input::read_json("float.json");
    }
    &*DATA
}

// STATIC DATA

macro_rules! static_data {
    ($($fn:ident $f1:ident $t:tt ; )*) => ($(
        fn $fn() -> &'static [$t] {
            lazy_static! {
                static ref DATA: Vec<$t> = {
                    json_data()
                        .$f1
                        .iter()
                        .map(|x| x.parse::<$t>().unwrap())
                        .collect()
                };
            }
            &*DATA
        }
    )*)
}

static_data! {
    f32_data f32_data f32 ;
    f64_data f64_data f64 ;
}

// GENERATORS

macro_rules! lexical_generator {
    ($group:ident, $name:expr, $iter:expr) => {{
        let mut buffer: [u8; BUFFER_SIZE] = [b'0'; BUFFER_SIZE];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|&x| {
                    black_box(x.to_lexical(&mut buffer));
                })
            })
        });
    }};
}

macro_rules! dtoa_generator {
    ($group:ident, $name:expr, $iter:expr) => {{
        let mut buffer = vec![b'0'; 256];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|x| {
                    dtoa::write(&mut buffer, *x).unwrap();
                    black_box(&buffer);
                    unsafe {
                        buffer.set_len(0);
                    } // Way faster than Vec::clear().
                })
            })
        });
    }};
}

macro_rules! ryu_generator {
    ($group:ident, $name:expr, $iter:expr, $fmt:ident) => {{
        let mut buffer: [u8; 256] = [b'0'; 256];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|x| unsafe {
                    black_box(ryu::raw::$fmt(*x, buffer.as_mut_ptr()));
                })
            })
        });
    }};
}

macro_rules! fmt_generator {
    ($group:ident, $name:expr, $iter:expr) => {{
        use std::io::Write;
        let mut buffer = vec![b'0'; 256];
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                $iter.for_each(|x| {
                    black_box(buffer.write_fmt(format_args!("{}", x)).unwrap());
                    unsafe {
                        buffer.set_len(0);
                    } // Way faster than Vec::clear().
                })
            })
        });
    }};
}

macro_rules! generator {
    ($group:ident, $type:expr, $iter:expr, $fmt:ident) => {{
        lexical_generator!($group, concat!("write_", $type, "_lexical"), $iter);
        dtoa_generator!($group, concat!("write_", $type, "_dtoa"), $iter);
        ryu_generator!($group, concat!("write_", $type, "_ryu"), $iter, $fmt);
        fmt_generator!($group, concat!("write_", $type, "_fmt"), $iter);
    }};
}

// BENCHES

fn json(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("json");
    group.measurement_time(Duration::from_secs(5));

    generator!(group, "f32", f32_data().iter(), format32);
    generator!(group, "f64", f64_data().iter(), format64);
}

criterion_group!(json_benches, json);
criterion_main!(json_benches);
