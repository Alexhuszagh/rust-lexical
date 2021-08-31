#!/bin/bash
# Build and run benchmarks using the default test harness.

set -ex

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"/../lexical-benchmark

cargo test --bench '*'
