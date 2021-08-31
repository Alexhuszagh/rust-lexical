#!/bin/bash
# Ensure formatting and clippy is done on nightly.

set -e

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

# Make sure we error on warnings, and don't format in-place.

# Do the formatting and clippy for all our project workspaces.
cargo +nightly fmt -- --check
cargo +nightly clippy --no-default-features -- --deny warnings
cargo +nightly clippy --features=compact -- --deny warnings
cargo +nightly clippy --features=format,radix -- --deny warnings
cargo +nightly clippy --all-features -- --deny warnings

# ASM, size, and benchmarks use separate workspaces, do those separately.
cd lexical-asm
cargo +nightly fmt -- --check
cargo +nightly clippy --all-features -- --deny warnings

cd ../lexical-size
cargo +nightly fmt -- --check
cargo +nightly clippy --all-features -- --deny warnings

cd ../lexical-benchmark
cargo +nightly fmt -- --check
cargo +nightly clippy --all-features --benches -- --deny warnings
