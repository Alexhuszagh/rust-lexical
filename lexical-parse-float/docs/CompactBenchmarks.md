# Compact Benchmarks

These benchmarks were run on an `Intel(R) Core(TM) i7-6560U CPU @ 2.20GHz` processor, on Linux 5.12.5/Fedora 34, and run against commit [961aefc](https://github.com/Alexhuszagh/rust-lexical-experimental/commit/961aefc5d7c1f4eb8b10c043a585644bc891c832). The Rust compiler version was `rustc 1.55.0-nightly (b41936b92 2021-07-20)`. The exact code and data used to run the benchmark can be seen [here](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical-benchmark/parse-float).

**Random**

A benchmark on randomly-generated numbers using different generator strategies.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-float/assets/random_features=compact.svg)

**Real**

A benchmark on float strings from real-world datasets, including NASA measurements, geolocation data, and more.

![Real Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-float/assets/real_features=compact.svg)

**Contrived**

Benchmarks on specially-crafted float strings to test performance of various different corner cases.

![Contrived Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-float/assets/contrived_features=compact.svg)

**Large**

Benchmarks on near-halfway, large float strings of increasing digit counts.

![Large Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-float/assets/large_features=compact.svg)

**Denormal**

Benchmarks on near-halfway, denormal float strings of increasing digit counts.

![Denormal Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-float/assets/denormal_features=compact.svg)
