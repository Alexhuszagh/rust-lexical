#!/bin/bash
# Build and run benchmarks using the default test harness.

set -ex

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
script_home=$(realpath "${script_dir}")
home=$(dirname "${script_home}")
cd "${home}/extras/benchmark"

cargo test --bench '*'
