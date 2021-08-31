# Benchmarks

These benchmarks were run on an `Intel(R) Core(TM) i7-6560U CPU @ 2.20GHz` processor, on Linux 5.13.12/Fedora 34, and run against commit [5955fe3](https://github.com/Alexhuszagh/rust-lexical-experimental/commit/5955fe34ead65d94b57ff3caff14122bcdd48b02). The Rust compiler version was `rustc 1.55.0-nightly (b41936b92 2021-07-20)`. The exact code and data used to run the benchmark can be seen [here](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical-benchmark/write-float).

**JSON**

A benchmark on randomly-generated numbers from a JSON document.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/json.svg)

**Random Uniform**

A benchmark on uniform, randomly-generated floats.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_uniform.svg)

**Random Uniform32**

A benchmark on uniform, randomly-generated 32-bit integers as floats.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_uniform32.svg)

**Random Simple Int32**

A benchmark on randomly-generated floats that are simple 32-bit integers.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_simple_int32.svg)

**Random Simple Int64**

A benchmark on randomly-generated floats that are simple 64-bit integers.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_simple_int64.svg)

**Random 1/Rand32**

A benchmark on randomly-generated floats that valid of `1/u32(..)`, where `u32` is uniformly distributed.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_one_over_rand32.svg)

**Random BigInts**

A benchmark on randomly-generated floats that consistent of 3, consecutive uniform 64-bit integers.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_big_ints.svg)

**Random BigInt.Int**

A benchmark on randomly-generated floats that consistent of large integral and fractional component.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_big_int_dot_int.svg)
