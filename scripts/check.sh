#!/bin/bash
# Ensure formatting and clippy is done on nightly.

set -e

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

# Make sure we error on warnings, and don't format in-place.

# Do the formatting and clippy for all our project workspaces.
cargo +nightly fmt -- --check
cargo +nightly clippy -- --deny warnings

# ASM and benchmarks use separate workspaces, do those separately.
cd lexical-asm
cargo +nightly fmt -- --check
cargo +nightly clippy -- --deny warnings

cd ../lexical-benchmark
cargo +nightly fmt -- --check
cargo +nightly clippy --benches -- --deny warnings
