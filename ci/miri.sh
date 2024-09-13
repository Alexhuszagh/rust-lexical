#!/bin/bash
# shellcheck disable=SC2086,SC2236
# Run main test suite.

set -ex

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
home=$(dirname "${script_dir}")
cd "${home}"

# Print our cargo version, for debugging.
cargo --version

# Test our Miri logic
rustup component add --toolchain nightly miri &2 > /dev/null || true

# these are our simple tests
cargo +nightly miri test --all-features
cargo +nightly miri test --features radix,format,write-integers,write-floats,parse-integers,parse-floats
cargo +nightly miri test --no-default-features --features compact,format,write-integers,write-floats,parse-integers,parse-floats

FEATURES=
if [ ! -z $ALL_FEATURES ]; then
    FEATURES=--all-features
fi

# we want comprehensive tests so let's do everything
# Test the write-float correctness tests.
cd "${home}"
cd lexical-write-float/etc/correctness
cargo run $FEATURES --release --bin shorter_interval
cargo run $FEATURES --release --bin random
cargo run $FEATURES --release --bin simple_random  -- --iterations 1000000
