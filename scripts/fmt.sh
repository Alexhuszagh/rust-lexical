#!/bin/bash
# Ensure formatting and clippy is done on nightly.

set -e

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
script_home=$(realpath "${script_dir}")
home=$(dirname "${script_home}")
cd "${home}"

cargo +nightly fmt

cd "${home}/extras"
cargo +nightly fmt

cd "${home}/extras/asm"
cargo +nightly fmt

cd "${home}/extras/size"
cargo +nightly fmt

cd "${home}/extras/benchmark"
cargo +nightly fmt
