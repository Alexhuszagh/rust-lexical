# Getting Started

In order to build and test lexical, only a modern Rust toolchain (1.51+) is required. However, for reasons described below, we highly recommend you install a recent (1.55+) nightly toolchain.

```bash
cargo +nightly build
cargo +nightly test
```

# Code Structure

Lexical is broken up into compact, relatively isolated workspaces to separate functionality based on the numeric conversion, minimizing compile times and simplifying testing feature-dependent code. The workspaces are:

- [lexical-util](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical-util): Shared utilities for all workspaces.
- [lexical-parse-integer](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical-parse-integer): Parse integers from string.
- [lexical-parse-float](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical-parse-float): Parse floats from string.
- [lexical-write-integer](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical-write-integer): Write integers to string.
- [lexical-write-float](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical-write-float): Write floats to string.
- [lexical-core](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical-core): Public API for numeric conversion routines without requiring a system allocator.
- [lexical](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical): Public API for numeric conversion routines with a system allocator.

Functionality is generally made **public** to separate the tests from the implementation, although non-documented members is not stable, and any changes to this code is not considered a breaking change. Tests are separated from the actual implementation, and comprehensively test each individual component.

Furthermore, any unsafe code uses the following conventions:

1. Each unsafe function must contain a `# Safety` section.
2. Unsafe operations/calls in unsafe functions must be marked as unsafe, with their safety guarantees clearly documented via a `// SAFETY:` section.

# Dependencies

In order to fully test and develop lexical, a recent, nightly compiler along with following Rust dependencies is required:

- Clippy
- Rustfmt
- Miri
- Valgrind
- Tarpaulin
- Fuzz
- Count

These dependencies may be installed via:

```bash
rustup toolchain install nightly
rustup +nightly component add clippy
rustup +nightly component add rustfmt
rustup +nightly component add miri
cargo +nightly install cargo-valgrind
cargo +nightly install cargo-tarpaulin
cargo +nightly install cargo-fuzz
cargo +nightly install cargo-count
```

In addition, the following non-Rust dependencies must be installed:

- Python3.6+
- python-magic (python-magic-win64 on Windows)
- Valgrind

# Development Process

The [scripts](https://github.com/Alexhuszagh/rust-lexical/tree/main/scripts) directory contains numerous scripts for testing, fuzzing, analyzing, and formatting code. Since many development features are nightly-only, this ensures the proper compiler features are used. This requires a recent version of a nightly compiler (1.51.0+) installed via Rustup, which can be invoked as `cargo +nightly`.

- [asm.sh](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/asm.sh): Emit assembly for numeric conversion routines, to identify performance regression.
- [bench.sh](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/bench.sh): Check the benchmarks compile and run.
- [check.sh](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/check.sh): Check rustfmt and clippy without formatting any code.
- [fmt.sh](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/fmt.sh): Run `cargo fmt` and `cargo clippy` in all projects and workspaces, on nightly.
- [fuzz.sh](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/fuzz.sh): Run fuzzer for a given target.
- [hooks.sh](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/hooks.sh): Install formatting and lint hooks on commits.
- [link.sh](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/link.sh): Rebuild all symbolic links.
- [size.py](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/size.py): Calculate lexical binary sizes.
- [test.sh](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/test.sh): Run the test suite with Valgrind and Miri.
- [timings.py](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/timings.py): Plot build times.
- [unsafe.sh](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/unsafe.sh): Count lines of code and metrics of unsafe code usage.

Please run [fmt.sh](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/fmt.sh) before committing any code, ideally by installing the pre-commit hook via [hooks.sh](https://github.com/Alexhuszagh/rust-lexical/blob/main/scripts/hooks.sh).

All PRs must pass the following checks:

```bash
# Check all safety sections and other features are properly documented.
RUSTFLAGS="--deny warnings" cargo +nightly build --features=lint
# Ensure all rustfmt and clippy checks pass.
scripts/check.sh
# Ensure all tests pass with common feature combinations. 
# Miri is too slow, so skip those tests for most commits.
SKIP_MIRI=1 scripts/test.sh
```

# Safety

In order to ensure memory safety even when using unsafe features, we have the following requirements.

- All code with local unsafety must be marked as an `unsafe` function.
- All unsafe macros must have a `# Safety` section in the documentation.
- All unsafe functions must have a `# Safety` section in the documentation.
- All code using `unsafe` functionality must have a `// SAFETY:` section on the previous line, and must contain an `unsafe` block, even in `unsafe` functions.
- If multiple lines have similar safety guarantees, a `// SAFETY:` section can be used for a block or small segment of code.

In order to very that the safety guarantees are met, any changes to `unsafe` code must be fuzzed, the test suite must be run with Valgrind, and must pass the following commands:

```bash
# Ensure `unsafe` blocks are used within `unsafe` functions.
RUSTFLAGS="--deny warnings" cargo +nightly build --features=lint
# Ensure clippy checks pass for `# Safety` sections.
cargo +nightly clippy --all-features -- --deny warnings
```

# Algorithm Changes

Each workspace has a "docs" directory containing detailed descriptions of algorithms and benchmarks. If you make any substantial changes to an algorithm, you should both update the algorithm description and the provided benchmarks.
