#!/bin/bash
# Ensure formatting and clippy is done on nightly.

set -e

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/..

cargo +nightly fmt

cd lexical-asm
cargo +nightly fmt

cd ../lexical-size
cargo +nightly fmt

cd ../lexical-benchmark
cargo +nightly fmt
