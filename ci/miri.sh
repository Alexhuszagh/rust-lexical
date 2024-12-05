#!/bin/bash
# shellcheck disable=SC2086,SC2236
# Run main test suite.

set -ex

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
script_home=$(realpath "${script_dir}")
home=$(dirname "${script_home}")
version="${CARGO_VERSION}"
cd "${home}"

# Print our cargo version, for debugging.
cargo ${version} --version

# Ensure we have all our testing data files
git submodule update --init

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
cd lexical-parse-float/etc/correctness
cargo +nightly miri run $FEATURES --release --bin test-parse-golang
# NOTE: This is **extraordinarily slow, mostly because of how the data is parsed
# as TOML which makes loading it take forever.
if [ -z $COMPREHENSIVE ]; then
    cargo +nightly miri run $FEATURES --release --bin test-parse-unittests
fi

# we want comprehensive tests so let's do everything
# Test the write-float correctness tests.
cd "${home}"
cd lexical-write-float/etc/correctness
cargo +nightly miri run $FEATURES --release --bin random
cargo +nightly miri run $FEATURES --release --bin shorter_interval
# NOTE: This is **extraordinarily slow.
if [ -z $COMPREHENSIVE ]; then
    cargo +nightly miri run $FEATURES --release --bin simple_random
fi
