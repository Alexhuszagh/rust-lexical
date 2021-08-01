#!/bin/bash
# Ensure formatting and clippy is done on nightly.

set -e

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

# Do the formatting and clippy for all our project workspaces.
cargo +nightly fmt
cargo +nightly clippy

# ASM and benchmarks use separate workspaces, do those separately.
cd lexical-asm
cargo +nightly fmt
cargo +nightly clippy

cd ../lexical-benchmark
cargo +nightly fmt
cargo +nightly clippy
