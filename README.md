lexical-experimental
====================

This is an experimental branch of [lexical](https://github.com/Alexhuszagh/rust-lexical), with a focus on fast compile times, optimal performance, and modularization. Each subcomponent, such as integer parsing, float parsing, will be broken in workspaces.

Currently, the various algorithm designs and benchmarks can be seen markdown files in each respective crate. The current crates are:

- lexical-util (shared utilities for all crates).
- lexical-parse-integer (parse integers from string).
- lexical-parse-float (parse floats from string).
- lexical-write-integer (write integers to string).
- lexical-write-float (write floats to string).
- lexical-core (meta-crate for all specific parsers).
- lexical (high-level wrappers when a system allocator exists).

# Documentation

Extensive documentation is currently found in the [docs](/docs) subdirectory, as well as the `docs` subdirectory of each workspace. Each subdirectory contains a `Algorithm.md`, which explains algorithm design and implementation, and `Benchmarks.md` which shows the performance of the algorithm compared to other, known implementations. Any additional design considerations will be found there as well.

# Code Structure

The project is split into compact, relatively isolated workspaces to enable fast compile times. Functionality is generally made **public**, although any non-documented members are not guaranteed to be stable. Tests are separated from the actual module, and comprehensively test each individual component.

Furthermore, unsafety uses the following conventions:

1. Each unsafe function must contain a `# Safety` section.
2. Allow unsafe operations/calls in an unsafe function must be marked as unsafe, with their safety guarantees clearly documented via a `// SAFETY:` section.

# Code Size vs. Performance

This implementation also places a heavy focus on code bloat: with algorithms both optimized for performance and size. By default, this focuses on performance, however, using the `compact` feature, you can also opt-in to reduced code size at the cost of performance. The compact algorithms minimize the use of pre-computed tables and other optimizations at the cost of performance.

# Developing

The [scripts](/scripts) directory contains numerous scripts for testing, fuzzing, analyzing, and formatting code. Since many development features are nightly-only, this ensures the proper compiler features are used.

- `asm.sh`: Emit assembly for common functionality.
- `fmt.sh`: Run `cargo fmt` and `cargo clippy` in all projects and workspaces, on nightly.
- `fuzz.sh`: Run fuzzer for a given target.
- `hooks.sh`: Install formatting and lint hooks on commits.
- `test.sh`: Run the test suite with Valgrind and Miri.

Please run `scripts/fmt.sh` before committing any code, ideally by installing the pre-commit hook via `scripts/hooks.sh`.

# License

Lexical is dual licensed under the Apache 2.0 license as well as the MIT license. See the LICENCE-MIT and the LICENCE-APACHE files for the licenses.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in lexical by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
