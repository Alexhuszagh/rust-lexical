#!/bin/bash

set -ex

# Check if we're not running from the project root.
config=.git/config
if [ ! -f "$config" ]; then
    >&2 echo "Error: script must be run from project root."
    exit 1
fi

# Run the float benches.
cargo bench --features=std,parse_floats,lemire,no_alloc --no-default-features

cd lexical-benchmark/lexical
cargo bench
