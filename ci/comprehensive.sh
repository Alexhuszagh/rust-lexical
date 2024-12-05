#!/bin/bash
# shellcheck disable=SC2086,SC2236
# Run a small subset of our comprehensive test suite.

set -ex

# Print our cargo version, for debugging.
cargo --version

# Change to our project home.
script_dir=$(dirname "${BASH_SOURCE[0]}")
script_home=$(realpath "${script_dir}")
home=$(dirname "${script_home}")
version="${CARGO_VERSION}"
cd "${home}"

# Ensure we have all our testing data files
git submodule update --init

run_tests() {
    # Test the parse-float correctness tests
    cd "${home}"
    cd lexical-parse-float/etc/correctness
    cargo ${version} run "${@}" --release --bin test-parse-golang
    cargo ${version} run "${@}" --release --bin test-parse-golang --features digit-separator
    cargo ${version} run "${@}" --release --bin test-parse-unittests

    # Test the write-float correctness tests.
    cd "${home}"
    cd lexical-write-float/etc/correctness
    cargo ${version} run "${@}" --release --bin shorter_interval
    cargo ${version} run "${@}" --release --bin random
    cargo ${version} run "${@}" --release --bin simple_random  -- --iterations 1000000
}

run_tests
run_tests --features=format
run_tests --all-features

cd "${home}"
if [ ! -z "${EXHAUSTIVE}" ]; then
# Test the parse-float correctness tests
    cd "${home}"
    cd lexical-parse-float/etc/correctness
    cargo ${version} run "${@}" --release --bin test-parse-random
fi
