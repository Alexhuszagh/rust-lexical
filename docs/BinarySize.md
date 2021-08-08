# Binary Size

The binary size comparisons were run on an `Intel(R) Core(TM) i7-6560U CPU @ 2.20GHz` processor, on Linux 5.12.5/Fedora 34, and run against commit [c858a0e](https://github.com/Alexhuszagh/rust-lexical-experimental/commit/c858a0ee9ed841a1d95f55eaf746f8c87e25f7bc). The Rust compiler version was `rustc 1.55.0-nightly (b41936b92 2021-07-20)`.

Each binary is generated using all optimization levels, and includes the result before and after stripping, with the core functionality black-boxed, to ensure optimization does not optimize-out the result.

**Empty**

This is the base size, in bytes, of an executable with no code body.
