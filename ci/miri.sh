#!/bin/bash
# Run main test suite.

set -ex

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
cd "$script_dir"/..

# Print our cargo version, for debugging.
cargo --version

# Test our Miri logic
rustup component add --toolchain nightly miri &2 > /dev/null || true
cargo +nightly miri test --all-features
cargo +nightly miri test --features radix,format,write-integers,write-floats,parse-integers,parse-floats
