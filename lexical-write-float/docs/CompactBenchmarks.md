# Compact Benchmarks

These benchmarks were run on an `Intel(R) Core(TM) i7-6560U CPU @ 2.20GHz` processor, on Linux 5.13.12/Fedora 34, and run against commit [961aefc](https://github.com/Alexhuszagh/rust-lexical-experimental/commit/961aefc5d7c1f4eb8b10c043a585644bc891c832). The Rust compiler version was `rustc 1.55.0-nightly (b41936b92 2021-07-20)`. The exact code and data used to run the benchmark can be seen [here](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical-benchmark/write-float).

**JSON**

A benchmark on randomly-generated numbers from a JSON document.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/json_features=compact.svg)

**Random Uniform**

A benchmark on uniform, randomly-generated floats.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_uniform_features=compact.svg)

**Random Uniform32**

A benchmark on uniform, randomly-generated 32-bit integers as floats.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_uniform32_features=compact.svg)

**Random Simple Int32**

A benchmark on randomly-generated floats that are simple 32-bit integers.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_simple_int32_features=compact.svg)

**Random Simple Int64**

A benchmark on randomly-generated floats that are simple 64-bit integers.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_simple_int64_features=compact.svg)

**Random 1/Rand32**

A benchmark on randomly-generated floats that valid of `1/u32(..)`, where `u32` is uniformly distributed.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_one_over_rand32_features=compact.svg)

**Random BigInts**

A benchmark on randomly-generated floats that consistent of 3, consecutive uniform 64-bit integers.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_big_ints_features=compact.svg)

**Random BigInt.Int**

A benchmark on randomly-generated floats that consistent of large integral and fractional component.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/random_big_int_dot_int_features=compact.svg)
