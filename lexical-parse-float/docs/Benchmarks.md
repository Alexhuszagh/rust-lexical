# Benchmarks

These benchmarks were run on an `Intel(R) Core(TM) i7-6560U CPU @ 2.20GHz` processor, on Linux 5.12.5/Fedora 34, and run against commit [a6dbf6d](https://github.com/Alexhuszagh/rust-lexical-experimental/commit/a6dbf6d6639758989f24d6750ee9711d29c9f6bd). The Rust compiler version was `rustc 1.55.0-nightly (b41936b92 2021-07-20)`. The exact code and data used to run the benchmark can be seen [here](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical-benchmark/parse-float).

**Random**

A benchmark on randomly-generated numbers using different generator strategies.

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-float/assets/random.svg)

**Real**

A benchmark on float strings from real-world datasets, including NASA measurements, geolocation data, and more.

![Real Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-float/assets/real.svg)

**Contrived**

Benchmarks on specially-crafted float strings to test performance of various different corner cases.

![Contrived Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-float/assets/contrived.svg)

**Large**

Benchmarks on near-halfway, large float strings of increasing digit counts.

![Large Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-float/assets/large.svg)

**Denormal**

Benchmarks on near-halfway, denormal float strings of increasing digit counts.

![Denormal Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-float/assets/denormal.svg)
