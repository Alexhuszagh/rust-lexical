#!/bin/bash
# Run profiling tools for custom targets and create an output.

set -ex

# Change to our project home.
script_dir=`dirname "${BASH_SOURCE[0]}"`
cd "$script_dir"

cargo build --bin "$1" --release
perf record -F 1000 "$script_dir"/../target/release/"$1"
perf report --hierarchy
