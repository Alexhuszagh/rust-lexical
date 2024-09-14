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
cd "${home}"

run_tests() {
    # Test the parse-float correctness tests
    cd "${home}"
    cd lexical-parse-float/etc/correctness
    cargo run "${@}" --release --bin test-parse-golang
    cargo run "${@}" --release --bin test-parse-unittests

    # Test the write-float correctness tests.
    cd "${home}"
    cd lexical-write-float/etc/correctness
    cargo run "${@}" --release --bin shorter_interval
    cargo run "${@}" --release --bin random
    cargo run "${@}" --release --bin simple_random  -- --iterations 1000000
}

run_tests
run_tests --features=format
run_tests --all-features

cd "${home}"
if [ ! -z "${EXHAUSTIVE}" ]; then
    if [ -z "${PYTHON}" ]; then
        PYTHON=python
    fi
    $PYTHON "${home}/lexical-parse-float/etc/correctness/test-parse-random/runtests.py"
fi
